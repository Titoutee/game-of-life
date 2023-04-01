pub fn str_repeat(base: &str, n: usize) -> String {
    /// Creates a new String containing `base` repeated n times
    let mut string = String::from(base);
    for _ in 0..n{
        string.push_str(base);
    }
    string
}

pub fn from_simple(simple: usize, rows: usize) -> (usize, usize) {
    (simple/rows, simple%rows)
}