use super::{List, cons, Nil};
use crate::test_helpers::WrappedInt;

fn nil_int_list() -> List<i32> {
    Nil
}

fn example_int_list() -> List<i32> {
    cons(1, cons(2, cons(3, Nil)))
}

#[test]
fn length_for_empty_list() {
    assert_eq!(nil_int_list().length(), 0)
}

#[test]
fn length_for_populated_list_i32() {
    assert_eq!(example_int_list().length(), 3);
}

#[test]
fn string_for_empty_list() {
    assert_eq!(nil_int_list().to_string(), "")
}

#[test]
fn string_for_populated_list_i32() {
    assert_eq!(
        example_int_list().to_string(),
        "1, 2, 3"
    );
}

#[test]
fn string_for_populated_list_str() {
    let cons_list =
        cons("one",
             cons("two",
                  cons("three", Nil)));
    assert_eq!(
        cons_list.to_string(),
        "one, two, three"
    );
}

#[test]
fn string_for_populated_list_struct_with_display() {
    let cons_list =
        cons(WrappedInt { i: 1 },
             cons(WrappedInt { i: 2 },
                  cons(WrappedInt { i: 3 }, Nil)));
    assert_eq!(
        cons_list.to_string(),
        "1, 2, 3"
    );
}

#[test]
fn to_vector_for_empty_list() {
    let expected: Vec<&i32> = vec![];
    assert_eq!(
        nil_int_list().to_vector(),
        expected
    );
}

#[test]
fn to_vector_for_populated_list_i32() {
    assert_eq!(
        example_int_list().to_vector(),
        vec![&1, &2, &3]
    );
}

#[test]
fn map_for_empty_list() {
    assert_eq!(
        nil_int_list().map(|i:&i32| i + 1).to_string(),
        ""
    );
}

#[test]
fn map_for_populated_list_i32() {
    assert_eq!(
        example_int_list().map(|i:&i32| i + 1).to_vector(),
        vec!(&2, &3, &4)
    );
}

fn add(i: &i32, j: i32) -> i32 {
    return i + j
}

#[test]
fn reduce_for_empty_list() {
    assert_eq!(
        nil_int_list().reduce(add),
        None
    );
}

#[test]
fn reduce_for_populated_list_i32() {
    assert_eq!(
        example_int_list().reduce(add),
        Some(6)
    );
}

fn join_strings(i: &i32, s: String) -> String {
    return format!("{}, {}", s, i);
}

#[test]
fn fold_for_empty_list() {
    assert_eq!(
        nil_int_list().fold_left(join_strings, "0".to_owned()),
        "0"
    );
}

#[test]
fn fold_for_populated_list_i32() {
    assert_eq!(
        example_int_list().fold_left(join_strings, "0".to_owned()),
        "0, 1, 2, 3"
    );
}

fn is_even(i: &i32) -> bool {
    return i % 2 == 0
}

#[test]
fn filter_for_empty_list() {
    assert_eq!(
        nil_int_list().filter(is_even),
        Nil
    );
}

#[test]
fn filter_for_populated_list_i32() {
    assert_eq!(
        example_int_list().filter(is_even),
        cons(2, Nil)
    );
}

#[test]
fn reverse_for_empty_list() {
    assert_eq!(
        nil_int_list().reverse(),
        Nil
    );
}

#[test]
fn reverse_for_populated_list_i32() {
    assert_eq!(
        example_int_list().reverse(),
        cons(3, cons(2, cons(1, Nil)))
    );
}
