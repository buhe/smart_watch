use anyhow::Result;

use crate::load::{AppContext, app::App};

pub struct Weather {

}

impl App for Weather {
    fn init(self: &mut Self, _ctx: &AppContext) -> Result<()> {
        Ok(())
    }

    fn run(self: &mut Self, _ctx: &AppContext) -> Result<()> {
        Ok(())
    }
}