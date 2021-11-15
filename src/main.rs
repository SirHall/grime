#![allow(unused_variables)]

use std::collections::HashMap;

use itertools::Itertools;
use rand::prelude::*;
use rayon::prelude::*;

#[derive(Clone, Copy)]
enum Outcome
{
    AWins,
    BWins,
    Tie,
}

#[derive(Default, Clone)]
struct Dice
{
    faces : Vec<u8>,
    name :  String,
}

impl Dice
{
    pub fn new(name : &str, faces : Vec<u8>) -> Self
    {
        Self {
            faces,
            name : String::from(name),
        }
    }

    pub fn roll(&self) -> u8 { self.faces[rand::thread_rng().gen_range(0..6)] }
}

struct ContestResults
{
    a :       Dice,
    b :       Dice,
    samples : u32,
    a_rolls : usize,
    b_rolls : usize,
    a_wins :  u32,
    b_wins :  u32,
    ties :    u32,
    outcome : Outcome,
    winner :  String,
}

impl ContestResults
{
    pub fn to_str(&self) -> String
    {
        let a_win_percent = ((100. / self.samples as f64) * self.a_wins as f64) as u32;
        let b_win_percent = ((100. / self.samples as f64) * self.b_wins as f64) as u32;
        let ties_percent = ((100. / self.samples as f64) * self.ties as f64) as u32;

        match self.outcome
        {
            Outcome::AWins => format!(
                "{: >7} {: >2} won with {: >2}% wins and {: >2}% ties, losing {: >2}% of the time to {: >7} {: >2}",
                self.a.name, self.a_rolls, a_win_percent, ties_percent, b_win_percent, self.b.name, self.b_rolls
            ),
            Outcome::BWins => format!(
                "{: >7} {: >2} won with {: >2}% wins and {: >2}% ties, losing {: >2}% of the time to {: >7} {: >2}",
                self.b.name, self.b_rolls, b_win_percent, ties_percent, a_win_percent, self.a.name, self.a_rolls
            ),
            Outcome::Tie => format!("Both {: >7} and {: >7} tied", self.a.name, self.b.name),
        }
    }
}

fn contest(a : &Dice, b : &Dice, a_rolls : usize, b_rolls : usize, samples : u32) -> ContestResults
{
    let mut a_wins = 0u32;
    let mut ties = 0u32;

    for _i in 0..samples
    {
        let a_res : u32 = (0..a_rolls).map(|_| -> u32 { a.roll() as u32 }).sum();
        let b_res : u32 = (0..b_rolls).map(|_| -> u32 { b.roll() as u32 }).sum();

        if a_res > b_res
        {
            a_wins += 1;
        }
        else if a_res == b_res
        {
            ties += 1;
        }
    }

    let b_wins = samples - a_wins - ties;

    let outcome = if a_wins > b_wins
    {
        Outcome::AWins
    }
    else
    {
        if a_wins == b_wins
        {
            Outcome::Tie
        }
        else
        {
            Outcome::BWins
        }
    };

    ContestResults {
        a : a.clone(),
        b : b.clone(),
        samples,
        a_rolls,
        b_rolls,
        a_wins,
        b_wins,
        ties,
        outcome : outcome.clone(),
        winner : match outcome
        {
            Outcome::AWins => a.name.clone(),
            Outcome::BWins => b.name.clone(),
            Outcome::Tie => String::from("tie"),
        },
    }
}

fn main()
{
    let red_d = Dice::new("Red", vec![4, 4, 4, 4, 4, 9]);
    let blue_d = Dice::new("Blue", vec![2, 2, 2, 7, 7, 7]);
    let olive_d = Dice::new("Olive", vec![0, 5, 5, 5, 5, 5]);
    let yellow_d = Dice::new("Yellow", vec![3, 3, 3, 3, 8, 8]);
    let magenta_d = Dice::new("Magenta", vec![1, 1, 6, 6, 6, 6]);

    let results = vec![red_d, blue_d, olive_d, yellow_d, magenta_d]
        .iter()
        .combinations(2)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|v| {
            (1..=10)
                .collect::<Vec<_>>()
                .par_iter()
                .map(|i| contest(&v[0], &v[1], *i, *i, 1000000))
                // .chain(vec![String::from("")].par_iter().map(|n| n.clone()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    // Print out results
    results
        .iter()
        .map(|result| result.to_str())
        .for_each(|msg| println!("{}", msg));

    println!("");
    // Print out winners ranked by number of wins
    let mut win_count = HashMap::new();

    results.iter().for_each(|n| {
        win_count.insert(
            n.winner.clone(),
            win_count.get(&n.winner).and_then(|v| Some(*v + 1)).unwrap_or(1),
        );
    });

    win_count
        .iter()
        .sorted_by(|(k1, v1), (k2, v2)| Ord::cmp(v1, v2).reverse())
        .for_each(|(k, v)| println!("{: <7} {: >3}", k, v));
}
