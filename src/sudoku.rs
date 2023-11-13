use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

pub struct Solution {}

struct Sudoku<'a> {
    board: &'a mut Vec<Vec<char>>,
    available: HashMap<(usize, usize), HashSet<char>>,
}

impl<'a> Sudoku<'a> {
    fn create_full_set() -> HashSet<char> {
        HashSet::from(['1', '2', '3', '4', '5', '6', '7', '8', '9'])
    }

    fn row(y: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..9).map(move |x| (x, y))
    }

    fn column(x: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..9).map(move |y| (x, y))
    }

    fn cell_box(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let box_x = x / 3;
        let box_y = y / 3;
        (0..3)
            .flat_map(move |mod_x| (0..3).map(move |mod_y| (box_x * 3 + mod_x, box_y * 3 + mod_y)))
    }

    fn remove(&mut self, coord: (usize, usize), values: &mut HashSet<char>) {
        if self.board[coord.1][coord.0] != '.' {
            values.remove(&self.board[coord.1][coord.0]);
        }
    }

    fn get_avaliable_for(&mut self, position: (usize, usize)) -> HashSet<char> {
        if self.board[position.1][position.0] != '.' {
            return HashSet::from([]);
        }
        let mut data: HashSet<char> = Self::create_full_set();
        Self::row(position.1).for_each(|coord| self.remove(coord, &mut data));
        Self::column(position.0).for_each(|coord| self.remove(coord, &mut data));
        Self::cell_box(position.0, position.1).for_each(|coord| self.remove(coord, &mut data));
        data
    }

    fn new(board: &'a mut Vec<Vec<char>>) -> Self {
        let data: HashMap<(usize, usize), HashSet<char>> = HashMap::with_capacity(81);
        Sudoku {
            available: data,
            board: board,
        }
    }

    fn init(&mut self) {
        for x in 0..9 {
            for y in 0..9 {
            // let x = 1;
            // let y = 4;
                let available = self.get_avaliable_for((x, y));
                if available.len() > 0 {
                    self.available.insert((x, y), available);
                }
            }
        }
    }

    fn set_only_one(&mut self) -> usize {
        let mut stack: Vec<((usize, usize), char)> = Vec::from([]);
        for (coord, val) in self.available.iter() {
            if val.len() == 1 {
                let v = val.iter().next().unwrap();
                stack.push((coord.clone(), v.to_owned()))
            }
        }
        let mut counter: usize = 0;
        while stack.len() > 0 {
            let item = stack.pop().unwrap();
            self.update(&item.0, item.1);
            Self::row(item.0.1).for_each(|coord| self.remove_available(&coord, &item.1));
            Self::column(item.0.0).for_each(|coord| self.remove_available(&coord, &item.1));
            Self::cell_box(item.0.0, item.0.1).for_each(|coord| self.remove_available(&coord, &item.1));
            counter += 1
        }
        println!("UPDATED: {}", counter);
        counter
    }

    fn update(&mut self, coord: &(usize, usize), val: char) {
        self.board[coord.1][coord.0] = val;
        println!("- [{:?}] -> {}", coord, val)
    }

    fn remove_available(&mut self, coord: &(usize, usize), val: &char) {
        if let Some(values) = self.available.get_mut(coord) {
            values.remove(val);
            if values.len() == 0 {
                self.available.remove(&coord);
            }
        }
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut data = Sudoku::new(board);
        data.init();
        println!("A: {:?}", data.available);


        let mut max_iter = 12;
        while data.available.len() > 0 && max_iter > 0 {
            println!("LEN: {}", data.available.len());
            let count = data.set_only_one();
            if count == 0 {
                break;
            }
            max_iter -= 1;
            println!("A: {:?}", data.available);
        }


        // let mut counter = 0;
        // for (coord, val) in data.available.iter() {
        //     if val.len() == 1 {
        //         counter += 1;
        //         println!("*: {:?}: {:?}", coord, val);
        //     }
        // }

        // // let mut counter = 0;
        // println!("A: {:?}", data.available);

        // Calc available values

        // Find cell with avaliable 1 value

        // Set value where available only one value
        // And save cell with update value

        // Iteration by updated values for remove unavaliadle value

        // Repeate
    }
}

#[cfg(test)]
mod single_number {
    use super::Solution;

    #[test]
    fn case1() {
        let sum = Solution::single_number(Vec::from([1,4,1]));
        assert_eq!(sum, 4);
    }
}