use mapgen::parser::Language;


fn main() {
    println!("Hello, world!");
    let language = Language::from_extension("rs").unwrap();
    println!("{:?}", language);
}
