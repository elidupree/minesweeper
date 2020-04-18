use std::cmp::min;

pub struct BoardIter {
    x: usize,
    y: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
    cur_x: usize,
    cur_y: usize,
}

impl Iterator for BoardIter {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        if self.cur_x == self.max_x {
            None
        } else {
            let res = (self.cur_x, self.cur_y);

            // Compute next one.
            self.cur_y += 1;
            if self.cur_x == self.x && self.cur_y == self.y {
                // Skip over the starting element.
                self.cur_y += 1
            }
            if self.cur_y >= self.max_y {
                self.cur_y = self.min_y;
                self.cur_x += 1;
            }
            Some(res)
        }
    }
}

pub fn board_iter(x: usize, y: usize, height: usize, width: usize) -> BoardIter {
    let min_x = if x == 0 { 0 } else { x - 1 };
    let min_y = if y == 0 { 0 } else { y - 1 };
    let max_x = min(x + 2, height);
    let max_y = min(y + 2, width);
    BoardIter {
        x: x,
        y: y,
        min_y: min_y,
        max_x: max_x,
        max_y: max_y,
        cur_x: min_x,
        cur_y: min_y,
    }
}
