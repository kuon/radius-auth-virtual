pub struct Config {
    pub(crate) servers: Vec<String>,
    pub(crate) shared_secret: String,
    pub(crate) debug: bool
}

impl Config {
    pub fn new() -> Self {
        Config {
            servers: vec![],
            shared_secret: String::new(),
            debug: true
        }
    }

    pub fn server<S: Into<String>>(mut self, hostname: S) -> Self {
        self.servers.push(hostname.into());
        self
    }

    pub fn shared_secret<S: Into<String>>(mut self, secret: S) -> Self {
        self.shared_secret = secret.into();
        self
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn no_debug(mut self) -> Self {
        self.debug = false;
        self
    }
}
