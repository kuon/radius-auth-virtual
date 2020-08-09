#[macro_use] extern crate assert_matches;

use radius_virtual::prelude::*;


#[test]
fn it_passes_auth() -> Result<(), Error> {
    let c = client()?;
    let cred = Credentials::with_username_password("testing", "password");
    let user = c.authenticate(&cred)?;
    assert_eq!(user.attributes.len(), 1);
    assert_eq!(user.attributes[0].vendor, 1);
    assert_eq!(user.attributes[0].subtype, 1);
    assert_eq!(user.attributes[0].data, vec![0xAA]);
    Ok(())
}


#[test]
fn it_fails_auth() -> Result<(), Error> {
    let c = client()?;
    let cred = Credentials::with_username_password("testing", "fail");
    let res = c.authenticate(&cred);
    assert_matches!(res, Err(Error::AuthReject));
    Ok(())
}

fn client() -> Result<Client, Error> {
    let path = std::env::current_dir()?;
    let path = path.join("../tests/config.toml");
    let conf = Config::read_file(path)?;
    Client::with_config(&conf)
}
