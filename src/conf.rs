use protocol;

const DEFAULT_HOST: &'static str = "localhost";
const DEFAULT_PORT: u16 = protocol::PORT;
const DEFAULT_USER: &'static str = "guest";
const DEFAULT_PASSWORD: &'static str = "guest";
const DEFAULT_VHOST: &'static str = "/";

#[derive(Debug, Clone)]
pub struct Conf {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub vhost: String,
}

impl Default for Conf {
    fn default() -> Conf {
        Conf {
            host: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
            user: DEFAULT_USER.to_string(),
            password: DEFAULT_PASSWORD.to_string(),
            vhost: DEFAULT_VHOST.to_string(),
        }
    }
}
