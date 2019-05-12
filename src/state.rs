use crate::decrypto;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub template: tera::Tera,
    pub wordlist: Vec<String>,
    pub games: Arc<Mutex<HashMap<String, decrypto::Decrypto>>>,
}
