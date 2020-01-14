
pub type Input = Vec<i32>;

pub trait Entropy<T: Ord> {
    fn compute(i1:T, i2:T) -> Result<String, Error>
}

#[derive(Debug, PartialEq)]
pub struct Conditional<T: Ord> {
    i1: T
}

impl Conditional<T: Ord> for Entropy {
    fn new(&self, i1:Input) -> self {
        self.i1 = i1
        self
    }

    fn compute(&self) -> Result<String, Error> {
        self.i1.iter().map(|&x| x + 1).collect::<Vec<_>>();
    }
}

