use rand::prelude::*;
use rand::distributions::Uniform;
use rand::seq::IteratorRandom;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
struct SPopulation<T: Candidate> {
    v: Vec<T>
}

impl<'a, T: Candidate + 'a> SPopulation<T> {
    fn fittest(&mut self) -> VecFittest<T> {
        self.v.sort_by(|v, w| v.fitness().partial_cmp(&w.fitness()).unwrap());

        VecFittest{v: &self.v, i: 0}
    }

    fn iter(&'a self) -> std::slice::Iter<'a, T> {
        self.v.iter()
    }

    fn new(n: usize) -> Self {
        let candidates: Vec<_> = (0..n).map(|_| T::random()).collect();

        SPopulation{v: candidates}
    }

    fn size(&self) -> usize {
        self.v.len()
    }

    fn push(&mut self, c: T) {
        self.v.push(c);
    }
}


impl<C: Candidate> FromIterator<C> for SPopulation<C> {
    fn from_iter<T: IntoIterator<Item = C>>(i: T) -> SPopulation<C> {
        let v: Vec<_> = i.into_iter().collect();

        SPopulation{v: v}
    }
}


impl<C: Candidate> Extend<C> for SPopulation<C> {
    fn extend<T: IntoIterator<Item = C>>(&mut self, e: T) {
        self.v.extend(e.into_iter());
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

struct VecFittest<'a, C: Candidate> {
    v: &'a [C],
    i: usize
}


impl<'a, C: Candidate + Copy> Iterator for VecFittest<'a, C> {
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.v[self.i];

        self.i += 1;

        Some(result)
    }
}


fn select<'a, C: Candidate + Copy>(p: &'a mut SPopulation<C>) -> SPopulation<C> {
    p.fittest().take(30).collect()
}


struct GenAlg<C: Candidate> {
    random_size: usize,
    fittest_size: usize,
    fittest_mutated_size: usize,
    older: SPopulation<C>
}


impl<C: Candidate + Copy> Iterator for GenAlg<C> {
    type Item = SPopulation<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = thread_rng();

        let new_size = self.older.size();
        let mut result = SPopulation::<C>::new(0);

        let mut selected = select(&mut self.older);

        result.extend((0..self.random_size).into_iter().map(|_| C::random()));
        result.extend(selected.fittest().take(self.fittest_size));
        result.extend(selected.iter().choose_multiple(&mut rng, self.fittest_mutated_size).iter().map(|x| x.mutate()));

        for _ in (self.random_size + self.fittest_size + self.fittest_mutated_size + 1)..=new_size {
            let older_candidate = self.older.iter().choose(&mut rng).unwrap();
            let selected_candidate = selected.iter().choose(&mut rng).unwrap();

            result.push(older_candidate.reproduce(&selected_candidate));
        }

        self.older = result.clone();

        Some(result)
    }
}

fn main() {
    let genalg = GenAlg::<f32>{random_size: 5, fittest_size: 5, fittest_mutated_size: 5, older: SPopulation::new(30)};

    for pop in genalg.take(1000) {
        println!("{:?}", pop);
    }
}
