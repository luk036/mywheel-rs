/// The `RepeatArray` struct represents an array that contains a single value repeated a specified
/// number of times.
/// 
/// Properties:
/// 
/// * `value`: The `value` property is a generic type `T` that represents the value that will be
/// repeated in the array.
/// * `size`: The `size` property represents the number of elements in the array.
pub struct RepeatArray<T> {
    value: T,
    size: usize,
}

impl<T: Copy> RepeatArray<T> {
    /// The function creates a new RepeatArray with a given value and size.
    /// 
    /// Arguments:
    /// 
    /// * `value`: The value parameter is the value that will be repeated in the array.
    /// * `size`: The `size` parameter represents the desired size of the `RepeatArray`. It specifies
    /// the number of elements that the `RepeatArray` should contain.
    /// 
    /// Returns:
    /// 
    /// The `new` function is returning an instance of the `RepeatArray<T>` struct.
    /// 
    /// Examples:
    /// 
    /// ```rust
    /// use mywheel_rs::array_like::RepeatArray;
    /// let array = RepeatArray::new(1, 5);
    /// assert_eq!(array.len(), 5);
    /// ```
    pub fn new(value: T, size: usize) -> RepeatArray<T> {
        RepeatArray { value, size }
    }
}

impl<T: Copy> std::ops::Index<usize> for RepeatArray<T> {
    type Output = T;

    /// The `index` function returns a reference to the value at the specified index.
    /// 
    /// Arguments:
    /// 
    /// * `_index`: The `_index` parameter is of type `usize`, which represents an index value used to
    /// access elements in a collection or array.
    /// 
    /// Returns:
    /// 
    /// The method is returning a reference to the value stored in the `self` object.
    /// 
    /// Examples:
    /// 
    /// ```rust
    /// use mywheel_rs::array_like::RepeatArray;
    /// let array = RepeatArray::new(1, 5);
    /// assert_eq!(array[0], 1);
    /// assert_eq!(array[1], 1);
    /// ```
    fn index(&self, _index: usize) -> &Self::Output {
        &self.value
    }
}

impl<T: Copy> std::iter::Iterator for RepeatArray<T> {
    type Item = T;

    /// The `next` function returns the next item in the iterator if there is one, otherwise it returns
    /// `None`.
    /// 
    /// Returns:
    /// 
    /// The method `next` returns an `Option<Self::Item>`.
    /// 
    /// Examples:
    /// 
    /// ```rust
    /// use mywheel_rs::array_like::RepeatArray;
    /// let mut array = RepeatArray::new(1, 5);
    /// assert_eq!(array.next(), Some(1));
    /// assert_eq!(array.next(), Some(1));
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        if self.size > 0 {
            self.size -= 1;
            Some(self.value)
        } else {
            None
        }
    }
}

impl<T: Copy> std::iter::ExactSizeIterator for RepeatArray<T> {
    /// The `len` function returns the size of a data structure.
    /// 
    /// Returns:
    /// 
    /// The `len` function is returning the value of `self.size`, which is of type `usize`.
    /// 
    /// Examples:
    /// 
    /// ```rust
    /// use mywheel_rs::array_like::RepeatArray;
    /// let array = RepeatArray::new(1, 5);
    /// assert_eq!(array.len(), 5);
    /// ```
    fn len(&self) -> usize {
        self.size
    }
}

/// The ShiftArray type represents an array that can be shifted to the left or right without copying or
/// moving its elements.
///
/// Properties:
///
/// * `start`: The `start` property represents the index of the first element in the `ShiftArray`. It
/// indicates the starting point from which elements are accessed or shifted.
/// * `lst`: The `lst` property is a vector that holds the elements of the `ShiftArray`. It is of type
/// `Vec<T>`, where `T` is a generic type parameter that can be replaced with any type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShiftArray<T> {
    pub start: usize,
    pub lst: Vec<T>,
}

impl<T> ShiftArray<T> {
    /// The function "new" initializes a new instance of a struct with a starting index of 0 and a given
    /// list.
    ///
    /// Arguments:
    ///
    /// * `lst`: The `lst` parameter is a `Vec<T>`, which is a vector of elements of type `T`.
    ///
    /// Returns:
    ///
    /// The `new` function is returning an instance of the struct that it is defined in.
    /// 
    /// Examples:
    /// 
    /// ```
    /// use mywheel_rs::array_like::ShiftArray;
    ///
    /// let mut shift_array = ShiftArray::new(vec![1, 2, 3]);
    /// assert_eq!(shift_array.start, 0);
    /// ```
    pub fn new(lst: Vec<T>) -> Self {
        Self { start: 0, lst }
    }

    /// The function sets the start value of a variable.
    ///
    /// Arguments:
    ///
    /// * `start`: The `start` parameter is of type `usize`, which represents an unsigned integer that
    /// can hold the size of any object in memory. It is used to set the value of the `start` field in
    /// the struct or object that this method belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use mywheel_rs::array_like::ShiftArray;
    ///
    /// let mut shift_array = ShiftArray::new(vec![1, 2, 3]);
    /// shift_array.set_start(1);
    /// assert_eq!(shift_array.start, 1);
    /// ```
    pub fn set_start(&mut self, start: usize) {
        self.start = start;
    }

    /// The `items` function returns an iterator that yields the index and reference to each element in
    /// the `lst` vector, with the index adjusted by the `start` value.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mywheel_rs::array_like::ShiftArray;
    /// let mut shift_array = ShiftArray::new(vec![1, 2, 3]);
    /// shift_array.set_start(1);
    /// for (i, v) in shift_array.items() {
    ///     assert_eq!(i, *v as usize);
    /// }
    /// ```
    pub fn items(&self) -> impl Iterator<Item = (usize, &T)> {
        self.lst
            .iter()
            .enumerate()
            .map(move |(i, v)| (i + self.start, v))
    }
}

impl<T> std::ops::Index<usize> for ShiftArray<T> {
    type Output = T;

    /// The `index` function returns a reference to an element in a list based on a given key.
    ///
    /// Arguments:
    ///
    /// * `key`: The `key` parameter is of type `usize` and represents the index of the element to be
    /// accessed in the `lst` field.
    ///
    /// Returns:
    ///
    /// The method `index` returns a reference to an element of `self.lst` at the specified index `key -
    /// self.start`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mywheel_rs::array_like::ShiftArray;
    /// let mut shift_array = ShiftArray::new(vec![1, 2, 3]);
    /// assert_eq!(shift_array[2], 3);
    /// shift_array.set_start(1);
    /// assert_eq!(shift_array[2], 2);
    /// ```
    fn index(&self, key: usize) -> &Self::Output {
        &self.lst[key - self.start]
    }
}

impl<T> std::ops::IndexMut<usize> for ShiftArray<T> {
    /// The function `index_mut` returns a mutable reference to an element in a list based on a given
    /// key.
    ///
    /// Arguments:
    ///
    /// * `key`: The `key` parameter is of type `usize` and represents the index of the element to be
    /// accessed in the `lst` vector.
    ///
    /// Returns:
    ///
    /// A mutable reference to an element in the `lst` vector, located at the index `key - self.start`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mywheel_rs::array_like::ShiftArray;
    /// let mut shift_array = ShiftArray::new(vec![1, 2, 3]);
    /// assert_eq!(shift_array[2], 3);
    /// shift_array.set_start(1);
    /// assert_eq!(shift_array[2], 2);
    /// shift_array[2] = 4;
    /// assert_eq!(shift_array[2], 4);
    /// ```
    fn index_mut(&mut self, key: usize) -> &mut Self::Output {
        &mut self.lst[key - self.start]
    }
}

impl<T: Clone> std::iter::Iterator for ShiftArray<T> {
    type Item = T;

    /// The `next` function returns the next item in a list if there is one, otherwise it returns
    /// `None`.
    /// 
    /// Returns:
    /// 
    /// an `Option<Self::Item>`.
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.lst.len() {
            let value = self.lst[self.start].clone();
            self.start += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<T: Copy> std::iter::ExactSizeIterator for ShiftArray<T> {
    /// The `len` function returns the length of a list, taking into account a starting index.
    /// 
    /// Returns:
    /// 
    /// the length of the list `lst` minus the value of `start`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mywheel_rs::array_like::ShiftArray;
    /// let shift_array = ShiftArray::new(vec![1, 2, 3]);
    /// assert_eq!(shift_array.len(), 3);
    /// ```
    fn len(&self) -> usize {
        self.lst.len() - self.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_array() {
        let arr: RepeatArray<i32> = RepeatArray::new(1, 10);
        assert_eq!(arr[4], 1);
        for i in arr {
            assert_eq!(i, 1);
        }
    }

    #[test]
    fn test_shift_array() {
        let mut a = ShiftArray::new(vec![2, 3, 5, 7, 11]);
        a.set_start(5);
        assert_eq!(a[6], 3);
        for (i, v) in a.items() {
            println!("{}: {}", i, v);
        }
    }
}
