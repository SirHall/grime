#![allow(unused_variables)]

use itertools::Itertools;
use rand::prelude::*;
use rayon::prelude::*;

enum Outcome
{
    AWins,
    BWins,
    Tie,
}

#[derive(Default)]
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

fn contest(a : &Dice, b : &Dice, a_rolls : usize, b_rolls : usize, samples : u32)
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

    let a_win_percent = ((100. / samples as f64) * a_wins as f64) as u32;
    let b_win_percent = ((100. / samples as f64) * b_wins as f64) as u32;
    let ties_percent = ((100. / samples as f64) * ties as f64) as u32;

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

    match outcome
    {
        Outcome::AWins => println!(
            "{: >7} {: >2} won with {: >2}% wins and {: >2}% ties, losing {: >2}% of the time to {: >7} {: >2}",
            a.name, a_rolls, a_win_percent, ties_percent, b_win_percent, b.name, b_rolls
        ),
        Outcome::BWins => println!(
            "{: >7} {: >2} won with {: >2}% wins and {: >2}% ties, losing {: >2}% of the time to {: >7} {: >2}",
            b.name, b_rolls, b_win_percent, ties_percent, a_win_percent, a.name, a_rolls
        ),
        Outcome::Tie => println!("Both dice tied"),
    }
}

fn main()
{
    let red_d = Dice::new("Red", vec![4, 4, 4, 4, 4, 9]);
    let blue_d = Dice::new("Blue", vec![2, 2, 2, 7, 7, 7]);
    let olive_d = Dice::new("Olive", vec![0, 5, 5, 5, 5, 5]);
    let yellow_d = Dice::new("Yellow", vec![3, 3, 3, 3, 8, 8]);
    let magenta_d = Dice::new("Magenta", vec![1, 1, 6, 6, 6, 6]);

    vec![red_d, blue_d, olive_d, yellow_d, magenta_d]
        .iter()
        .combinations(2)
        .map(|v| {
            (1..=10)
                .collect::<Vec<_>>()
                .par_iter()
                .map(|i| contest(&v[0], &v[1], *i, *i, 10000000))
                .count()
        })
        .count();
}
