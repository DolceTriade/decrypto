use crate::decrypto;
use crate::game;

use actix::*;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub game: String,
    pub addr: Option<Addr<game::Ws>>,
}

impl Player {
    pub fn new(name: &str, game: &str) -> Self {
        Player {
            name: name.to_string(),
            game: game.to_string(),
            addr: None,
        }
    }
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Player {{ name: {}, game: {} addr: {} }}",
            &self.name,
            &self.game,
            self.addr.is_some()
        )
    }
}

pub struct AppState {
    pub template: tera::Tera,
    pub wordlist: Vec<String>,
    pub games: Arc<Mutex<HashMap<String, Addr<decrypto::Decrypto>>>>,
    pub players: Arc<Mutex<HashMap<String, Player>>>,
}
