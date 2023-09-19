use rand::seq::SliceRandom;

struct Board {
    pub board: Vec<u32>,
    pub stack: Vec<Vec<u32>>,
    pub index: u32,
    pub counter: u32,
}

impl Board {
    fn new() -> Board {
        Board {
            board: vec![0; 81],
            stack: vec![],
            index: 0,
            counter: 0,
        }
    }

    fn draw_board(&self) {
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

        println!("{}", board_map.join("\n"));

        // self.drawDebug();
    }

    fn get_column(&self, index: u32) -> Vec<u32> {
        let mut column: Vec<u32> = vec![];

        if index > 9 {
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

    fn create_number(&mut self) {
        let mut possible_numbers: Vec<u32> = vec![];
        if (self.index as usize) < self.stack.len() && !self.stack[self.index as usize].is_empty() {
            self.stack[self.index as usize] = self.stack[self.index as usize]
                .iter()
                .filter(|&&n| n != self.board[self.index as usize])
                .cloned()
                .collect();

            possible_numbers = self.stack[self.index as usize].clone();
        } else {
            possible_numbers = self.get_possible_numbers_by_index(self.index);
        }

        if possible_numbers.len() == 0 {
            self.board[self.index as usize] = 0;
            self.index -= 1;
            return;
        }

        let random_number = *possible_numbers.choose(&mut rand::thread_rng()).unwrap();

        while self.index >= self.stack.len() as u32 {
            self.stack.push(Vec::new());
        }
        self.stack[self.index as usize] = possible_numbers;
        self.board[self.index as usize] = random_number;
        self.index += 1;
    }

    fn is_finished(&self) -> bool {
        // return this.board[this.board.length - 1] !== 0;
        if self.board[self.board.len() - 1] == 0 {
            return false;
        }
        return true;
    }

    fn generate(&mut self) {
        self.draw_board();
        loop {
            self.create_number();
            self.counter += 1;
            self.draw_board();

            if self.is_finished() {
                break;
            }
        }
    }
}

fn main() {
    let mut board = Board::new();
    board.generate();
}
