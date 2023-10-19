#[derive(Copy, Clone, Debug)]
pub struct Vector<const N: usize> {
    values: [f32; N],
}

impl<const N: usize> Vector<N> {
    pub fn new(values: [f32; N]) -> Self {
        Self { values }
    }

    pub fn dot(&self, vector: &Vector<N>) -> f32 {
        self.values
            .iter()
            .zip(vector.values)
            .map(|(a, b)| a * b)
            .sum::<f32>()
    }

    pub fn avg(&self, vector: &Vector<N>) -> Vector<N> {
        let average = self
            .values
            .iter()
            .zip(vector.values)
            .map(|(a, b)| (a + b) / 2.0)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Vector { values: average }
    }

    pub fn sub(&self, vector: &Vector<N>) -> Vector<N> {
        let difference = self
            .values
            .iter()
            .zip(vector.values)
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Vector { values: difference }
    }

    pub fn add(&self, vector: &Vector<N>) -> Vector<N> {
        let total = self
            .values
            .iter()
            .zip(vector.values)
            .map(|(a, b)| a + b)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Vector { values: total }
    }

    pub fn squared_euclidian_distance(&self, vector: &Vector<N>) -> f32 {
        self.values
            .iter()
            .zip(vector.values)
            .map(|(a, b)| (a - b).powi(2))
            .sum()
    }

    /// Rust does not implement hash for the f32 type. This is a workaround
    /// since we need to be able to identify a vector's contents for deduplication
    pub fn hashkey(&self) -> HashKey<N> {
        let hash = self
            .values
            .iter()
            .map(|a| a.to_bits())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        HashKey(hash)
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct HashKey<const N: usize>([u32; N]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vector = Vector::new([0.0, 0.0, 0.0]);
        assert_eq!(vector.values, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_add() {
        let a = Vector::new([1.0, 2.0, 3.0]);
        let b = Vector::new([4.0, 5.0, 6.0]);
        let c = a.add(&b);
        assert_eq!(c.values, [5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_sub() {
        let a = Vector::new([1.0, 2.0, 3.0]);
        let b = Vector::new([4.0, 5.0, 6.0]);
        let c = a.sub(&b);
        assert_eq!(c.values, [-3.0, -3.0, -3.0]);
    }

    #[test]
    fn test_avg() {
        let a = Vector::new([1.0, 2.0, 3.0]);
        let b = Vector::new([4.0, 5.0, 6.0]);
        let c = a.avg(&b);
        assert_eq!(c.values, [2.5, 3.5, 4.5]);
    }

    #[test]
    fn test_dot() {
        let a = Vector::new([1.0, 2.0, 3.0]);
        let b = Vector::new([4.0, 5.0, 6.0]);
        let c = a.dot(&b);
        assert_eq!(c, 32.0);
    }

    #[test]
    fn test_hashkey() {
        let a = Vector::new([1.0, 2.0, 3.0]);
        let b = Vector::new([1.0, 2.0, 3.0]);
        let c = Vector::new([1.0, 2.0, 4.0]);
        assert_eq!(a.hashkey(), b.hashkey());
        assert_ne!(a.hashkey(), c.hashkey());
    }
}
