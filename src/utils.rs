use chrono::prelude::*;
use pwhash::bcrypt;
use sanitize_html::{rules::predefined::DEFAULT, sanitize_str};
use std::fs::File;
use std::io::prelude::*;
use std::{fs, io};

pub fn now() -> chrono::naive::NaiveDateTime {
    Utc::now().naive_local()
}

pub fn read_file_to_string(path: &String) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_file_to_bytes(path: &String) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents = Vec::<u8>::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

pub fn encrypt(password: &str) -> String {
    bcrypt::hash(password).unwrap()
}

pub fn verify(password: &str, hashed: &str) -> bool {
    bcrypt::verify(password, hashed)
}

pub fn sanitize_html(input: &str) -> String {
    sanitize_str(&DEFAULT, input).unwrap()
}

pub fn random(top: usize, bottom: usize) -> usize {
    (rand::random::<usize>() % top) + bottom
}

pub struct SiteMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub img_url: Option<String>,
}

pub fn get_metadata_from_url(url: &str) -> SiteMetadata {
    match opengraph::scrape(url, Default::default()) {
        Ok(object) => {
            let title = Some(object.title);
            let description = object.description;
            let img_url = object.images.first().map(|img| img.url.clone());

            SiteMetadata {
                title: title,
                description: description,
                img_url: img_url,
            }
        }
        Err(e) => {
            log::error!("{:?}", e);
            SiteMetadata {
                title: None,
                description: None,
                img_url: None,
            }
        }
    }
}

// Load public certificate from file.
pub fn load_certs(filename: &str) -> io::Result<Vec<rustls::Certificate>> {
    // Open certificate file.
    let certfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(certfile);

    // Load and return certificate.
    let certs = rustls_pemfile::certs(&mut reader)
        .map_err(|_| error("failed to load certificate".into()))?;
    Ok(certs.into_iter().map(rustls::Certificate).collect())
}

// Load private key from file.
pub fn load_private_key(filename: &str) -> io::Result<rustls::PrivateKey> {
    // Open keyfile.
    let keyfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(keyfile);

    // Load and return a single private key.
    let keys = rustls_pemfile::pkcs8_private_keys(&mut reader)
        .map_err(|_| error("failed to load private key".into()))?;

    Ok(rustls::PrivateKey(keys[0].clone()))
}

fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

#[test]
fn test_encryption() {
    // Hash a password with default parameters.
    let h_new = encrypt("password");

    assert!(verify("password", &h_new));
}

#[test]
fn opengraph_scrape() {
    match opengraph::scrape("https://popeyemagazine.jp/", Default::default()) {
        Ok(object) => {
            assert_eq!(format!("{:?}", object), "hg");
        }
        Err(_) => println!("error occured"),
    }
}
