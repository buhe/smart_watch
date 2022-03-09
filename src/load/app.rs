use super::AppContext;

pub trait App {
    fn init(ctx: AppContext);

    fn run(ctx: AppContext);

    fn clear(ctx: AppContext);
}