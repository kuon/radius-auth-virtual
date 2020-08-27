use nss_db::Db;
use nss_db::Error;
use radius::User;
use radius::Attribute;

mod helpers;
use helpers::*;

#[test]
fn it_store_user() -> Result<(), Error> {
    let conf = config()?;

    assert!(conf.debug());

    let mut db = Db::with_config(&conf)?;
    let mut user = User::new("testing");
    user.attributes.push(Attribute {
        vendor: 1,
        subtype: 1,
        data: vec![0xAA],
    });
    let user = conf.map_user(&user).ok_or(Error::UserNotFound)?;
    db.store_user(&user)?;
    let user_r = db.get_user("testing")?;
    assert_eq!(user, user_r);
    Ok(())
}

