#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate libnss;

use libnss::interop::Response;
use libnss::passwd::{Passwd, PasswdHooks};
use libnss::shadow::{Shadow, ShadowHooks};

use radius_virtual::prelude::*;

struct VirtualPasswd;
libnss_passwd_hooks!(radius_virtual, VirtualPasswd);

impl PasswdHooks for VirtualPasswd {
    fn get_entry_by_name(name: String) -> Response<Passwd> {
        let user = lookup(&name);
        match user {
            None => Response::NotFound,
            Some(user) => Response::Success(Passwd {
                name: user.config.username.clone(),
                passwd: "x".to_string(),
                uid: user.config.uid,
                gid: user.config.gid,
                gecos: format!(
                    "Mapped RADIUS account {}->{}",
                    name, user.config.username
                )
                .to_string(),
                dir: user.config.home,
                shell: user.config.shell,
            }),
        }
    }
    fn get_all_entries() -> Response<Vec<Passwd>> {
        Response::Success(vec![])
    }
    fn get_entry_by_uid(_uid: libc::uid_t) -> Response<Passwd> {
        Response::NotFound
    }
}

struct VirtualShadow;
libnss_shadow_hooks!(radius_virtual, VirtualShadow);

impl ShadowHooks for VirtualShadow {
    fn get_entry_by_name(name: String) -> Response<Shadow> {
        let user = lookup(&name);
        match user {
            None => Response::NotFound,
            Some(user) => Response::Success(Shadow {
                name: user.config.username,
                passwd: "!".to_string(),
                last_change: -1,
                change_min_days: -1,
                change_max_days: -1,
                change_warn_days: -1,
                change_inactive_days: -1,
                expire_date: -1,
                reserved: 0,
            }),
        }
    }
    fn get_all_entries() -> Response<Vec<Shadow>> {
        Response::Success(vec![])
    }
}

fn lookup<S: Into<String>>(name: S) -> Option<db::User> {
    let config = Config::system();

    let config = match config {
        Ok(config) => config,
        _ => return None,
    };

    let db = Db::with_config(&config);

    let mut db = match db {
        Ok(db) => db,
        _ => return None,
    };
    db.get_user(name).ok()
}
