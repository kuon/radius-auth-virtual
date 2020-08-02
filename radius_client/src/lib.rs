

pub mod config;
pub use config::Config;

pub mod error;
pub use error::Error;
pub use error::Error::*;

mod bindings;
use bindings::*;


use std::net::{SocketAddr, ToSocketAddrs};

/*
pub fn encode_test(txt: &str) -> String {
    unsafe {
        let mut out_len: size_t = 0;
        let out =
            base64_encode(txt.as_ptr() as _, txt.len() as size_t, &mut out_len);
        let slice = std::slice::from_raw_parts(out as _, out_len as _);
        let res = std::str::from_utf8(slice).unwrap().to_owned();
        _os_free(out as _);
        res
    }
}
*/

#[derive(Debug)]
pub struct Attributes;

pub fn authenticate_user<S: Into<String>>(
    config: &Config,
    username: S,
    password: S,
) -> Result<Attributes, Error> {
    let ctx = Context::try_with_config(&config)?;

    Ok(Attributes)
}

