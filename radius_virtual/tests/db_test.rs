use radius_virtual::prelude::*;

mod helpers;
use helpers::*;

#[test]
fn it_store_user() -> Result<(), Error> {
    let conf = config()?;
    let mut db = Db::with_config(&conf)?;
    let mut user = user::User::new("testing");
    user.attributes.push(user::Attribute {
        vendor: 1,
        subtype: 1,
        data: vec![0xAA],
    });
    let user = db::User::lookup(&conf, &user).ok_or(Error::UserNotFound)?;
    db.store_user(&user)?;
    let user_r = db.get_user("testing")?;
    assert_eq!(user, user_r);
    Ok(())
}
