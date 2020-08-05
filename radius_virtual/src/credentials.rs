pub struct Credentials {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Credentials {
    pub fn with_username_password<S: Into<String>>(
        username: S,
        password: S,
    ) -> Self {
        Credentials {
            username: username.into(),
            password: password.into(),
        }
    }
}
