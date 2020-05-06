mod int_list_stats;

use int_list_stats::IntList;

pub fn demo_int_list() {
    println!("{}", IntList { list: vec![1, 1, 4, 1, 2, 3, 10, 5, 6, 7, 8, 9, 4] });
    println!("{}", IntList { list: vec![] });
}
