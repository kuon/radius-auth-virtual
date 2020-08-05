#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate libnss;

use libnss::interop::Response;
use libnss::passwd::{Passwd, PasswdHooks};
use libnss::shadow::{Shadow, ShadowHooks};

struct VirtualPasswd;
libnss_passwd_hooks!(radius_virtual, VirtualPasswd);

impl PasswdHooks for VirtualPasswd {
    fn get_entry_by_name(name: String) -> Response<Passwd> {
        if name == "radiususer" {
            return Response::Success(Passwd {
                name: "normaluser".to_string(),
                passwd: "x".to_string(),
                uid: 1011,
                gid: 1011,
                gecos: "Test Account".to_string(),
                dir: "/home/normaluser".to_string(),
                shell: "/bin/bash".to_string(),
            });
        }

        Response::NotFound
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
        if name == "radiususer" {
            return Response::Success(Shadow {
                name: "normaluser".to_string(),
                passwd: "!".to_string(),
                last_change: -1,
                change_min_days: -1,
                change_max_days: -1,
                change_warn_days: -1,
                change_inactive_days: -1,
                expire_date: -1,
                reserved: 0,
            });
        }
        Response::NotFound
    }
    fn get_all_entries() -> Response<Vec<Shadow>> {
        Response::Success(vec![])
    }
}