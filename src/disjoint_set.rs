use std::{
    cell::Cell,
    collections::{hash_map::Entry, HashMap},
};

/// A disjoint-set data structure for tracking which elements are joined, without managing any additional data associated to the elements.
///
/// This structure has methods like [`join`] or [`is_joined`] to modify or query which data is joined to which. For all of these, the elements are identified with their corresponding index. A `DisjointSet` of [`len`] `n` tracks elements from `0` to `n - 1`.   
///
/// [`join`]: DisjointSet::join
/// [`is_joined`]: DisjointSet::is_joined
/// [`len`]: DisjointSet::len
///
/// # Examples
///
/// ```
/// use disjoint::DisjointSet;
///
/// // Initially, elements are totally disjoint.
/// let mut ds = DisjointSet::with_len(3); // {0}, {1}, {2}
/// assert!(ds.is_joined(0, 0));
/// assert!(!ds.is_joined(0, 1));
///
/// // Elements can be joined together
/// ds.join(0, 1); // {0, 1}, {2}
/// assert!(ds.is_joined(0, 1));
/// assert!(ds.is_joined(1, 0));
/// assert!(!ds.is_joined(0, 2));
///
/// // Since 0 was joined to 1, if we join 1 to 2, then 0 is joined to 2.
/// ds.join(1, 2); // {0, 1, 2}
/// assert!(ds.is_joined(0, 2));
/// ```
/// For a real word application example, see [the crate examples].
///
/// [the crate examples]: crate#examples
#[allow(clippy::missing_inline_in_public_items)]
#[derive(Debug, Clone)]
pub struct DisjointSet {
    parents: Vec<Cell<usize>>,
    ranks: Vec<u8>,
}

impl Default for DisjointSet {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl DisjointSet {
    #[inline]
    #[must_use]
    fn get_parent(&self, id: usize) -> usize {
        self.parents[id].get()
    }

    #[inline]
    fn set_parent(&self, id: usize, new: usize) {
        self.parents[id].set(new);
    }

    #[inline]
    #[must_use]
    fn get_mut_rank(&mut self, id: usize) -> &mut u8 {
        &mut self.ranks[id]
    }

    /// Returns an element of the subset containing `child`.
    /// This exact element is returned for every member of the subset.
    ///
    /// # Important
    ///
    /// The specific choice of the returned element is an implementation detail.
    /// There are no further guarantees beyond what is documented here.
    /// If you just want to check if two elements are in the same subset, use [`is_joined`].
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSet;
    ///
    /// let mut ds = DisjointSet::with_len(3); // {0}, {1}, {2}
    /// assert_eq!(ds.root_of(0), 0);
    /// assert_eq!(ds.root_of(1), 1);
    /// assert_eq!(ds.root_of(2), 2);
    ///
    ///
    /// ds.join(0, 1); // {0, 1}, {2}
    /// assert_eq!(ds.root_of(0), ds.root_of(1));
    /// assert_ne!(ds.root_of(0), ds.root_of(2));
    ///
    /// ds.join(1, 2); // {0, 1, 2}
    /// assert_eq!(ds.root_of(0), ds.root_of(1));
    /// assert_eq!(ds.root_of(0), ds.root_of(2));
    /// ```
    ///
    /// [`is_joined`]: DisjointSet::is_joined
    #[inline]
    #[must_use]
    pub fn root_of(&self, mut child: usize) -> usize {
        let mut parent = self.get_parent(child);

        if child == parent {
            return child;
        };

        loop {
            let grandparent = self.get_parent(parent);
            if parent == grandparent {
                return parent;
            }

            self.set_parent(child, grandparent);
            child = parent;
            parent = grandparent;
        }
    }

    /// Constructs a new `DisjointSet` with `len` elements, named `0` to `n - 1`, each in its own set.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSet;
    ///
    /// let mut ds = DisjointSet::with_len(4);
    ///
    /// // The disjoint set contains 4 elements.
    /// assert_eq!(ds.len(), 4);
    ///
    /// // Two elements i and j are not joined in the same set, unless i = j.
    /// assert!(!ds.is_joined(0, 3));
    /// assert!(ds.is_joined(1, 1));
    ///
    /// ```
    #[inline]
    #[must_use]
    pub fn with_len(len: usize) -> Self {
        Self {
            parents: (0..len).map(Cell::new).collect(),
            ranks: vec![0; len],
        }
    }

    /// Constructs a new, empty `DisjointSet` with at least the specified capacity.
    ///
    /// It will be able to hold at least `capacity` elements without
    /// reallocating. This method is allowed to allocate for more elements than
    /// `capacity`. If `capacity` is 0, it will not allocate.
    ///
    /// It is important to note that although the returned `DisjointSet` has the
    /// minimum *capacity* specified, it will have a zero *length*.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSet;
    ///
    /// let mut ds = DisjointSet::with_capacity(10);
    ///
    /// // It contains no elements, even though it has capacity for more.
    /// assert_eq!(ds.len(), 0);
    ///
    /// // These are all done without reallocating...
    /// for _ in 0..10 {
    ///     ds.add_singleton();
    /// }
    ///
    /// // ...but this may make the disjoint set reallocate.
    /// ds.add_singleton();
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            parents: Vec::with_capacity(capacity),
            ranks: Vec::with_capacity(capacity),
        }
    }

    /// Adds a new element, not joined to any other element. Returns the index
    /// of the new element.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSet;
    ///
    /// let mut ds = DisjointSet::with_len(1);
    /// assert_eq!(ds.add_singleton(), 1);
    /// assert_eq!(ds.len(), 2);
    /// assert!(!ds.is_joined(0, 1));
    /// ```
    #[inline]
    pub fn add_singleton(&mut self) -> usize {
        let id = self.len();
        self.parents.push(Cell::new(id));
        self.ranks.push(0);
        id
    }

    /// If `first_element` and `second_element` are in different sets, joins them together and returns `true`.
    ///
    /// Otherwise, does nothing and returns `false`.
    ///
    /// # Panics
    ///
    /// Panics if `first_element` or `second_element` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSet;
    ///
    /// // Initially, each element is in its own set.
    /// let mut ds = DisjointSet::with_len(4); // {0}, {1}, {2}, {3}
    /// assert!(!ds.is_joined(0, 3));
    ///
    /// // By joining 0 to 1 and 2 to 3, we get two sets of two elements each.
    /// ds.join(0, 1); // {0, 1}, {2}, {3}
    /// ds.join(2, 3); // {0, 1}, {2, 3}
    /// assert!(ds.is_joined(0, 1));
    /// assert!(ds.is_joined(2, 3));
    /// assert!(!ds.is_joined(0, 3));
    ///
    /// // By further joining 2 to 3, all elements are now in the same set.
    /// ds.join(1, 2); // {0, 1, 2, 3}
    /// assert!(ds.is_joined(0, 3));
    /// ```
    #[inline]
    pub fn join(&mut self, first_element: usize, second_element: usize) -> bool {
        fn slow_path(ds: &mut DisjointSet, first_element: usize, second_element: usize) -> bool {
            let root_first = ds.root_of(first_element);
            let root_second = ds.root_of(second_element);

            if root_first == root_second {
                return false;
            }

            let rank_second = *ds.get_mut_rank(root_second);
            let rank_first = ds.get_mut_rank(root_first);

            if *rank_first < rank_second {
                ds.set_parent(root_first, root_second);
            } else {
                if *rank_first == rank_second {
                    *rank_first += 1;
                }
                ds.set_parent(root_second, root_first);
            }

            true
        }

        // Immediate parent check.
        if self.get_parent(first_element) == self.get_parent(second_element) {
            return false;
        }

        slow_path(self, first_element, second_element)
    }

    /// Returns `true` if `first_element` and `second_element` are in the same subset.
    ///
    /// # Panics
    ///
    /// Panics if `first_element` or `second_element` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSet;
    ///
    /// // Initially, elements are only joined to themselves.
    /// let mut ds = DisjointSet::with_len(3); // {0}, {1}, {2}
    /// assert!(ds.is_joined(0, 0));
    /// assert!(!ds.is_joined(0, 1));
    /// assert!(!ds.is_joined(0, 2));
    ///
    /// // By joining 1 to 0, we implicitely join 0 to 1.
    /// ds.join(1, 0); // {0, 1}, {2}
    /// assert!(ds.is_joined(1, 0));
    /// assert!(ds.is_joined(0, 1));
    ///
    /// // By joining 0 to 1 and 1 to 2, we implicitely join 0 to 2.
    /// ds.join(1, 2); // {0, 1, 2}
    /// assert!(ds.is_joined(0, 2));
    /// ```
    #[must_use]
    #[inline]
    pub fn is_joined(&self, first_element: usize, second_element: usize) -> bool {
        self.root_of(first_element) == self.root_of(second_element)
    }

    /// Returns the number of elements in the disjoint set, regardless of how they are joined together.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSet;
    ///
    /// let mut ds = DisjointSet::with_len(4);
    /// assert_eq!(ds.len(), 4);
    ///
    /// ds.join(1, 3);
    /// assert_eq!(ds.len(), 4);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.parents.len()
    }

    /// Returns `true` if the disjoint set contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSet;
    ///
    /// assert!(DisjointSet::new().is_empty());
    /// assert!(DisjointSet::with_len(0).is_empty());
    /// assert!(!DisjointSet::with_len(10).is_empty());
    /// ```
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.parents.is_empty()
    }

    /// Constructs a new, empty `DisjointSet`.
    ///
    /// The disjoint set will not allocate until elements are added to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// let mut vec: Vec<i32> = Vec::new();
    /// ```
    #[must_use]
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    pub fn new() -> Self {
        Self {
            parents: Vec::new(),
            ranks: Vec::new(),
        }
    }

    /// Clears the `DisjointSet`.
    /// 
    /// The disjoint set will retain its capacity, so adding elements will not
    /// allocate.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use disjoint::DisjointSet;
    ///
    /// let mut set = DisjointSet::new();
    /// set.add_singleton();
    /// set.add_singleton();
    /// set.clear();
    /// assert_eq!(set.len(), 0);
    /// // Does not allocate!
    /// set.add_singleton();
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.parents.clear();
        self.ranks.clear();
    }

    /// Returns a `Vec` of all sets. Each entry corresponds to one set, and is a `Vec` of its elements.
    ///
    /// The sets are ordered by their smallest contained element. The elements inside each sets are ordered.
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint::DisjointSet;
    ///
    /// let mut ds = DisjointSet::with_len(4); // {0}, {1}, {2}, {3}
    /// ds.join(3, 1); // {0}, {1, 3}, {2}
    /// assert_eq!(ds.sets(), vec![vec![0], vec![1, 3], vec![2]]);
    /// ```
    #[must_use]
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn sets(&self) -> Vec<Vec<usize>> {
        let mut result = Vec::new();
        let mut root_to_result_id = HashMap::new();

        for index in 0..self.len() {
            let root = self.root_of(index);
            let &mut result_id = root_to_result_id.entry(root).or_insert_with(|| {
                let id = result.len();
                result.push(Vec::with_capacity(1));
                id
            });
            result[result_id].push(index);
        }

        result
    }
}

impl PartialEq for DisjointSet {
    #[must_use]
    #[allow(clippy::missing_inline_in_public_items)]
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut self_root_to_other_root = HashMap::with_capacity(self.len());

        for i in 0..self.len() {
            let self_root = self.root_of(i);
            let other_root = other.root_of(i);

            match self_root_to_other_root.entry(self_root) {
                Entry::Occupied(entry) => {
                    if other_root != *entry.get() {
                        return false;
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(other_root);
                }
            }
        }

        true
    }
}

impl Eq for DisjointSet {}

#[cfg(test)]
mod test {
    use crate::DisjointSet;

    #[test]
    fn join_returns_false_even_if_immediate_parent_check_fails() {
        let mut ds = DisjointSet::with_len(4);

        ds.join(0, 1);
        ds.join(2, 3);
        ds.join(2, 0);

        assert_ne!(ds.parents[1], ds.parents[3]);
        assert!(!ds.join(1, 3));
    }

    #[test]
    fn clear_removes_elements_without_removing_capacity() {
        let mut set = DisjointSet::new();
        set.add_singleton();
        set.add_singleton();
        let capacity = set.parents.capacity();
        set.clear();
        assert_eq!(set.parents.capacity(), capacity);
    }
}
