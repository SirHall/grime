#![allow(unused_variables)]

use core::panic;
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

#[derive(Clone, Copy)]
pub enum Dice
{
    Red,
    Blue,
    Olive,
    Yellow,
    Magenta,
}

impl Dice
{
    pub fn roll(&self) -> u8 { self.faces()[rand::thread_rng().gen_range(0..6)] }

    pub fn faces(&self) -> [u8; 6]
    {
        match self
        {
            Dice::Red => [4, 4, 4, 4, 4, 9],
            Dice::Blue => [2, 2, 2, 7, 7, 7],
            Dice::Olive => [0, 5, 5, 5, 5, 5],
            Dice::Yellow => [3, 3, 3, 3, 8, 8],
            Dice::Magenta => [1, 1, 6, 6, 6, 6],
        }
    }

    pub fn name(&self) -> &str
    {
        match self
        {
            Dice::Red => "Red",
            Dice::Blue => "Blue",
            Dice::Olive => "Olive",
            Dice::Yellow => "Yellow",
            Dice::Magenta => "Magenta",
        }
    }

    pub fn fstr(s : &str) -> Dice
    {
        match s.to_lowercase().as_str()
        {
            "r" | "red" => Dice::Red,
            "b" | "blue" => Dice::Blue,
            "o" | "olive" | "g" | "green" => Dice::Olive,
            "y" | "yellow" => Dice::Yellow,
            "m" | "magenta" | "p" | "purple" => Dice::Magenta,
            _ => panic!("{} is not a recognized dice code", s),
        }
    }
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
                self.a.name(),
                self.a_rolls,
                a_win_percent,
                ties_percent,
                b_win_percent,
                self.b.name(),
                self.b_rolls
            ),
            Outcome::BWins => format!(
                "{: >7} {: >2} won with {: >2}% wins and {: >2}% ties, losing {: >2}% of the time to {: >7} {: >2}",
                self.b.name(),
                self.b_rolls,
                b_win_percent,
                ties_percent,
                a_win_percent,
                self.a.name(),
                self.a_rolls
            ),
            Outcome::Tie => format!("Both {: >7} and {: >7} tied", self.a.name(), self.b.name()),
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
            Outcome::AWins => String::from(a.name()),
            Outcome::BWins => String::from(b.name()),
            Outcome::Tie => String::from("tie"),
        },
    }
}

fn main()
{
    let results = vec![Dice::Red, Dice::Blue, Dice::Olive, Dice::Yellow, Dice::Magenta]
        .iter()
        .combinations(2)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|v| {
            (1..=10)
                .collect::<Vec<_>>()
                .par_iter()
                .map(|i| contest(&v[0], &v[1], *i, *i, 1000000))
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
