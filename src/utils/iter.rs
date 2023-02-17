pub trait AlmostEqualDivision<T> {
    fn divide_collection_by(self, div: usize) -> Vec<Vec<T>>
    where
        T: Clone;
}

//TODO: try to implement it in a more generic way and not only for Vec<T>
impl<T: Clone> AlmostEqualDivision<T> for Vec<T> {
    fn divide_collection_by(self, div: usize) -> Vec<Vec<T>> {
        let at = self.len() - (self.len() % div);
        if at == 0 {
            return self.into_iter().map(|e| vec![e]).collect();
        }

        let (all, remainder) = self.split_at(at);

        all.chunks_exact(self.len() / div)
            .map(|arr| arr.to_vec())
            .into_iter()
            .zip(remainder.iter())
            .map(|(mut arr, elem)| {
                arr.push(elem.clone());
                arr
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::iter::AlmostEqualDivision;

    #[test]
    fn ensure_fn_almost_equal_div_works_as_expected_1() {
        let out = (0..10).collect::<Vec<i32>>().divide_collection_by(3);
        for x in out {
            assert!(x.len() <= 4);
            assert!(x.len() >= 3);
        }
    }

    #[test]
    fn ensure_fn_almost_equal_div_works_as_expected_2() {
        let vec: Vec<i32> = Vec::new();
        let out = vec.divide_collection_by(3);
        for x in out {
            assert_eq!(x.len(), 0);
        }
    }

    #[test]
    fn ensure_fn_almost_equal_div_works_as_expected_3() {
        let out = (0..9).collect::<Vec<i32>>().divide_collection_by(3);
        for x in out {
            assert_eq!(x.len(), 3);
        }
    }

    #[test]
    fn ensure_fn_almost_equal_div_works_as_expected_4() {
        let out = (0..1).collect::<Vec<i32>>().divide_collection_by(3);
        for x in out {
            assert!(x.len() <= 1);
        }
    }

    #[test]
    fn ensure_fn_almost_equal_div_works_as_expected_5() {
        let out = (0..4).collect::<Vec<i32>>().divide_collection_by(10);
        for x in out {
            assert!(x.len() <= 1);
        }
    }
}
