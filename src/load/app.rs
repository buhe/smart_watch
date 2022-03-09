use super::AppContext;


use anyhow::Result;
pub trait App {
    fn init(self: &Self, ctx: &AppContext) -> Result<()> ;

    fn run(self: &Self, ctx: &AppContext) -> Result<()> ;

    // fn clear(ctx: AppContext);
}