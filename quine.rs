fn main() {
	println!("{}{}{}{}{}{}{}{}",S,0x72 as char,0x23 as char,0x22 as char,S,0x22 as char,0x23 as char,0x3B as char);
}
const S:&str = r#"fn main() {
	println!("{}{}{}{}{}{}{}{}",S,0x72 as char,0x23 as char,0x22 as char,S,0x22 as char,0x23 as char,0x3B as char);
}
const S:&str = "#;
