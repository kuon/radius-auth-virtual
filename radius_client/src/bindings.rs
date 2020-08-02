use ::std::os::raw::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::net::{SocketAddr, ToSocketAddrs};

use crate::config::Config;
use crate::error::Error;

#[repr(C)]
struct ContextPrivate {
    private: [u8; 0],
}

pub struct Context {
    ptr: *mut ContextPrivate,
}

impl Context {
    pub fn try_with_config(config: &Config) -> Result<Self, Error> {
        if unsafe { rc_init() } != 0 {
            return Err(Error::OSInitFailed);
        }

        let ptr = unsafe { rc_create_context() };

        if ptr.is_null() {
            return Err(Error::Memory);
        }

        let ctx = Context { ptr };

        if config.shared_secret.is_empty() {
            return Err(Error::NoSharedSecret);
        }

        if config.shared_secret.len() > 256 {
            return Err(Error::SharedSecretTooLong);
        }

        let cs = CString::new(&config.shared_secret[..]).unwrap();

        unsafe {
            if rc_set_shared_secret(ctx.ptr, cs.as_ptr()) != 0 {
                return Err(Error::Memory);
            }
        }

        if config.servers.is_empty() {
            return Err(Error::NoServer);
        }

        for server in config.servers.iter() {
            let addrs = server.to_socket_addrs()?;

            for addr in addrs {
                let (ip, ipv6, port) =
                match addr {
                    SocketAddr::V4(v4) => {
                        (v4.ip().octets().as_ptr(), false, v4.port())
                    },
                    SocketAddr::V6(v6) => {
                        (v6.ip().octets().as_ptr(), true, v6.port())
                    },
                };
                unsafe {
                    if rc_add_server(ctx.ptr, ip, ipv6 as _, port) != 0 {
                        return Err(Error::InvalidServer(addr.to_string()));
                    }
                }
            }
        }

        if config.debug {
            unsafe { rc_enable_debug(ctx.ptr) };
        }

        unsafe {
            if rc_finish_init(ctx.ptr) != 0 {
                return Err(Error::RadiusClient);
            }
        }

        Ok(ctx)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            rc_destroy_context(self.ptr);
            rc_deinit();
        }
    }
}

extern "C" {
    fn rc_init() -> c_int;
    fn rc_deinit();
    fn rc_create_context() -> *mut ContextPrivate;
    fn rc_destroy_context(ctx: *mut ContextPrivate);
    fn rc_add_server(
        ctx: *mut ContextPrivate,
        ip: *const u8,
        ipv6: c_int,
        port: u16,
    ) -> c_int;
    fn rc_set_shared_secret(
        ctx: *mut ContextPrivate,
        txt: *const c_char,
    ) -> c_int;
    fn rc_enable_debug(ctx: *mut ContextPrivate);
    fn rc_finish_init(ctx: *mut ContextPrivate) -> c_int;
}
