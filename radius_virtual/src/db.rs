use crate::config::Config;
use crate::user::User;
use crate::error::*;
use sqlite::Connection;

pub struct Db {
    conn: Connection
}


impl Db {
    pub fn with_config(config: &Config) -> Result<Self, Error> {
        let path = &config.db.path;

        let flags = sqlite::OpenFlags::new()
            .set_create()
            .set_full_mutex()
            .set_read_write();

        let db = Db {
            conn: Connection::open_with_flags(path, flags)?
        };

        db.conn.execute(
            "
            CREATE TABLE IF NOT EXISTS users (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              username TEXT,
              last_login INTEGER,
              serialized_user BLOB
            )
            "
        )?;
        Ok(db)
    }

    pub fn store_user(&mut self, user: &User) -> Result<(), Error> {
        Ok(())
    }
}
