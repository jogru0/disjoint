use disjoint::DisjointSet;

fn verify_subsets(disjoint_set: &DisjointSet, expected_subsets: &[Vec<usize>]) {
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
    let disjoint_set = DisjointSet::new(5);

    let expected_subsets = [vec![0], vec![1], vec![2], vec![3], vec![4]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn constructor_constructs_correct_len_and_if_empty() {
    for size in 0..100 {
        let disjoint_set = DisjointSet::new(size);
        assert_eq!(disjoint_set.len(), size);
        assert_eq!(disjoint_set.is_empty(), size == 0);
    }
}

#[test]
fn joining_and_is_joined_dont_change_len() {
    for size in 3..103 {
        let mut disjoint_set = DisjointSet::new(size);

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
    let mut disjoint_set = DisjointSet::new(3);
    disjoint_set.join(0, 1);
    let expected_subsets = [vec![0, 1], vec![2]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_element_with_itself() {
    let mut disjoint_set = DisjointSet::new(3);
    disjoint_set.join(1, 1);
    let expected_subsets = [vec![0], vec![1], vec![2]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_one_pair_twice() {
    let mut disjoint_set = DisjointSet::new(4);
    disjoint_set.join(0, 1);
    disjoint_set.join(0, 1);
    let expected_subsets = [vec![0, 1], vec![2], vec![3]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_one_pair_and_reversed() {
    let mut disjoint_set = DisjointSet::new(2);
    disjoint_set.join(0, 1);
    disjoint_set.join(1, 0);
    let expected_subsets = [vec![0, 1]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_two_pairs_overlapping() {
    let mut disjoint_set = DisjointSet::new(4);
    disjoint_set.join(0, 1);
    disjoint_set.join(1, 3);
    let expected_subsets = [vec![0, 1, 3], vec![2]];
    verify_subsets(&disjoint_set, &expected_subsets);
}

#[test]
fn join_two_pairs_non_overlapping() {
    let mut disjoint_set = DisjointSet::new(10);
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
    let mut disjoint_set = DisjointSet::new(5);
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
    let mut disjoint_set = DisjointSet::new(5);
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
    let mut disjoint_set = DisjointSet::new(1000);
    disjoint_set.join(1000, 5);
}

#[test]
#[should_panic]
fn panic_join_second_element_oob() {
    let mut disjoint_set = DisjointSet::new(100);
    disjoint_set.join(0, 1000000000000);
}

#[test]
#[should_panic]
fn panic_join_both_elements_oob() {
    let mut disjoint_set = DisjointSet::new(0);
    disjoint_set.join(0, 0);
}

#[test]
#[should_panic]
fn panic_is_joined_first_element_oob() {
    let disjoint_set = DisjointSet::new(1000);
    let _ = disjoint_set.is_joined(1000, 5);
}

#[test]
#[should_panic]
fn panic_is_joined_second_element_oob() {
    let disjoint_set = DisjointSet::new(100);
    let _ = disjoint_set.is_joined(0, 1000000000000);
}

#[test]
#[should_panic]
fn panic_is_joined_both_elements_oob() {
    let disjoint_set = DisjointSet::new(0);
    let _ = disjoint_set.is_joined(0, 0);
}
