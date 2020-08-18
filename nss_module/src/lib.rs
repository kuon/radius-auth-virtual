
#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate libnss;

use libnss::interop::Response;
use libnss::passwd::{Passwd, PasswdHooks};
use libnss::shadow::{Shadow, ShadowHooks};

use nss_db::setup_log;
use nss_db::Config;
use nss_db::Db;
use nss_db::User;


const SYSLOG_NAME: &str = "nss_radius_virtual";
const RADIUS_SHELL: &str = "/usr/bin/radius_shell";

struct VirtualPasswd;
libnss_passwd_hooks!(radius_virtual, VirtualPasswd);

impl PasswdHooks for VirtualPasswd {
    fn get_entry_by_name(name: String) -> Response<Passwd> {
        setup_log(SYSLOG_NAME);
        let user = lookup(&name);
        match user {
            None => Response::Success(Passwd {
                name: "normaluser".to_string(),
                passwd: "x".to_string(),
                uid: 1011,
                gid: 1011,
                gecos: "NON EXISTENT".to_string(),
                dir: "/tmp".to_string(),
                shell: RADIUS_SHELL.to_string()
            }),
            Some(user) => Response::Success(Passwd {
                name: user.mapping.username.clone(),
                passwd: "x".to_string(),
                uid: user.mapping.uid,
                gid: user.mapping.gid,
                gecos: format!(
                    "Mapped RADIUS account {}->{}",
                    name, user.mapping.username
                )
                .to_string(),
                dir: user.mapping.home,
                shell: RADIUS_SHELL.to_string()
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
                name: user.mapping.username,
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

fn lookup<S: Into<String>>(name: S) -> Option<User> {
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
