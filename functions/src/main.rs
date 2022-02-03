fn main() {
    //let x = another_function(5, -10); // it return '()' an emoty tuple
    //println!("The value of x: {}", x);
    println!("The value of function is: {}", ret_function());
}

fn another_function(x: u32, y: i32) {
	println!("The value of x:u32: {}", x);
	println!("The value of y:i32: {}", y);
}

fn ret_function() -> i32 {
	10
}