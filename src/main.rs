use rand::prelude::*;
use rand::distributions::Uniform;
use rand::seq::IteratorRandom;
use rand_pcg::Pcg64;

trait Candidate {
    fn fitness(&self) -> f32;
    fn mutate(self) -> Self;
    fn random() -> Self;
    fn reproduce(&self, other: &Self) -> Self;
}

impl Candidate for f32 {
    fn fitness(&self) -> Self {
        (25.0 - self.powf(2.0)).abs()
    }

    fn mutate(self) -> Self {
        self * 1.1
    }

    fn random() -> Self {
        let d = rand::distributions::Uniform::new_inclusive(1.0, 30.0);
        let mut rng = thread_rng();

        (&mut rng).sample(d)
    }

    fn reproduce(&self, other: &Self) -> Self {
        (self + other) / 2.0
    }
}



fn select<T: Candidate>(mut v: Vec<T>) -> Vec<T> {
    v.sort_by(|v, w| v.fitness().partial_cmp(&w.fitness()).unwrap());

    v.into_iter().take(30).collect()
}

fn new_generation<T: Candidate + Copy>(older: Vec<T>) -> Vec<T> {
    let mut rng = thread_rng();

    let new_size = older.len();
    let mut result = vec![];

    let selected = select(older.clone());

    result.extend((0..5).into_iter().map(|_| T::random()));
    result.extend(selected.iter().copied().take(5));
    result.extend(selected.choose_multiple(&mut rng, 5).map(|x| x.mutate().clone()));

    for _ in 16..=new_size {
        let older_candidate = *older.choose(&mut rng).unwrap();
        let selected_candidate = *selected.choose(&mut rng).unwrap();

        result.push(older_candidate.reproduce(&selected_candidate));
    }

    result
}

fn main() {
    let d = rand::distributions::Uniform::new_inclusive(1.0, 30.0);
    let mut rng = thread_rng();

    let mut candidates: Vec<_> = (1..100).map(|_| f32::random()).collect();

    for _ in 1..100 {
        candidates = new_generation(candidates);

        println!("{:?}",candidates);
    }
}
