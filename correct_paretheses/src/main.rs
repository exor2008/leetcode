fn main() {
    println!("{}", check());
}

fn check() -> bool {
    let s = String::from("{}[]({[]})");
    let mut stack: Vec<u32> = vec![];

    for l in s.chars() {
        match l {
            '{' | '(' | '[' => stack.push(l as u32),
            _ => {
                if let Some(c) = stack.pop() {
                    if l as u32 - c > 3 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}
