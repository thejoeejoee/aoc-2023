use std::cmp::{max};
use std::collections::{HashMap, HashSet};
use std::ops::Add;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, PartialOrd, Ord)]
struct Pos(i32, i32);

impl Add for &Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug)]
struct Engine {
    state: HashMap<Pos, char>,
}

impl Engine {
    fn max_pos(&self) -> Pos {
        let mut res = Pos(0, 0);

        self.state.iter().for_each(|(p, _)| {
            res.0 = max(res.0, p.0);
            res.1 = max(res.1, p.1);
        });
        return res;
    }
    fn neighbors(from: &Vec<Pos>) -> Vec<Pos> {
        let mut all = HashSet::<Pos>::new();

        from.iter().flat_map(|pos| {
            vec![
                // Pos(0, 0),
                Pos(0, -1),
                Pos(0, 1),
                Pos(-1, 0),
                Pos(-1, -1),
                Pos(-1, 1),
                Pos(1, 0),
                Pos(1, -1),
                Pos(1, 1),
            ].iter().filter_map(|off| {
                let new = pos + off;
                if from.contains(&new) { None } else { Some(new) }
            }).collect::<Vec<Pos>>()
        }).for_each(|pos| { all.insert(pos); });
        all.drain().collect()
    }
}

impl TryFrom<String> for Engine {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let state = s
            .split("\n")
            .enumerate()
            .fold(
                HashMap::new(),
                |mut state, (y, line)| {
                    line.chars().enumerate().filter(|(_, c)| c != &'.').for_each(|(x, c)| {
                        state.entry(Pos(x as i32, y as i32)).or_insert(c);
                    });
                    state
                });
        Ok(Engine { state })
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Engine {
    Engine::try_from(input.to_string()).unwrap()
}

#[aoc(day3, part1)]
fn part1(input: &Engine) -> u32 {
    let max = input.max_pos();
    let mut result = 0u32;
    for y in 0..max.1 + 1 {
        let mut left_x = 0i32;
        while left_x < max.0 {
            // println!("left_x: {}", left_x);
            let positions: Vec<Pos> = (left_x..).take_while(|x|
                input.state
                    .get(&Pos(*x, y))
                    .is_some_and(|c| c.is_digit(10))
            ).map(|x| Pos(x, y)).collect();


            if !Engine::neighbors(&positions)
                .iter()
                .any(|p| input.state.get(p).is_some_and(|c| !c.is_digit(10))) {
                // no symbol around
                left_x += 1;
                continue;
            }

            // symbol around
            let number: u32 = positions
                .iter()
                .filter_map(|p| input.state.get(p))
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            left_x += positions.len() as i32;
            // println!("number: {}", number);
            result += number;
        }
    }
    result
}

#[aoc(day3, part2)]
fn part2(input: &Engine) -> u32 {
    let max = input.max_pos();

    let mut gears = HashMap::<Pos, Vec<u32>>::new();

    for y in 0..max.1 + 1 {
        let mut left_x = 0i32;
        while left_x < max.0 {
            // println!("left_x: {}", left_x);
            let positions: Vec<Pos> = (left_x..).take_while(|x|
                input.state
                    .get(&Pos(*x, y))
                    .is_some_and(|c| c.is_digit(10))
            ).map(|x| Pos(x, y)).collect();

            let neights = Engine::neighbors(&positions);
            let symbol_positions: Vec<&Pos> = neights
                .iter()
                .filter(|p| input.state.get(p).is_some_and(|c| !c.is_digit(10)))
                .collect();

            if symbol_positions.is_empty() {
                // no symbol around
                left_x += 1;
                continue;
            }

            let gear = symbol_positions
                .iter()
                .filter(|p| input.state.get(p).is_some_and(|c| c == &'*'))
                .next();

            if gear.is_none() {
                left_x += positions.len() as i32;
                continue;
            }

            // symbol around
            let number: u32 = positions
                .iter()
                .filter_map(|p| input.state.get(p))
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            left_x += positions.len() as i32;
            gears
                .entry(**gear.unwrap())
                .or_insert(Vec::new())
                .push(number);
        }
    }
    // println!("{:?}", gears);
    gears
        .iter()
        .filter(|(_, nums)| nums.len() == 2)
        .map(|(_, numbers)| numbers.iter().product::<u32>())
        .sum()
}
