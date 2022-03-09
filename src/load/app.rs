use super::AppContext;

pub trait App {
    fn init(self: &Self, ctx: &AppContext);

    fn run(self: &Self, ctx: &AppContext);

    // fn clear(ctx: AppContext);
}