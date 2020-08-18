use crate::config::Config;
use crate::error::*;
use crate::user::User;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sqlite::{Connection, Value};
use std::time::{SystemTime, UNIX_EPOCH};

const VERSION: &str = "1";

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn with_config(config: &Config) -> Result<Self, Error> {
        let path = &config.db.path;

        let flags = sqlite::OpenFlags::new()
            .set_create()
            .set_full_mutex()
            .set_read_write();

        let db = Db {
            conn: Connection::open_with_flags(path, flags)?,
        };

        if cfg!(unix) {
            use std::fs::Permissions;
            use std::os::unix::fs::PermissionsExt;
            let perms = Permissions::from_mode(0o600);
            std::fs::set_permissions(path, perms)?;
        }

        let mut version_ok = true;

        db.conn.iterate("PRAGMA user_version", |pairs| {
            for &(_column, value) in pairs.iter() {
                let value = value.unwrap();
                if value != "0" && value != VERSION {
                    version_ok = false;
                }
            }
            true
        })?;

        if version_ok == false {
            return Err(Error::IncompatibleDbVersion);
        }
        db.conn
            .execute(format!("PRAGMA user_version = {}", VERSION))?;

        db.conn.execute(
            "
            CREATE TABLE IF NOT EXISTS users (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              username TEXT,
              last_login INTEGER,
              serialized_user BLOB,
              cookie TEXT
            );
            CREATE UNIQUE INDEX IF NOT EXISTS users_unique
            ON users(username);
            ",
        )?;
        Ok(db)
    }

    pub fn store_user(&mut self, user: &User) -> Result<String, Error> {
        let cookie: String =
            thread_rng().sample_iter(&Alphanumeric).take(32).collect();

        let mut stm = self
            .conn
            .prepare(
                "INSERT INTO users
                (username, last_login, serialized_user, cookie)
                VALUES (?, ?, ?, ?)
                ON CONFLICT (username)
                DO UPDATE SET
                last_login=excluded.last_login,
                serialized_user=excluded.serialized_user,
                cookie=excluded.cookie
                ",
            )?
            .cursor();

        let buf = serde_cbor::to_vec(&user)?;

        stm.bind(&[
            Value::String(user.radius.username.clone()),
            Value::Integer(now()),
            Value::Binary(buf),
            Value::String(cookie.clone()),
        ])?;

        stm.next()?;
        Ok(cookie)
    }

    pub fn get_user<S: Into<String>>(
        &self,
        username: S,
    ) -> Result<User, Error> {
        let mut stm = self
            .conn
            .prepare("SELECT (serialized_user) FROM users WHERE username = ?")?
            .cursor();

        stm.bind(&[Value::String(username.into())])?;

        self.run_user_query(stm)
    }

    pub fn get_user_with_cookie<S: Into<String>>(
        &self,
        username: S,
        cookie: S,
    ) -> Result<User, Error> {
        let mut stm = self
            .conn
            .prepare("SELECT (serialized_user) FROM users WHERE username = ? AND cookie = ?")?
            .cursor();

        stm.bind(&[
            Value::String(username.into()),
            Value::String(cookie.into()),
        ])?;

        self.run_user_query(stm)
    }

    fn run_user_query(&self, stm: sqlite::Cursor) -> Result<User, Error> {
        let mut stm = stm;

        if let Some(row) = stm.next().unwrap_or(None) {
            let data = row[0].as_binary().ok_or_else(|| Error::UserNotFound)?;
            let user: User = serde_cbor::from_slice(data)?;
            Ok(user)
        } else {
            Err(Error::UserNotFound)
        }
    }
}

fn now() -> i64 {
    let start = SystemTime::now();
    let now = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    now.as_secs() as i64
}
