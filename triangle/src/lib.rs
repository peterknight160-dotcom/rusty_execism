pub struct Triangle {
    triangle_type: TriangleType,
}
#[derive(PartialEq, Eq, Clone, Debug)]
enum TriangleType {
    Equilateral,
    Isosceles,
    Scalene,
}

impl Triangle {
    pub fn build<T>(sides: [T; 3]) -> Option<Triangle>
    where
        T: Default +PartialOrd + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
        // <T> is a placeholder for the type of the sides. It can be either an integer or a float, depending on the feature flag.
    {
        let mut max_side = sides[0];
        sides.iter().for_each(|&side| {
            if side > max_side {
                max_side = side;
            }
        });

        let sum_of_sides = sides[0] + sides[1] + sides[2];
        if sum_of_sides > max_side+  max_side && max_side > T::default() {
            let triangle_type = if sides[0] == sides[1] && sides[1] == sides[2] {
                TriangleType::Equilateral
            } else if sides[0] == sides[1] || sides[1] == sides[2] || sides[0] == sides[2] {
                TriangleType::Isosceles
            } else {
                TriangleType::Scalene
            };
            Some(Triangle { triangle_type })
            //Some(Triangle { triangle_type })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.triangle_type == TriangleType::Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self.triangle_type == TriangleType::Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        self.triangle_type == TriangleType::Isosceles
            || self.triangle_type == TriangleType::Equilateral
    }
}
