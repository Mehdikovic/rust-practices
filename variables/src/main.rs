fn main() {
	// CONST
	/*
	const CONST_VALUE : u32 = 100_000;
	println!("Const value is: {}", CONST_VALUE);
	*/
	

	// immutability
	/*
	let x = 5;
    println!("The value is: {}", x);
    x = 6; // WE HAVE COMPILE ERROR HERE!
    println!("The value is: {}", x);
	*/

	// mutability
	/*
	let mut x = 5;
    println!("The value is: {}", x);
    x = 6;
    println!("The value is: {}", x);
	*/

	// shadowing
	/*
	let x = 5;
	let x = x + 1;
	let x = x * 2;
	println!("The value is: {}", x); // THE X WILL BE 12 and shadowed the previous ones
	*/
}
