use crossbeam::scope;

fn main() {
    let arr = [12, 31, 3, 41, 3410, 231, 314123, 4121];
    let max = find_max(&arr).unwrap();
    println!("{:?}", max);
}

// finds the maximum number in an array
fn find_max(arr: &[i32]) -> Option<i32> {
    let THRESHOLD = 2;

    // *why do this?
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid_array = arr.len() / 2;
    // split the array in the middle and create a tuple of two arrays
    let (left, right) = arr.split_at(mid_array);

    //  find the max of the left and right array
    crossbeam::scope(|s| {
        let left_thread = s.spawn(|_| find_max(left));
        let right_thread = s.spawn(|_| find_max(right));

        let max_left = left_thread.join().unwrap();
        let max_right = right_thread.join().unwrap();
        // *return the max value here
        max_left.max(max_right)
    })
    .unwrap()
}
