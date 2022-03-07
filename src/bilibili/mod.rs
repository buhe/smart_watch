use std::collections::HashMap;

use anyhow::Result;
use embedded_svc::io::Bytes;
use esp_idf_svc::http::client::EspHttpClient;
use embedded_svc::http::{client::*};
const MID: &str = env!("MID");
// const API: String = 
pub fn init(client: &mut EspHttpClient) -> Result<Profile> {
    let url = format!("https://api.bilibili.com/x/web-interface/card?mid={}", MID);

    // info!("About to fetch content from {}", url);

    // let mut client = EspHttpClient::new_default()?;

    let response = client.get(&url)?.submit()?;

    let body: Result<Vec<u8>, _> = Bytes::<_, 64>::new(response.reader()).collect();

    let body = body?;
    let str = String::from_utf8_lossy(&body).into_owned();
    // println!(
    //     "Body \n{:?}",
    //     &str
    // );

    let resp: Resp = serde_json::from_str(&str).unwrap();
    // println!("Hello, world!bugu22: {:?}", users.len());
    Ok(Profile{followers: resp.data.follower,
        followings: resp.data.card.friend,
        starts: 0,
        display: resp.data.card.name,
        avatar: resp.data.card.face, 
        vender: "bilibili".to_string(),
        optional: HashMap::new()})
}


// https://api.bilibili.com/x/web-interface/card?mid=9798718

use crate::profile::Profile;

use self::entry::Resp;

pub mod entry;