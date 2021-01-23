use puzzles;

#[test]
fn test_two_number_sum_with_hash_map () {
    let list_1 = [1, 4, -1, 12, 111, 44];
    let list_2 = [-4, -4, 3, 5, 77, 98];

    assert_eq!(puzzles::two_number_sum_with_hash_map(&list_1, 11), [-1, 12], "Addends do not add to sum {}.", 11);
    assert_eq!(puzzles::two_number_sum_with_hash_map(&list_2, -8), [-4, -4], "Addends do not add to sum {}.", -8)
}
