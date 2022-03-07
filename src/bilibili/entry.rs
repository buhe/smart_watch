use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Resp {
    pub code: usize,
    pub message: String,
    pub ttl: usize,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub card: Card,
    pub follower: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub name: String,
    pub face: String,
    pub friend: usize,
}