use rand::prelude::*;
use rand::distributions::Uniform;
use rand::seq::IteratorRandom;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
struct SPopulation<T: Candidate>(Vec<T>);

impl<'a, T: Candidate + Clone + 'a> SPopulation<T> {
    fn fittest(&self) -> VecFittest<T> {
        let SPopulation(v) = self;

        let mut result = VecFittest{v: v.clone(), i: 0};

        result.v.sort_by(|v, w| v.fitness().partial_cmp(&w.fitness()).unwrap());

        result
    }

    fn iter(&'a self) -> std::slice::Iter<'a, T> {
        let SPopulation(v) = self;

        v.iter()
    }

    fn new(n: usize) -> Self {
        let candidates: Vec<_> = (0..n).map(|_| T::random()).collect();

        SPopulation(candidates)
    }

    fn random() -> T {
        T::random()
    }

    fn size(&self) -> usize {
        let SPopulation(v) = self;

        v.len()
    }

    fn push(&mut self, c: T) {
        let SPopulation(v) = self;

        v.push(c);
    }
}


impl<C: Candidate> FromIterator<C> for SPopulation<C> {
    fn from_iter<T: IntoIterator<Item = C>>(i: T) -> SPopulation<C> {
        let v: Vec<_> = i.into_iter().collect();

        SPopulation(v)
    }
}


impl<C: Candidate> Extend<C> for SPopulation<C> {
    fn extend<T: IntoIterator<Item = C>>(&mut self, e: T) {
        let SPopulation(v) = self;

        v.extend(e.into_iter());
    }
}


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
        let d = Uniform::new_inclusive(1.0, 30.0);
        let mut rng = thread_rng();

        (&mut rng).sample(d)
    }

    fn reproduce(&self, other: &Self) -> Self {
        (self + other) / 2.0
    }
}


trait Population<'a> {
    type F: Iterator<Item = Self::C>;
    type C: Candidate + 'a;
    type I: Iterator<Item = &'a Self::C>;

    fn fittest(&self) -> Self::F;
    fn iter(&self) -> Self::I;
    fn new() -> Self;
    fn random() -> Self::C;
    fn size(&self) -> usize;
}

struct VecFittest<C: Candidate> {
    v: Vec<C>,
    i: usize
}


impl<C: Candidate + Copy> Iterator for VecFittest<C> {
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.v[self.i];

        self.i += 1;

        Some(result)
    }
}


fn select<'a, C: Candidate + Copy>(p: &'a SPopulation<C>) -> SPopulation<C> {
    p.fittest().take(30).collect()
}

fn new_generation<C: Candidate + Copy>(older: SPopulation<C>) -> SPopulation<C> {
    let mut rng = thread_rng();

    let new_size = older.size();
    let mut result = SPopulation::new(0);

    let selected = select(&older);

    result.extend((0..5).into_iter().map(|_| SPopulation::random()));
    result.extend(selected.fittest().take(5));
    result.extend(selected.iter().choose_multiple(&mut rng, 5).iter().map(|x| x.mutate()));

    for _ in 16..=new_size {
        let older_candidate = older.iter().choose(&mut rng).unwrap();
        let selected_candidate = selected.iter().choose(&mut rng).unwrap();

        result.push(older_candidate.reproduce(&selected_candidate));
    }

    result
}

fn main() {
    let mut candidates = SPopulation::<f32>::new(30);

    for _ in 1..100 {
        candidates = new_generation(candidates);

        println!("{:?}",candidates);
    }
}
