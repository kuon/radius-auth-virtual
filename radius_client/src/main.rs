use radius_client::*;

fn main() {
    let config = Config::new()
        .server("172.105.81.33:1814")
        .server("2a01:7e01::f03c:92ff:fe86:d68f:1812")
        .server("radius.omnisens.goyman.com:1812")
        .shared_secret("testing123")
        .debug();

    let result = authenticate_user(&config, "testing", "password");

    println!("Auth result: {:#?}", result);

}
