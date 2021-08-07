use rand::prelude::*;
use std::iter::FromIterator;


pub trait Candidate {
    fn fitness(&self) -> f32;
    fn mutate(self) -> Self;
    fn random<R: Rng>(rng: &mut R) -> Self;
    fn reproduce(&self, other: &Self) -> Self;
}


#[derive(Clone, Debug)]
pub struct Population<T: Candidate> {
    v: Vec<T>
}


impl<'a, T: Candidate + 'a> Population<T> {
    pub fn iter(&'a self) -> std::slice::Iter<'a, T> {
        self.v.iter()
    }

    pub fn new(n: usize) -> Self {
        let mut rng = thread_rng();

        let candidates: Vec<_> = (0..n).map(|_| T::random(&mut rng)).collect();

        let mut result = Population{v: candidates};

        result.sort();

        result
    }

    pub fn size(&self) -> usize {
        self.v.len()
    }

    pub fn push(&mut self, c: T) {
        self.v.push(c);

        self.sort();
    }

    fn sort(&mut self) {
        self.v.sort_by(|v, w| v.fitness().partial_cmp(&w.fitness()).unwrap());
    }
}


impl<C: Candidate> FromIterator<C> for Population<C> {
    fn from_iter<T: IntoIterator<Item = C>>(i: T) -> Population<C> {
        let v: Vec<_> = i.into_iter().collect();

        Population{v: v}
    }
}


impl<C: Candidate> Extend<C> for Population<C> {
    fn extend<T: IntoIterator<Item = C>>(&mut self, e: T) {
        self.v.extend(e.into_iter());

        self.sort()
    }
}
