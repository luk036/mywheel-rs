/// The `RepeatArray` struct represents an array that contains a single value repeated a specified
/// number of times.
///
/// Properties:
///
/// * `value`: The `value` property is a generic type `T` that represents the value that will be repeated in the array.
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
    ///           the number of elements that the `RepeatArray` should contain.
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
    /// assert_eq!(array[0], 1);
    /// assert_eq!(array[1], 1);
    /// ```
    pub fn new(value: T, size: usize) -> RepeatArray<T> {
        RepeatArray { value, size }
    }

    pub fn get(&self, _index: usize) -> T {
        self.value
    }

    pub fn iter(&self) -> RepeatArrayIterator<T> {
        RepeatArrayIterator {
            value: self.value,
            size: self.size,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

pub struct RepeatArrayIterator<T> {
    value: T,
    size: usize,
}

impl<T: Copy> Iterator for RepeatArrayIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size > 0 {
            self.size -= 1;
            Some(self.value)
        } else {
            None
        }
    }
}

impl<T: Copy> ExactSizeIterator for RepeatArrayIterator<T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<T: Copy> std::ops::Index<usize> for RepeatArray<T> {
    type Output = T;

    /// The `index` function returns a reference to the value at the specified index.
    ///
    /// Arguments:
    ///
    /// * `_index`: The `_index` parameter is of type `usize`, which represents an index value used to
    ///             access elements in a collection or array.
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

/// The ShiftArray type represents an array that can be shifted to the left or right without copying or
/// moving its elements.
///
/// Properties:
///
/// * `start`: The `start` property represents the index of the first element in the `ShiftArray`. It
///             indicates the starting point from which elements are accessed or shifted.
/// * `lst`: The `lst` property is a vector that holds the elements of the `ShiftArray`. It is of type
///             `Vec<T>`, where `T` is a generic type parameter that can be replaced with any type.
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
    /// assert_eq!(shift_array.lst, vec![1, 2, 3]);
    /// assert_eq!(shift_array.len(), 3);
    /// assert_eq!(shift_array[0], 1);
    /// ```
    pub fn new(lst: Vec<T>) -> Self {
        Self { start: 0, lst }
    }

    /// The function sets the start value of a variable.
    ///
    /// Arguments:
    ///
    /// * `start`: The `start` parameter is of type `usize`, which represents an unsigned integer that
    ///             can hold the size of any object in memory. It is used to set the value of the `start` field in
    ///             the struct or object that this method belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use mywheel_rs::array_like::ShiftArray;
    ///
    /// let mut shift_array = ShiftArray::new(vec![1, 2, 3]);
    /// shift_array.set_start(1);
    /// assert_eq!(shift_array.start, 1);
    /// assert_eq!(shift_array.len(), 3);
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

    pub fn iter(&self) -> ShiftArrayIterator<'_, T> {
        ShiftArrayIterator {
            array: self,
            current: self.start,
        }
    }

    pub fn len(&self) -> usize {
        self.lst.len()
    }
}

pub struct ShiftArrayIterator<'a, T> {
    array: &'a ShiftArray<T>,
    current: usize,
}

impl<'a, T: Clone> Iterator for ShiftArrayIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.array.lst.len() {
            let value = self.array.lst[self.current].clone();
            self.current += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<T> std::ops::Index<usize> for ShiftArray<T> {
    type Output = T;

    /// The `index` function returns a reference to an element in a list based on a given key.
    ///
    /// Arguments:
    ///
    /// * `key`: The `key` parameter is of type `usize` and represents the index of the element to be
    ///          accessed in the `lst` field.
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
    ///          accessed in the `lst` vector.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_array() {
        let arr: RepeatArray<i32> = RepeatArray::new(1, 10);
        assert_eq!(arr.len(), 10);
        assert_eq!(arr[4], 1);
        for i in arr.iter() {
            assert_eq!(i, 1);
        }
    }

    #[test]
    fn test_shift_array() {
        let mut a = ShiftArray::new(vec![2, 3, 5, 7, 11]);
        a.set_start(5);
        assert_eq!(a[6], 3);
        assert_eq!(a.len(), 5);
        a[6] = 13;
        assert_eq!(a[6], 13);
        let mut cnt = 5;
        for v in a.iter() {
            assert_eq!(v, a[cnt]);
            cnt += 1;
        }
        for (i, v) in a.items() {
            println!("{}: {}", i, v);
        }
    }

    #[test]
    fn test_repeat_array2() {
        let repeat_array: RepeatArray<i32> = RepeatArray::new(1, 5);
        assert_eq!(repeat_array.value, 1);
        assert_eq!(repeat_array.size, 5);
        assert_eq!(repeat_array[0], 1);
        assert_eq!(repeat_array[1], 1);
        assert_eq!(repeat_array[2], 1);
        assert_eq!(repeat_array[3], 1);
        assert_eq!(repeat_array[4], 1);
        assert_eq!(repeat_array.get(0), 1);
        assert_eq!(repeat_array.get(1), 1);
        assert_eq!(repeat_array.get(2), 1);
        assert_eq!(repeat_array.get(3), 1);
        assert_eq!(repeat_array.get(4), 1);
        for i in repeat_array.iter() {
            assert_eq!(i, 1);
        }
    }

    #[test]
    fn test_shift_array2() {
        let mut shift_array: ShiftArray<i32> = ShiftArray::new(vec![1, 2, 3, 4, 5]);
        shift_array.set_start(3);
        assert_eq!(shift_array[6], 4);
        assert_eq!(shift_array[7], 5);
        shift_array[6] = 8;
        assert_eq!(shift_array[6], 8);
        for (i, v) in shift_array.items() {
            assert_eq!(v, &shift_array[i]);
        }
    }
}
