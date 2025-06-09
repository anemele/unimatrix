use rand::Rng;

pub struct Status {
    pub y_step: u16,
}

impl Status {
    pub fn change_y_step(&mut self, change: i16) {
        match change {
            1 => {
                if self.y_step < 5 {
                    self.y_step += 1;
                }
            }
            -1 => {
                if self.y_step > 1 {
                    self.y_step -= 1
                }
            }
            _ => (),
        }
    }
}

pub fn random_character(//s: &'static str
) -> char {
    // let e: Vec<char> = s.chars().collect();
    // // dbg!(&e);
    // let idx = rand::rng().random_range(0..e.len() - 1);
    // // dbg!(idx);
    // e[idx]

    rand::rng().random_range(0x30..=0x31).into()
}
