use sdio_sdhc::sdcard::Card;
use fat32::base::Volume;
use serde::Deserialize;

pub static mut CONFIG_BUF: [u8; 512] = [0; 512];

#[derive(Debug)]
pub enum ConfigError {
    NoConfig,
    FormatError
}

#[derive(Deserialize)]
pub struct Config {
    pub wifi_ssid: &'static str,
    pub wifi_pwd: &'static str,
    pub server: &'static str,
    pub port: u16,
    pub token: &'static str,
}

impl Config {
    pub fn get_config(buf: &'static mut [u8]) -> Result<Config, ConfigError> {
        let card = Card::init().unwrap();
        let volume = Volume::new(card);
        let len =  volume.root_dir().
            load_file("config.json").unwrap().
            read(buf).unwrap();

        let config = match core::str::from_utf8(&buf[0..len]) {
            Ok(str) => { str }
            Err(_) => { return Err(ConfigError::NoConfig); }
        };

        return if let Ok(config) = serde_json_core::from_str::<Config>(config) {
            Ok(config)
        } else {
            Err(ConfigError::FormatError)
        }
    }
}