// apply test attribute to the whole module
#![cfg(test)]

// specifies that the following function is a test, we can have get / set functions in a test file to assist a test that aren't tests themselves
#[test]
fn test_get_full_name_input() {
    let test_result = super::get_full_name("John", "Doe");

    // same as Laravel asserts
    assert_eq!(test_result, "John Doe");
}

#[test]
// should panic effectively asserts that we should return a panic
#[should_panic]
fn test_get_full_name_first_name_chars() {
    _ = super::get_full_name("$Jo^n", "Doe");
}

#[test]
#[should_panic]
fn test_get_full_name_last_name_chars() {
    // we want to test the last name so we've typed the first name correctly, else we'd hit the panic there, and the test would still be successful 
    _ =super::get_full_name("John", "D*e");
}