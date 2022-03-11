use std::time::Instant;

use anyhow::Result;
use embedded_svc::io::Bytes;
use embedded_svc::http::{client::*};

use crate::load::{AppContext, app::App};
const URL: &str = "https://wttr.in/?format=2";
pub struct Weather {

}

impl App for Weather {
    fn init(self: &mut Self, _ctx: &AppContext) -> Result<()> {
        Ok(())
    }

    fn run(self: &mut Self, ctx: &mut AppContext) -> Result<()> {
        // api interval 1m
        let now = Instant::now();
        if now.elapsed().as_secs() / 60 == 0 {
            println!("at 1m");
            let response = ctx.http.get(URL)?.submit()?;

            let body: Result<Vec<u8>, _> = Bytes::<_, 64>::new(response.reader()).collect();

            let body = body?;
            let str = String::from_utf8_lossy(&body).into_owned();

            println!("res {}", &str);
        }
        Ok(())
    }
}