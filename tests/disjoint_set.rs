use disjoint::DisjointSet;

fn verify_subsets(disjoint_set: &DisjointSet, expected_subsets_ordered: &[Vec<usize>]) {
    assert_eq!(
        disjoint_set.len(),
        expected_subsets_ordered
            .iter()
            .map(|subset| subset.len())
            .sum()
    );

    for (subset_id, subset) in expected_subsets_ordered.iter().enumerate() {
        for (other_subset_id, other_subset) in expected_subsets_ordered.iter().enumerate() {
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

    assert_eq!(disjoint_set.get_sets(), expected_subsets_ordered)
}

#[test]
fn constructor_prodcues_singletons() {
    let disjoint_set = DisjointSet::with_len(5);

    let expected_subsets = [vec![0], vec![1], vec![2], vec![3], vec![4]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn constructor_constructs_correct_len_and_if_empty() {
    for size in 0..100 {
        let disjoint_set = DisjointSet::with_len(size);
        assert_eq!(disjoint_set.len(), size);
        assert_eq!(disjoint_set.is_empty(), size == 0);
    }
}

#[test]
fn joining_and_is_joined_dont_change_len() {
    for size in 3..103 {
        let mut disjoint_set = DisjointSet::with_len(size);

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
    let mut disjoint_set = DisjointSet::with_len(3);
    disjoint_set.join(0, 1);
    let expected_subsets = [vec![0, 1], vec![2]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_element_with_itself() {
    let mut disjoint_set = DisjointSet::with_len(3);
    disjoint_set.join(1, 1);
    let expected_subsets = [vec![0], vec![1], vec![2]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_one_pair_twice() {
    let mut disjoint_set = DisjointSet::with_len(4);
    disjoint_set.join(0, 1);
    disjoint_set.join(0, 1);
    let expected_subsets = [vec![0, 1], vec![2], vec![3]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_one_pair_and_reversed() {
    let mut disjoint_set = DisjointSet::with_len(2);
    disjoint_set.join(0, 1);
    disjoint_set.join(1, 0);
    let expected_subsets = [vec![0, 1]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_two_pairs_overlapping() {
    let mut disjoint_set = DisjointSet::with_len(4);
    disjoint_set.join(0, 1);
    disjoint_set.join(1, 3);
    let expected_subsets = [vec![0, 1, 3], vec![2]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_two_pairs_non_overlapping() {
    let mut disjoint_set = DisjointSet::with_len(10);
    disjoint_set.join(0, 4);
    disjoint_set.join(2, 7);
    let expected_subsets = [
        vec![0, 4],
        vec![1],
        vec![2, 7],
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
    let mut disjoint_set = DisjointSet::with_len(5);
    disjoint_set.join(0, 1);
    disjoint_set.join(2, 3);
    let expected_subsets = [vec![0, 1], vec![2, 3], vec![4]];
    verify_subsets(&disjoint_set, &expected_subsets);

    disjoint_set.join(0, 3);
    let expected_subsets = [vec![0, 1, 2, 3], vec![4]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn arbitrary_sequence_of_join_and_is_joined() {
    let mut disjoint_set = DisjointSet::with_len(5);
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
    let mut disjoint_set = DisjointSet::with_len(1000);
    disjoint_set.join(1000, 5);
}

#[test]
#[should_panic]
fn panic_join_second_element_oob() {
    let mut disjoint_set = DisjointSet::with_len(100);
    disjoint_set.join(0, 1000000000000);
}

#[test]
#[should_panic]
fn panic_join_both_elements_oob() {
    let mut disjoint_set = DisjointSet::new();
    disjoint_set.join(0, 0);
}

#[test]
#[should_panic]
fn panic_is_joined_first_element_oob() {
    let disjoint_set = DisjointSet::with_len(1000);
    let _ = disjoint_set.is_joined(1000, 5);
}

#[test]
#[should_panic]
fn panic_is_joined_second_element_oob() {
    let disjoint_set = DisjointSet::with_len(100);
    let _ = disjoint_set.is_joined(0, 1000000000000);
}

#[test]
#[should_panic]
fn panic_is_joined_both_elements_oob() {
    let disjoint_set = DisjointSet::new();
    let _ = disjoint_set.is_joined(0, 0);
}

#[test]
#[should_panic]
fn clone_clones() {
    let mut disjoint_set = DisjointSet::with_len(5);
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
    let l = DisjointSet::with_len(5);
    let r = DisjointSet::with_len(6);

    assert_ne!(l, r);
}

#[test]
fn same_len_not_equal() {
    let mut l = DisjointSet::with_len(5);
    let mut r = DisjointSet::with_len(5);

    l.join(2, 4);
    r.join(2, 3);

    assert_ne!(l, r);
}

#[test]
fn different_joining_order_equal() {
    let mut l = DisjointSet::with_len(5);
    let mut r = DisjointSet::with_len(5);

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
    let ds = DisjointSet::new();

    let sets = ds.get_sets();

    let expected: Vec<Vec<_>> = Vec::new();

    assert_eq!(sets, expected);
}

#[test]
fn get_sets_singletons() {
    let ds = DisjointSet::with_len(10);

    let sets = ds.get_sets();

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
    let mut ds = DisjointSet::with_len(8);

    ds.join(0, 1);
    ds.join(2, 3);
    ds.join(4, 5);
    ds.join(6, 7);
    ds.join(0, 3);
    ds.join(5, 7);
    ds.join(3, 4);

    let sets = ds.get_sets();

    let expected = [vec![0, 1, 2, 3, 4, 5, 6, 7]];

    assert_eq!(sets, expected);
}

#[test]
fn get_sets_complex() {
    let mut ds = DisjointSet::with_len(5);

    ds.join(0, 3);
    ds.join(2, 1);
    ds.add_singleton();
    ds.join(2, 5);

    let sets = ds.get_sets();

    let expected = [vec![0, 3], vec![1, 2, 5], vec![4]];

    assert_eq!(sets, expected);
}

#[test]
fn construct_with_capacity() {
    let ds = DisjointSet::with_capacity(30);
    assert!(ds.is_empty());
}

#[test]
fn add_singleton_produces_singleton() {
    let mut ds = DisjointSet::with_len(3);
    ds.join(0, 2);
    verify_subsets(&ds, &[vec![0, 2], vec![1]]);
    ds.add_singleton();
    verify_subsets(&ds, &[vec![0, 2], vec![1], vec![3]]);
    ds.add_singleton();
    verify_subsets(&ds, &[vec![0, 2], vec![1], vec![3], vec![4]]);
}

#[test]
fn can_join_elements_added_later() {
    let mut ds = DisjointSet::with_len(3);

    ds.join(0, 2);

    ds.add_singleton();
    ds.add_singleton();
    ds.add_singleton();

    ds.join(4, 5);
    ds.join(3, 0);

    let expected = [vec![0, 2, 3], vec![1], vec![4, 5]];
    verify_subsets(&ds, &expected);
}

#[test]
#[should_panic]
fn with_capacity_panics() {
    let _ = DisjointSet::with_capacity(isize::MAX as usize - 1);
}

#[test]
fn different_ways_of_empty_construction() {
    let empty_with_len = DisjointSet::with_len(0);
    assert!(empty_with_len.is_empty());

    let empty_new = DisjointSet::new();
    assert!(empty_new.is_empty());

    let empty_default = DisjointSet::default();
    assert!(empty_new.is_empty());

    assert_eq!(empty_with_len, empty_new);
    assert_eq!(empty_with_len, empty_default);
}
