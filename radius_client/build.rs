use std::env;
use std::path::PathBuf;

const CONFIG_WINDOWS: &'static [(&'static str, &'static str)] = &[
    ("CONFIG_NATIVE_WINDOWS", "TRUE"),
    ("CONFIG_OS", "win32"),
    ("CONFIG_ELOOP", "eloop_win"),
];

const CONFIG_UNIX: &'static [(&'static str, &'static str)] = &[
    ("CONFIG_OS", "unix"),
    ("CONFIG_ELOOP", "eloop"),
    ("CONFIG_ELOOP_EPOLL", "TRUE"),
];

const CONFIG_COMMON: &'static [(&'static str, &'static str)] = &[
    ("CONFIG_IEEE8021X_EAPOL", "TRUE"),
    ("CONFIG_EAP_MD5", "TRUE"),
    ("CONFIG_EAP_MSCHAPV2", "TRUE"),
    ("CONFIG_EAP_TLS", "TRUE"),
    ("CONFIG_EAP_PEAP", "TRUE"),
    ("CONFIG_EAP_TTLS", "TRUE"),
    ("CONFIG_EAP_FAST", "TRUE"),
    ("CONFIG_IPV6", "TRUE"),
    ("CONFIG_TLS", "openssl"),
];

const FILES_WINDOWS: &'static [&'static str] = &["src/utils/os_win32.c"];

const FILES_UNIX: &'static [&'static str] = &["src/utils/os_unix.c"];

const FILES_COMMON: &'static [&'static str] = &[
    "src/utils/wpa_debug.c",
    "src/utils/common.c",
    "src/utils/eloop.c",
    "src/utils/ip_addr.c",
    "src/utils/wpabuf.c",
    "src/crypto/crypto_openssl.c",
    "src/radius/radius_client.c",
    "src/radius/radius.c",
];

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/wrapper.h");

    let target = env::var("TARGET").unwrap();
    let windows = target.contains("windows");

    // We cannot use cfg! here because build.rs target is host machine
    let (config, files) = if windows {
        (CONFIG_WINDOWS, FILES_WINDOWS)
    } else {
        (CONFIG_UNIX, FILES_UNIX)
    };

    let openssl_include = if windows {
        let artifacts = openssl_src::Build::new().build();
        let (lib_dir, include_dir) = {
            (
                artifacts.lib_dir().to_string_lossy(),
                artifacts.include_dir().to_string_lossy(),
            )
        };

        println!("cargo:rustc-link-search=native={}", lib_dir);
        println!("cargo:include={}", include_dir);

        println!("cargo:rustc-link-lib=static=crypto");
        println!("cargo:rustc-link-lib=static=ssl");

        include_dir.into_owned()
    } else {
        println!("cargo:rustc-link-lib=dylib=crypto");
        println!("cargo:rustc-link-lib=dylib=ssl");

        String::from(".")
    };

    let mut build = cc::Build::new();
    load_config(&mut build, CONFIG_COMMON);
    load_config(&mut build, config);

    add_files(&mut build, FILES_COMMON);
    add_files(&mut build, files);

    build
        .flag_if_supported("-Wno-unused-parameter")
        .flag("-includewin_patch.h")
        .file("src/bindings.c")
        .include("src")
        .include("../hostap/src")
        .include("../hostap/src/utils")
        .include(openssl_include)
        .compile("hostap.a");
}

fn load_config(build: &mut cc::Build, config: &[(&str, &str)]) {
    for c in config.iter() {
        build.define(c.0, c.1);
    }
}

fn add_files(build: &mut cc::Build, files: &[&str]) {
    for f in files.iter() {
        let path = PathBuf::from("../hostap/");
        build.file(&path.join(f));
    }
}
