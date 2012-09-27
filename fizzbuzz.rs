// Language: Rust
// Web site: http://www.rust-lang.org/
// Ubuntu:   Build compiler from source

fn main() {
    let mut i = 1;
    while i <= 100 {
        io::println( if i % 15 == 0 { "FizzBuzz" }
                     else if i % 3 == 0 { "Fizz" }
                     else if i % 5 == 0 { "Buzz" }
                     else { #fmt("%d", i) });
        i += 1;
    }
}