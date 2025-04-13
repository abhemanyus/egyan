use std::{
    env::temp_dir,
    error::Error,
    fs::{read_to_string, File},
    io::{copy, BufReader, BufWriter, Write},
    str::FromStr,
};

use reqwest::Url;

pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(None)
            .build()
            .expect("unable to build client");
        Self { client }
    }
    pub fn text<T: AsRef<str>>(&self, url: T) -> Result<String, Box<dyn Error>> {
        let path = format!(
            "{}/{}",
            temp_dir().to_string_lossy(),
            Url::from_str(url.as_ref())?
                .path()
                .replace('/', "")
                .to_string()
        );
        let text = read_to_string(&path);
        match text {
            Ok(text) => Ok(text),
            Err(_) => {
                let text = self.client.get(url.as_ref()).send()?.text()?;
                std::fs::write(&path, text)?;
                self.text(url)
            }
        }
    }
    pub fn bytes<T: AsRef<str>>(&self, url: T) -> Result<BufReader<File>, Box<dyn Error>> {
        let path = format!(
            "{}/{}",
            temp_dir().to_string_lossy(),
            Url::from_str(url.as_ref())?
                .path()
                .replace('/', "")
                .to_string()
        );
        match File::open(&path) {
            Ok(file) => {
                let bytes = BufReader::new(file);
                Ok(bytes)
            }
            Err(_) => {
                let page = self.client.get(url.as_ref()).send()?;
                let mut bufreader = BufReader::new(page);
                let mut bufwriter = BufWriter::new(File::create(&path)?);
                copy(&mut bufreader, &mut bufwriter)?;
                bufwriter.flush()?;
                drop(bufwriter);

                let bufreader = BufReader::new(File::open(&path)?);
                Ok(bufreader)
            }
        }
    }
}
