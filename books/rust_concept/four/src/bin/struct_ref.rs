#[derive(Debug)]
struct Person {
	name: String,
	age: u8
}

#[derive(Debug)]
struct Parents <'a, 'b>{
	father: &'a Person,
	mother: &'b Person
}

impl<'a, 'b> Parents<'a, 'b> {
	fn new(father: &'a Person, mother: &'b Person) -> Parents<'a, 'b> {
		Parents{father, mother}
	}
}

fn main() {
	let taro = Person {name: String::from("taro"), age: 50};
	let hanako = Person {name: String::from("hanako"), age: 48};

	let sato = Parents::new(
		&taro, &hanako
	);
	println!("{:?}", sato);
}
