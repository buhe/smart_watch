// gpio36
use anyhow::Result;
use embedded_hal::digital::v2::OutputPin;

use crate::load::{AppContext, app::App};
pub struct CatPlay {
}

impl App for CatPlay {
    fn init(self: &mut Self, ctx: &mut AppContext) -> Result<()> {
        ctx.gpio26.set_high()?;
        Ok(())
    }

    fn run(self: &mut Self, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }
}