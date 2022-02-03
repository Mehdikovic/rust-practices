fn main() {
	/*
    let s1 = String::from("Hello world!");
    let s2 = s1;

    println!("the s1 is: {}", s2);
    println!("the s1 is: {}", s1);
    */

    /*
    let s1 = String::from("hello");

    take_ownership(s1);

    // we have compile error here
    //println!("The value s1 is: {}", s1); 
    */


    /*
    let s1 = String::from("hello");
    let s2 = s1 + " world";
    //println!("The value s1 is: {}", s1); 
    println!("The value s2 is: {}", s2); 
	*/

    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}

/*
fn take_ownership(s: String) {
	println!("parameter 's' is: {}", s);
}
*/

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}