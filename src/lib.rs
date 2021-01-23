use std::collections::HashMap;

pub fn two_number_sum_with_hash_map (addends: &[i32; 6], sum: i32) -> [i32; 2] {
    let mut addends_that_make_sum: [i32; 2] = [0; 2];
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
