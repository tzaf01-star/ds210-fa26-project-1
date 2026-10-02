//! Running many games and reporting what happened.

use crate::secret_keeper::Dealer;
use crate::strategies::{Approach, bad, binary, clever, jump, linear, lucky, random};

/// What a run of games revealed about a strategy.
#[derive(Debug)]
pub struct Results {
    /// The fewest questions any single game needed.
    pub best: u32,
    /// The average across every game.
    pub mean: f64,
    /// The most any single game needed.
    pub worst: u32,
    /// The standard deviation: how spread out the games were around the mean.
    pub std_dev: f64,
}

impl Results {
    /// Is this strategy better than `other`?
    /// **You decide what better means**, using any of the four fields, or several of them.
    /// There is no single right answer (though there are wrong ones), so make a choice
    /// you can defend.
    pub fn is_better_than(&self, other: &Results) -> bool {
        self.mean < other.mean
    }
}

/// Play `rounds` games of `approach` on the range `[min, max)` and report how many
/// questions they took. Every game is dealt from one `Dealer`, so no secret
/// repeats until the whole range has been used.
pub fn measure(approach: Approach, min: u32, max: u32, rounds: u32) -> Results {
let mut dealer = Dealer::new(min,max);
let mut best = 0;
let mut worst = 0;
let mut total=0;
let mut total_squared = 0.0;

for r in 0..rounds {
    let mut keeper = dealer.deal();

    match &approach {
        Approach::Bad => bad(&mut keeper, min, max),
        Approach::Random => random(&mut keeper, min, max),
        Approach::Linear => linear(&mut keeper, min, max),
        Approach::Binary => binary(&mut keeper, min, max),
        Approach::Jump => jump(&mut keeper, min, max),
        Approach::Lucky => lucky(&mut keeper, min, max),
        Approach::Clever => clever(&mut keeper, min, max),
    };

    let questions = keeper.questions_asked();
    
    if r == 0 {
    best = questions;
    worst = questions;
    } else {
    if questions < best {
        best = questions;
    }

    if questions > worst {
        worst = questions;
    }}

total = total + questions;

let questions_f64 = questions as f64;
        total_squared = total_squared + questions_f64 * questions_f64;
    }

    let mean = total as f64 / rounds as f64;

    let variance = total_squared / rounds as f64 - mean * mean;
    let std_dev = variance.sqrt();

    Results {
        best,
        mean,
        worst,
        std_dev,
    }
}