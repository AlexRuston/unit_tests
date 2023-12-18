mod unit_tests;

fn main() {
    let full_name = get_full_name("John", "Doe");

    println!("\nHello {}\nNice to meet you <3", full_name);
}

fn get_full_name(first_name: &str, last_name: &str) -> String {
    let bad_chars: &[char; 12] = &['!', '£', '$', '%', '^', '&', '*', '(', ')', '-', '+', '='];
    
    // exit program if the user tries any funny business
    if first_name.contains(bad_chars) || last_name.contains(bad_chars){
        panic!("Your name is weird with those special chars in it");
    }
    
    let mut full_name: String = "".to_string();

    full_name.push_str(first_name);
    full_name.push_str(" ");
    full_name.push_str(last_name);

    return full_name;
}
