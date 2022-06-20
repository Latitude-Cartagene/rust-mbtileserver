use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use lazy_static::lazy_static;
use log::warn;
use regex::Regex;

use crate::errors::{Error, Result};
use crate::tiles::{discover_tilesets, Tilesets};

lazy_static! {
    static ref DURATION_RE: Regex = Regex::new(r"\d+[smhd]").unwrap();
}

#[derive(Parser, Default, Debug, Clone)]
#[clap(about = "A simple mbtiles server")]
#[clap(version)]
pub struct Args {
    #[clap(long, short, default_value = "./tiles", help = "Tiles directory")]
    pub directory: PathBuf,
    #[clap(skip)]
    pub tilesets: Option<Tilesets>,
    #[clap(short, long, default_value_t = 3000, help = "Server port")]
    pub port: u16,
    #[clap(
        long,
        default_value = "localhost,127.0.0.1,[::1]",
        value_delimiter = ',',
        help = "\"*\" matches all domains and \".<domain>\" matches all subdomains for the given domain"
    )]
    pub allowed_hosts: Vec<String>,
    #[clap(
        short = 'H',
        long,
        help = "Add custom header. Can be used multiple times."
    )]
    pub header: Vec<String>,
    #[clap(skip)]
    pub headers: Vec<(String, String)>,
    #[clap(long, help = "Disable preview map")]
    pub disable_preview: bool,
    #[clap(long, help = "Allow reloading tilesets with /reload endpoint")]
    pub allow_reload_api: bool,
    #[clap(long, help = "Allow reloading timesets with a SIGHUP")]
    pub allow_reload_signal: bool,
    #[clap(long, help = "An interval at which tilesets get reloaded")]
    pub reload_interval: Option<String>,
    #[clap(skip)]
    pub real_reload_interval: Option<Duration>,
    #[clap(long, help = "Disable fs watcher for automatic tileset reloading")]
    pub disable_watcher: bool,
}

impl Args {
    /// Update args after the initially parsing them with Clap
    pub fn post_parse(mut self) -> Result<Self> {
        if !self.directory.is_dir() {
            return Err(Error::Config(format!(
                "Directory does not exists: {}",
                self.directory.display()
            )));
        }
        self.tilesets = Some(discover_tilesets(String::new(), self.directory.clone()));
        self.allowed_hosts
            .iter_mut()
            .for_each(|v| *v = v.trim().to_string());

        for header in &self.header {
            let kv: Vec<&str> = header.split(':').collect();
            if kv.len() == 2 {
                let k = kv[0].trim();
                let v = kv[1].trim();
                if !k.is_empty() && !v.is_empty() {
                    self.headers.push((k.to_string(), v.to_string()))
                } else {
                    warn!("Invalid header: {header}");
                }
            } else {
                warn!("Invalid header: {header}");
            }
        }

        self.real_reload_interval = match self.reload_interval {
            Some(ref str) => {
                let mut duration = Duration::ZERO;
                for mat in DURATION_RE.find_iter(str) {
                    let mut mat = mat.as_str().to_owned();
                    let char = mat.chars().nth(mat.len() - 1).unwrap();
                    mat.truncate(mat.len() - 1);
                    let multiplier = match char {
                        's' => 1,
                        'm' => 60,
                        'h' => 60 * 60,
                        'd' => 60 * 60 * 24,
                        _ => return Err(Error::Config("Invalid value for duration".to_string())),
                    };
                    let qty = match mat.parse::<u64>() {
                        Ok(v) => v,
                        Err(_) => {
                            return Err(Error::Config("Invalid value for duration".to_string()))
                        }
                    };
                    duration += Duration::from_secs(multiplier * qty);
                }
                Some(duration)
            }
            None => None,
        };

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_missing_directory() {
        let dir = TempDir::new("tiles").unwrap();
        let dir_name = dir.path().to_str().unwrap().to_string();
        dir.close().unwrap();
        let args = Args::try_parse_from(&["", &format!("-d {dir_name}")])
            .unwrap()
            .post_parse();
        match args {
            Ok(_) => (),
            Err(err) => {
                assert!(format!("{err}").starts_with("Directory does not exists"));
            }
        };
    }

    #[test]
    fn test_valid_headers() {
        let args = Args::try_parse_from(&[
            "",
            "--header",
            "cache-control: public,max-age=14400",
            "--header",
            "access-control-allow-origin: *",
        ])
        .unwrap()
        .post_parse()
        .unwrap();
        println!("{:?}", args.headers);
        assert_eq!(
            args.headers,
            vec![
                (
                    "cache-control".to_string(),
                    "public,max-age=14400".to_string(),
                ),
                ("access-control-allow-origin".to_string(), "*".to_string(),)
            ]
        );
    }

    #[test]
    fn test_invalid_headers() {
        let app = Args::try_parse_from(&["", "-H"]);
        assert!(app.is_err());

        let args = Args::try_parse_from(&["", "-H k:"])
            .unwrap()
            .post_parse()
            .unwrap();
        assert_eq!(args.headers, vec![]);

        let args = Args::try_parse_from(&["", "-H :v"])
            .unwrap()
            .post_parse()
            .unwrap();
        assert_eq!(args.headers, vec![]);
    }
}
