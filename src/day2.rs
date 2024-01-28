use std::cmp::{max};
use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    sets: Vec<HashMap<String, u32>>
}

impl TryFrom<String> for Game {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        // println!("GAME");
        let sets = s[s.find(": ").unwrap()+": ".len()..]
            .split(";")
            .map(|set| {
                // 7 blue, 5 red
                // println!("\tSET");
                set
                    .split(",")
                    .map(|pair| {
                        // println!("{}", pair);
                        let parts = pair.split(" ").collect::<Vec<&str>>();
                        let mut items = parts.iter().rev();
                        (
                            items.next().unwrap().to_string(),
                            items.next().unwrap().parse::<u32>().unwrap()
                        )
                    })
                    .fold(
                        HashMap::new(),
                        |mut m, (c, v)| {
                            // println!("\t\t{}: {}", c, v);
                            *m.entry(c).or_insert(0) = v;
                            m
                        }
                    )
            })
            .collect::<Vec<HashMap<String, u32>>>();
        Ok(Game { sets })
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Game> {
    input.split('\n').map(|line| Game::try_from(line.to_string()).unwrap()).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> u32 {
    input.iter().enumerate().map(|(i, g)| {
        let total = g.sets.iter().fold(HashMap::new(), |mut acc, set| {
            set.iter().for_each(|(c, v)| {
                *acc.entry(c).or_insert(0) = max(*acc.get(c).unwrap_or(&0u32), *v)
            });
            return acc
        });

        return if !vec![
            total.get(&"red".to_string()).unwrap_or(&0u32) <= &12u32,
            total.get(&"green".to_string()).unwrap_or(&0u32) <= &13u32,
            total.get(&"blue".to_string()).unwrap_or(&0u32) <= &14u32,
        ].iter().all(|v| *v) {
            // println!("NOT possible: {:?}\t{:?}", total, g);
            0
        } else {
            // println!("    possible: {:?}\t{:?}", total, g);
            i + 1
        }
    }).sum::<usize>() as u32
}
#[aoc(day2, part2)]
fn part2(input: &[Game]) -> u32 {
    input.iter().enumerate().map(|(_, g)| {
        let total = g.sets.iter().fold(HashMap::new(), |mut acc, set| {
            set.iter().for_each(|(c, v)| {
                *acc.entry(c).or_insert(*v) = max(
                    *acc.get(c).unwrap_or(&0u32),
                    *v,
                )
            });
            return acc
        });
        total.get(&"red".to_string()).unwrap_or(&0u32) *
        total.get(&"green".to_string()).unwrap_or(&0u32) *
        total.get(&"blue".to_string()).unwrap_or(&0u32)
    }).sum::<u32>()
}