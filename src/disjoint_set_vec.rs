use std::ops::{Deref, DerefMut};

use crate::DisjointSet;

/// A data structure for managing a `Vec<T>` of data together with a [`DisjointSet`] to keep track of which data entries are joined.
///
/// This structure exposes parts of the `Vec<T>` interface like [`push`], or access to the stored data via indexing (`container[index]`).
///
/// This structure also has methods like [`join`] or [`is_joined`] to modify or query which data is joined to which. These all work with the indices of the data.
///
/// The macro [`disjoint_set_vec!`] can be used for convenient array-like construction of [`DisjointSet`]s.
///
/// [`push`]: DisjointSetVec::push
/// [`join`]: DisjointSetVec::join
/// [`is_joined`]: DisjointSetVec::is_joined
/// [`disjoint_set_vec!`]: crate::disjoint_set_vec!
///
/// # Examples
///
/// ```
/// use disjoint::disjoint_set_vec;
///
/// // Initially, elements are totally disjoint.
/// let mut dsv = disjoint_set_vec!['a', 'b']; // {'a'}, {'b'}
///
/// // Joining 'a' and 'b' together via their indices.
/// dsv.join(0, 1); // {'a', 'b'}
///
/// // Adding 'c', not joined to anything.
/// dsv.push('c'); // {'a', 'b'}, {'c'}
///
/// // Change 'b' to 'y'.
/// dsv[1] = 'y'; // {'a', 'y'}, {'c'}
///
/// // Verify that 'a' is currently joined to 'y', but not to 'd'.
/// assert_eq!(dsv[0], 'a');
/// assert_eq!(dsv[1], 'y');
/// assert_eq!(dsv[2], 'c');
/// assert!(dsv.is_joined(0, 1));
/// assert!(!dsv.is_joined(0, 2));
/// ```
#[allow(clippy::missing_inline_in_public_items)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisjointSetVec<T> {
    data: Vec<T>,
    indices: DisjointSet,
}

impl<T> Default for DisjointSetVec<T> {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl<IntoVec, T> From<IntoVec> for DisjointSetVec<T>
where
    Vec<T>: From<IntoVec>,
{
    #[must_use]
    #[inline]
    fn from(value: IntoVec) -> Self {
        let data = Vec::from(value);
        let len = data.len();
        Self {
            data,
            indices: DisjointSet::with_len(len),
        }
    }
}

impl<T> DisjointSetVec<T> {
    /// Returns a `&Vec<T>` of all values.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::disjoint_set_vec;
    ///
    /// let dsv = disjoint_set_vec![
    ///     ("a", true),
    ///     ("b", false),
    ///     ("c", true),
    /// ];
    ///
    /// assert_eq!(*dsv.values(), [
    ///     ("a", true),
    ///     ("b", false),
    ///     ("c", true),
    /// ]);
    /// ```
    #[must_use]
    #[inline]
    pub const fn values(&self) -> &Vec<T> {
        &self.data
    }

    /// Returns a `&DisjointSet` of all indices and the information of how they are joined.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::disjoint_set_vec;
    ///
    /// let mut dsv = disjoint_set_vec![3; 10];
    ///
    /// dsv.join(2, 4);
    /// let indices = dsv.indices();
    ///
    /// assert!(indices.is_joined(2, 4));
    /// assert!(!indices.is_joined(3, 4));
    /// ```
    #[must_use]
    #[inline]
    pub const fn indices(&self) -> &DisjointSet {
        &self.indices
    }

    /// Constructs a new, empty `DisjointSetVec<T>` with at least the specified capacity.
    ///
    /// It will be able to hold at least `capacity` elements without
    /// reallocating. This method is allowed to allocate for more elements than
    /// `capacity`. If `capacity` is 0, it will not allocate.
    ///
    /// It is important to note that although the returned `DisjointSetVec<T>` has the
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
    /// let mut dsv = DisjointSetVec::with_capacity(10);
    ///
    /// // It contains no elements, even though it has capacity for more.
    /// assert_eq!(dsv.len(), 0);
    ///
    /// // These are all done without reallocating...
    /// for _ in 0..10 {
    ///     dsv.push("test");
    /// }
    ///
    /// // ...but this may make the disjoint set reallocate.
    /// dsv.push("test");
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            indices: DisjointSet::with_capacity(capacity),
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
    /// let mut dsv: DisjointSetVec<i32> = DisjointSetVec::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            indices: DisjointSet::new(),
        }
    }

    /// Clears the `DisjointSetVec`.
    /// 
    /// The disjoint set will retain its capacity, so adding elements will not
    /// allocate.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use disjoint::DisjointSetVec;
    ///
    /// let mut set = DisjointSetVec::new();
    /// set.push(123);
    /// set.push(456);
    /// set.clear();
    /// assert_eq!(set.values().len(), 0);
    /// assert!(set.values().capacity() >= 2);
    /// // Does not allocate!
    /// set.push(789);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
        self.indices.clear();
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
    /// use disjoint::disjoint_set_vec;
    ///
    /// let mut dsv = disjoint_set_vec![true];
    /// dsv.push(false);
    /// assert_eq!(dsv.len(), 2);
    /// assert!(dsv[0]);
    /// assert!(!dsv[1]);
    /// assert!(!dsv.is_joined(0, 1));
    /// ```
    #[inline]
    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.indices.add_singleton();
    }

    /// Returns the index of an element of the subset containing the element at `child_index`.
    /// This exact index is returned for all indices of elements of the subset.
    ///
    /// # Important
    ///
    /// The specific choice of the returned index is an implementation detail.
    /// There are no further guarantees beyond what is documented here.
    /// If you just want to check if two elements are in the same subset, use [`is_joined`].
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::disjoint_set_vec;
    ///
    /// let mut dsv = disjoint_set_vec!['a', 'b', 'c']; // {a}, {b}, {c}
    /// assert_eq!(dsv.root_of(0), 0);
    /// assert_eq!(dsv.root_of(1), 1);
    /// assert_eq!(dsv.root_of(2), 2);
    ///
    ///
    /// dsv.join(0, 1); // {a, b}, {c}
    /// assert_eq!(dsv.root_of(0), dsv.root_of(1));
    /// assert_ne!(dsv.root_of(0), dsv.root_of(2));
    ///
    /// dsv.join(1, 2); // {a, b, c}
    /// assert_eq!(dsv.root_of(0), dsv.root_of(1));
    /// assert_eq!(dsv.root_of(0), dsv.root_of(2));
    /// ```
    ///
    /// [`is_joined`]: DisjointSetVec::is_joined
    #[must_use]
    #[inline]
    pub fn root_of(&self, child_index: usize) -> usize {
        self.indices.root_of(child_index)
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
    /// use disjoint::disjoint_set_vec;
    ///
    /// // Initially, elements are only joined to themselves.
    /// let mut dsv = disjoint_set_vec!['a', 'b', 'c']; // {'a'}, {'b'}, {'c'}
    /// assert!(dsv.is_joined(0, 0));
    /// assert!(!dsv.is_joined(0, 1));
    /// assert!(!dsv.is_joined(0, 2));
    ///
    /// // By joining 'b' to 'a', we implicitely join 'a' to 'b'.
    /// dsv.join(1, 0); // {'a', 'b'}, {'c'}
    /// assert!(dsv.is_joined(1, 0));
    /// assert!(dsv.is_joined(0, 1));
    ///
    /// // By joining 'a' to 'b' and 'b' to 'c', we implicitely join 'a' to 'c'.
    /// dsv.join(1, 2); // {'a', 'b', 'c'}
    /// assert!(dsv.is_joined(0, 2));
    /// ```
    #[must_use]
    #[inline]
    pub fn is_joined(&self, first_index: usize, second_index: usize) -> bool {
        self.indices.is_joined(first_index, second_index)
    }

    /// If elements at `first_index` and `second_index` are in different sets, joins them together and returns `true`.
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
    /// use disjoint::disjoint_set_vec;
    ///
    /// // Initially, each element is in its own set.
    /// let mut dsv = disjoint_set_vec!['a', 'b', 'c', 'd']; // {'a'}, {'b'}, {'c'}, {'d'}
    /// assert!(!dsv.is_joined(0, 3));
    ///
    /// // By joining 'a' to 'b' and 'c' to 'd', we get two sets of two elements each.
    /// dsv.join(0, 1); // {'a', 'b'}, {'c'}, {'d'}
    /// dsv.join(2, 3); // {'a', 'b'}, {'c', 'd'}
    /// assert!(dsv.is_joined(0, 1));
    /// assert!(dsv.is_joined(2, 3));
    /// assert!(!dsv.is_joined(0, 3));
    ///
    /// // By further joining 'b' to 'c', all elements are now in the same set.
    /// dsv.join(1, 2); // {'a', 'b', 'c', 'd'}
    /// assert!(dsv.is_joined(0, 3));
    /// ```
    #[inline]
    pub fn join(&mut self, first_index: usize, second_index: usize) -> bool {
        self.indices.join(first_index, second_index)
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

impl<T> Deref for DisjointSetVec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        &self.data
    }
}

impl<T> DerefMut for DisjointSetVec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}
