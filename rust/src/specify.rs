pub fn specify(a: i32) -> i32 {
    let x = match a {
        0..=3 => 3,
        0..=10 => 10,
        0..=23 => 11,
        0..=42 => 0,
        _ => 42,
    };
    let y = if a == 5 { 10 } else { 15 };
    return x + y;
}

#[test]
fn test() {
    assert!(specify(5) == 20);
}
