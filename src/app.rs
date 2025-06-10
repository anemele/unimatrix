use crate::col::Columns;
use crate::st::Status;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io::Write, thread, time::Duration};

pub fn run() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;

    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        cursor::Hide,
        SetBackgroundColor(Color::Black)
    )?;

    let (width, height) = terminal::size()?;

    let mut columns = Columns::new(width, height);

    let mut status = Status { sleep: 200 };

    loop {
        columns.frame(&mut stdout)?;

        stdout.flush()?;

        thread::sleep(Duration::from_millis(status.sleep.into()));

        if event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Resize(width, height) => {
                    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
                    columns = Columns::new(width, height);
                }
                Event::Key(key) => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Left => status.change_sleep_duration(-1),
                    KeyCode::Right => status.change_sleep_duration(1),
                    _ => (),
                },
                _ => (),
            }
        }
    }

    execute!(
        stdout,
        cursor::Show,
        LeaveAlternateScreen,
        SetBackgroundColor(Color::Reset),
        SetForegroundColor(Color::Reset)
    )?;
    terminal::disable_raw_mode()?;

    Ok(())
}
