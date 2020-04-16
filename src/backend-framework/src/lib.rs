pub mod prng;
pub mod shuffler;

use std::collections::HashMap;

pub struct Players {
    out_by_player_id: HashMap<String, ClientOut>,
}

impl Players {

    pub fn new() -> Self {
        Players {
            out_by_player_id: HashMap::new(),
        }
    }

    pub fn add(&mut self, player_id: String, client_out: ClientOut) {
        self.out_by_player_id.insert(player_id, client_out);
    }

    pub fn contains(&self, player_id: &String) -> bool {
        self.out_by_player_id.contains_key(player_id)
    }

    pub fn count(&self) -> usize {
        self.out_by_player_id.len()
    }

    pub fn player_ids(&self) -> Vec<String> {
        self.out_by_player_id
            .keys()
            .map(|k| k.to_owned())
            .collect()
    }

    pub fn send_msg<M: prost::Message>(&self, player_id: &String, message: M) {
        match self.out_by_player_id.get(player_id) {
            None => println!("ERROR: Cannot send_msg. Player '{}' not found.", player_id),
            Some(out) => out.send(message)
        }
    }

    pub fn send_err(&self, player_id: &String, message: &str) {
        match self.out_by_player_id.get(player_id) {
            None => println!("ERROR: Cannot send_err. Player '{}' not found.", player_id),
            Some(out) => out.send_err(message)
        }
    }
}

pub struct ClientOut {
    // TODO
}

impl ClientOut {

    pub fn send<M: prost::Message>(&self, _message: M) {
        unimplemented!()
    }

    pub fn send_err(&self, _message: &str) {
        unimplemented!()
    }
}

pub struct Holder<T>(Option<T>);

impl<T> Holder<T> {
    pub fn new(item: T) -> Self {
        Holder(Some(item))
    }

    pub fn take(&mut self) -> T {
        self.0.take().expect("Invalid state: Holder.take() called when it was empty")
    }

    pub fn put(&mut self, item: T) {
        if self.0.is_some() {
            panic!("Invalid state: Holder.put() called when it was full");
        }
        self.0.replace(item);
    }
}
