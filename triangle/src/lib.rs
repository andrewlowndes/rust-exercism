use std::cmp::PartialOrd;
use std::ops::Add;

pub struct Triangle<T: Copy + PartialOrd + Add<Output = T> + Default> {
    sides: [T; 3],
}

impl<T: Copy + PartialOrd + Add<Output = T> + Default> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let zero = T::default();

        if sides[0] == zero
            || sides[1] == zero
            || sides[2] == zero
            || (sides[0] + sides[1]) < sides[2]
            || (sides[1] + sides[2]) < sides[0]
            || (sides[2] + sides[0]) < sides[1]
        {
            return None;
        }

        Some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1]
            && self.sides[0] != self.sides[2]
            && self.sides[1] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        !self.is_scalene()
    }
}
