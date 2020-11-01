#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

const MAX_MOVES: u8 = 100;

pub fn solve_buckets(capacity_1: u8, capacity_2: u8, goal: u8) -> Option<BucketStats> {
    let mut buckets = (capacity_1, 0);
    let mut moves = 1;

    if capacity_2 == goal {
        return Some(BucketStats {
            moves: moves + 1,
            goal_bucket: Bucket::Two,
            other_bucket: buckets.0,
        });
    }

    while moves < MAX_MOVES {
        match buckets {
            (a, _) if a == goal => {
                return Some(BucketStats {
                    moves,
                    goal_bucket: Bucket::One,
                    other_bucket: buckets.1,
                })
            }
            (_, b) if b == goal => {
                return Some(BucketStats {
                    moves,
                    goal_bucket: Bucket::Two,
                    other_bucket: buckets.0,
                })
            }
            (0, _) => {
                buckets.0 = capacity_1;
            }
            (_, b) if b == capacity_2 => {
                buckets.1 = 0;
            }
            _ => {
                let transfer_amount = buckets.0.min(capacity_2 - buckets.1);
                buckets.1 += transfer_amount;
                buckets.0 -= transfer_amount;
            }
        }

        moves += 1;
    }

    None
}

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    match start_bucket {
        Bucket::One => solve_buckets(capacity_1, capacity_2, goal),
        Bucket::Two => solve_buckets(capacity_2, capacity_1, goal).map(|result| {
            let swapped_bucket = match result.goal_bucket {
                Bucket::One => Bucket::Two,
                Bucket::Two => Bucket::One,
            };

            BucketStats {
                goal_bucket: swapped_bucket,
                ..result
            }
        }),
    }
}
