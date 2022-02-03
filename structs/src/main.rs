fn main() {
    println!("Begin of Program");

    let user1 = User {
    	name: String::from("Mettkov"),
    	email: String::from("some@gmail.com"),
    	active_count: 1
    };

    let user2 = User {
    	..user1
    };

    //user2.email = user2.email + "weee";

    println!("user1");
    println!("Name is {}", user1.name); // error! we moved user1.name into user2.name
    println!("Email is {}", user1.email); //error! we moved user1.email into user2.email
    println!("Active Count is {}", user1.active_count);
    
    println!("");
    
    println!("user2");
    println!("Name is {}", user2.name);
    println!("Email is {}", user2.email);
    println!("Active Count is {}", user2.active_count);
}

struct User {
	name: String,
	email: String,
	active_count: u64
}