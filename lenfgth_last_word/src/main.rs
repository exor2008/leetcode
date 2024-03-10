fn main() {
    println!(
        "{}",
        length_of_last_word(String::from("   fly   me    to the   moom   "))
    );
}

fn length_of_last_word(s: String) -> i32 {
    s.trim_end()
        .rsplit(" ")
        .next()
        .unwrap()
        .len()
        .try_into()
        .unwrap()
}
