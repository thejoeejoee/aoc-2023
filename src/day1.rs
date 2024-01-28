
#[aoc_generator(day1, part1)]
pub fn input_generator1(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();

            vec![
                digits[0],
                digits[digits.len() -1]
            ]
                .iter()
                .map(|v| v.to_string())
                .collect::<String>().parse::<u32>()
                .unwrap()
        })
        .collect()
}

#[aoc_generator(day1, part2)]
pub fn input_generator2(input: &str) -> Vec<u32> {
    let replacements = vec![
        // keep pred/succ of the digits
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];
    input
        .lines()
        .map(|line| {
            let mut line = line.to_string();
            loop {
                let to_replace = replacements
                    .iter()
                    .filter(|(from, _)| line.contains(from))
                    .min_by_key(|(from, _)| line.find(from));

                // println!("{:?} {:?}", to_replace, line);
                if let Some((from, to)) = to_replace {
                    line = line.replacen(from, to, 1);
                    continue;
                }
                break;
            }
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            let n = vec![
                digits[0],
                digits[digits.len() -1]
            ]
                .iter()
                .map(|v| v.to_string())
                .collect::<String>().parse::<u32>()
                .unwrap();

            println!("{}: {}", line, n);
            n
        })
        .collect()
}


#[aoc(day1, part1)]
#[aoc(day1, part2)]
pub fn part1(input: &[u32]) -> u32 {
    input.iter().sum()
}