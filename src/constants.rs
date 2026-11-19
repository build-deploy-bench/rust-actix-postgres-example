pub const AUTHORIZATION: &str = "Authorization";

pub const BIND_HOST: &str = "0.0.0.0";

// Used when the provider does not inject PORT.
pub const DEFAULT_PORT: u16 = 8080;

pub mod env_key {
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const PORT: &str = "PORT";
    pub const FRONTEND_ORIGIN: &str = "FRONTEND_ORIGIN";
    pub const SECRET_KEY: &str = "SECRET_KEY";
}
