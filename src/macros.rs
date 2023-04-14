/// Creates a [`DisjointSetVec`] containing the specified elements as separate singletons, not joined to each other.
///
/// `disjoint_set_vec!` allows `DisjointSetVec`s to be defined with the same syntax as array expressions.
/// There are two forms of this macro:
///
/// - Create a `DisjointSetVec` containing a given list of elements:
///
/// ```
/// use disjoint::disjoint_set_vec;
///
/// let ds = disjoint_set_vec!['a', 'b'];
/// assert_eq!(ds[0], 'a');
/// assert_eq!(ds[1], 'b');
/// assert!(!ds.is_joined(0, 1));
/// ```
///
/// - Create a `DisjointSetVec` from a given element and size:
///
/// ```
/// use disjoint::disjoint_set_vec;
///
/// let ds = disjoint_set_vec![1; 3];
/// assert_eq!(*ds.values(), [1, 1, 1]);
/// assert!(!ds.is_joined(0, 2));
/// ```
///
/// Note that unlike array expressions this syntax supports all elements
/// which implement [`Clone`] and the number of elements doesn't have to be
/// a constant.
///
/// This will use `clone` to duplicate an expression, so one should be careful
/// using this with types having a nonstandard `Clone` implementation. For
/// example, `disjoint_set_vec![Rc::new(1); 5]` will create five references
/// to the same boxed integer value, not five references pointing to independently
/// boxed integers.
///
/// Also, note that `disjoint_set_vec![expr; 0]` is allowed, and produces an empty container.
/// This will still evaluate `expr`, however, and immediately drop the resulting value, so
/// be mindful of side effects.
///
/// [`DisjointSetVec`]: crate::DisjointSetVec
#[macro_export]
macro_rules! disjoint_set_vec {
    ($elem:expr; $n:expr) => (
        $crate::DisjointSetVec::from(vec![$elem; $n])
    );
    ($($x:expr),*) => (
        $crate::DisjointSetVec::from(vec![$($x),*])
    );
    ($($x:expr,)*) => (disjoint_set_vec![$($x),*]);
}
