#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        let database_url =
            dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
        let jwt_secret = dotenv::var("JWT_SECRET").expect("DATABASE_URL must be set in .env file");
        let port = dotenv::var("PORT")
            .expect("DATABASE_URL must be set in .env file")
            .parse::<u16>()
            .expect("DATABASE_URL must be set in .env file");
        Self {
            database_url,
            jwt_secret,
            port,
        }
    }
}
