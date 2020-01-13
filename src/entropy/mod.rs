
pub type Input = Vec<i32>;

pub trait Entropy {
    fn compute(i1:Input, i2:Input) -> Result<String, Error>
}

#[derive(Debug, PartialEq)]
pub struct Conditional {
    i1: Input
}

impl Conditional for Entropy {
    fn new(&self, i1:Input) -> self {
        self.i1 = i1
        self
    }

    fn compute(&self) -> Result<String, Error> {
        self.i1.iter().map(|&x| x + 1).collect::<Vec<_>>();
    }
}

