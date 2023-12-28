use std::{fs, io::Error};

use serde::de::DeserializeOwned;

pub fn read_json<T: DeserializeOwned>(path: &str) -> Result<T, Error> {
    let content = fs::read_to_string(path)?; // Utilise ? pour propager l'erreur
    let parsed = serde_json::from_str::<T>(&content)?; // Utilise ? pour propager l'erreur
    Ok(parsed)
}
