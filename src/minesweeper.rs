use std::{
    collections::HashSet,
    fmt::{Display, Write},
  };

use crate::random::random_range;

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
    Flagged,
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    lost: bool,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);

                if !self.open_fields.contains(&pos) {
                    if self.lost && self.mines.contains(&pos) {
                        f.write_str("💣 ")?;
                    } else if self.flagged_fields.contains(&pos) {
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("🟪 ")?;
                    }
                } else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    let mine_count = self.neighbor_mines_count(pos);

                    if mine_count > 0 {
                        write!(f, " {} ", mine_count)?;
                    } else {
                        f.write_str("⬜ ")?;
                    }
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();
                while mines.len() < mine_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }
                mines
            },
            flagged_fields: HashSet::new(),
            lost: false,
        }
    }

    pub fn iter_neighbor(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    pub fn neighbor_mines_count(&self, (x, y): Position) -> u8 {
        self.iter_neighbor((x,y)).filter(|pos| self.mines.contains(pos)).count() as u8
    }

    pub fn open(&mut self, position: Position) -> Option<OpenResult> {
        if self.lost || self.open_fields.contains(&position) {
            return None
        }

        if self.flagged_fields.contains(&position) {
            return Some(OpenResult::Flagged);
        }
        self.open_fields.insert(position);

        let is_mine = self.mines.contains(&position);

        if is_mine {
            self.lost = true;
            Some(OpenResult::Mine)
        } else {
            let mine_count = self.neighbor_mines_count(position);

            if mine_count == 0 {
                for neighbor in self.iter_neighbor(position) {
                    if !self.open_fields.contains(&neighbor) {
                        self.open(neighbor);
                    }
                }
            }

            Some(OpenResult::NoMine(mine_count))
        }
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.lost {
            return
        }

        if !self.open_fields.contains(&pos) {
            if !self.flagged_fields.contains(&pos) {
                self.flagged_fields.insert(pos);
            } else {
                self.flagged_fields.remove(&pos);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::minesweeper::Minesweeper;

    #[test]
    fn test() {
        let mut ms = Minesweeper::new(10, 10, 25);
        ms.open((5, 5));
        ms.toggle_flag((2, 7));
        println!("{}", ms);

        ms.toggle_flag((5,6));
        ms.open((5, 6));
        ms.toggle_flag((2, 7));

        println!("{}", ms);
    }

    // #[test]
    // fn neighbor() {
    //     let minesweeper = Minesweeper::new(10, 10, 20);
    //     let position = (9, 3);
    //     let nh_iter = minesweeper.neighbor(position);

    //     let nh: Vec<_> = nh_iter.collect();

    //     // Print the Vec<Position>
    //     println!("{:?}", nh);
    // }

    // #[test]
    // fn mine_count() {
    //     let minesweeper = Minesweeper::new(10, 10, 50);
    //     println!("{:?}", minesweeper);

    //     let position = (9, 3);

    //     let nh_iter = minesweeper.neighbor(position);
    //     let nh: Vec<_> = nh_iter.collect();

    //     // Print the Vec<Position>
    //     println!("{:?}", nh);

    //     let count = minesweeper.neighbor_mines_count(position);

    //     println!("{}", count);
    // }
}
