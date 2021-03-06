use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Error, ErrorKind, Read, Result, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub funds: Vec<String>,
}

impl Config {
    // 配置文件默认为 $HOME/.config/fds/config.toml
    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        let p = match path {
            Some(v) => v,
            None => Config::default_config_path(),
        };

        if !p.exists() {
            return Err(Error::new(ErrorKind::NotFound, "config file not found"));
        }

        let mut w = OpenOptions::new().read(true).write(false).open(p)?;

        let mut buffer = String::new();
        let _ = w.read_to_string(&mut buffer)?;
        let cfg = toml::from_str(buffer.as_str()).unwrap();
        Ok(cfg)
    }

    pub fn add(&mut self, code: String) -> Result<()> {
        if !self.funds.contains(&code) {
            self.funds.push(code);
            self.flush()?
        }
        Ok(())
    }

    fn flush(&self) -> Result<()> {
        let w = OpenOptions::new()
            .write(true)
            .create(true)
            .open(Self::default_config_path())?;
        let mut writer = BufWriter::new(w);
        let content = toml::to_string(self).unwrap();
        let _ = writer.write(content.as_bytes());
        writer.flush().unwrap();
        Ok(())
    }

    pub fn default_config_path() -> PathBuf {
        let mut path = match dirs::home_dir() {
            Some(v) => v,
            _ => PathBuf::from("./"),
        };
        path.push(".config/fds");
        if !path.exists() {
            fs::create_dir_all(&path).unwrap();
        }

        path.push("config.toml");
        if path.exists() {
            return path;
        }

        let w = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .unwrap();

        let mut writer = BufWriter::new(w);
        let _ = writer.write(b"funds = []\n");
        writer.flush().unwrap();
        path
    }
}
