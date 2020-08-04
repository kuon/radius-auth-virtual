
mod bindings;

pub mod config;
pub mod error;
pub mod client;
pub mod user;
pub mod credentials;
pub mod prelude;



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





