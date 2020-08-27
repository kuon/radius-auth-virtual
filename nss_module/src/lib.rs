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

const SYSLOG_NAME: &str = "nss_radius_virtual";

struct VirtualPasswd;
libnss_passwd_hooks!(radius_virtual, VirtualPasswd);

impl PasswdHooks for VirtualPasswd {
    fn get_entry_by_name(name: String) -> Response<Passwd> {
        if name == "root" {
            return Response::NotFound;
        }

        let root = nix::unistd::Uid::from_raw(0);
        if nix::unistd::geteuid() != root {
            return Response::NotFound;
        }

        setup_log(SYSLOG_NAME);

        let config = match Config::system() {
            Ok(config) => config,
            Err(err) => {
                error!("Cannot read system configuration: {}", err);
                return Response::Unavail;
            }
        };

        if config.debug() {
            log::set_max_level(log::LevelFilter::Debug)
        }

        debug!("Looking up user {}", name);

        let db = match Db::with_config(&config) {
            Ok(db) => db,
            Err(err) => {
                error!("Cannot read user database: {}", err);
                return Response::Unavail;
            }
        };

        let user = db.get_user(&name).ok();

        match user {
            None => {
                debug!(
                    "Didn't find any user for {}, \
                         fallling back to default user {}",
                    name, config.mapping.default_user.username
                );
                Response::Success(Passwd {
                    name: config.mapping.default_user.username.clone(),
                    passwd: "x".to_string(),
                    uid: config.mapping.default_user.uid,
                    gid: config.mapping.default_user.gid,
                    gecos: "Radius default user".to_string(),
                    dir: config.mapping.default_user.home.clone(),
                    shell: config.mapping.default_user.shell.clone(),
                })
            }
            Some(user) => {
                debug!(
                    "Found user {}, mapped to {}",
                    name, user.mapping.username
                );
                Response::Success(Passwd {
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
                    shell: config.mapping.default_user.shell.clone(),
                })
            }
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
        if name == "root" {
            return Response::NotFound;
        }

        let config = match Config::system() {
            Ok(config) => config,
            _ => return Response::Unavail,
        };

        let db = match Db::with_config(&config) {
            Ok(db) => db,
            _ => return Response::Unavail,
        };

        let user = db.get_user(name).ok();
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
