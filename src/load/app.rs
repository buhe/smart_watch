use super::AppContext;


use anyhow::Result;
pub trait App {
    fn init(self: &mut Self, ctx: &mut AppContext) -> Result<()> ;

    fn run(self: &mut Self, ctx: &mut AppContext) -> Result<()> ;

    // fn clear(ctx: AppContext);
}