pub fn main() {
    // Hello!
}

pub fn sum_array(input: &[i32]) -> i32 {
    let mut sum = 0;
    for i in input {
        sum += i;
    }

    sum
}
