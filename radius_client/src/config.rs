pub struct Server {
    pub(crate) address: String,
    pub(crate) shared_secret: Option<String>,
    pub(crate) timeout: u16,
}

pub struct Config {
    pub(crate) shared_secret: Option<String>,
    pub(crate) servers: Vec<Server>,
    pub(crate) debug: bool,
    pub(crate) timeout: u16,
    pub(crate) attributes: Vec<(u32, u8)>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            servers: vec![],
            shared_secret: None,
            timeout: 10,
            debug: false,
            attributes: vec![],
        }
    }

    pub fn server<S: Into<Server>>(mut self, opts: S) -> Self {
        let s = opts.into();

        if let None = self.shared_secret {
            self.shared_secret = s.shared_secret.clone();
        }

        self.servers.push(s);
        self
    }

    pub fn shared_secret<S: Into<String>>(mut self, secret: S) -> Self {
        self.shared_secret = Some(secret.into());
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

    pub fn timeout(mut self, timeout: u16) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn attributes(mut self, attrs: &[(u32, u8)]) -> Self {
        self.attributes.append(&mut attrs.to_vec());
        self
    }

    pub fn attribute(mut self, attr: (u32, u8)) -> Self {
        self.attributes.push(attr);
        self
    }
}

impl From<String> for Server {
    fn from(address: String) -> Self {
        Server {
            address: address,
            shared_secret: None,
            timeout: 0,
        }
    }
}

impl From<&str> for Server {
    fn from(address: &str) -> Self {
        Server {
            address: address.to_string(),
            shared_secret: None,
            timeout: 0,
        }
    }
}

impl<S: Into<String>> From<(S, u16)> for Server {
    fn from(pair: (S, u16)) -> Self {
        Server {
            address: pair.0.into(),
            shared_secret: None,
            timeout: pair.1,
        }
    }
}

impl<S: Into<String>> From<(S, String)> for Server {
    fn from(pair: (S, String)) -> Self {
        Server {
            address: pair.0.into(),
            shared_secret: Some(pair.1),
            timeout: 0,
        }
    }
}

impl<S: Into<String>> From<(S, &str)> for Server {
    fn from(pair: (S, &str)) -> Self {
        Server {
            address: pair.0.into(),
            shared_secret: Some(pair.1.to_string()),
            timeout: 0,
        }
    }
}

impl<S: Into<String>> From<(S, S, u16)> for Server {
    fn from(pair: (S, S, u16)) -> Self {
        Server {
            address: pair.0.into(),
            shared_secret: Some(pair.1.into()),
            timeout: pair.2,
        }
    }
}
