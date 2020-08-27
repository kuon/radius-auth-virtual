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

struct PamRadius;

impl PamServiceModule for PamRadius {
    fn authenticate(
        pamh: Pam,
        _flags: PamFlag,
        _args: Vec<String>,
    ) -> PamError {
        setup_log(SYSLOG_NAME);

        let config = match Config::system() {
            Ok(config) => config,
            Err(err) => {
                error!("Cannot read configuration file: {}", err);
                return PamError::SERVICE_ERR;
            }
        };

        if config.debug() {
            log::set_max_level(log::LevelFilter::Debug)
        }

        let username = match pamh.get_user(None) {
            Ok(Some(u)) => u,
            Ok(None) => {
                error!("Cannot get username");
                return PamError::USER_UNKNOWN;
            }
            Err(e) => return e,
        };

        let username = match username.to_str() {
            Ok(u) => u,
            _ => {
                error!("Cannot convert username to string");
                return PamError::USER_UNKNOWN;
            }
        };

        debug!("Got username {}", username);

        if username == "root" {
            return PamError::USER_UNKNOWN;
        }

        let pass = match pamh.get_authtok(None) {
            Ok(Some(p)) => p,
            Ok(None) => {
                error!("Cannot get password");
                return PamError::AUTH_ERR;
            }
            Err(e) => return e,
        };

        let pass = match pass.to_str() {
            Ok(p) => p,
            _ => {
                error!("Cannot convert password to string");
                return PamError::AUTH_ERR;
            }
        };

        debug!("Got password for {}", username);

        let client = Client::with_config(&config.radius);

        let client = match client {
            Ok(client) => client,
            Err(err) => {
                error!("Cannot create radius client: {}", err);
                return PamError::SERVICE_ERR;
            }
        };

        let db = Db::with_config(&config);

        let mut db = match db {
            Ok(db) => db,
            Err(err) => {
                error!("Cannot read database: {}", err);
                return PamError::SERVICE_ERR;
            }
        };

        let cred = Credentials::with_username_password(username, pass);

        let res = client.authenticate(&cred);

        let radius_user = match res {
            Ok(user) => user,
            Err(Error::AuthReject) => return PamError::AUTH_ERR,
            Err(err) => {
                error!("Radius error: {}", err);
                return PamError::SERVICE_ERR;
            }
        };

        let res = config.map_user(&radius_user);

        let user = match res {
            Some(user) => user,
            _ => return PamError::AUTH_ERR,
        };

        let res = db.store_user(&user);

        let cookie = match res {
            Ok(s) => s,
            Err(err) => {
                error!("Cannot write to database: {}", err);
                return PamError::SERVICE_ERR;
            }
        };

        let res = pamh.putenv(&format!("RADIUS_USER={}", username));
        if let Err(_) = res {
            return PamError::SERVICE_ERR;
        }

        let res = pamh.putenv(&format!("RADIUS_USER_COOKIE={}", cookie));
        if let Err(_) = res {
            return PamError::SERVICE_ERR;
        }

        match res {
            Ok(_) => return PamError::SUCCESS,
            Err(err) => {
                error!("Cannot set environment variables: {}", err);
                return PamError::SERVICE_ERR;
            }
        };
    }

    fn setcred(pamh: Pam, _flags: PamFlag, _args: Vec<String>) -> PamError {
        setup_log(SYSLOG_NAME);

        let config = Config::system();

        let config = match config {
            Ok(config) => config,
            Err(err) => {
                error!("Cannot read configuration file: {}", err);
                return PamError::SERVICE_ERR;
            }
        };

        let db = Db::with_config(&config);

        let db = match db {
            Ok(db) => db,
            Err(err) => {
                error!("Cannot read database: {}", err);
                return PamError::SERVICE_ERR;
            }
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

        if user == "root" {
            return PamError::USER_UNKNOWN;
        }

        let user = db.get_user(user);

        match user {
            Ok(_user) => PamError::SUCCESS,
            _ => PamError::AUTH_ERR,
        }
    }
}

pam_module!(PamRadius);
