use std::env;

fn main() {
    // the number of times the dial is left pointing at 0 after any rotation in the sequence.
    let args: Vec<_> = env::args().collect();
    let arg = args.iter().skip(1).next();

    let input = if let Some(arg) = arg
        && arg == "test"
    {
        include_str!("./input-test.txt")
    } else {
        include_str!("./input.txt")
    };

    let lines: Vec<&str> = input.split('\n').collect();
    println!("{}", count_zero_rotations(&lines));
}

static MAX_DIAL: i32 = 99;
static MIN_DIAL: i32 = 0;

fn count_zero_rotations(movements: &[&str]) -> usize {
    let mut dial = 50;
    let mut zeros = 0;
    for mov in movements {
        if mov.is_empty() {
            continue;
        }

        print!("The dial is rotated {mov} ");
        let mut chars = mov.chars().into_iter();
        let Some(dir) = chars.next() else {
            panic!("Invalid entry: {mov}");
        };

        let clicks = chars.collect::<String>();
        let clicks: i32 = clicks.parse().unwrap();

        let mut passed_zero_times = 0;
        print!(" [ start at {dial} ] ");
        let mut at_zero = false;
        (0..clicks).for_each(|_| {
            match dir {
                'R' => dial += 1,
                'L' => dial -= 1,
                _ => panic!("Invalid entry: {mov}"),
            }

            if dial > MAX_DIAL {
                dial = MIN_DIAL;
            }

            if dial < MIN_DIAL {
                dial = MAX_DIAL;
            }

            if at_zero {
                at_zero = false;
            }

            if dial == 0 {
                assert!(!at_zero);
                at_zero = true;
                zeros += 1;
            }
        });

        assert!(dial >= MIN_DIAL, "{dial} NOT >= {MIN_DIAL}");
        assert!(dial <= MAX_DIAL, "{dial} NOT <= {MAX_DIAL}");

        print!("to point at {dial}");

        if passed_zero_times > 0 {
            print!("; during this rotation, it points at 0 for {passed_zero_times} times");
            zeros += passed_zero_times;
        }

        print!(".");

        println!();
    }

    zeros
}
