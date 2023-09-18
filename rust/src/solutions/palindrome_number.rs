pub fn solve(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut reversed = 0;
    let mut og = x;

    while og > 0 {
        let digit = og % 10;
        reversed = reversed * 10 + digit;
        og /= 10;
    }

    return x == reversed;
}
