use std::collections::HashSet;

fn main() {
    // 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124

    let mut invalid_ids = HashSet::new();

    let ranges = include_str!("ranges.txt").split(',').collect::<Vec<_>>();
    // for each range in the file
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

        // iterate through each item on the range
        for item in start..=end {
            let item_str = format!("{item}");
            if item_str.len() % 2 != 0 {
                continue;
            }
            if is_invalid(&item_str) {
                println!("{item_str}");
                invalid_ids.insert(item);
            }
        }
    }

    let sum: usize = invalid_ids.iter().sum();
    println!("{sum}");
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
}
