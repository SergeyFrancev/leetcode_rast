

fn success_clone_push(data: &mut HashMap<i32, Rc<RefCell<Vec<i32>>>>) {
    {
        let mut ss = data.get(&2).unwrap();
        // let mut s = ss.borrow_mut();
        // let mut t = data.get_mut(&3).unwrap().borrow_mut();
        // s.reverse();
        // s.extend(t.iter());
        data.insert(3, ss.clone());
    }

    {
        let mut s1 = data.get(&3).unwrap();
        data.insert(4, s1.clone());
    }

    {
        let mut s0 = data.get(&2).unwrap();
        let mut s1 = data.get(&4).unwrap();
        data.get(&4).unwrap().borrow_mut().push(4);
        s0.clone_from(&s1);
        data.get(&1).unwrap().borrow_mut().push(5);
    }

    // data.remove(&2);
    // data.remove(&3);
    println!("D {:?}", data);
}