extern crate alloc;

use alloc::rc::Rc;

use disjoint::{disjoint_set_vec, DisjointSet, DisjointSetVec};

fn verify_subsets<T>(disjoint_set: &DisjointSetVec<T>, expected_subsets: &[Vec<usize>]) {
    assert_eq!(
        disjoint_set.len(),
        expected_subsets.iter().map(|subset| subset.len()).sum()
    );

    for (subset_id, subset) in expected_subsets.iter().enumerate() {
        for (other_subset_id, other_subset) in expected_subsets.iter().enumerate() {
            for &member in subset {
                for &other_member in other_subset {
                    assert_eq!(
                        disjoint_set.is_joined(member, other_member),
                        subset_id == other_subset_id
                    )
                }
            }
        }
    }
}

#[test]
fn constructor_prodcues_singletons() {
    let disjoint_set = disjoint_set_vec!["a", "b", "c", "d", "e"];

    let expected_subsets = [vec![0], vec![1], vec![2], vec![3], vec![4]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn constructor_constructs_correct_len_and_if_empty() {
    for size in 0..100 {
        let disjoint_set = disjoint_set_vec![true; size];
        assert_eq!(disjoint_set.len(), size);
        assert_eq!(disjoint_set.is_empty(), size == 0);
    }
}

#[test]
fn joining_and_is_joined_dont_change_len() {
    for size in 3..103 {
        let mut disjoint_set = disjoint_set_vec![4.5; size];

        disjoint_set.join(0, 1);
        assert!(!disjoint_set.is_joined(1, 2));
        disjoint_set.join(0, 0);
        disjoint_set.join(0, 2);
        assert!(disjoint_set.is_joined(0, 0));
        assert!(disjoint_set.is_joined(1, 2));

        assert_eq!(disjoint_set.len(), size);
        assert!(!disjoint_set.is_empty());
    }
}

#[test]
fn join_one_pair() {
    let mut disjoint_set = disjoint_set_vec![3, 3, 4];
    disjoint_set.join(0, 1);
    let expected_subsets = [vec![0, 1], vec![2]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_element_with_itself() {
    let mut disjoint_set = disjoint_set_vec![-1, -2, -3];
    disjoint_set.join(1, 1);
    let expected_subsets = [vec![0], vec![1], vec![2]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_one_pair_twice() {
    let mut disjoint_set = disjoint_set_vec!["test"; 4];
    disjoint_set.join(0, 1);
    disjoint_set.join(0, 1);
    let expected_subsets = [vec![0, 1], vec![2], vec![3]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_one_pair_and_reversed() {
    let mut disjoint_set = disjoint_set_vec![true, false];
    disjoint_set.join(0, 1);
    disjoint_set.join(1, 0);
    let expected_subsets = [vec![0, 1]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_two_pairs_overlapping() {
    let mut disjoint_set = disjoint_set_vec!["test"; 4];
    disjoint_set.join(0, 1);
    disjoint_set.join(1, 3);
    let expected_subsets = [vec![0, 1, 3], vec![2]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_two_pairs_non_overlapping() {
    let mut disjoint_set = disjoint_set_vec![3; 10];
    disjoint_set.join(0, 4);
    disjoint_set.join(2, 7);
    let expected_subsets = [
        vec![0, 4],
        vec![2, 7],
        vec![1],
        vec![3],
        vec![5],
        vec![6],
        vec![8],
        vec![9],
    ];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_two_non_trivial_subsets() {
    let mut disjoint_set = disjoint_set_vec![3; 5];
    disjoint_set.join(0, 1);
    disjoint_set.join(2, 3);
    let expected_subsets = [vec![0, 1], vec![2, 3], vec![4]];
    verify_subsets(&disjoint_set, &expected_subsets);

    disjoint_set.join(0, 3);
    let expected_subsets = [vec![0, 1, 2, 3], vec![4]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
#[allow(renamed_and_removed_lints)]
#[allow(clippy::cyclomatic_complexity)]
fn arbitrary_sequence_of_join_and_is_joined() {
    let mut disjoint_set = disjoint_set_vec![3; 5];
    assert!(!disjoint_set.is_joined(1, 2));
    assert!(!disjoint_set.is_joined(1, 2));
    assert!(disjoint_set.is_joined(1, 1));
    assert!(!disjoint_set.is_joined(1, 2));
    disjoint_set.join(1, 4);
    assert!(!disjoint_set.is_joined(1, 2));
    assert!(!disjoint_set.is_joined(1, 2));
    assert!(disjoint_set.is_joined(1, 4));
    assert!(!disjoint_set.is_joined(1, 2));
    assert!(disjoint_set.is_joined(4, 1));
    disjoint_set.join(1, 4);
    assert!(!disjoint_set.is_joined(1, 2));
    assert!(!disjoint_set.is_joined(1, 2));
    assert!(disjoint_set.is_joined(1, 4));
    assert!(!disjoint_set.is_joined(1, 2));
    assert!(disjoint_set.is_joined(4, 1));
    disjoint_set.join(4, 1);
    disjoint_set.join(1, 1);
    assert!(disjoint_set.is_joined(1, 1));
    assert!(!disjoint_set.is_joined(1, 2));
    assert!(!disjoint_set.is_joined(1, 3));
    assert!(disjoint_set.is_joined(1, 4));
    disjoint_set.join(2, 3);
    assert!(!disjoint_set.is_joined(1, 3));
    assert!(disjoint_set.is_joined(3, 2));
    assert!(!disjoint_set.is_joined(1, 2));
    disjoint_set.join(1, 2);
    assert!(disjoint_set.is_joined(2, 4));
    assert!(disjoint_set.is_joined(3, 2));
    assert!(disjoint_set.is_joined(1, 2));
    assert!(!disjoint_set.is_joined(0, 2));
    disjoint_set.join(1, 1);
    disjoint_set.join(1, 1);
    assert!(!disjoint_set.is_joined(0, 2));
    assert!(disjoint_set.is_joined(0, 0));
    disjoint_set.join(4, 4);

    let expected_subsets = [vec![0], vec![1, 2, 3, 4]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
#[should_panic]
fn panic_join_first_element_oob() {
    let mut disjoint_set = disjoint_set_vec![3; 1000];
    disjoint_set.join(1000, 5);
}

#[test]
#[should_panic]
fn panic_join_second_element_oob() {
    let mut disjoint_set = disjoint_set_vec![3; 100];
    disjoint_set.join(0, 1_000_000_000_000);
}

#[test]
#[should_panic]
fn panic_join_both_elements_oob() {
    let mut disjoint_set = disjoint_set_vec![3; 0];
    disjoint_set.join(0, 0);
}

#[test]
#[should_panic]
fn panic_is_joined_first_element_oob() {
    let disjoint_set = disjoint_set_vec![3; 1000];
    let _ = disjoint_set.is_joined(1000, 5);
}

#[test]
#[should_panic]
fn panic_is_joined_second_element_oob() {
    let disjoint_set = disjoint_set_vec![3; 100];
    let _ = disjoint_set.is_joined(0, 1_000_000_000_000);
}

#[test]
#[should_panic]
fn panic_is_joined_both_elements_oob() {
    let disjoint_set = disjoint_set_vec![3; 0];
    let _ = disjoint_set.is_joined(0, 0);
}

#[test]
#[should_panic]
fn clone_clones() {
    let mut disjoint_set = disjoint_set_vec![3; 5];
    disjoint_set.join(2, 4);
    disjoint_set.join(3, 4);

    let expected_subsets = [vec![0], vec![0], vec![2, 3, 4]];

    let clone = disjoint_set.clone();
    assert_eq!(clone, disjoint_set);
    verify_subsets(&disjoint_set, &expected_subsets);
    verify_subsets(&clone, &expected_subsets);
}

#[test]
fn different_len_not_equal() {
    let l = disjoint_set_vec![3; 5];
    let r = disjoint_set_vec![3; 6];

    assert_ne!(l, r);
}

#[test]
fn same_vec_not_equal() {
    let mut l = disjoint_set_vec![3; 5];
    let mut r = disjoint_set_vec![3; 5];

    l.join(2, 4);
    r.join(2, 3);

    assert_ne!(l, r);
}

#[test]
fn same_ds_not_equal() {
    let mut l = disjoint_set_vec![4; 5];
    let mut r = disjoint_set_vec![3; 5];

    l.join(2, 4);
    r.join(2, 4);

    assert_ne!(l, r);
}

#[test]
fn different_joining_order_equal() {
    let mut l = disjoint_set_vec![3; 5];
    let mut r = disjoint_set_vec![3; 5];

    l.join(2, 4);
    l.join(1, 3);
    l.join(1, 2);

    r.join(4, 1);
    r.join(0, 0);
    r.join(1, 2);
    r.join(2, 2);
    r.join(2, 3);

    assert_eq!(l, r);
}

#[test]
fn indices_sets_empty() {
    let dsv = disjoint_set_vec![3; 0];

    let sets = dsv.indices().sets();

    let expected: Vec<Vec<_>> = Vec::new();

    assert_eq!(sets, expected);
}

#[test]
fn indices_sets_singletons() {
    let dsv = disjoint_set_vec![3; 10];

    let sets = dsv.indices().sets();

    let expected = vec![
        vec![0],
        vec![1],
        vec![2],
        vec![3],
        vec![4],
        vec![5],
        vec![6],
        vec![7],
        vec![8],
        vec![9],
    ];

    assert_eq!(sets, expected);
}

#[test]
fn indices_sets_all_in_one() {
    let mut dsv = disjoint_set_vec![3; 8];

    dsv.join(0, 1);
    dsv.join(2, 3);
    dsv.join(4, 5);
    dsv.join(6, 7);
    dsv.join(0, 3);
    dsv.join(5, 7);
    dsv.join(3, 4);

    let sets = dsv.indices().sets();

    let expected = [vec![0, 1, 2, 3, 4, 5, 6, 7]];

    assert_eq!(sets, expected);
}

#[test]
fn indices_sets_complex() {
    let mut dsv = disjoint_set_vec![3; 5];

    dsv.join(0, 3);
    dsv.join(2, 1);
    dsv.push(100);
    dsv.join(2, 5);

    let sets = dsv.indices().sets();

    let expected = [vec![0, 3], vec![1, 2, 5], vec![4]];

    assert_eq!(sets, expected);
}

#[test]
fn construct_with_capacity() {
    let dsv: DisjointSetVec<&&Option<&&char>> = DisjointSetVec::with_capacity(30);
    assert!(dsv.is_empty());
}

#[test]
fn push_produces_singleton() {
    let mut dsv = disjoint_set_vec![-1, -2, -3];
    dsv.join(0, 2);
    verify_subsets(&dsv, &[vec![0, 2], vec![1]]);
    assert_eq!(dsv.push(0), 3);
    verify_subsets(&dsv, &[vec![0, 2], vec![1], vec![3]]);
    assert_eq!(dsv.push(0), 4);
    verify_subsets(&dsv, &[vec![0, 2], vec![1], vec![3], vec![4]]);
}

#[test]
fn can_join_elements_added_later() {
    let mut dsv = disjoint_set_vec![-1, -2, -3];

    dsv.join(0, 2);

    dsv.push(3);
    dsv.push(4);
    dsv.push(-33);

    dsv.join(4, 5);
    dsv.join(3, 0);

    let expected = [vec![0, 2, 3], vec![1], vec![4, 5]];
    verify_subsets(&dsv, &expected);
}

#[test]
fn get_works_in_all_circumstances() {
    let mut dsv = disjoint_set_vec!["a", "a", "b"];

    assert_eq!(dsv.first(), Some(&"a"));
    assert_eq!(dsv.get(1), Some(&"a"));
    assert_eq!(dsv.get(2), Some(&"b"));
    assert_eq!(dsv.get(3), None);
    assert_eq!(dsv.get(4), None);

    dsv.join(0, 2);

    assert_eq!(dsv.first(), Some(&"a"));
    assert_eq!(dsv.get(1), Some(&"a"));
    assert_eq!(dsv.get(2), Some(&"b"));
    assert_eq!(dsv.get(3), None);
    assert_eq!(dsv.get(4), None);

    dsv[0] = "c";

    assert_eq!(dsv.first(), Some(&"c"));
    assert_eq!(dsv.get(1), Some(&"a"));
    assert_eq!(dsv.get(2), Some(&"b"));
    assert_eq!(dsv.get(3), None);
    assert_eq!(dsv.get(4), None);

    dsv.push("d");

    assert_eq!(dsv.first(), Some(&"c"));
    assert_eq!(dsv.get(1), Some(&"a"));
    assert_eq!(dsv.get(2), Some(&"b"));
    assert_eq!(dsv.get(3), Some(&"d"));
    assert_eq!(dsv.get(4), None);
}

#[test]
fn iter_works() {
    let mut dsv = disjoint_set_vec!["a", "a", "b"];

    dsv.join(0, 2);
    dsv.push("d");

    let expected = ["a", "a", "b", "d"];

    assert!(dsv.iter().eq(expected.iter()));
}

#[test]
fn iter_mut_works() {
    let mut dsv = disjoint_set_vec![3, 4, 5];

    dsv.join(0, 2);
    dsv.push(0);

    let clone = dsv.clone();

    for val in dsv.iter_mut() {
        *val *= 2;
    }

    let mut expected_dsv = disjoint_set_vec![6, 8, 10, 0];
    expected_dsv.join(0, 2);

    assert_eq!(dsv, expected_dsv);
    assert_ne!(dsv, clone);
}

#[test]
fn indexing_works() {
    let mut dsv = disjoint_set_vec![3, 4, 5];

    dsv.join(0, 2);
    dsv.push(0);

    assert_eq!(dsv[0], 3);
    assert_eq!(dsv[1], 4);
    assert_eq!(dsv[2], 5);
    assert_eq!(dsv[3], 0);
}

#[test]
fn root_of_works() {
    let mut dsv = disjoint_set_vec!['a', 'b', 'c'];
    assert_eq!(dsv.root_of(0), 0);
    assert_eq!(dsv.root_of(1), 1);
    assert_eq!(dsv.root_of(2), 2);

    dsv.join(0, 1);
    assert_eq!(dsv.root_of(0), dsv.root_of(1));
    assert_ne!(dsv.root_of(0), dsv.root_of(2));

    dsv.join(1, 2);
    assert_eq!(dsv.root_of(0), dsv.root_of(1));
    assert_eq!(dsv.root_of(0), dsv.root_of(2));
}

#[test]
#[should_panic]
fn indexing_panics() {
    let dsv = disjoint_set_vec![3, 4, 5];
    let _ = dsv[3];
}

#[test]
fn mutable_indexing_works() {
    let mut dsv = disjoint_set_vec![3, 4, 5];

    dsv.join(0, 2);
    dsv.push(0);

    dsv[1] = 33;

    assert_eq!(dsv[0], 3);
    assert_eq!(dsv[1], 33);
    assert_eq!(dsv[2], 5);
    assert_eq!(dsv[3], 0);
}

#[test]
#[should_panic]
fn mutable_indexing_panics() {
    let mut dsv = disjoint_set_vec![3, 4, 5];
    dsv[3] = 3;
}

#[test]
fn into_iter_works() {
    let expected = vec![3, 4, 5];

    let dsv = DisjointSetVec::from(expected.clone());

    let mut vec = Vec::with_capacity(3);

    for val in dsv {
        vec.push(val);
    }

    assert_eq!(vec, expected);
}

#[test]
fn ref_into_iter_works() {
    let expected = vec![3, 4, 5];

    let dsv = DisjointSetVec::from(expected.clone());

    let mut vec = Vec::with_capacity(3);

    for &val in &dsv {
        vec.push(val);
    }

    assert_eq!(vec, expected);
}

#[test]
fn mut_ref_into_iter_works() {
    let mut dsv = disjoint_set_vec![2, 3];

    for val in &mut dsv {
        *val += 10;
    }

    assert_eq!(dsv.first(), Some(&12));
    assert_eq!(dsv.get(1), Some(&13));
    assert_eq!(dsv.get(2), None);
}

#[test]
fn different_ways_of_empty_construction() {
    let empty_macro: DisjointSetVec<bool> = disjoint_set_vec![];
    assert!(empty_macro.is_empty());

    let empty_from: DisjointSetVec<bool> = DisjointSetVec::from(vec![]);
    assert!(empty_from.is_empty());

    let empty_new = DisjointSetVec::new();
    assert!(empty_new.is_empty());

    let empty_default = DisjointSetVec::default();
    assert!(empty_new.is_empty());

    let empty_macro_with_len = disjoint_set_vec![true; 0];
    assert!(empty_macro_with_len.is_empty());

    assert_eq!(empty_macro, empty_new);
    assert_eq!(empty_macro, empty_default);
    assert_eq!(empty_macro, empty_from);
    assert_eq!(empty_macro, empty_macro_with_len);

    std::vec::from_elem(3, 5);
}

#[test]
fn disjoint_set_vec_macro_corresponds_to_vec_macro() {
    assert_eq!(disjoint_set_vec![], DisjointSetVec::<bool>::from(vec![]));
    assert_eq!(
        disjoint_set_vec![2, 3, 4],
        DisjointSetVec::from(vec![2, 3, 4])
    );
    assert_eq!(disjoint_set_vec![3; 10], DisjointSetVec::from(vec![3; 10]));
}

#[test]
fn disjoint_set_vec_macro_is_consistent() {
    assert_eq!(disjoint_set_vec![], disjoint_set_vec![true; 0]);
    assert_eq!(disjoint_set_vec!['c', 'c'], disjoint_set_vec!['c'; 2]);
}

#[test]
fn disjoint_set_vec_macro_is_not_trivial() {
    assert_ne!(disjoint_set_vec![], disjoint_set_vec!['c']);
    assert_ne!(disjoint_set_vec!['c', 'c'], disjoint_set_vec!['c'; 3]);
}

#[test]
fn disjoint_set_vec_macro_clones() {
    let dsv = disjoint_set_vec![Rc::new(3); 10];

    assert_eq!(Rc::strong_count(&dsv[0]), 10);
}

#[test]
fn values_as_expected() {
    let values = vec![2, 3, 66];

    let dsv = DisjointSetVec::from(values.clone());

    assert_eq!(*dsv.values(), values);
}

#[test]
fn indices_as_expected() {
    let mut dsv = disjoint_set_vec![(); 100];
    let mut ds = DisjointSet::with_len(100);

    for (i, j) in &[
        (2, 3),
        (2, 5),
        (4, 10),
        (2, 7),
        (8, 9),
        (9, 2),
        (2, 7),
        (10, 99),
        (0, 10),
    ] {
        ds.join(*i, *j);
        dsv.join(*i, *j);
    }

    assert_eq!(*dsv.indices(), ds);
}

#[test]
fn from_vec_constructor() {
    let dsv = DisjointSetVec::from(vec![2; 3]);

    let mut expected = DisjointSetVec::with_capacity(3);

    for _ in 0..3 {
        expected.push(2);
    }

    assert_eq!(dsv, expected);
}

//Doesn't work on 1.31.
// #[test]
// fn from_array_constructor() {
//     let dsv = DisjointSetVec::from([2, 3, 6]);

//     let mut expected = DisjointSetVec::with_capacity(3);
//     expected.push(2);
//     expected.push(3);
//     expected.push(6);

//     assert_eq!(dsv, expected);
// }

#[test]
fn from_slice_constructor() {
    let dsv = DisjointSetVec::from(&[2, 3, 6][..]);

    let mut expected = DisjointSetVec::with_capacity(3);
    expected.push(2);
    expected.push(3);
    expected.push(6);

    assert_eq!(dsv, expected);
}
