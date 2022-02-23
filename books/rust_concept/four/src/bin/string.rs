fn main() {
	let _c = 'c';
	let _a = 'あ';

	let ss = "Hello";
	println!("{}", ss);
	println!("{}", &ss[0..2]);

	let st = "あいうえお";
	println!("{}", &st[0..6]);
	println!("{}", &st[0..3]);

	let _st1 = "Hello".to_string();
	let _st2 = String::from("Hello");
	let mut st = String::from("Hello");
	st.push_str(", World");
	println!("{}", st);
	println!("{}", &st[0..6]);

}
