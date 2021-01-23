use puzzles;

#[test]
fn test_two_number_sum_with_hash_map () {
    let list_1 = [1, 4, -1, 12, 111, 44];
    let list_2 = [-4, -4, 3, 5, 77, 98];

    assert_eq!(puzzles::two_number_sum_with_hash_map(&list_1, 11), [-1, 12], "Addends do not add to sum {}.", 11);
    assert_eq!(puzzles::two_number_sum_with_hash_map(&list_2, -8), [-4, -4], "Addends do not add to sum {}.", -8)
}

#[test]
fn test_two_number_sum_with_pointers () {
    let mut list_1 = [88, 77, 66, 55, 44, 11];
    let mut list_2 = [-1, -44, -43, -12, 649, 43];

    assert_eq!(puzzles::two_number_sum_with_pointers(&mut list_1, 55), [11, 44], "Addends do not add to sum {}", 55);
    assert_eq!(puzzles::two_number_sum_with_pointers(&mut list_2, 0), [-43, 43], "Addends do not add to sum {}", 55);
}
