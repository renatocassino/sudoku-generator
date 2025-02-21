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
        std::thread::sleep(std::time::Duration::from_millis(0));
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

        if self.board[index as usize] != 0 {
            return self.create_number(index + 1);
        }

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
        let line = " ".repeat(200);
        for _ in 0..40 {
            println!("{}", line);
        }
        self.stdout.flush().unwrap();
    }

    fn fill_diagonal(&mut self, start_index: u32) {
        let mut numbers: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        numbers.shuffle(&mut rand::thread_rng());

        let mut index = start_index;
        for _i in 0..3 {
            for _j in 0..3 {
                let number = numbers.pop().unwrap();
                self.board[index as usize] = number;
                self.draw_board();
                index += 1;
            }
            index += 9 - 3;
        }
    }

    fn fill_diagonals(&mut self) {
        let indexes_to_start = [0, 30, 60];

        for index in indexes_to_start.iter() {
            self.fill_diagonal(*index);
        }
    }

    fn remove_random(&mut self) {
        let mut indexes: Vec<u32> = (0..81).collect();
        indexes.shuffle(&mut rand::thread_rng());

        for _ in 0..40 {
            let index = indexes.pop().unwrap();
            self.board[index as usize] = 0;
            self.draw_board();
        }
    }

    fn generate(&mut self, to_play: bool) {
        self.clear_terminal();
        self.draw_board();
        self.fill_diagonals();
        self.create_number(0);
        if to_play {
            self.remove_random();
        }
        self.draw_board();
    }

    fn solve(&mut self) {
        self.clear_terminal();
        self.draw_board();
        self.create_number(0);
        self.draw_board();
    }
}

fn main() {
    let mut board = Board::new(stdout());
    board.generate(true);

    // board.board = vec![
    //     0, 9, 7, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 5, 4, 6, 0, 0, 1, 7, 0, 0, 0, 0, 9,
    //     0, 0, 0, 0, 0, 5, 2, 0, 0, 0, 9, 8, 0, 0, 7, 0, 3, 0, 0, 0, 0, 1, 0, 0, 6, 0, 0, 0, 4, 0,
    //     0, 1, 0, 0, 0, 8, 5, 3, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0,
    // ];
    board.solve();
}
