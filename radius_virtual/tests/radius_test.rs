use radius_virtual::prelude::*;

#[test]
fn it_passes_auth() -> Result<(), Error> {
    let c = client()?;
    let cred = Credentials::with_username_password("testing", "password");
    let user = c.authenticate(&cred)?;
    Ok(())
}

fn client() -> Result<Client, Error> {
    let path = std::env::current_dir()?;
    let path = path.join("../tests/config.toml");
    let conf = Config::read_file(path)?;
    Client::with_config(&conf)
}
