fn main() {
	// simple loop
	/*
    loop {
    	println!("Hello loop!");
    	break;
    }
    */

    // easy loop
    /*
    let mut counter = 0;

    let result = loop {
    	counter += 1;

    	if counter == 15 {
    		break counter * 2;
    	}
    };

    println!("result is: {}", result);
    */

    // for loop
    /*
    let a = [10, 20, 30, 40, 50];

    for item in a.iter() {
    	println!("result is: {}", item);	
    }
    */

    for item in (1..4).rev() { // note the (1..4) to build a Range, (Inclusive..Exclusive)
    	println!("result is: {}", item);	
    }
}
