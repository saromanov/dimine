
pub type Input = Vec<i32>;

pub trait Entropy {
    fn compute(i1:Input, i2:Input) -> Result<String, Error>
}

