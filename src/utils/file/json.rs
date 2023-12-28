use std::{fs, io::Error};

use serde::{de::DeserializeOwned, Serialize};

pub fn read_json<T: DeserializeOwned>(path: &str) -> Result<T, Error> {
    let content = fs::read_to_string(path)?; // Utilise ? pour propager l'erreur
    let parsed = serde_json::from_str::<T>(&content)?; // Utilise ? pour propager l'erreur
    Ok(parsed)
}
pub fn write_json<T: Serialize>(path: &str, element: T) -> Result<&str, Error> {
    let content_string = serde_json::to_string(&element)?;
    match fs::write(path, content_string.as_bytes()) {
        Ok(_) => Ok("Write ok"),
        Err(e) => Err(e),
    }
}
