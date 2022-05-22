pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut queue: Vec<Vec<i32>> = Vec::new();

    if amount == 0 {
        return 0;
    }

    for coin in &coins {
        queue.push(vec![*coin]);
    }

    loop {
        if queue.is_empty() {
            return -1;
        }

        let coin_combo: Vec<i32> = queue.pop().expect("failed to read queue");
        let valuation: i32 = coin_combo.iter().sum();

        if valuation == amount {
            return coin_combo.len() as i32;
        }

        if valuation > amount {
            continue;
        }

        if valuation < amount {
            for coin in &coins {
                let mut next_iteration: Vec<i32> = coin_combo.clone();
                next_iteration.push(*coin);
                queue.push(next_iteration);
            }
        }

        continue;
    }
}

fn main() {
    dbg!(coin_change(vec![1, 3, 5], 11));
    dbg!(coin_change(vec![2], 3));
    dbg!(coin_change(vec![1], 0));
}
