fn main() {
	let mut str1 = String::from("String1"); 
    // what's gonna happen here, we don't have GC in rust, so...?
    // does compiler automatically call drop here?
    str1 = String::from("String2");

    println!("str is: {}", str1);
}
