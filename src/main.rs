extern crate rand;
extern crate time;
extern crate fnv;

#[macro_use]
mod utils;

use rand::*;
use std::collections::{HashMap};
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;

type Hasher = BuildHasherDefault<FnvHasher>;

fn main() {
    let mut m: HashMap<_, _, Hasher> = HashMap::default();
    let mut rnd = rand::thread_rng();
    let source = (1..50_000_000).map(|_| rnd.gen::<i32>()).collect::<Vec<_>>();
    
    time!("Insertion", {
        for x in source.iter().cloned() {
            m.entry(x).or_insert(0);
        }
    });
    
    time!("Lookup", {
        let mut _acc = 0;
        for x in source.iter() {
            if let Some(x) = m.get(x) {
                _acc += x % 10
            }
        }
    })
}