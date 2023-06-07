pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();
    for ch in input.chars().rev() {
        reversed.push(ch);
    }
    reversed
}

pub fn main() {
    let input = "Hello, world!";
    let reversed = reverse(input);
    println!("Reversed: {}", reversed);
}
