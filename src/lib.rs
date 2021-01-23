use std::collections::HashMap;

pub fn two_number_sum_with_hash_map (addends: &[i32; 6], sum: i32) -> [i32; 2] {
    let mut addends_that_make_sum = [0; 2];
    let mut addends_map: HashMap<i32, bool> = HashMap::new();

    for number in addends {
        let x = sum - number;

        if addends_map.get(&x) == Some(&true) {
            addends_that_make_sum = [x, *number];
            
            break;
        }

        addends_map.insert(*number, true);
    }

    addends_that_make_sum.sort();
    addends_that_make_sum
}

pub fn two_number_sum_with_pointers (addends: &mut [i32; 6], sum: i32) -> [i32; 2] {
    addends.sort();

    let mut left_pointer = 0;
    let mut right_pointer = addends.len() - 1;
    let mut left = addends[left_pointer as usize];
    let mut right = addends[right_pointer as usize];

    loop {
        let sum_to_test = left + right;

        if sum_to_test == sum {
            break;
        }
        
        if sum_to_test > sum {
            right_pointer -= 1;
            right = addends[right_pointer as usize];
        } 
        
        if sum_to_test < sum {
            left_pointer += 1;
            left = addends[left_pointer as usize];
        } 
    }

    [left, right]
}
