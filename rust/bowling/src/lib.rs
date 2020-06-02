#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Clone, Debug)]
struct Frame {
    last: bool,
    pins: u16,
    rolls: Vec<u16>,
}

impl Frame {
    pub fn new(last: bool) -> Self {
        Frame {
            last: last,
            pins: 10,
            rolls: Vec::new(),
        }
    }

    pub fn score(&self) -> u16 {
        self.rolls.iter()
            .sum()
    }

    pub fn roll(&mut self, pins: u16) -> Option<Error> {
        if self.pins < pins {
            return Some(Error::NotEnoughPinsLeft);
        }

        self.pins -= pins;
        self.rolls.push(pins);

        if self.last && self.rolls.len() < 3 {
            if pins == 10 {
                self.pins += 10;
            } else if self.score() == 10 {
                self.pins += 10;
            }
        }
        None
    }

    pub fn is_strike(&self) -> bool {
        self.rolls.len() > 0 &&
            self.rolls[0] == 10
    }

    pub fn is_spare(&self) -> bool {
        self.rolls.len() == 2 &&
            self.rolls.iter()
                .sum::<u16>() == 10
    }

    pub fn done(&self) -> bool {
        !(self.last &&
            (self.is_strike() || self.is_spare()) &&
            self.rolls.len() < 3) &&
            (self.rolls.len() >= 2 || self.is_strike())
    }
}

pub struct BowlingGame {
    current: Frame,
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            current: Frame::new(false),
            frames: Vec::new(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames.len() == 10 {
            return Err(Error::GameComplete);
        }

        match self.current.roll(pins) {
            Some(err) => Err(err),
            None => {
                if self.current.done() {
                    self.frames.push(self.current.clone());
                    self.current = Frame::new(self.frames.len() == 9)
                }
                Ok(())
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.len() < 10 {
            return None;
        }

        let mut score: u16 = 0;
        for (i, frame) in self.frames.iter()
            .enumerate() {
            score += frame.score();

            let future_rolls = self.get_rolls_after_frame(i + 1);
            if frame.is_strike() {
                score += future_rolls.iter()
                    .take(2)
                    .sum::<u16>();
            } else if frame.is_spare() {
                score += future_rolls.iter()
                    .take(1)
                    .sum::<u16>();
            }
        }
        Some(score)
    }

    fn get_rolls_after_frame(&self, from_frame: usize) -> Vec<u16> {
        self.frames[from_frame..]
            .iter()
            .flat_map::<Vec<u16>, _>(|f| {
                f.rolls
                    .iter()
                    .map(|&n| n)
                    .collect()
            })
            .collect()
    }
}
