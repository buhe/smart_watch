use anyhow::Result;

use crate::load::{AppContext, app::App};

pub struct Weather {

}

impl App for Weather {
    fn init(self: &Self, _ctx: &AppContext) -> Result<()> {
        Ok(())
    }

    fn run(self: &Self, _ctx: &AppContext) -> Result<()> {
        Ok(())
    }
}