mod reverse_string;

fn main() {
    let file_name = "reverse_string";
    match file_name {
        file_name => reverse_string::main(),
        _ => println!("Invalid file name"),
    }
}
