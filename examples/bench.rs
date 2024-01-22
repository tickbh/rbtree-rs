extern crate rbtree;
use rbtree::RBTree;
use std::time::{Duration, Instant};
use std::cmp;

fn duration_to_num(duration: Duration) -> u64 {
    duration.as_secs() * 1_000_000_000 + duration.subsec_nanos() as u64
}

fn test_insert(repeat: u64, insert: u64) {
    let mut sum = 0;
    let mut max = 0;
    let mut min = ::std::u64::MAX;

    let mut get_sum = 0;
    let mut get_max = 0;
    let mut get_min = ::std::u64::MAX;

    let mut remove_sum = 0;
    let mut remove_max = 0;
    let mut remove_min = ::std::u64::MAX;


    for _ in 0..repeat {
        let now = Instant::now();
        let mut a = RBTree::new();
        for i in 0..insert {
            a.insert(i, i * 2);
        }
        let new_now = Instant::now();
        let cost = duration_to_num(new_now.duration_since(now));
        sum += cost;
        max = cmp::max(cost, max);
        min = cmp::min(cost, min);


        let now = Instant::now();
        assert_eq!(a.get(&20).unwrap(), &40);
        let new_now = Instant::now();
        let cost = duration_to_num(new_now.duration_since(now));
        get_sum += cost;
        get_max = cmp::max(cost, get_max);
        get_min = cmp::min(cost, get_min);


        let now = Instant::now();
        assert_eq!(a.remove(&20).unwrap(), 40);
        let new_now = Instant::now();
        let cost = duration_to_num(new_now.duration_since(now));
        remove_sum += cost;
        remove_max = cmp::max(cost, remove_max);
        remove_min = cmp::min(cost, remove_min);
    }

    println!("-----------All Test Repeat: {}, All Tree Num: {}-------------------", repeat, insert);
    println!("Insert Test,           Max Cost: {}us, Min Cost: {}us, Aver Cost: {}us", 
            max / 1000,
            min / 1000,
            sum / 1000 / repeat);

    println!("Get data by key=20,    Max Cost: {}ns, Min Cost: {}ns, Aver Cost: {}ns", 
            get_max,
            get_min,
            get_sum / repeat);

    println!("Remove data by key=20, Max Cost: {}ns, Min Cost: {}ns, Aver Cost: {}ns", 
            remove_max,
            remove_min,
            remove_sum / repeat);

    println!("-----------End Tree Test----------------------------------------------\n");

}

fn main() {
    test_insert(10, 1000);
    test_insert(10, 10000);
    test_insert(10, 100000);
    test_insert(10, 1000000);
    // test_insert(10, 10000000);
    // test_insert(10, 100000000);
    // test_insert(10, 1000000000);
    // test_insert(10, 10000000000);
}
