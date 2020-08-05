#[macro_use]
extern crate pamsm;

use pamsm::{Pam, PamError, PamFlag, PamLibExt, PamServiceModule};

struct PamTime;

impl PamServiceModule for PamTime {
    fn authenticate(
        pamh: Pam,
        _flags: PamFlag,
        _args: Vec<String>,
    ) -> PamError {
        let pass = match pamh.get_authtok(None) {
            Ok(Some(p)) => p,
            Ok(None) => return PamError::AUTH_ERR,
            Err(e) => return e,
        };

        let user = match pamh.get_user(None) {
            Ok(Some(u)) => u,
            Ok(None) => return PamError::USER_UNKNOWN,
            Err(e) => return e,
        };

        if user.to_str().unwrap_or("") == "radiususer" {
            PamError::SUCCESS
        } else {
            PamError::AUTH_ERR
        }
    }

    fn setcred(
        pamh: Pam,
        _flags: PamFlag,
        _args: Vec<String>,
    ) -> PamError {

        let user = match pamh.get_user(None) {
            Ok(Some(u)) => u,
            Ok(None) => return PamError::USER_UNKNOWN,
            Err(e) => return e,
        };

        if user.to_str().unwrap_or("") == "radiususer" {
            PamError::SUCCESS
        } else {
            PamError::AUTH_ERR
        }
    }
}

pam_module!(PamTime);
