use std::time::Instant;

use anyhow::Result;
use embedded_svc::io::Bytes;
use embedded_svc::http::{client::*, SendHeaders};
use embedded_hal::digital::v2::OutputPin;

use crate::load::{AppContext, app::App};

const URL: &str = "https://jcxivsbsjuqmeafnwuwk.supabase.co/rest/v1/onoff?id=eq.1&select=en";

pub struct CatPlay {
    pub count: Option<Instant>,
    pub cond: u64,
}

impl App for CatPlay {
    fn init(self: &mut Self, _ctx: &mut AppContext) -> Result<()> {
        self.count = Some(Instant::now());
        Ok(())
    }

    fn run(self: &mut Self, ctx: &mut AppContext) -> Result<()> {
        let e = self.count.unwrap().elapsed().as_secs();
         if e % 4 == 0 && e != self.cond {
            self.cond = e;
            let response = ctx.http.get(URL)?
            .header("apikey", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImpjeGl2c2JzanVxbWVhZm53dXdrIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NDcwNjYwOTEsImV4cCI6MTk2MjY0MjA5MX0.YP7o3MKM7sxsNioyuVuVqTIgdgJbKz638njLOnT9DRA")
            .header("Authorization", "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImpjeGl2c2JzanVxbWVhZm53dXdrIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NDcwNjYwOTEsImV4cCI6MTk2MjY0MjA5MX0.YP7o3MKM7sxsNioyuVuVqTIgdgJbKz638njLOnT9DRA").submit()?;

            let body: Result<Vec<u8>, _> = Bytes::<_, 64>::new(response.reader()).collect();
            let body = body?;
            let str = String::from_utf8_lossy(&body).into_owned();
            if str == "[{\"en\":0}]" {
                println!("stop cat play");
                ctx.gpio16.as_mut().unwrap().set_low()?;
            } else {
                println!("start cat play");
                ctx.gpio16.as_mut().unwrap().set_high()?;
            }
        }
        Ok(())
    }
}