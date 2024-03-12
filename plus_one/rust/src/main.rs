fn main() {
    let v = plus_one([9, 9].to_vec());
    println!("{:?}", v);
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits.clone();
    let mut mem = true;
    for num in digits.iter_mut().rev() {
        if mem {
            mem = false;
            *num += 1;
        }

        if *num == 10 {
            *num = 0;
            mem = true;
        }
    }

    if mem {
        digits.insert(0, 1);
    }

    digits
}
