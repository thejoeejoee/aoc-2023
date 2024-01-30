
use std::collections::{HashSet};


#[derive(Debug)]
struct Card {
    no: u32,
    winning: HashSet<u8>,
    owned: HashSet<u8>,
}

impl Card {
    fn score(&self) -> u32 {
        match self.matching_numbers() {
            0 => { 0 }
            v => 2u32.pow(v - 1)
        }
    }

    fn matching_numbers(&self) -> u32 {
        self.winning.intersection(&self.owned).count() as u32
    }
}

impl TryFrom<String> for Card {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {

        let parts: Vec<&str> = s[s.find(": ").unwrap() + 2..].split("|").collect();

        Ok(Card {
            no: s[s.find(" ").unwrap()..s.find(":").expect("sep")].trim().trim().parse().unwrap(),
            winning: HashSet::from_iter(parts[0].trim().split_whitespace().map(|s| s.parse().unwrap()).into_iter()),
            owned: HashSet::from_iter(parts[1].trim().split_whitespace().map(|s| s.parse().unwrap()).into_iter()),
        })
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Card> {
    input
        .split("\n")
        .map(|s| Card::try_from(s.to_string()).unwrap())
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &Vec<Card>) -> u32 {
    input
        .iter()
        .map(|c| c.score())
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &Vec<Card>) -> u32 {
    let mut state: Vec<(u32, &Card)> = Vec::from_iter(
        input.iter().map(|c| (1, c))
    );

    input.iter().for_each(|c| {
        // println!("Processing card: {:?}", c);
        let count_of_this_card = state[(c.no as usize) - 1].0;
        (c.no+1..c.no+c.matching_numbers()+1).for_each(|target_card| {
            // println!("\tAdvancing card i={:?}", target_card);
            state[(target_card as usize) - 1].0 += count_of_this_card;
        });
        // println!("State {:?}", state.iter().map(|(v, c)| (c.no, v)).collect::<Vec<_>>())
    });


    state.iter().map(|(s, _)| s).sum()
}
