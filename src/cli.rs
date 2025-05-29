use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    /// use asynchronous scrolling
    #[clap(short, long, default_value_t = false)]
    pub asynchronous: bool,

    /// use all bold characters
    #[clap(short = 'b', long = "all-bold", default_value_t = false)]
    pub all_bold: bool,
    /// do not use bold characters
    #[clap(short = 'n', long = "no-bold", default_value_t = false)]
    pub no_bold: bool,

    /// foreground color
    #[clap(short, long, default_value = "green")]
    pub color: String,
    /// background color
    #[clap(short = 'g', long, default_value = "default")]
    pub bg_color: String,

    /// some characters will continuously change in place
    #[clap(short, long, default_value_t = false)]
    pub flashers: bool,

    /// ignore all keyboard input
    #[clap(short, long, default_value_t = false)]
    pub ignore_keyboard: bool,

    /// character set. See details below
    #[clap(short = 'l', long = "character-list")]
    pub character_list: Option<String>,

    /// disable on-screen status
    #[clap(short = 'o', long, default_value_t = false)]
    pub status_off: bool,

    /// speed, 0-100
    #[clap(short, long, default_value_t = 85)]
    pub speed: u8,

    /// time
    #[clap(short, long, default_value_t = -1)]
    pub time: i64,

    /// your own string of characters to display
    #[clap(short = 'u', long, default_value = "")]
    pub custom_characters: String,

    /// runs a single "wave" of green rain then exists
    #[clap(short = 'w', long, default_value_t = false)]
    pub single_wave: bool,
}
