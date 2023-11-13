use std::{collections::HashMap, rc::Rc, cell::RefCell};

pub struct Solution {

}

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut data: HashMap<i32, Rc<RefCell<Vec<i32>>>> = HashMap::new();
        for item in adjacent_pairs {
            // let item = adjacent_pairs.pop().unwrap();
            println!("----------------------------");
            println!("I: {:?}", item);


            let key0 = item[0].clone();
            let key1 = item[1].clone();

            let e0 = data.get(&key0);
            let e1 = data.get(&key1);

            let mut removed_keys: Vec<i32> = Vec::with_capacity(2);
            let mut new_vec: Vec<i32> = Vec::from([]);
            if e0.is_none() && e1.is_none() {
                let value = Rc::new(RefCell::new(item));
                data.insert(key0, value.clone());
                data.insert(key1, value.clone());
            } else if e0.is_some() && e1.is_some() {
                let mut start = e0.unwrap().borrow_mut();
                let mut end = e1.unwrap().borrow_mut();
                if item.contains(&start[0]) {
                    start.reverse();
                }
                if item.contains(&end[end.len() - 1]) {
                    end.reverse();
                }
                new_vec.extend(start.clone());
                new_vec.extend(end.clone());
                // start.extend(end.clone());
                // end.clone_from(&start.clone());
                removed_keys.push(key0);
                removed_keys.push(key1);
            } else {
                let source = match e0 {
                    Some(s) => s,
                    None => e1.unwrap(),
                };
                let mut new_point: i32 = 0;
                {
                    let mut source_value = source.borrow_mut();
                    new_point = match item[0] == source_value[0] || item[0] == source_value[source_value.len() - 1] {
                        true => item[1],
                        false => item[0],
                    };
                    if item.contains(&source_value[0]) {
                        removed_keys.push(source_value[0]);
                        source_value.insert(0, new_point);
                        // println!("  >> insert");
                    } else {
                        removed_keys.push(source_value[source_value.len() - 1]);
                        source_value.push(new_point);
                        // println!("  >> push to {:?}", source_value);
                        // println!("  >> push to {:?}", data.get(&1).unwrap());
                    }
                }

                // println!("   source: {:?} ", source_value);
                // std::mem::drop(source_value);
                // println!("   source: {:?} ", source);
                data.insert(new_point, source.clone());

                // let a = data.get(&start[0]).unwrap();
                // let b = data.get(&start[start.len() - 1]).unwrap();
                // println!("   ==: {:?}", a == b);
            }
            if new_vec.len() > 0 {
                let key_s = new_vec[0];
                let key_e = new_vec[new_vec.len() - 1];
                let r = Rc::new(RefCell::new(new_vec));
                data.insert(key_s, r.clone());
                data.insert(key_e, r.clone());
            }
            println!("R: {:?} ", removed_keys);
            while removed_keys.len() > 0 {
                let k = removed_keys.pop().unwrap();
                data.remove(&k);
            }
            println!("D: {:?} ", data);
        }

        // assert!(data.len() == 2);
        // println!("== {:?}", data.get(&8).unwrap() == data.get(&2).unwrap());
        // println!("== {:?}", data.get(&1).unwrap() == data.get(&5).unwrap());
        let t = data.values().next().unwrap().borrow();
        t.clone()
    }
}