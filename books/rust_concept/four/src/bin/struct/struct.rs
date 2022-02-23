struct Person {
	name: String,
	age: u8,
}
fn main() {
	let taro = Person{name: String::from("taro"), age: 10};
	println!("{} {}", taro.name, taro.age);

	let mut taro = Person{name: String::from("taro"), age: 10};
	taro.age = 30;
	println!("{} {}", taro.name, taro.age);

	let name = String::from("hana");
	let age = 30;
	let hana = Person{
		name, age,
	};

	println!("{}, {}", hana.name, hana.age);


	let taro = Person{name: String::from("taro"), age: 20};
	let jiro = Person{
		name: String::from("jiro"),
		..taro
	};
	println!("{}, {}", jiro.name, jiro.age);
}
