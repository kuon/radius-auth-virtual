use anyhow::{Context, Result};
use nss_db::Config;
use nss_db::Db;
use std::process::Command;

fn main() -> Result<()> {
    let username =
        std::env::var("RADIUS_USER").context("Cannot get radius username")?;
    let cookie = std::env::var("RADIUS_USER_COOKIE")
        .context("Cannot get radius cookie")?;

    let root = nix::unistd::Uid::from_raw(0);

    nix::unistd::setuid(root).context("Cannot escalate privileges")?;

    let config = Config::system().context("Cannot read system config file")?;
    let db = Db::with_config(&config)
        .context("Cannot initialize database with config")?;

    let user = db
        .get_user_with_cookie(username, cookie)
        .context("Cannot find user, username or cookie invalid")?;

    std::env::remove_var("RADIUS_USER_COOKIE");

    let uid = nix::unistd::Uid::from_raw(user.mapping.uid);
    let gid = nix::unistd::Gid::from_raw(user.mapping.gid);

    nix::unistd::setgid(gid).context("Cannot set group")?;
    nix::unistd::setuid(uid).context("Cannot set user")?;
    let home: std::path::PathBuf = user.mapping.home.into();
    nix::unistd::chdir(&home).context("Cannot change directory")?;

    std::env::remove_var("RADIUS_USER_COOKIE");

    std::env::set_var("HOME", home);
    std::env::set_var("USER", &user.mapping.username);
    std::env::set_var("LOGNAME", &user.mapping.username);

    if let Ok(path) = std::env::var("MAIL") {
        let path: std::path::PathBuf = path.into();
        let dir = path.parent().context("Cannot determine MAIL")?;
        let path = dir.join(&user.mapping.username);
        std::env::set_var("MAIL", path);
    }

    let mut shell = Command::new(user.mapping.shell)
        .spawn()
        .context("Shell failed to start")?;

    shell.wait().context("Failed to wait on shell")?;
    Ok(())
}
