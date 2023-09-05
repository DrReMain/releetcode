fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut min1 = f64::INFINITY;
    let mut min2 = f64::INFINITY;
    let mut minc = f64::INFINITY;
    let mut m: HashMap<i32, bool> = HashMap::new();

    for item in nums1 {
        min1 = if min1 < (item as f64) {
            min1
        } else {
            item as f64
        };

        m.insert(item, true);
    }

    for item in nums2 {
        min2 = if min2 < (item as f64) {
            min2
        } else {
            item as f64
        };

        if let Some(_value) = m.get(&item) {
            minc = if minc < (item as f64) {
                minc
            } else {
                item as f64
            }
        }
    }

    if minc < f64::INFINITY {
        return minc as i32;
    }

    return if min1 < min2 {
        (min1 * 10.0 + min2) as i32
    } else {
        (min2 * 10.0 + min1) as i32
    };
}
