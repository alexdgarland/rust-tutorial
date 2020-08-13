mod int_list_stats;
mod pig_latin;

use int_list_stats::IntList;
pub use pig_latin::demo::demo_pig_latin;

pub fn demo_int_list() {
    info!("Showing populated int list");
    println!("{}", IntList { list: vec![1, 1, 4, 1, 2, 3, 10, 5, 6, 7, 8, 9, 4] });
    info!("Showing empty int list");
    println!("{}", IntList { list: vec![] });
}
