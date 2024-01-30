use std::ops::{RangeInclusive, RangeFrom};

#[derive(Debug)]
struct RoutingTable {
    routes: Vec<(
        RangeInclusive<u32>,
        RangeFrom<u32>,
    )>,
}

impl RoutingTable {
    fn route(&self, seed: u32) -> u32 {
        self.routes.iter().find_map(|(from, to)| {
            // TODO: dirty cloning of ranges
            let p = from.clone().position(|v| v == seed);
            // println!("Position in {:?}? ={:?}", from, p);
            return if let Some(source_idx) = p {
                to.clone().nth(source_idx)
            } else { None };
        })
            // .inspect(|v| {
                // println!("Routing {:?} to {:?}", seed, v);
            // })
        .unwrap_or(seed)
    }
}

impl TryFrom<Vec<String>> for RoutingTable {
    type Error = ();

    fn try_from(s: Vec<String>) -> Result<Self, Self::Error> {
        Ok(RoutingTable {
            routes: s.iter().map(|line| {
                let [to, from, length] = line
                    .split_whitespace()
                    .filter_map(|v| v.parse().ok())
                    .collect::<Vec<u32>>()[..] else { panic!("invalid input") };
                (
                    from..=from + length,
                    to..,
                )
            }).collect()
        })
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> (Vec<u32>, Vec<RoutingTable>) {
    let lines = input
        .split("\n")
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>();

    // println!("{:?}", lines);

    let mut lines = lines.iter();

    let seeds: Vec<u32> = lines
        .next()
        .unwrap()[7..]
        .split_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .collect();

    lines.next();
    let _ = lines
        .by_ref() // lower efficiency, but doesn't take ownership
        .take_while(|l| !l.contains("map:"));

    let mut tables = Vec::new();
    loop {
        let routes = lines
            .by_ref() // otherwise, take_while takes ownership()
            .take_while(|l| !l.contains("map:"))
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if routes.is_empty() { break; };

        tables.push(RoutingTable::try_from(routes).unwrap())
    }

    // for table in &tables {
    //     println!("TABLE {:?}", table.routes);
    // }

    (seeds, tables)
}

#[aoc(day5, part1)]
fn part1(input: &(Vec<u32>, Vec<RoutingTable>)) -> u32 {
    let seeds = &(*input).0;
    let tables = &(*input).1;

    seeds.iter().map(|&seed| {
        // println!("Routing seed: {:?}", seed);
        tables.iter()
            .fold(seed, |to_route, table| table.route(to_route))
    }
    )
        // .inspect(|v| println!("Routed to {:?}", v))
        .min().unwrap()
}

