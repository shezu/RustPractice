use std::ops::{Add,Mul};

#[derive(Debug)]
pub struct RightAngleTriangle<T> {
    pub base: T,
    pub perpendicular: T
}

impl<T : Add<Output = T> + Mul<Output = T> + Copy> RightAngleTriangle<T> {
    pub fn calculate_hypotenus(&self) -> T{
        let base_square = self.base * self.base;
        let perpendicular_square = self.perpendicular * self.perpendicular;
        return base_square + perpendicular_square
    }
}
