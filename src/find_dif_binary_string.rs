use std::{cell::RefCell, collections::VecDeque, mem, rc::Rc};

#[derive(Debug)]
struct Pos(Option<Rc<RefCell<Pos>>>, Option<Rc<RefCell<Pos>>>);

#[derive(Debug)]
struct Combi {
    root: Rc<RefCell<Pos>>,
}

impl Combi {
    fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(Pos(None, None))),
        }
    }

    // fn feed(&mut self, variant: String) {
    //     let mut cursor = Rc::clone(&self.root);
    //     for ch in variant.as_bytes() {
    //         // let mut item = cursor.pop().unwrap();
    //         let mut x = *cursor.borrow_mut();
    //         match ch {
    //             b'0' => {
    //                 if x.0.is_none() {
    //                     let child = Rc::new(RefCell::new(Pos(None, None)));
    //                     // cursor.clone_from(&child.clone());
    //                     x.0 = Some(child);
    //                 }
    //                 // Some(x.0.unwrap())
    //             },
    //             b'1' => {
    //                 let mut x = cursor.borrow_mut();
    //                 if x.1.is_none() {
    //                     let child = Rc::new(RefCell::new(Pos(None, None)));
    //                     x.1 = Some(child);
    //                 }
    //                 cursor.clone_from(&x.1.unwrap());
    //             },
    //             _ => {}
    //         };
    //     }
    // }
}
pub struct Solution {}

impl Solution {
    pub fn find_different_binary_string(mut nums: Vec<String>) -> String {
        let mut store = vec![false; usize::pow(2, nums[0].len() as u32)];
        while nums.len() > 0 {
            let binary_string = nums.pop().unwrap();
            match usize::from_str_radix(binary_string.as_str(), 2) {
                Ok(binary_value) => {
                    // println!("Binary String: {}", binary_string);
                    // println!("Equivalent Binary Value: {}", binary_value);
                    store[binary_value] = true;
                }
                Err(err) => {
                    // eprintln!("Error: {}", err);
                }
            }
        }
        for (idx, i) in store.iter().enumerate() {
            if i == &false {
                return format!("{:0width$b}", idx, width = nums.len());
            }
        }

        String::from("")
    }
}
