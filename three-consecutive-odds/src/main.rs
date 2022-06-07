fn num_odds((found_odds, previous_odds): (bool, i32), next: &i32) -> (bool, i32) {
    if found_odds {
        return (found_odds, previous_odds);
    }

    // if the next number is odd
    if next % 2 != 0 {
        // if this is the third odd, we're done
        if previous_odds == 2 {
            return (true, previous_odds + 1);
        }

        return (found_odds, previous_odds + 1);
    }

    // otherwise reset counter
    return (found_odds, 0);
}

pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    let start: (bool, i32) = (false, 0);
    let (three_consecutive_odds, _) = arr.iter().fold(start, num_odds);

    three_consecutive_odds
}

fn main() {
    let examples = vec![vec![2, 6, 4, 1], vec![1, 2, 34, 3, 4, 5, 7, 23, 12]];

    for example in examples {
        dbg!(&example);
        dbg!(three_consecutive_odds(example));
    }
}
