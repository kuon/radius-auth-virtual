use std::ffi::CStr;
use std::ffi::CString;
use std::net::{SocketAddr, ToSocketAddrs};

use crate::bindings::*;
use crate::config::Config;
use crate::credentials::Credentials;
use crate::error::Error;
use crate::user::Attribute;
use crate::user::User;

pub struct Client {
    ctx: *mut Context,
}

impl Client {
    pub fn try_with_config(config: &Config) -> Result<Self, Error> {
        if unsafe { rc_init() } != 0 {
            return Err(Error::OSInitFailed);
        }

        let ctx = unsafe { rc_create_context() };

        if ctx.is_null() {
            return Err(Error::Memory);
        }

        let client = Client { ctx };

        if config.servers.is_empty() {
            return Err(Error::NoServer);
        }

        for server in config.servers.iter() {
            let addrs = server.address.to_socket_addrs();
            let addrs = match addrs {
                Err(_) => {
                    format!("{}:1812", server.address).to_socket_addrs()?
                }
                Ok(addrs) => addrs,
            };

            let shared_secret =
                match (&config.shared_secret, &server.shared_secret) {
                    (None, None) => return Err(Error::NoSharedSecret),
                    (_, Some(s)) => s.clone(),
                    (Some(s), _) => s.clone(),
                };

            if shared_secret.len() > 256 {
                return Err(Error::SharedSecretTooLong);
            }

            let cs = CString::new(&shared_secret[..]).unwrap();

            for addr in addrs {
                let (ip, ipv6, port) = match addr {
                    SocketAddr::V4(v4) => {
                        (v4.ip().octets().as_ptr(), false, v4.port())
                    }
                    SocketAddr::V6(v6) => {
                        (v6.ip().octets().as_ptr(), true, v6.port())
                    }
                };
                let timeout = match server.timeout {
                    0 => config.timeout,
                    t => t,
                };

                let timeout = if timeout < 1 { 1 } else { timeout.min(30) };

                unsafe {
                    if rc_add_server(
                        client.ctx,
                        cs.as_ptr(),
                        ip,
                        ipv6 as _,
                        port,
                        timeout,
                    ) != 0
                    {
                        return Err(Error::InvalidServer(addr.to_string()));
                    }
                }
            }
        }

        if config.debug {
            unsafe { rc_enable_debug(client.ctx) };
        }

        for (vendor, subtype) in config.attributes.iter() {
            unsafe {
                if rc_add_attribute(client.ctx, *vendor, *subtype) != 0 {
                    return Err(Error::Memory);
                }
            }
        }

        Ok(client)
    }

    pub fn authenticate(
        &self,
        credentials: &Credentials,
    ) -> Result<User, Error> {
        let res = unsafe {
            let u = CString::new(&credentials.username[..]).unwrap();
            let p = CString::new(&credentials.password[..]).unwrap();
            rc_authenticate(self.ctx, u.as_ptr(), p.as_ptr())
        };
        return match res {
            AuthResult::ACCEPT => {
                let mut user = User::new();
                self.copy_attributes(&mut user);
                Ok(user)
            }
            AuthResult::REJECT => Err(Error::AuthReject),
            AuthResult::ERROR => Err(Error::RadiusClient),
            AuthResult::NO_SERV => Err(Error::NoServer),
            AuthResult::SERV_TIMEOUT => Err(Error::ServerTimeout),
        };
    }

    pub fn copy_attributes(&self, user: &mut User) {
        let mut count = 0;
        unsafe {
            let attrs = rc_get_attributes(self.ctx, &mut count);

            let attrs = std::slice::from_raw_parts(attrs, count as _);

            for raw_attr in attrs {
                let data =
                    std::slice::from_raw_parts(raw_attr.data, raw_attr.len);
                let attr = Attribute {
                    vendor: raw_attr.vendor,
                    subtype: raw_attr.subtype,
                    data: data.to_vec(),
                };
                user.add_attribute(attr);
            }
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            rc_destroy_context(self.ctx);
            rc_deinit();
        }
    }
}
