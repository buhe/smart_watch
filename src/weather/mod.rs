use anyhow::Result;

use crate::load::{AppContext, app::App};
const URL: &str = "https://wttr.in/?format=2";
pub struct Weather {

}

impl App for Weather {
    fn init(self: &mut Self, _ctx: &AppContext) -> Result<()> {
        Ok(())
    }

    fn run(self: &mut Self, _ctx: &AppContext) -> Result<()> {
        // api interval 1m
        Ok(())
    }
}