use ::std::os::raw::*;

#[repr(C)]
pub(crate) struct Context {
    private: [u8; 0],
}

#[repr(C)]
pub(crate) enum AuthResult {
    ACCEPT = 0,
    REJECT,
    ERROR,
    NO_SERV,
    SERV_TIMEOUT,
}


#[repr(C)]
pub(crate) struct VendorAttribute {
    pub(crate) vendor: u32,
    pub(crate) subtype: u8,
    pub(crate) data: *mut u8,
    pub(crate) len: usize
}

extern "C" {
    pub(crate) fn rc_init() -> c_int;
    pub(crate) fn rc_deinit();
    pub(crate) fn rc_create_context() -> *mut Context;
    pub(crate) fn rc_destroy_context(ctx: *mut Context);
    pub(crate) fn rc_add_server(
        ctx: *mut Context,
        shared_secret: *const c_char,
        ip: *const u8,
        ipv6: c_int,
        port: u16,
        timeout: u16,
    ) -> c_int;
    pub(crate) fn rc_enable_debug(ctx: *mut Context);
    pub(crate) fn rc_add_attribute(
        ctx: *mut Context,
        vendor: u32,
        subtype: u8,
    ) -> c_int;
    pub(crate) fn rc_authenticate(
        ctx: *mut Context,
        username: *const c_char,
        password: *const c_char,
    ) -> AuthResult;
    pub(crate) fn rc_get_attributes(
        ctx: *mut Context,
        count: *mut c_int 
    ) -> *mut VendorAttribute;
}
