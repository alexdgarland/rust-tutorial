use super::{List, cons, Nil};
use crate::test_helpers::WrappedInt;
use crate::cons_enum::cons_list_from_vector;

fn nil_int_list() -> List<i32> {
    Nil
}

fn example_int_list() -> List<i32> {
    cons_list_from_vector(vec ![1, 2, 3])
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
fn from_empty_vector_i32() {
    let vector: Vec<i32> = vec![];
    let list = cons_list_from_vector(vector);
    assert_eq!(list, Nil);
}

#[test]
fn from_populated_vector_i32() {
    let expected_list = cons(1, cons(2, cons(3,  Nil)));
    let actual_list = cons_list_from_vector(vec![1, 2, 3]);
    assert_eq!(actual_list, expected_list);

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

#[test]
fn clone_for_empty_list() {
    assert_eq!(
        nil_int_list().clone(),
        Nil
    )
}

#[test]
fn clone_for_populated_list_i32() {
    let original = example_int_list();
    let cloned = original.clone();
    assert_eq!(cloned, original);
    let original_pointer_address = format!("{:p}", &original);
    let cloned_pointer_address = format!("{:p}", &cloned);
    assert_ne!(original_pointer_address, cloned_pointer_address);
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

#[test]
fn take_for_empty_list() {
    assert_eq!(
        nil_int_list().take(2),
        Nil
    );
}

#[test]
fn take_for_populated_list_i32() {
    assert_eq!(
        example_int_list().take(2),
        cons(1, cons(2, Nil))
    );
}

#[test]
fn take_more_than_length_for_populated_list_i32() {
    assert_eq!(
        example_int_list().take(5),
        cons(1, cons(2, cons(3, Nil)))
    );
}

#[test]
fn take_while_for_empty_list() {
    assert_eq!(
        nil_int_list().take_while(|i: &i32| *i < 3),
        Nil
    );
}

#[test]
fn take_while_for_populated_list_i32() {
    assert_eq!(
        example_int_list().take_while(|i: &i32| *i < 3),
        cons(1, cons(2, Nil))
    );
}

#[test]
fn take_while_more_than_length_for_populated_list_i32() {
    assert_eq!(
        example_int_list().take_while(|i: &i32| *i < 100),
        cons(1, cons(2, cons(3, Nil)))
    );
}

#[test]
fn drop_for_empty_list() {
    assert_eq!(
        nil_int_list().drop(2),
        Nil
    );
}

#[test]
fn drop_for_populated_list_i32() {
    assert_eq!(
        example_int_list().drop(2),
        cons(3, Nil)
    );
}

#[test]
fn drop_more_than_length_for_populated_list_i32() {
    assert_eq!(
        example_int_list().drop(5),
        Nil
    );
}

#[test]
fn drop_while_for_empty_list() {
    assert_eq!(
        nil_int_list().drop_while(is_even),
        Nil
    );
}

#[test]
fn drop_while_for_populated_list_i32() {
    let list = cons_list_from_vector(vec![2, 4, 6, 1, 8, 10, 12]);
    assert_eq!(
        list.drop_while(is_even),
        cons_list_from_vector(vec![1, 8, 10, 12])
    );
}

#[test]
fn drop_while_condition_always_applies_for_populated_list_i32() {
    let list = cons_list_from_vector(vec![2, 4, 6, 8, 10, 12]);
    assert_eq!(
        list.drop_while(is_even),
        Nil
    );
}

#[test]
fn drop_while_condition_never_applies_for_populated_list_i32() {
    let list = cons_list_from_vector(vec![1, 3, 5, 7, 9, 11]);
    assert_eq!(
        list.drop_while(is_even),
        list
    );
}

#[test]
fn for_each_can_use_closure_to_add_to_vector_from_list_i32() {
    let mut vector: Vec<i32> = vec![];
    let list = example_int_list();

    list.for_each(
        |el| {
            vector.push(el.clone())
        }
    );

    assert_eq!(vector, vec![1, 2, 3])
}
