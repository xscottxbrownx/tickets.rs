use std::env;
use model::Snowflake;
use ed25519_dalek::PublicKey;

pub struct Config {
    pub server_addr: Box<str>,
    pub main_bot_id: Snowflake,
    pub main_bot_token: Box<str>,
    pub main_public_key: ed25519_dalek::PublicKey,
    pub redis: RedisConfig,
    pub database: DatabaseConfig,
}

pub struct RedisConfig {
    pub address: Box<str>,
    pub threads: usize,
}

pub struct DatabaseConfig {
    pub uri: Box<str>,
    pub threads: u32,
}

impl Config {
    pub fn from_envvar() -> Config {
        Config {
            server_addr: Config::get_envvar("SERVER_ADDR").into_boxed_str(),
            main_bot_id: Snowflake(Config::get_envvar("PUBLIC_BOT_ID").parse().unwrap()),
            main_bot_token: Config::get_envvar("PUBLIC_TOKEN").into_boxed_str(),
            main_public_key: Config::read_public_key(),
            redis: RedisConfig {
                address: Config::get_envvar("REDIS_ADDR").into_boxed_str(),
                threads: Config::get_envvar("REDIS_THREADS").parse().unwrap(),
            },
            database: DatabaseConfig {
                uri: Config::get_envvar("DATABASE_URI").into_boxed_str(),
                threads: Config::get_envvar("DATABASE_THREADS").parse().unwrap(),
            },
        }
    }

    pub fn get_envvar(name: &str) -> String {
        env::var(name).expect(&format!("envvar {} was missing!", name)[..])
    }

    fn read_public_key() -> ed25519_dalek::PublicKey {
        let key = Config::get_envvar("PUBLIC_PUBLIC_KEY");

        let mut bytes = [0u8; 32];
        hex::decode_to_slice(key, &mut bytes).unwrap();

        PublicKey::from_bytes(&bytes).unwrap()
    }
}