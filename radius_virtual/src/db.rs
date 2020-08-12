use crate::config::Config;
use crate::error::*;
use crate::user::User;
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

        let mut version_ok = true;

        db.conn.iterate("PRAGMA user_version", |pairs| {
            for &(column, value) in pairs.iter() {
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
              serialized_user BLOB
            );
            CREATE UNIQUE INDEX IF NOT EXISTS users_unique
            ON users(username);
            ",
        )?;
        Ok(db)
    }

    pub fn store_user(&mut self, user: &User) -> Result<(), Error> {
        let mut stm = self
            .conn
            .prepare(
                "INSERT INTO users
                (username, last_login, serialized_user)
                VALUES (?, ?, ?)
                ON CONFLICT (username)
                DO UPDATE SET
                last_login=excluded.last_login,
                serialized_user=excluded.serialized_user
                ",
            )?
            .cursor();

        let buf = serde_cbor::to_vec(&user)?;

        stm.bind(&[
            Value::String(user.username.clone()),
            Value::Integer(now()),
            Value::Binary(buf),
        ])?;

        stm.next()?;
        Ok(())
    }

    pub fn get_user<S: Into<String>>(
        &mut self,
        username: S,
    ) -> Result<User, Error> {
        let mut stm = self
            .conn
            .prepare("SELECT (serialized_user) FROM users WHERE username = ?")?
            .cursor();

        stm.bind(&[
            Value::String(username.into()),
        ])?;

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
