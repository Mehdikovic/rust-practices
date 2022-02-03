#[derive(Debug)]
struct Rect {
	width: u32,
	height: u32
}

impl Rect {
	// methods
	fn area(&self) -> u32{
		self.width * self.height	
	}

	// associative function [it's like the static methods on objects]
	fn square(size: u32) -> Rect {
		Rect {
			width: size,
			height: size
		}
	}
}



fn main() {
    let rect1 = Rect {
    	width: 20,
    	height: 40
    };

    println!("The area of rect1 is: {}", rect1.area());
    println!("The area of Rect::square(50) is: {}", Rect::square(50).area());
	println!("");
	println!("Rect type is: {:#?}", rect1);
	println!("Rect type is: {:#?}", Rect::square(50));

}