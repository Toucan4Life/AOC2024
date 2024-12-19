use itertools::Itertools;

fn part1(input: &str) -> i32 {
    let (mut x, mut y): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|l| {
            l.split(" ")
                .filter_map(|str_num| str_num.parse::<i32>().ok())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    x.sort();
    y.sort();

    x.iter().zip(y).map(|(a, b)| (a - b).abs()).sum()
}

fn part2(input: &str) -> i32 {
    let (x,y): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|l| {
            l.split(" ")
                .filter_map(|str_num| str_num.parse::<i32>().ok())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    let grouped_y= y.iter().counts();

    x.iter()
        .map(|num_x| *grouped_y.get(num_x).unwrap_or(&0) as i32 * num_x)
        .sum()
}

fn main() {
    let answer1 = part1(include_str!("input.txt"));
    println!("{}", answer1);
    let answer2 = part2(include_str!("input.txt"));
    println!("{}", answer2);
}

#[test]
fn test1() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";

    assert_eq!(11, part1(input));
}

#[test]
fn test2() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";

    assert_eq!(31, part2(input));
}
