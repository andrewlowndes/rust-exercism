#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const FRAMES_PER_GAME: u16 = 10;
const PINS_PER_FRAME: u16 = 10;
const THROWS_PER_FRAME: u16 = 2;

struct Frame {
    throws: Vec<u16>,
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}

impl Frame {
    pub fn new() -> Self {
        Frame { throws: vec![] }
    }
}

pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: vec![Frame::new()],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let frame_number = self.frames.len() as u16;

        if frame_number > FRAMES_PER_GAME {
            return Err(Error::GameComplete);
        }

        let is_final_frame = frame_number == FRAMES_PER_GAME;
        let frame = self.frames.last_mut().unwrap();
        let pins_down: u16 = frame.throws.iter().sum();

        let pins_left = PINS_PER_FRAME - (pins_down % PINS_PER_FRAME);
        let mut fill_ball = 0;

        if is_final_frame && pins_down + pins >= PINS_PER_FRAME {
            fill_ball = 1;
        }

        if pins > pins_left {
            return Err(Error::NotEnoughPinsLeft);
        }

        frame.throws.push(pins);

        let num_throws = frame.throws.len() as u16;

        let end_frame =
            num_throws >= THROWS_PER_FRAME + fill_ball || (!is_final_frame && pins_left == pins);
        if end_frame {
            self.frames.push(Frame::new());
        }

        Ok(())
    }

    fn frame_score(&self, index: u16) -> u16 {
        let frame = self.frames.get(index as usize).unwrap();

        if index == FRAMES_PER_GAME - 1 {
            return frame.throws.iter().sum();
        }

        let frame_score = frame.throws.iter().sum();

        let is_strike = (frame.throws.len() as u16) < THROWS_PER_FRAME;
        if is_strike {
            let next_frame = self.frames.get((index + 1) as usize).unwrap();

            if next_frame.throws.len() == 1 {
                let third_frame = self.frames.get((index + 2) as usize).unwrap();
                return frame_score
                    + next_frame.throws.first().unwrap()
                    + third_frame.throws.first().unwrap();
            }

            return frame_score + next_frame.throws.iter().take(2).sum::<u16>();
        }

        let is_spare = frame_score == PINS_PER_FRAME;
        if is_spare {
            let next_frame = self.frames.get((index + 1) as usize).unwrap();
            return frame_score + next_frame.throws.first().unwrap();
        }

        frame_score
    }

    pub fn score(&self) -> Option<u16> {
        let num_frames = self.frames.len() as u16;

        if num_frames > FRAMES_PER_GAME {
            Some(
                (0..FRAMES_PER_GAME)
                    .map(|number| self.frame_score(number))
                    .sum(),
            )
        } else {
            None
        }
    }
}
