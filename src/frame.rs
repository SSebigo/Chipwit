pub const FRAME_WIDTH: usize = 64;
pub const FRAME_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Frame {
    cols: usize,
    data: [[u8; FRAME_WIDTH]; FRAME_HEIGHT],
    rows: usize,
}

impl Frame {
    pub fn new() -> Frame {
        Self {
            cols: FRAME_WIDTH,
            data: [[0; FRAME_WIDTH]; FRAME_HEIGHT],
            rows: FRAME_HEIGHT,
        }
    }

    pub fn at(&mut self, c: usize, r: usize) -> u8 {
        self.data[r][c]
    }

    pub fn draw_to_stdout(&self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.data[r][c] > 0 {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
        println!("");
    }

    pub fn set_all(&mut self, val: u8) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                self.data[r][c] = val;
            }
        }
    }

    pub fn set_one(&mut self, c: usize, r: usize, val: u8) {
        self.data[r][c] ^= val;
    }
}
