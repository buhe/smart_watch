use std::collections::HashMap;
#[derive(Debug)]
pub struct Profile {
    pub followers: usize,
    pub followings: usize,
    pub starts: usize,
    pub display: String,
    pub avatar: String,
    pub vender: String,
    pub optional: HashMap<String,String>,
}