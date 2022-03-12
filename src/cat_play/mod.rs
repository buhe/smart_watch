// gpio36
use anyhow::Result;

use crate::load::{AppContext, app::App};
pub struct CatPlay {
}

impl App for CatPlay {
    fn init(self: &mut Self, _ctx: &AppContext) -> Result<()> {
        Ok(())
    }

    fn run(self: &mut Self, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }
}