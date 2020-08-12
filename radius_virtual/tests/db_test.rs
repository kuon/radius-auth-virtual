use radius_virtual::prelude::*;

mod helpers;
use helpers::*;

#[test]
fn it_store_user() -> Result<(), Error> {
    let conf = config()?;
    let mut db = Db::with_config(&conf)?;
    let user = User::new("testing");
    db.store_user(&user)?;
    let user_r = db.get_user("testing")?;
    assert_eq!(user, user_r);
    Ok(())
}
