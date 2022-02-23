#[derive(Debug)]
// #[derive(Clone, Copy)]
struct Person {
	name: String,
	age: u8
}

impl Person {
	fn new(name: String, age: u8) -> Person {
		Person{name, age}
	}

	fn age_incr(&self, incr: u8) -> u8 {
		self.age + incr
	}

	fn age_incr_replace(&mut self, incr: u8) {
		self.age += incr;
	}
}

fn main() {
	let mut taro = Person::new(String::from("taro"), 10);
	println!("{:?}", taro);
	taro.age_incr_replace(5);
	let age_plut1 = taro.age_incr(1);
	println!("{}", age_plut1);
	println!("{:?}", taro);
}
