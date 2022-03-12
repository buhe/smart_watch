use std::time::Instant;

use anyhow::Result;
use embedded_svc::io::Bytes;
use embedded_svc::http::{client::*};
use embedded_hal::digital::v2::OutputPin;

use crate::load::{AppContext, app::App};

const URL: &str = "https://jcxivsbsjuqmeafnwuwk.supabase.co/rest/v1/onoff?id=eq.1&select=*";
pub struct CatPlay {
    pub count: Option<Instant>,
}

impl App for CatPlay {
    fn init(self: &mut Self, ctx: &mut AppContext) -> Result<()> {
        ctx.gpio26.set_low()?;
        Ok(())
    }

    fn run(self: &mut Self, ctx: &mut AppContext) -> Result<()> {
         if self.count.unwrap().elapsed().as_secs() % 1 == 0 {
            println!("at 1s {:?}", self.count.unwrap().elapsed().as_secs());
            
            let response = ctx.http.get(URL)?.submit()?;

            let body: Result<Vec<u8>, _> = Bytes::<_, 64>::new(response.reader()).collect();

            let body = body?;
            let str = String::from_utf8_lossy(&body).into_owned();

            println!("res {}", &str);
        }
        Ok(())
    }
}