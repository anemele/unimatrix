use crate::col::Columns;
use crate::st::Status;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size,
    },
};
use std::{io::Write, thread, time::Duration};

pub fn run() -> anyhow::Result<()> {
    // 初始化终端
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        Hide,
        SetBackgroundColor(Color::Black)
    )?;

    let (width, height) = size()?;

    // 创建列
    let mut columns = Columns::new(width, height);

    let mut status = Status { speed: 2 };

    // 主循环
    loop {
        columns.frame(&mut stdout, status.speed)?;

        // 刷新输出
        stdout.flush()?;

        // 控制帧率
        thread::sleep(Duration::from_millis(100));

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
                    KeyCode::Left => status.change_y_step(-1),
                    KeyCode::Right => status.change_y_step(1),
                    _ => (),
                },
                _ => (),
            }
        }
    }

    // 恢复终端
    execute!(
        stdout,
        Show,
        LeaveAlternateScreen,
        SetBackgroundColor(Color::Reset),
        SetForegroundColor(Color::Reset)
    )?;
    disable_raw_mode()?;

    Ok(())
}
