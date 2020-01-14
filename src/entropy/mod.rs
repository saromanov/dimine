use std::ops::Add;

pub type Input = Vec<i32>;

pub trait Entropy<T> {
    fn new(i1:Vec<T>) -> Self;
    fn compute(&self) -> Option<i32>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Conditional<T> {
    i1: Vec<T>
}

impl <T:Add<Output = T> + Copy> Entropy<T> for Conditional<T> {
   fn new(i1:Vec<T>) -> Conditional<T> {
        Conditional {
            i1:i1,
        }
    }

    fn compute(&self) -> Option<i32> {
        self.i1.iter().map(|&x| x).collect::<Vec<_>>();
        None
    }
}

