#[derive(Debug)]
struct Bucket {
    min: i32,
    max: i32,
}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut queue: Vec<Bucket> = Vec::new();

    for int in nums {
        println!("evaluating {int}");

        // first, see if the num matches any buckets
        let mut matches: Vec<&mut Bucket> = queue
            .iter_mut()
            .filter(
                |bucket| {
                    int + 1 == bucket.min    // int is before min
            || int == bucket.max + 1
                }, // int is after max
            )
            .collect();

        let num_matches = matches.len();
        println!("matches: {num_matches}");

        match matches.len() {
            // no buckets matched means we add a new bucket
            0 => {
                println!("no matches found, adding bucket for {int}");
                queue.push(Bucket { min: int, max: int });
            }

            // one bucket matched means we modify the bucket
            1 => {
                let bucket = &mut matches[0];
                println!("found match for {int}: {bucket:?}");

                // check to see if int is new min of bucket
                if int + 1 == bucket.min {
                    dbg!((*bucket).min = int);
                }

                // see if int is new max of bucket
                if int == bucket.max + 1 {
                    dbg!((*bucket).max = int);
                }

                println!("modified: {bucket:?}");
            }

            // two buckets matched means we merge two buckets
            2 => {
                println!("two matches for {int} [merge]: {matches:?}");
                todo!("need to merge buckets for {int}: {matches:?}")
            }
            _ => panic!("too many matches for {int}: {matches:?}"),
        }
    }

    todo!("{queue:?}")
}

fn main() {
    let examples = vec![
        vec![100, 4, 200, 1, 3, 2],
        vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1],
    ];

    for example in examples {
        dbg!(&example);
        dbg!(longest_consecutive(example));
    }
}
