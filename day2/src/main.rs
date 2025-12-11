use std::collections::HashSet;

fn main() {
    let ranges = include_str!("ranges.txt").split(',').collect::<Vec<_>>();
    println!("{}", sum_part2_invalid_ids(ranges));
}

#[allow(dead_code)]
fn sum_part2_invalid_ids(ranges: Vec<&str>) -> usize {
    let mut invalid_ids = HashSet::new();
    for range_str in ranges {
        print!("- {range_str}");
        let range_str = range_str.trim();
        // finds start and end of the range
        let Some((starts, ends)) = range_str.split_once('-') else {
            panic!("Invalid range string: {range_str}");
        };

        let start: usize = starts.parse().unwrap();
        let end: usize = ends.parse().unwrap();

        let this_invalid_ids = find_part2_invalid_ids(start, end);
        println!(": {this_invalid_ids:?}");
        invalid_ids.extend(this_invalid_ids);
    }

    invalid_ids.iter().sum()
}

fn find_part2_invalid_ids(start: usize, end: usize) -> Vec<usize> {
    let mut invalid_ids = vec![];
    for item in start..=end {
        let item_str = format!("{item}");
        if is_part2_invalid(&item_str) {
            invalid_ids.push(item);
        }
    }
    invalid_ids
}

fn is_part2_invalid(id: &str) -> bool {
    // we need to identify that 123123123 is also invalid
    // because now we allow the sequences to repeat any
    // number of times

    // so I think first we need to figure out which numbers
    // the id is divisible by, starting with 2 and ending with
    // the len?

    // so in our case 123123123 length is 9, so we'd test with
    // 2 (no), 3 (yes), 4 (no), 5 (no), 6 (no), 7 (no)

    let len = id.len();
    for chunk_size in 1..id.len() {
        // println!("id: {id} testing with: {chunk_size}");
        let is_divisible = len % chunk_size == 0;
        if !is_divisible {
            continue;
        }

        let parts = id.chars().collect::<Vec<char>>();
        let parts = parts.chunks(chunk_size).collect::<Vec<_>>();
        // println!("  id: {id} - parts: {parts:?}");

        // it is valid if no parts are equal the first at this
        // chunk size
        let valid = parts[1..].iter().any(|&p| p != parts[0]);

        if !valid {
            return true;
        }
    }

    false
}

fn sum_invalid_ids(ranges: Vec<&str>) -> usize {
    // 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124

    let mut invalid_ids = HashSet::new();
    for range_str in ranges {
        let range_str = range_str.trim();
        // finds start and end of the range
        let Some((starts, ends)) = range_str.split_once('-') else {
            panic!("Invalid range string: {range_str}");
        };

        println!("Range [{starts}]-[{ends}]");
        let start: usize = starts.parse().unwrap();
        let end: usize = ends.parse().unwrap();

        // for len = 2, it can be 1
        // for len = 3, there's no way it can have any
        // for len = 4, it can only be 2
        // for len = 5, it can't work
        // for len = 6, it can only be 3
        // for len = 7, it can't work
        // for len = 8, it can only be 4

        invalid_ids.extend(find_invalid_ids(start, end));
    }

    invalid_ids.iter().sum()
}

fn find_invalid_ids(start: usize, end: usize) -> Vec<usize> {
    let mut invalid_ids = vec![];
    for item in start..=end {
        let item_str = format!("{item}");
        if item_str.len() % 2 != 0 {
            continue;
        }
        if is_invalid(&item_str) {
            println!("{item_str}");
            invalid_ids.push(item);
        }
    }
    invalid_ids
}

fn is_invalid(id: &str) -> bool {
    let parts = id.chars().collect::<Vec<char>>();
    let parts = parts.chunks(id.len() / 2).collect::<Vec<_>>();

    let mut repeats = true;
    for part in &parts[1..] {
        if *part != parts[0] {
            repeats = false;
        }
    }

    repeats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_sum_for_range() {
        let ranges = include_str!("ranges-test.txt")
            .split(',')
            .collect::<Vec<_>>();
        assert_eq!(4174379265, sum_part2_invalid_ids(ranges));
    }

    #[test]
    fn test_find_part2_invalid_ids() {
        assert_eq!(vec![11, 22], find_part2_invalid_ids(11, 22));
        assert_eq!(vec![99, 111], find_part2_invalid_ids(99, 111));
        assert_eq!(vec![999, 1010], find_part2_invalid_ids(999, 1010));
        assert_eq!(
            find_part2_invalid_ids(1188511880, 1188511890),
            vec![1188511885]
        );
        assert_eq!(find_part2_invalid_ids(222220, 222224), vec![222222]);
        assert_eq!(find_part2_invalid_ids(1698522, 1698528), vec![]);
        assert_eq!(find_part2_invalid_ids(446443, 446449), vec![446446]);
        assert_eq!(find_part2_invalid_ids(38593856, 38593862), vec![38593859]);
        assert_eq!(find_part2_invalid_ids(565653, 565659), vec![565656]);
        assert_eq!(
            find_part2_invalid_ids(824824821, 824824827),
            vec![824824824]
        );
        assert_eq!(
            find_part2_invalid_ids(2121212118, 2121212124),
            vec![2121212121]
        );
    }

    #[test]
    fn test_part2_invalid() {
        assert!(is_part2_invalid("11"));
        assert!(!is_part2_invalid("12"));
        assert!(is_part2_invalid("22"));
        assert!(is_part2_invalid("99"));
        assert!(is_part2_invalid("111"));
        assert!(!is_part2_invalid("115"));
        assert!(is_part2_invalid("999"));
        assert!(is_part2_invalid("1010"));
        assert!(is_part2_invalid("222222"));
        assert!(is_part2_invalid("123123123"));
        assert!(is_part2_invalid("1188511885"));
    }

    #[test]
    fn test_sum_for_range() {
        let ranges = include_str!("ranges.txt").split(',').collect::<Vec<_>>();
        assert_eq!(12850231731, sum_invalid_ids(ranges));
    }

    #[test]
    fn test_sum_for_test_range() {
        let ranges = include_str!("ranges-test.txt")
            .split(',')
            .collect::<Vec<_>>();
        assert_eq!(1227775554, sum_invalid_ids(ranges));
    }

    #[test]
    fn test_invalid() {
        assert!(is_invalid("11"));
        assert!(is_invalid("22"));
        assert!(is_invalid("99"));
        assert!(is_invalid("111"));
        assert!(is_invalid("999"));
        assert!(is_invalid("1010"));
        assert!(is_invalid("222222"));
        assert!(is_invalid("123123123"));
        assert!(is_invalid("1188511885"));
    }
}
