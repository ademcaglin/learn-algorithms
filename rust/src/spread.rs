pub fn vector_usage() -> usize {
    let mut vec1 = vec![1, 2, 3, 4];
    let vec2 = vec![10, 20, 30, 40];
    vec1.extend(vec2);
    return vec1.len();
}

#[test]
fn test_vector() {
    assert!(vector_usage() == 8);
}
