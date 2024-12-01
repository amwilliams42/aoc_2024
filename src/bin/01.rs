advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();

    for line in input.lines() {
        // split line into two numbers separated by spaces
        let mut iter = line.split_whitespace();
        list1.push(iter.next().unwrap().parse::<i32>().unwrap());
        list2.push(iter.next().unwrap().parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();
    // vec of differences between elements of list1 and list2
    let diff: Vec<i32> = list1.iter().zip(list2.iter()).map(|(a, b)| (a - b).abs()).collect();
    // sum of all differences
    let sum: i32 = diff.iter().sum();
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();

    for line in input.lines() {
        // split line into two numbers separated by spaces
        let mut iter = line.split_whitespace();
        list1.push(iter.next().unwrap().parse::<i32>().unwrap());
        list2.push(iter.next().unwrap().parse::<i32>().unwrap());
    }

    let mut similarity: usize = 0;

    for &n in list1.iter() {
        let count = list2.iter().filter(|&&x| x == n).count();
        similarity += count * n as usize;
    }

    Some(similarity as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
