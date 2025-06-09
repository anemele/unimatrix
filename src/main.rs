mod cli;
mod col;
mod consts;
mod r;
mod st;

fn main() -> anyhow::Result<()> {
    r::run()
}
