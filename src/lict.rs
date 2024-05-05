/// The `Lict` struct is a generic type in Rust that represents a list with a range and a vector.
///
/// Properties:
///
/// * `lst`: The `lst` property is a vector that stores elements of type `T`. It is used to store the
/// elements of the `Lict` struct.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lict<T> {
    // pub rng: std::ops::Range<usize>,
    pub lst: Vec<T>,
}

impl<T> Lict<T> {
    /// The function `new` creates a new instance of a struct with a range and a vector.
    ///
    /// Arguments:
    ///
    /// * `lst`: A vector of elements of type T.
    ///
    /// Returns:
    ///
    /// The `new` function is returning an instance of the struct that it is defined in.
    ///
    /// # Examples
    ///
    /// ```
    /// use mywheel_rs::lict::Lict;
    ///
    /// assert_eq!(Lict::new(vec![1, 2, 3]), Lict { lst: vec![1, 2, 3] });
    /// ```
    #[inline]
    pub fn new(lst: Vec<T>) -> Self {
        // let len = lst.len();
        Self {
            // rng: 0..len,
            lst,
        }
    }

    #[inline]
    pub fn get(&self, key: usize) -> Option<&T> {
        self.lst.get(key)
    }

    #[inline]
    pub fn set(&mut self, key: usize, new_value: T) {
        if let Some(value) = self.lst.get_mut(key) {
            *value = new_value;
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.lst.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.lst.is_empty()
    }

    /// The `values` function returns an iterator over the values in a list.
    ///
    /// Returns:
    ///
    /// The `values` function is returning an iterator over the elements of the `lst` field of the struct.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::lict::Lict;
    ///
    /// let list = Lict::new(vec![1, 2, 3]);
    /// let mut cnt = 0;
    /// for value in list.values() {
    ///     cnt += 1;
    ///     assert_eq!(value, &cnt);
    /// }
    /// ```
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.lst.iter()
    }

    /// The function `items` returns an iterator that yields the index and reference to each element in
    /// the `lst` vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::lict::Lict;
    ///
    /// let list = Lict::new(vec![1, 2, 3]);
    /// assert_eq!(list.items().collect::<Vec<_>>(), vec![(0, &1), (1, &2), (2, &3)]);
    /// ```
    #[inline]
    pub fn items(&self) -> impl Iterator<Item = (usize, &T)> {
        self.lst.iter().enumerate()
    }

    /// The function checks if a given key is within the range of the lst vector.
    ///
    /// Arguments:
    ///
    /// * `key`: The `key` parameter is of type `usize`, which represents an unsigned integer. It is
    /// used to specify the index of an element in the `lst` array.
    ///
    /// Returns:
    ///
    /// A boolean value is being returned.
    ///
    /// # Examples:
    ///
    /// ```
    /// use mywheel_rs::lict::Lict;
    ///
    /// assert_eq!(Lict::new(vec![1, 2, 3]).contains(0), true);
    /// assert_eq!(Lict::new(vec![1, 2, 3]).contains(3), false);
    /// ```
    #[inline]
    pub fn contains(&self, key: usize) -> bool {
        key < self.lst.len()
    }
}

impl<T> std::ops::Index<usize> for Lict<T> {
    type Output = T;

    /// The `index` function returns a reference to an element in a list based on the given key.
    ///
    /// Arguments:
    ///
    /// * `key`: The `key` parameter is of type `usize`. It represents the index of the element in the
    /// `lst` field that you want to access.
    ///
    /// Returns:
    ///
    /// The method `index` is returning a reference to an element in the `lst` field of the struct. The
    /// type of the returned value is `&Self::Output`, which is a reference to the output type
    /// associated with the struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use mywheel_rs::lict::Lict;
    ///
    /// let lict = Lict::new(vec![1, 2, 3]);
    /// assert_eq!(lict[0], 1);
    /// assert_eq!(lict[1], 2);
    /// ```
    #[inline]
    fn index(&self, key: usize) -> &Self::Output {
        &self.lst[key]
    }
}

impl<T> std::ops::IndexMut<usize> for Lict<T> {
    /// The function `index_mut` returns a mutable reference to an element in a list based on the given
    /// key.
    ///
    /// Arguments:
    ///
    /// * `key`: The `key` parameter is of type `usize`. It represents the index of the element in the
    /// `lst` field that you want to access and modify.
    ///
    /// Returns:
    ///
    /// A mutable reference to an element in the `lst` vector at the given `key` index.
    ///
    /// # Examples
    ///
    /// ```
    /// use mywheel_rs::lict::Lict;
    ///
    /// let mut lict = Lict::new(vec![1, 2, 3]);
    /// lict[0] = 10;
    /// assert_eq!(lict[0], 10);
    /// ```
    #[inline]
    fn index_mut(&mut self, key: usize) -> &mut Self::Output {
        &mut self.lst[key]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lict() {
        let a = Lict::new(vec![0; 8]);
        // for i in &mut a {
        //     a[i] = i * i;
        // }
        for (i, v) in a.items() {
            println!("{}: {}", i, v);
        }
        assert!(a.contains(3));
    }

    #[test]
    fn test_lict2() {
        let mut a = Lict::new(vec![1, 4, 3, 6]);
        assert_eq!(a[2], 3);
        assert!(a.contains(3));
        assert_eq!(a.len(), 4);
        assert_eq!(a.values().collect::<Vec<&i32>>(), vec![&1, &4, &3, &6]);
        assert_eq!(
            a.items().collect::<Vec<(usize, &i32)>>(),
            vec![(0, &1), (1, &4), (2, &3), (3, &6)]
        );
        // assert_eq!(a.keys(), vec![0, 1, 2, 3]);
        a[2] = 7;
        assert_eq!(a[2], 7);
    }
}
