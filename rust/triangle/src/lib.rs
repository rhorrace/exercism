use std::ops::Add;

pub struct Triangle<T> {
    sides: Vec<T>,
}

impl <T: Default + PartialOrd + Add<Output=T> + Copy>Triangle<T> {
    pub fn build(mut sides: [T; 3]) -> Option<Triangle<T>> {
        sides.sort_by(|a,b| a.partial_cmp(b).unwrap());
        if sides[0] == T::default() {
            None
        } else if sides[0] + sides[1] < sides[2] {
            None
        } else {
            Some(Triangle {
                sides: sides.to_vec(),
            })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.iter()
            .all(|&s| s == self.sides[0])
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && ! self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] ||
            self.sides[0] == self.sides[2] ||
            self.sides[1] == self.sides[2]
    }
}
