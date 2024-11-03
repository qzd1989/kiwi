use kiwi_lib::commands::fs;
fn main() {
    let path = "C:\\scripts\\hello1".to_string();
    let files = fs::read_dir(path).unwrap();
    println!("{:#?}", files);
}
