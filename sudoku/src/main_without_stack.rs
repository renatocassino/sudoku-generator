use crossterm::{cursor, queue};
use rand::seq::SliceRandom;
extern crate rand;
use std::io::{stdout, Stdout, Write};
extern crate sys_info;

struct Board {
    pub board: Vec<u32>,
    pub counter: u32,
    pub stdout: Stdout,
}

impl Board {
    fn new(stdout: Stdout) -> Board {
        Board {
            board: vec![0; 81],
            counter: 0,
            stdout,
        }
    }

    fn draw_board(&mut self) {
        let mut board_map: Vec<String> = vec![];
        board_map.push(format!("counter: {}", self.counter));
        board_map.push(String::from(""));

        let mut line: Vec<String> = vec![];
        let horizontal_line = String::from(" ------+-------+------ ");

        for (i, cell) in self.board.iter().enumerate() {
            if i % 9 == 0 && i > 0 {
                board_map.push(format!(" {}", line.join(" ")));
                line = vec![];
            } else if i % 3 == 0 && i > 0 {
                line.push(String::from("|"));
            }
            line.push(cell.to_string());

            if i == 27 || i == 54 {
                board_map.push(horizontal_line.clone());
            }
        }
        board_map.push(format!(" {} ", line.join(" ")));

        queue!(self.stdout, cursor::MoveTo(0, 0)).unwrap();
        println!("{}", board_map.join("\n"));
        self.stdout.flush().unwrap();
    }

    fn get_column(&self, index: u32) -> Vec<u32> {
        let mut column: Vec<u32> = vec![];

        if index >= 9 {
            let mut i = index - 9;
            while i > 0 {
                if self.board[i as usize] == 0 {
                    i -= 9;
                    continue;
                }
                column.push(self.board[i as usize]);
                if i < 9 {
                    break;
                }
                i -= 9;
            }
        }

        let mut i = index + 9;
        while i < self.board.len() as u32 {
            if self.board[i as usize] == 0 {
                i += 9;
                continue;
            }
            column.push(self.board[i as usize]);
            i += 9;
        }

        return column;
    }

    fn get_line(&self, index: u32) -> Vec<u32> {
        let mut line: Vec<u32> = vec![];
        let start = index - (index % 9);
        let mut i = start;
        while i < start + 9 {
            if self.board[i as usize] == 0 {
                i += 1;
                continue;
            }
            line.push(self.board[i as usize]);
            i += 1;
        }

        return line;
    }

    fn get_quadrant_number(&self, index: u32) -> [u32; 2] {
        let quadrant_x = (index / 3) % 3;
        let quadrant_y = (index / 9) / 3;

        return [quadrant_x, quadrant_y];
    }

    fn get_quadrant_by_index(&self, index: u32) -> Vec<u32> {
        let mut quadrant: Vec<u32> = vec![];
        let [quadrant_x, quadrant_y] = self.get_quadrant_number(index);

        let start = (quadrant_y * 27) + (quadrant_x * 3);
        let mut i = start;
        while i < start + 3 {
            if self.board[i as usize] == 0 {
                i += 1;
                continue;
            }
            quadrant.push(self.board[i as usize]);
            i += 1;
        }

        i = start + 9;
        while i < start + 12 {
            if self.board[i as usize] == 0 {
                i += 1;
                continue;
            }
            quadrant.push(self.board[i as usize]);
            i += 1;
        }

        i = start + 18;
        while i < start + 21 {
            if self.board[i as usize] == 0 {
                i += 1;
                continue;
            }
            quadrant.push(self.board[i as usize]);
            i += 1;
        }

        return quadrant;
    }

    fn get_possible_numbers_by_index(&self, index: u32) -> Vec<u32> {
        let numbers: Vec<u32> = [
            &self.get_column(index)[..],
            &self.get_line(index)[..],
            &self.get_quadrant_by_index(index)[..],
        ]
        .concat();

        let mut all_numbers = std::collections::HashSet::new();
        all_numbers.insert(1);
        all_numbers.insert(2);
        all_numbers.insert(3);
        all_numbers.insert(4);
        all_numbers.insert(5);
        all_numbers.insert(6);
        all_numbers.insert(7);
        all_numbers.insert(8);
        all_numbers.insert(9);

        for number in numbers {
            all_numbers.remove(&number);
        }

        return all_numbers.into_iter().collect();
    }

    fn create_number(&mut self, index: u32) -> bool {
        if index >= 81 {
            return true;
        }

        self.draw_board();

        let mut possible_numbers = self.get_possible_numbers_by_index(index);
        possible_numbers.shuffle(&mut rand::thread_rng());

        for number in possible_numbers {
            self.board[index as usize] = number;
            self.counter += 1;
            if self.create_number(index + 1) {
                return true;
            }
        }

        self.board[index as usize] = 0;
        return false;
    }

    fn clear_terminal(&mut self) {
        queue!(self.stdout, cursor::MoveTo(0, 0)).unwrap();
        for _ in 0..40 {
            println!("                                             ");
        }
        self.stdout.flush().unwrap();
    }

    fn generate(&mut self) {
        self.clear_terminal();
        self.draw_board();
        self.create_number(0);
        self.draw_board();
    }
}

fn main() {
    let mut board = Board::new(stdout());
    board.generate();
}
