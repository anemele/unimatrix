mod app;
mod cli;
mod col;
mod consts;
mod st;

fn main() -> anyhow::Result<()> {
    app::run()
}
