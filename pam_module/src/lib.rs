#[macro_use]
extern crate log;

#[macro_use]
extern crate pamsm;

use pamsm::{Pam, PamError, PamFlag, PamLibExt, PamServiceModule};

use nss_db::setup_log;
use nss_db::Config;
use nss_db::Db;
use radius::Client;
use radius::Credentials;
use radius::Error;


const SYSLOG_NAME: &str = "pam_radius_virtual";

struct PamTime;

impl PamServiceModule for PamTime {
    fn authenticate(
        pamh: Pam,
        _flags: PamFlag,
        _args: Vec<String>,
    ) -> PamError {
        setup_log(SYSLOG_NAME);

        let username = match pamh.get_user(None) {
            Ok(Some(u)) => u,
            Ok(None) => return PamError::USER_UNKNOWN,
            Err(e) => return e,
        };

        let username = match username.to_str() {
            Ok(u) => u,
            _ => return PamError::USER_UNKNOWN,
        };

        let pass = match pamh.get_authtok(None) {
            Ok(Some(p)) => p,
            Ok(None) => return PamError::AUTH_ERR,
            Err(e) => return e,
        };

        let pass = match pass.to_str() {
            Ok(p) => p,
            _ => return PamError::AUTH_ERR,
        };

        let config = Config::system();

        let config = match config {
            Ok(config) => config,
            _ => return PamError::SERVICE_ERR,
        };

        let client = Client::with_config(&config.radius);

        let client = match client {
            Ok(client) => client,
            _ => return PamError::SERVICE_ERR,
        };

        let db = Db::with_config(&config);

        let mut db = match db {
            Ok(db) => db,
            _ => return PamError::SERVICE_ERR,
        };

        let cred = Credentials::with_username_password(username, pass);

        let res = client.authenticate(&cred);

        let radius_user = match res {
            Ok(user) => user,
            Err(Error::AuthReject) => return PamError::AUTH_ERR,
            _ => return PamError::SERVICE_ERR,
        };

        let res = config.map_user(&radius_user);

        let user = match res {
            Some(user) => user,
            _ => return PamError::AUTH_ERR,
        };

        let res = db.store_user(&user);


        let cookie = match res {
            Ok(s) => s,
            _ => return PamError::SERVICE_ERR,
        };

        let res = pamh.putenv(&format!("RADIUS_USER={}", username));
        if let Err(_) = res  {
            return PamError::SERVICE_ERR;
        }

        let res = pamh.putenv(&format!("RADIUS_USER_COOKIE={}", cookie));
        if let Err(_) = res  {
            return PamError::SERVICE_ERR;
        }


        match res {
            Ok(_) => return PamError::SUCCESS,
            _ => return PamError::SERVICE_ERR,
        };
    }

    fn setcred(pamh: Pam, _flags: PamFlag, _args: Vec<String>) -> PamError {
        setup_log(SYSLOG_NAME);

        let config = Config::system();

        let config = match config {
            Ok(config) => config,
            _ => return PamError::SERVICE_ERR,
        };


        let db = Db::with_config(&config);

        let mut db = match db {
            Ok(db) => db,
            _ => return PamError::SERVICE_ERR,
        };

        let user = match pamh.get_user(None) {
            Ok(Some(u)) => u,
            Ok(None) => return PamError::USER_UNKNOWN,
            Err(e) => return e,
        };

        let user = match user.to_str() {
            Ok(u) => u,
            _ => return PamError::USER_UNKNOWN,
        };

        let user = db.get_user(user);

        match user {
            Ok(_user) => PamError::SUCCESS,
            _ => PamError::AUTH_ERR
        }
    }
}

pam_module!(PamTime);
