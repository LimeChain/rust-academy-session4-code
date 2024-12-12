pub trait Add<Rhs = Self> {
    type Output;

    fn add(&self, rhs: &Rhs) -> Self::Output;
}

#[derive(Debug, Clone)]
pub struct Banica {
    pub a: u32,
}

impl Add<Banica> for () where /* contraints */ {
    type Output = i32;

    fn add(&self, rhs: &Banica) -> Self::Output {
        5
    }
}

//          V Generics & V Associated types
impl<R, T: Add<Self, Output = R>> Add<T> for Banica {
    type Output = R;

    fn add(&self, rhs: &T) -> Self::Output {
        rhs.add(self)
    }
}
