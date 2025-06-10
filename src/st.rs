use rand::Rng;

pub struct Status {
    pub sleep: u16,
}

impl Status {
    pub fn change_sleep_duration(&mut self, change: i16) {
        match change {
            1 => {
                if self.sleep < 300 {
                    self.sleep += 10;
                }
            }
            -1 => {
                if self.sleep > 100 {
                    self.sleep -= 10
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

    rand::rng().random_range(0x30..0x3a).into()
}
