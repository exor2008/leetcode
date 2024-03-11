fn main() {
    let c = jump(vec![2, 1, 1, 1, 1]);
    println!("{}", c);
}

fn jump(nums: Vec<i32>) -> i32 {
    let nums: Vec<(usize, usize, usize)> = nums
        .into_iter()
        .enumerate()
        .map(|(idx, num)| (idx, num as usize, idx + num as usize))
        .collect();

    let mut idx = 0;
    let l = nums.len();
    let mut n_jumps = 0;

    if l == 1 {
        return 0;
    } else if l == 1 {
        return 1;
    }

    if nums[idx].2 >= l - 1 {
        return 1;
    }

    loop {
        let (_, num, mut jump) = nums[idx];

        if num == 1 {
            idx += 1;
            (_, _, jump) = nums[idx];
        } else {
            let cur = nums[idx + 1..jump + 1]
                .iter()
                .max_by_key(|(_idx, _num, jump)| jump)
                .unwrap();

            (idx, _, jump) = *cur;
        }

        n_jumps += 1;

        if jump >= l - 1 {
            if idx == l - 1 {
                return n_jumps;
            } else {
                return n_jumps + 1;
            }
        }
    }
}
