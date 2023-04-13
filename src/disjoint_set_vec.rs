use std::{
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

use crate::DisjointSet;

/// A data structure for managing a `Vec<T>` of data together with a [`DisjointSet`] to keep track of which data entries are joined.
///
/// This structure exposes parts of the `Vec<T>` interface like [`push`], or access to the stored data via indexing (`container[index]`).
///
/// This structure also has methods like [`join`] or [`get_index_sets`] to modify or query which data is joined to which. These all work with the indices of the data.  
///
/// [`push`]: DisjointSetVec::push
/// [`join`]: DisjointSetVec::join
/// [`get_index_sets`]: DisjointSetVec::get_index_sets
///
/// # Examples
///
/// ```
/// use disjoint::DisjointSetVec;
///
/// // Initially, elements are totally disjoint.
/// let mut ds = DisjointSetVec::from(vec!['a', 'b']); // {'a'}, {'b'}
///
/// // Joining 'a' and 'b' together via their indices.
/// ds.join(0, 1); // {'a', 'b'}
///
/// // Adding 'd', not joined to anything.
/// ds.push('c'); // {'a', 'b'}, {'c'}
///
/// // Change 'b' to 'y'.
/// ds[1] = 'y'; // {'a', 'y'}, {'c'}
///
/// // Verify that 'a' is currently joined to 'y', but not to 'd'.
/// assert_eq!(ds[0], 'a');
/// assert_eq!(ds[1], 'y');
/// assert_eq!(ds[2], 'c');
/// assert!(ds.is_joined(0, 1));
/// assert!(!ds.is_joined(0, 2));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisjointSetVec<T> {
    data: Vec<T>,
    sets: DisjointSet,
}

impl<T> Default for DisjointSetVec<T> {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Vec<T>> for DisjointSetVec<T> {
    #[inline]
    #[must_use]
    fn from(value: Vec<T>) -> Self {
        let len = value.len();
        Self {
            data: value,
            sets: DisjointSet::with_len(len),
        }
    }
}

impl<T> DisjointSetVec<T> {
    /// Constructs a new, empty `DisjointSetVec` with at least the specified capacity.
    ///
    /// It will be able to hold at least `capacity` elements without
    /// reallocating. This method is allowed to allocate for more elements than
    /// `capacity`. If `capacity` is 0, it will not allocate.
    ///
    /// It is important to note that although the returned `DisjointSetVec` has the
    /// minimum *capacity* specified, it will have a zero *length*.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSetVec;
    ///
    /// let mut ds = DisjointSetVec::with_capacity(10);
    ///
    /// // It contains no elements, even though it has capacity for more.
    /// assert_eq!(ds.len(), 0);
    ///
    /// // These are all done without reallocating...
    /// for _ in 0..10 {
    ///     ds.push("test");
    /// }
    ///
    /// // ...but this may make the disjoint set reallocate
    /// ds.push("test");
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            sets: DisjointSet::with_capacity(capacity),
        }
    }

    /// Constructs a new, empty `DisjointSetVec<T>`.
    ///
    /// This operation itself will not allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// use disjoint::DisjointSetVec;
    ///
    /// let mut ds: DisjointSetVec<i32> = DisjointSetVec::new();
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            data: Vec::new(),
            sets: DisjointSet::new(),
        }
    }

    /// Returns the number of elements, regardless of how they are joined together.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSetVec;
    ///
    /// let mut ds = DisjointSetVec::from(vec![10, 20, 30, 20]);
    /// assert_eq!(ds.len(), 4);
    ///
    /// ds.join(1, 2);
    /// assert_eq!(ds.len(), 4);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns a reference to the element at `index`, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSetVec;
    ///
    /// let ds = DisjointSetVec::from(vec![10, 40, 30]);
    /// assert_eq!(ds.get(1), Some(&40));
    /// assert_eq!(ds.get(3), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Returns `true` if the collection contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSetVec;
    ///
    /// let mut ds = DisjointSetVec::new();
    /// assert!(ds.is_empty());
    ///
    /// ds.push(2);
    /// assert!(!ds.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns an iterator over the collection.
    ///
    /// The iterator yields all items in index order, independent of how they are joined.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSetVec;
    ///
    /// let mut ds = DisjointSetVec::from(vec![10, 40, 30]);
    ///
    /// ds.join(0, 2);
    ///
    /// let mut iterator = ds.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&10));
    /// assert_eq!(iterator.next(), Some(&40));
    /// assert_eq!(iterator.next(), Some(&30));
    /// assert_eq!(iterator.next(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    /// Returns an iterator over the collection that allows modifying each value.
    ///
    /// The iterator yields all items in index order, independent of how they are joined.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSetVec;
    ///
    /// let mut ds = DisjointSetVec::from(vec![10, 40, 30]);
    ///
    /// for elem in ds.iter_mut() {
    ///     *elem /= 10;
    /// }
    ///
    /// assert_eq!(ds[0], 1);
    /// assert_eq!(ds[1], 4);
    /// assert_eq!(ds[2], 3);
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }

    /// Appends an element to the back of a collection, not joined to any other element.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSetVec;
    ///
    /// let mut ds = DisjointSetVec::from(vec![true]);
    /// ds.push(false);
    /// assert_eq!(ds.len(), 2);
    /// assert!(ds[0]);
    /// assert!(!ds[1]);
    /// assert!(!ds.is_joined(0, 1));
    /// ```
    #[inline]
    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.sets.add_singleton();
    }

    /// Returns `true` if elements at `first_index` and `second_index` are in the same subset.
    ///
    /// # Panics
    ///
    /// Panics if `first_index` or `second_index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSetVec;
    ///
    /// // Initially, elements are only joined to themserves.
    /// let mut ds = DisjointSetVec::from(vec!['a', 'b', 'c']); // {'a'}, {'b'}, {'c'}
    /// assert!(ds.is_joined(0, 0));
    /// assert!(!ds.is_joined(0, 1));
    /// assert!(!ds.is_joined(0, 2));
    ///
    /// // By joining 'b' to 'a', we implicitely join 'a' to 'b'.
    /// ds.join(1, 0); // {'a', 'b'}, {'c'}
    /// assert!(ds.is_joined(1, 0));
    /// assert!(ds.is_joined(0, 1));
    ///
    /// // By joining 'a' to 'b' and 'b' to 'c', we implicitely join 'a' to 'c'.
    /// ds.join(1, 2); // {'a', 'b', 'c'}
    /// assert!(ds.is_joined(0, 2));
    /// ```
    #[must_use]
    #[inline]
    pub fn is_joined(&self, first_index: usize, second_index: usize) -> bool {
        self.sets.is_joined(first_index, second_index)
    }

    /// If elments at `first_index` and `second_index` are in different sets, joins them together and returns `true`.
    ///
    /// Otherwise, does nothing and returns `false`.
    ///
    /// # Panics
    ///
    /// Panics if `first_index` or `second_index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSetVec;
    ///
    /// // Initially, each element is in its own set.
    /// let mut ds = DisjointSetVec::from(vec!['a', 'b', 'c', 'd']); // {'a'}, {'b'}, {'c'}, {'d'}
    /// assert!(!ds.is_joined(0, 3));
    ///
    /// // By joining 'a' to 'b' and 'c' to 'd', we get two sets of two elements each.
    /// ds.join(0, 1); // {'a', 'b'}, {'c'}, {'d'}
    /// ds.join(2, 3); // {'a', 'b'}, {'c', 'd'}
    /// assert!(ds.is_joined(0, 1));
    /// assert!(ds.is_joined(2, 3));
    /// assert!(!ds.is_joined(0, 3));
    ///
    /// // By further joining 'b' to 'c', all elements are now in the same set.
    /// ds.join(1, 2); // {'a', 'b', 'c', 'd'}
    /// assert!(ds.is_joined(0, 3));
    /// ```
    #[inline]
    pub fn join(&mut self, first_index: usize, second_index: usize) -> bool {
        self.sets.join(first_index, second_index)
    }

    /// Returns a `Vec` of all index sets. Each entry corresponds to one set, and is a `Vec` of the indices of its elements.
    ///
    /// The sets are ordered by their smallest contained index. The indices inside each sets are ordered.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSetVec;
    ///
    /// let mut ds = DisjointSetVec::from(vec!['a', 'b', 'c', 'd']); // {'a'}, {'b'}, {'c'}, {'d'}
    /// ds.join(3, 1); // {'a'}, {'b', 'd'}, {'c'}
    /// assert_eq!(ds.get_index_sets(), vec![vec![0], vec![1, 3], vec![2]]);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_index_sets(&self) -> Vec<Vec<usize>> {
        self.sets.get_sets()
    }
}

impl<T> Index<usize> for DisjointSetVec<T> {
    type Output = T;

    #[inline]
    #[must_use]
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<T> IndexMut<usize> for DisjointSetVec<T> {
    #[inline]
    #[must_use]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl<T> IntoIterator for DisjointSetVec<T> {
    type Item = <Vec<T> as IntoIterator>::Item;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.data)
    }
}

impl<'a, T> IntoIterator for &'a DisjointSetVec<T> {
    type Item = <&'a Vec<T> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self.data)
    }
}

impl<'a, T> IntoIterator for &'a mut DisjointSetVec<T> {
    type Item = <&'a mut Vec<T> as IntoIterator>::Item;
    type IntoIter = <&'a mut Vec<T> as IntoIterator>::IntoIter;

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&mut self.data)
    }
}
