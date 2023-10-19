use crate::vector::Vector;

pub(crate) struct HyperPlane<const N: usize> {
    coefficients: Vector<N>,
    constant: f32,
}
impl<const N: usize> HyperPlane<N> {
    pub fn new(coefficients: Vector<N>, constant: f32) -> Self {
        Self {
            coefficients,
            constant,
        }
    }

    pub fn is_point_above(&self, point: &Vector<N>) -> bool {
        self.coefficients.dot(point) + self.constant >= 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_point_above() {
        let plane = HyperPlane::new(Vector::new([1.0, 0.0]), 0.0);
        let point = Vector::new([1.0, 1.0]);
        assert!(plane.is_point_above(&point));
    }
}
