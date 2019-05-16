extern crate indexmap;
extern crate rand;

use crate::game;
use crate::state;

use actix::prelude::*;
use indexmap::set::IndexSet;
use rand::Rng;
use std::collections::{HashMap, HashSet};

enum State {
    Setup,
    Rounds,
    GuessWords,
    Winner(String),
    Tie,
}

pub struct Decrypto {
    team_a: Team,
    team_b: Team,
    state: State,
    // UUID -> name
    players: HashMap<String, state::Player>,
}

pub struct Team {
    players: IndexSet<String>,
    active_player_index: usize,
    words: [String; 4],
    intercepts: u8,
    miscommunications: u8,
    rounds: Vec<Round>,
}

pub struct Round {
    clue_giver: usize,
    order: [u8; 3],
    clues: [String; 3],
    guess: [u8; 3],
    spy_guess: [u8; 3],
}

impl Decrypto {
    pub fn new(wordlist: &[String]) -> Self {
        assert!(wordlist.len() > 8);
        let words = pick_words(wordlist);
        assert!(words.len() == 8);
        Decrypto {
            team_a: Team {
                players: IndexSet::new(),
                active_player_index: 0,
                words: clone_slice(&words[0..4]),
                intercepts: 0,
                miscommunications: 0,
                rounds: Vec::new(),
            },
            team_b: Team {
                players: IndexSet::new(),
                active_player_index: 0,
                words: clone_slice(&words[4..8]),
                intercepts: 0,
                miscommunications: 0,
                rounds: Vec::new(),
            },
            state: State::Setup,
            players: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, uuid: &str, player: &state::Player) {
        self.players.insert(uuid.to_string(), player.clone());
    }

    pub fn add_player_a(&mut self, player: &str) -> Result<(), String> {
        add_player_to_team(player, &mut self.team_a)?;
        let json = json!({"command": "joined_team_a", "name": player.to_string()}).to_string();
        return self.send_to_players(&json, None);
    }

    pub fn add_player_b(&mut self, player: &str) -> Result<(), String> {
        add_player_to_team(player, &mut self.team_b)?;
        let json = json!({"command": "joined_team_b", "name": player.to_string()}).to_string();
        return self.send_to_players(&json, None);
    }

    pub fn remove_player_a(&mut self, player: &str) -> Result<(), String> {
        remove_player_from_team(player, &mut self.team_a)?;
        let json = json!({"command": "left_team_a", "name": player.to_string()}).to_string();
        return self.send_to_players(&json, None);
    }

    pub fn remove_player_b(&mut self, player: &str) -> Result<(), String> {
        remove_player_from_team(player, &mut self.team_b)?;
        let json = json!({"command": "left_team_b", "name": player.to_string()}).to_string();
        return self.send_to_players(&json, None);
    }

    pub fn new_round(&mut self) -> Result<(), String> {
        new_round_for_team(&mut self.team_a)?;
        new_round_for_team(&mut self.team_b)?;
        self.state = State::Rounds;
        return Ok(());
    }

    pub fn guess_a(&mut self, guess: [u8; 3]) -> Result<(), String> {
        return guess_team(&guess, &mut self.team_a);
    }

    pub fn guess_b(&mut self, guess: [u8; 3]) -> Result<(), String> {
        return guess_team(&guess, &mut self.team_a);
    }

    pub fn spy_guess_a(&mut self, guess: [u8; 3]) -> Result<(), String> {
        return spy_guess_team(&guess, &mut self.team_a, &mut self.team_b);
    }

    pub fn spy_guess_b(&mut self, guess: [u8; 3]) -> Result<(), String> {
        return spy_guess_team(&guess, &mut self.team_b, &mut self.team_a);
    }

    pub fn give_clues_a(&mut self, clues: &[String; 3]) -> Result<(), String> {
        return give_clues_team(clues, &mut self.team_a);
    }

    pub fn give_clues_b(&mut self, clues: &[String; 3]) -> Result<(), String> {
        return give_clues_team(clues, &mut self.team_b);
    }

    pub fn maybe_advance_game(&mut self) -> Result<(), String> {
        match self.state {
            State::Setup => {
                if self.team_a.players.len() < 2 {
                    return Err("Team A must have at least 2 players!".to_string());
                }
                if self.team_b.players.len() < 2 {
                    return Err("Team B must have at least 2 players!".to_string());
                }
                return self.new_round();
            }
            State::Rounds => {
                if !is_round_complete_for_team(&self.team_a) {
                    return Err("Round not complete for Team A".to_string());
                }
                if !is_round_complete_for_team(&self.team_b) {
                    return Err("Round not complete for Team B".to_string());
                }
                if self.team_a.intercepts == 2 {
                    self.state = State::Winner("Team A".to_string());
                } else if self.team_a.miscommunications == 2 {
                    self.state = State::Winner("Team B".to_string());
                } else if self.team_b.intercepts == 2 {
                    self.state = State::Winner("Team B".to_string());
                } else if self.team_b.miscommunications == 2 {
                    self.state = State::Winner("Team A".to_string());
                } else if self.team_a.rounds.len() == 8 && self.team_b.rounds.len() == 8 {
                    self.state = State::GuessWords;
                }
            }
            _ => {}
        }
        return Ok(());
    }

    fn send_to_players(&self, json: &str, team: Option<&Team>) -> Result<(), String> {
        let mut ret = Ok(());
        let msg = game::SendCommand {
            json: json.to_string(),
        };
        self.players.values().for_each(|player| {
            if let Some(addr) = &player.addr {
                if let Some(t) = team {
                    if t.players.contains(&player.name) {
                        return;
                    }
                }
                let res = addr.send(msg.clone()).wait();
                if !ret.is_err() {
                    ret = res;
                }
            }
        });
        return ret.map_err(|e| format!("{:?}", &e));
    }
}

fn pick_words(wordlist: &[String]) -> Vec<String> {
    let mut rand = rand::thread_rng();
    let mut picks: HashSet<usize> = HashSet::new();
    while picks.len() != 8 {
        let index: usize = rand.gen::<usize>() % wordlist.len();
        picks.insert(index);
    }
    return picks.iter().map(|x| wordlist[*x].to_owned()).collect();
}

fn clone_slice(slice: &[String]) -> [String; 4] {
    let mut a: [String; 4] = Default::default();
    a.clone_from_slice(slice);
    a
}

fn add_player_to_team(player: &str, team: &mut Team) -> Result<(), String> {
    if team.players.contains(player) {
        return Err(format!("{} is already on team.", player));
    }
    if !team.rounds.is_empty() {
        return Err("Game alread started.".to_string());
    }
    team.players.insert(player.to_string());
    return Ok(());
}

fn remove_player_from_team(player: &str, team: &mut Team) -> Result<(), String> {
    if !team.rounds.is_empty() {
        return Err("Game alread started.".to_string());
    }
    if !team.players.contains(player) {
        return Ok(());
    }
    team.players.remove(player);
    return Ok(());
}

fn generate_order() -> Result<[u8; 3], String> {
    let mut rand = rand::thread_rng();
    let mut picks: HashSet<u8> = HashSet::new();
    while picks.len() != 3 {
        let index: u8 = rand.gen::<u8>() % 3;
        picks.insert(index + 1);
    }
    let mut array: [u8; 3] = [0; 3];
    let mut i = 0;
    for pick in picks {
        array[i] = pick;
        i += 1;
    }
    return Ok(array);
}

fn new_round_for_team(team: &mut Team) -> Result<(), String> {
    if team.rounds.len() > 8 {
        return Err("Too many rounds!".to_string());
    }
    team.active_player_index = (team.active_player_index + 1) % team.players.len();
    team.rounds.push(Round {
        clue_giver: team.active_player_index,
        order: generate_order()?,
        clues: Default::default(),
        guess: Default::default(),
        spy_guess: Default::default(),
    });
    return Ok(());
}

fn guess_team(guess: &[u8; 3], team: &mut Team) -> Result<(), String> {
    if let Some(round) = team.rounds.last_mut() {
        round.guess.copy_from_slice(guess);
        if &round.order != guess {
            team.miscommunications += 1;
        }
    } else {
        return Err("No rounds!".to_string());
    }
    return Ok(());
}

fn spy_guess_team(guess: &[u8; 3], team: &mut Team, scoring_team: &mut Team) -> Result<(), String> {
    if let Some(round) = team.rounds.last_mut() {
        round.spy_guess.copy_from_slice(guess);
        if &round.order == guess {
            scoring_team.intercepts += 1;
        }
    } else {
        return Err("No rounds!".to_string());
    }
    return Ok(());
}

fn give_clues_team(clues: &[String; 3], team: &mut Team) -> Result<(), String> {
    if let Some(round) = team.rounds.last_mut() {
        round.clues.clone_from_slice(clues);
    } else {
        return Err("No rounds!".to_string());
    }
    return Ok(());
}

fn is_round_complete_for_team(team: &Team) -> bool {
    if let Some(round) = team.rounds.last() {
        let empty_clues: [String; 3] = Default::default();
        let empty_guess: [u8; 3] = Default::default();
        if round.clues == empty_clues {
            return false;
        }
        if round.guess == empty_guess {
            return false;
        }
        if round.spy_guess == empty_guess {
            return false;
        }
        return true;
    }
    return false;
}
