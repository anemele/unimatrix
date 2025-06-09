use crossterm::execute;
use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, SetForegroundColor},
};
use rand::Rng;

use crate::st::random_character;

struct Column {
    x: u16,
    y: u16,
    length: u16,
}

impl Column {
    fn new(x: u16, max_y: u16) -> Self {
        let mut rng = rand::rng();
        let length = rng.random_range(5..max_y);

        Self {
            x,
            y: rng.random_range(0..max_y),
            length,
        }
    }

    fn clear_tail(&mut self, stdout: &mut impl std::io::Write, y_step: u16) -> anyhow::Result<()> {
        execute!(
            stdout,
            MoveTo(self.x, 0),
            SetForegroundColor(Color::Black),
            Print(' ')
        )?;

        for i in 0..y_step {
            let yi = self.y + i;
            if yi < self.length {
                break;
            }
            execute!(
                stdout,
                MoveTo(self.x, yi - self.length),
                SetForegroundColor(Color::Black),
                Print(' ')
            )?;
        }

        Ok(())
    }

    fn update(&mut self, max_y: u16, y_step: u16) {
        self.y += y_step;

        // 重置超出屏幕的列
        if self.y > max_y + self.length {
            let mut rng = rand::rng();
            self.y = 0;
            self.length = rng.random_range(5..max_y / 3);
        }
    }

    fn draw(&mut self, stdout: &mut impl std::io::Write) -> anyhow::Result<()> {
        execute!(
            stdout,
            MoveTo(self.x, self.y),
            SetForegroundColor(Color::White),
            Print(random_character())
        )?;

        let mut rng = rand::rng();
        // 绘制整个列
        for i in 1..self.length {
            if self.y <= i {
                break;
            }
            let y = self.y - i;
            if y < 500 {
                execute!(
                    stdout,
                    MoveTo(self.x, y),
                    SetForegroundColor(Color::Rgb {
                        r: 0,
                        g: rng.random_range(50..=255),
                        b: 0
                    }),
                    Print(random_character())
                )?;
            }
        }

        Ok(())
    }
}

pub struct Columns {
    columns: Vec<Column>,
    height: u16,
}

impl Columns {
    pub fn new(width: u16, height: u16) -> Self {
        let columns = (0..width)
            .step_by(2)
            .map(|x| Column::new(x, height))
            .collect();
        Self { columns, height }
    }

    pub fn frame(&mut self, stdout: &mut impl std::io::Write, y_step: u16) -> anyhow::Result<()> {
        // 更新和绘制所有列
        for column in &mut self.columns {
            column.clear_tail(stdout, y_step)?;
            column.update(self.height, y_step);
            column.draw(stdout)?;
        }

        Ok(())
    }
}
