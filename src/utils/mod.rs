//! Some useful utils to use


/// Creates a new String containing `base` repeated n times
pub fn str_repeat(base: &str, n: usize) -> String {
    let mut string = String::from(base);
    for _ in 0..n{
        string.push_str(base);
    }
    string
}

/// Converts an index for a flattened `Vec` to a doublet (row, col) in this order
pub fn from_simple(simple: usize, rows: usize) -> (usize, usize) {
    (simple/rows, simple%rows)
}