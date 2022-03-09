use super::AppContext;

pub trait App {
    fn init(self: &Self,ctx: &AppContext);

    fn run(ctx: AppContext);

    fn clear(ctx: AppContext);
}