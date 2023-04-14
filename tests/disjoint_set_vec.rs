use disjoint::DisjointSetVec;

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
    let disjoint_set = DisjointSetVec::from(vec!["a", "b", "c", "d", "e"]);

    let expected_subsets = [vec![0], vec![1], vec![2], vec![3], vec![4]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn constructor_constructs_correct_len_and_if_empty() {
    for size in 0..100 {
        let disjoint_set = DisjointSetVec::from(vec![true; size]);
        assert_eq!(disjoint_set.len(), size);
        assert_eq!(disjoint_set.is_empty(), size == 0);
    }
}

#[test]
fn joining_and_is_joined_dont_change_len() {
    for size in 3..103 {
        let mut disjoint_set = DisjointSetVec::from(vec![4.5; size]);

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
    let mut disjoint_set = DisjointSetVec::from(vec![3, 3, 4]);
    disjoint_set.join(0, 1);
    let expected_subsets = [vec![0, 1], vec![2]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_element_with_itself() {
    let mut disjoint_set = DisjointSetVec::from(vec![-1, -2, -3]);
    disjoint_set.join(1, 1);
    let expected_subsets = [vec![0], vec![1], vec![2]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_one_pair_twice() {
    let mut disjoint_set = DisjointSetVec::from(vec!["test"; 4]);
    disjoint_set.join(0, 1);
    disjoint_set.join(0, 1);
    let expected_subsets = [vec![0, 1], vec![2], vec![3]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_one_pair_and_reversed() {
    let mut disjoint_set = DisjointSetVec::from(vec![true, false]);
    disjoint_set.join(0, 1);
    disjoint_set.join(1, 0);
    let expected_subsets = [vec![0, 1]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_two_pairs_overlapping() {
    let mut disjoint_set = DisjointSetVec::from(vec!["test"; 4]);
    disjoint_set.join(0, 1);
    disjoint_set.join(1, 3);
    let expected_subsets = [vec![0, 1, 3], vec![2]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_two_pairs_non_overlapping() {
    let mut disjoint_set = DisjointSetVec::from(vec![3; 10]);
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
    let mut disjoint_set = DisjointSetVec::from(vec![3; 5]);
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
    let mut disjoint_set = DisjointSetVec::from(vec![3; 5]);
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
    let mut disjoint_set = DisjointSetVec::from(vec![3; 1000]);
    disjoint_set.join(1000, 5);
}

#[test]
#[should_panic]
fn panic_join_second_element_oob() {
    let mut disjoint_set = DisjointSetVec::from(vec![3; 100]);
    disjoint_set.join(0, 1_000_000_000_000);
}

#[test]
#[should_panic]
fn panic_join_both_elements_oob() {
    let mut disjoint_set = DisjointSetVec::from(vec![3; 0]);
    disjoint_set.join(0, 0);
}

#[test]
#[should_panic]
fn panic_is_joined_first_element_oob() {
    let disjoint_set = DisjointSetVec::from(vec![3; 1000]);
    let _ = disjoint_set.is_joined(1000, 5);
}

#[test]
#[should_panic]
fn panic_is_joined_second_element_oob() {
    let disjoint_set = DisjointSetVec::from(vec![3; 100]);
    let _ = disjoint_set.is_joined(0, 1_000_000_000_000);
}

#[test]
#[should_panic]
fn panic_is_joined_both_elements_oob() {
    let disjoint_set = DisjointSetVec::from(vec![3; 0]);
    let _ = disjoint_set.is_joined(0, 0);
}

#[test]
#[should_panic]
fn clone_clones() {
    let mut disjoint_set = DisjointSetVec::from(vec![3; 5]);
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
    let l = DisjointSetVec::from(vec![3; 5]);
    let r = DisjointSetVec::from(vec![3; 6]);

    assert_ne!(l, r);
}

#[test]
fn same_len_not_equal() {
    let mut l = DisjointSetVec::from(vec![3; 5]);
    let mut r = DisjointSetVec::from(vec![3; 5]);

    l.join(2, 4);
    r.join(2, 3);

    assert_ne!(l, r);
}

#[test]
fn different_joining_order_equal() {
    let mut l = DisjointSetVec::from(vec![3; 5]);
    let mut r = DisjointSetVec::from(vec![3; 5]);

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
fn get_sets_empty() {
    let ds = DisjointSetVec::from(vec![3; 0]);

    let sets = ds.get_index_sets();

    let expected: Vec<Vec<_>> = Vec::new();

    assert_eq!(sets, expected);
}

#[test]
fn get_sets_singletons() {
    let ds = DisjointSetVec::from(vec![3; 10]);

    let sets = ds.get_index_sets();

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
fn get_sets_all_in_one() {
    let mut ds = DisjointSetVec::from(vec![3; 8]);

    ds.join(0, 1);
    ds.join(2, 3);
    ds.join(4, 5);
    ds.join(6, 7);
    ds.join(0, 3);
    ds.join(5, 7);
    ds.join(3, 4);

    let sets = ds.get_index_sets();

    let expected = [vec![0, 1, 2, 3, 4, 5, 6, 7]];

    assert_eq!(sets, expected);
}

#[test]
fn get_sets_complex() {
    let mut ds = DisjointSetVec::from(vec![3; 5]);

    ds.join(0, 3);
    ds.join(2, 1);
    ds.push(100);
    ds.join(2, 5);

    let sets = ds.get_index_sets();

    let expected = [vec![0, 3], vec![1, 2, 5], vec![4]];

    assert_eq!(sets, expected);
}

#[test]
fn construct_with_capacity() {
    let ds: DisjointSetVec<&&Option<&&char>> = DisjointSetVec::with_capacity(30);
    assert!(ds.is_empty());
}

#[test]
fn add_singleton_produces_singleton() {
    let mut ds = DisjointSetVec::from(vec![-1, -2, -3]);
    ds.join(0, 2);
    verify_subsets(&ds, &[vec![0, 2], vec![1]]);
    ds.push(0);
    verify_subsets(&ds, &[vec![0, 2], vec![1], vec![3]]);
    ds.push(0);
    verify_subsets(&ds, &[vec![0, 2], vec![1], vec![3], vec![4]]);
}

#[test]
fn can_join_elements_added_later() {
    let mut ds = DisjointSetVec::from(vec![-1, -2, -3]);

    ds.join(0, 2);

    ds.push(3);
    ds.push(4);
    ds.push(-33);

    ds.join(4, 5);
    ds.join(3, 0);

    let expected = [vec![0, 2, 3], vec![1], vec![4, 5]];
    verify_subsets(&ds, &expected);
}

#[test]
fn get_works_in_all_circumstances() {
    let mut ds = DisjointSetVec::from(vec!["a", "a", "b"]);

    assert_eq!(ds.get(0), Some(&"a"));
    assert_eq!(ds.get(1), Some(&"a"));
    assert_eq!(ds.get(2), Some(&"b"));
    assert_eq!(ds.get(3), None);
    assert_eq!(ds.get(4), None);

    ds.join(0, 2);

    assert_eq!(ds.get(0), Some(&"a"));
    assert_eq!(ds.get(1), Some(&"a"));
    assert_eq!(ds.get(2), Some(&"b"));
    assert_eq!(ds.get(3), None);
    assert_eq!(ds.get(4), None);

    ds[0] = "c";

    assert_eq!(ds.get(0), Some(&"c"));
    assert_eq!(ds.get(1), Some(&"a"));
    assert_eq!(ds.get(2), Some(&"b"));
    assert_eq!(ds.get(3), None);
    assert_eq!(ds.get(4), None);

    ds.push("d");

    assert_eq!(ds.get(0), Some(&"c"));
    assert_eq!(ds.get(1), Some(&"a"));
    assert_eq!(ds.get(2), Some(&"b"));
    assert_eq!(ds.get(3), Some(&"d"));
    assert_eq!(ds.get(4), None);
}

#[test]
fn iter_works() {
    let mut ds = DisjointSetVec::from(vec!["a", "a", "b"]);

    ds.join(0, 2);
    ds.push("d");

    let expected = ["a", "a", "b", "d"];

    assert!(ds.iter().eq(expected.iter()));
}

#[test]
fn iter_mut_works() {
    let mut ds = DisjointSetVec::from(vec![3, 4, 5]);

    ds.join(0, 2);
    ds.push(0);

    let clone = ds.clone();

    for val in ds.iter_mut() {
        *val *= 2;
    }

    let mut expected_ds = DisjointSetVec::from(vec![6, 8, 10, 0]);
    expected_ds.join(0, 2);

    assert_eq!(ds, expected_ds);
    assert_ne!(ds, clone);
}

#[test]
fn indexing_works() {
    let mut ds = DisjointSetVec::from(vec![3, 4, 5]);

    ds.join(0, 2);
    ds.push(0);

    assert_eq!(ds[0], 3);
    assert_eq!(ds[1], 4);
    assert_eq!(ds[2], 5);
    assert_eq!(ds[3], 0);
}

#[test]
#[should_panic]
fn indexing_panics() {
    let ds = DisjointSetVec::from(vec![3, 4, 5]);
    let _ = ds[3];
}

#[test]
fn mutable_indexing_works() {
    let mut ds = DisjointSetVec::from(vec![3, 4, 5]);

    ds.join(0, 2);
    ds.push(0);

    ds[1] = 33;

    assert_eq!(ds[0], 3);
    assert_eq!(ds[1], 33);
    assert_eq!(ds[2], 5);
    assert_eq!(ds[3], 0);
}

#[test]
#[should_panic]
fn mutable_indexing_panics() {
    let mut ds = DisjointSetVec::from(vec![3, 4, 5]);
    ds[3] = 3;
}

#[test]
fn into_iter_works() {
    let expected = vec![3, 4, 5];

    let ds = DisjointSetVec::from(expected.clone());

    let mut vec = Vec::with_capacity(3);

    for val in ds {
        vec.push(val);
    }

    assert_eq!(vec, expected);
}

#[test]
fn ref_into_iter_works() {
    let expected = vec![3, 4, 5];

    let ds = DisjointSetVec::from(expected.clone());

    let mut vec = Vec::with_capacity(3);

    for &val in &ds {
        vec.push(val);
    }

    assert_eq!(vec, expected);
}

#[test]
fn mut_ref_into_iter_works() {
    let mut ds = DisjointSetVec::from(vec![2, 3]);

    for val in &mut ds {
        *val += 10;
    }

    assert_eq!(ds.get(0), Some(&12));
    assert_eq!(ds.get(1), Some(&13));
    assert_eq!(ds.get(2), None);
}

#[test]
fn different_ways_of_empty_construction() {
    let empty_from: DisjointSetVec<bool> = DisjointSetVec::from(vec![]);
    assert!(empty_from.is_empty());

    let empty_new = DisjointSetVec::new();
    assert!(empty_new.is_empty());

    let empty_default = DisjointSetVec::default();
    assert!(empty_new.is_empty());

    assert_eq!(empty_from, empty_new);
    assert_eq!(empty_from, empty_default);
}
