
pub type Input = Vec<i32>;

pub trait Entropy<T: Ord> {
    fn compute(i1:T, i2:T) -> Result<String, String>;
}

#[derive(Debug, PartialEq)]
pub struct Conditional<T: Ord> {
    i1: T
}

impl <T:Ord> Entropy<T> for Conditional<T> {
    fn new(&self, i1:T) -> Self {
        self.i1 = i1
    }

    fn compute(&self) -> Result<String, String> {
        self.i1.iter().map(|&x| x + 1).collect::<Vec<_>>();
        Ok("aaa")
    }
}

