extern crate indexmap;
extern crate rand;

use crate::game;
use crate::state;

use actix::prelude::*;
use indexmap::map::IndexMap;
use indexmap::set::IndexSet;
use rand::Rng;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
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
    players: IndexMap<String, state::Player>,
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

impl Actor for Decrypto {
    type Context = Context<Self>;
}

#[derive(Message)]
pub struct AddPlayerToGame {
    pub uuid: String,
    pub player: state::Player,
}

impl Handler<AddPlayerToGame> for Decrypto {
    type Result = ();

    fn handle(&mut self, msg: AddPlayerToGame, _: &mut Context<Self>) {
        self.add_player(msg.uuid, msg.player);
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), String>")]
pub struct AddPlayerToTeamA {
    pub player: String,
}

impl Handler<AddPlayerToTeamA> for Decrypto {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: AddPlayerToTeamA, _: &mut Context<Self>) -> Self::Result {
        return self.add_player_a(msg.player);
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), String>")]
pub struct AddPlayerToTeamB {
    pub player: String,
}

impl Handler<AddPlayerToTeamB> for Decrypto {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: AddPlayerToTeamB, _: &mut Context<Self>) -> Self::Result {
        return self.add_player_b(msg.player);
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), String>")]
pub struct LeaveTeam {
    pub player: String,
}

impl Handler<LeaveTeam> for Decrypto {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: LeaveTeam, _: &mut Context<Self>) -> Self::Result {
        return self.leave_team(msg.player);
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), String>")]
pub struct PlayerConnected {
    pub uuid: String,
    pub addr: Addr<game::Ws>,
}

impl Handler<PlayerConnected> for Decrypto {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: PlayerConnected, _: &mut Context<Self>) -> Self::Result {
        info!("handler player_connected");
        return self.player_connected(msg.uuid, msg.addr);
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), String>")]
pub struct PlayerDisconnected {
    pub uuid: String,
}

impl Handler<PlayerDisconnected> for Decrypto {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: PlayerDisconnected, _: &mut Context<Self>) -> Self::Result {
        return self.player_disconnected(msg.uuid);
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), String>")]
pub struct StartGame {
    pub uuid: String,
}

impl Handler<StartGame> for Decrypto {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: StartGame, _: &mut Context<Self>) -> Self::Result {
        return self.start_game(msg.uuid);
    }
}

impl Decrypto {
    pub fn new(wordlist: &[String]) -> Self {
        assert!(wordlist.len() >= 8);
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
            players: IndexMap::new(),
        }
    }

    fn add_player(&mut self, uuid: String, player: state::Player) {
        info!("Adding player {} {}", &uuid, &player.name);
        self.players.insert(uuid, player);
    }

    fn player_connected(&mut self, uuid: String, addr: Addr<game::Ws>) -> Result<(), String> {
        info!("decrypto::player_connected");
        match self.players.get_mut(&uuid) {
            Some(player) => {
                if let Some(old_addr) = player.addr.replace(addr) {
                    info!("decrypto::player_connected forcedisconnect");
                    old_addr.do_send(game::ForceDisconnect {});
                }
            }
            None => return Err(format!("Player with UUID {} not in room!", &uuid)),
        }
        let player = self.players.get(&uuid).unwrap();
        let json =
            json!({"command": "player_connected", "player": player.name.clone()}).to_string();
        info!("decrypto::player_connected send json to players");
        self.send_to_players(&json, None)?;
        info!("decrypto::player_connected send players to player");
        self.players.values().try_for_each(|p| {
            if &p.name == &player.name {
                return Ok(());
            }
            self.send_to_player(
                &player,
                &json!({"command": "player_connected", "player": p.name.clone()}).to_string(),
            )
        })?;
        info!("decrypto::player_connected send teams to player");
        [(&self.team_a, "a"), (&self.team_b, "b")]
            .iter()
            .try_for_each(|team| {
                team.0.players.iter().try_for_each(|p| {
                    self.send_to_player(
                        &player,
                        &json!({"command": "joined_team", "name": p, "team": team.1}).to_string(),
                    )
                })
            })?;
        info!("decrypto::player_connected send host to player");
        self.send_to_player(
            &player,
            &json!({"command": "new_host", "player": &self.players.get_index(0).unwrap().1.name})
                .to_string(),
        )?;
        info!("decrypto::player_connected send rounds to player");
        if self.state != State::Setup {
            if let Some(teams) = self.team_for_player(&player.name) {
                for round_number in 0..teams.0.rounds.len() {
                    let round_json = build_round_info(round_number, teams.0, teams.1)?;
                    self.send_to_player(&player, &round_json.to_string())?;
                }
                self.send_to_player(
                    &player,
                    &json!({"command": "words", "words": &teams.0.words}).to_string(),
                )?;
            }
        }
        info!("decrypto::player_connected done");
        return Ok(());
    }

    fn player_disconnected(&mut self, uuid: String) -> Result<(), String> {
        let mut new_host = false;
        if self.state == State::Setup {
            match self.players.get_full(&uuid) {
                Some((idx, _, player)) => {
                    if self.team_a.players.contains(&player.name) {
                        self.remove_player_a(player.name.clone())?;
                    } else if self.team_b.players.contains(&player.name) {
                        self.remove_player_b(player.name.clone())?;
                    }
                    if idx == 0 && self.players.len() > 1 {
                        new_host = true;
                    }
                }
                None => {
                    info!("players in room : {:?}", &self.players);
                    return Err(format!("Player with UUID {} not in room!", &uuid));
                }
            }
        }
        match self.players.remove(&uuid).as_mut() {
            Some(player) => {
                player.addr.take();
                {
                    let json =
                        json!({"command": "player_disconnected", "player": player.name.clone()})
                            .to_string();
                    self.send_to_players(&json, None)?;
                }
                if new_host {
                    let json =
                        json!({"command": "new_host", "player": &self.players.get_index(0).unwrap().1.name}).to_string();
                    self.send_to_players(&json, None)?;
                }
                return Ok(());
            }
            None => return Err(format!("Player with UUID {} not in room!", &uuid)),
        }
    }

    fn add_player_a(&mut self, player: String) -> Result<(), String> {
        info!("Adding player to team a: {}", &player);
        if self.team_b.players.contains(&player) {
            return Err(format!("{} is already on team B", &player));
        }
        add_player_to_team(&player, &mut self.team_a)?;
        let json = json!({"command": "joined_team", "name": player, "team": "a"}).to_string();
        return self.send_to_players(&json, None);
    }

    fn add_player_b(&mut self, player: String) -> Result<(), String> {
        info!("Adding player to team b: {}", &player);
        if self.team_a.players.contains(&player) {
            return Err(format!("{} is already on team A", &player));
        }
        add_player_to_team(&player, &mut self.team_b)?;
        let json = json!({"command": "joined_team", "name": player, "team": "b"}).to_string();
        return self.send_to_players(&json, None);
    }

    fn leave_team(&mut self, player: String) -> Result<(), String> {
        info!("Player leaving team: {}", &player);
        if self.state != State::Setup {
            return Err("Cannot leave after the game has started!".to_string());
        }
        let mut found = false;
        for p in &self.players {
            if p.1.name == player {
                found = true;
                break;
            }
        }
        if !found {
            return Err(format!("'{}' is not in the game!", &player));
        }
        if self.team_a.players.contains(&player) {
            return self.remove_player_a(player);
        } else if self.team_b.players.contains(&player) {
            return self.remove_player_b(player);
        } else {
            return Err(format!("'{}' is not on a team!", &player));
        }
    }

    fn remove_player_a(&mut self, player: String) -> Result<(), String> {
        remove_player_from_team(&player, &mut self.team_a)?;
        let json = json!({"command": "left_team", "name": player, "team": "a"}).to_string();
        return self.send_to_players(&json, None);
    }

    pub fn remove_player_b(&mut self, player: String) -> Result<(), String> {
        remove_player_from_team(&player, &mut self.team_b)?;
        let json = json!({"command": "left_team", "name": player, "team": "b"}).to_string();
        return self.send_to_players(&json, None);
    }

    pub fn new_round(&mut self) -> Result<(), String> {
        new_round_for_team(&mut self.team_a)?;
        new_round_for_team(&mut self.team_b)?;
        self.state = State::Rounds;
        return self.send_round_info();
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
                self.send_words()?;
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
                } else {
                    self.new_round()?;
                }
            }
            _ => {}
        }
        return Ok(());
    }

    pub fn start_game(&mut self, uuid: String) -> Result<(), String> {
        match self.players.get_full(&uuid) {
            Some((idx, _, _)) => {
                if idx != 0 {
                    return Err("Only game host can start the game!".to_string());
                }
            }
            None => return Err("Unknown player tried to start the game.".to_string()),
        }
        return self.maybe_advance_game();
    }

    fn team_for_player(&self, name: &str) -> Option<(&Team, &Team)> {
        if self.team_a.players.contains(name) {
            return Some((&self.team_a, &self.team_b));
        } else if self.team_b.players.contains(name) {
            return Some((&self.team_b, &self.team_b));
        }
        return None;
    }

    fn send_to_players(&self, json: &str, team: Option<&Team>) -> Result<(), String> {
        let msg = game::SendCommand {
            json: json.to_string(),
        };
        return self.players.values().try_for_each(|player| {
            if let Some(addr) = &player.addr {
                if let Some(t) = team {
                    if t.players.contains(&player.name) {
                        return Ok(());
                    }
                }
                addr.do_send(msg.clone());
            }
            return Ok(());
        });
    }

    fn send_round_info(&mut self) -> Result<(), String> {
        let json_a = build_round_info(self.team_a.rounds.len() - 1, &self.team_a, &self.team_b)?;
        let json_b = build_round_info(self.team_b.rounds.len() - 1, &self.team_b, &self.team_a)?;
        self.send_to_players(&json_a.to_string(), Some(&self.team_a))?;
        self.send_to_players(&json_b.to_string(), Some(&self.team_b))?;
        let round_a = self.team_a.rounds.last().unwrap();
        let round_b = self.team_b.rounds.last().unwrap();
        let json_order_a = json!({"command": "order", "number": self.team_a.rounds.len(), "order": &round_a.order.clone()});
        let json_order_b = json!({"command": "order", "number": self.team_b.rounds.len(), "order": &round_b.order.clone()});
        let round_a_clue_giver = self.team_a.players.get_index(round_a.clue_giver).unwrap();
        let round_b_clue_giver = self.team_b.players.get_index(round_b.clue_giver).unwrap();
        self.send_to_player_name(round_a_clue_giver, &json_order_a.to_string())?;
        self.send_to_player_name(round_b_clue_giver, &json_order_b.to_string())?;
        return Ok(());
    }

    fn send_words(&mut self) -> Result<(), String> {
        let json_a = json!({"command": "words", "words": &self.team_a.words});
        let json_b = json!({"command": "words", "words": &self.team_b.words});
        self.send_to_players(&json_a.to_string(), Some(&self.team_a))?;
        self.send_to_players(&json_b.to_string(), Some(&self.team_b))?;
        return Ok(());
    }

    fn send_to_player_name(&self, name: &str, json: &str) -> Result<(), String> {
        let msg = game::SendCommand {
            json: json.to_string(),
        };
        self.players
            .iter()
            .filter_map(|p| {
                if &p.1.name == name {
                    return p.1.addr.clone();
                }
                return None;
            })
            .for_each(|addr| addr.do_send(msg.clone()));
        return Ok(());
    }

    fn send_to_player(&self, player: &state::Player, json: &str) -> Result<(), String> {
        let msg = game::SendCommand {
            json: json.to_string(),
        };
        player.addr.as_ref().unwrap().do_send(msg);
        return Ok(());
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

fn build_round_info(
    round_number: usize,
    team: &Team,
    other_team: &Team,
) -> Result<serde_json::Value, String> {
    let round = team.rounds.get(round_number).ok_or(format!(
        "Team rounds out of bounds {} <> {}.",
        &round_number,
        team.rounds.len()
    ))?;
    let other_round = other_team.rounds.get(round_number).ok_or(format!(
        "Other Team rounds out of bounds {} <> {}.",
        &round_number,
        other_team.rounds.len()
    ))?;
    let clue_giver = team.players.get_index(round.clue_giver).unwrap();
    let enemy_clue_giver = other_team
        .players
        .get_index(other_round.clue_giver)
        .unwrap();
    let mut json = json!({
        "command": "round",
        "number": round_number,
        "clue_giver": clue_giver,
        "enemy_clue_giver": enemy_clue_giver,
    });
    let map = json.as_object_mut().unwrap();
    if round
        .clues
        .iter()
        .fold(true, |is_set, clue| is_set && !clue.is_empty())
    {
        map.insert("clues".to_string(), json!(round.clues.clone()));
    }

    let guessed = round
        .guess
        .iter()
        .fold(true, |is_set, guess| is_set && *guess > 0);
    if guessed {
        map.insert("guess".to_string(), json!(round.guess.clone()));
    }

    let spy_guessed = round
        .spy_guess
        .iter()
        .fold(true, |is_set, guess| is_set && *guess > 0);
    if spy_guessed {
        map.insert("spy_guess".to_string(), json!(round.spy_guess.clone()));
    }

    if guessed && spy_guessed {
        map.insert("order".to_string(), json!(round.order.clone()));
    }

    return Ok(json);
}
