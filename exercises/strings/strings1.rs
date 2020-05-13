// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)


fn main() {
    let answer = current_favorr();
    println!("My favorite color is {}", answer);
}

fn current_favorr() -> String {
    String::from("blue")
}
