pub fn fibonacci_reccursive_simple(n: i32) -> u64 {
    if n <= 0 {
        panic!("{} is not positive!", n);
    }
    match n {
        1 | 2 => 1,
        _ => fibonacci_reccursive(n - 1) + fibonacci_reccursive(n - 2),
    }
}

pub fn fibonacci_recursive(n: u32) -> u32 {
    fn inner(n: u32, penultimate: u32, last: u32) -> u32 {
        match n {
            0 => penultimate,
            1 => last,
            _ => inner(n - 1, last, penultimate + last),
        }
    }
    inner(n, 0, 1)
}

pub fn fibonacci_imperative(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut penultimate;
            let mut last = 1;
            let mut fib = 0;
            for _ in 0..n {
                penultimate = last;
                last = fib;
                fib = penultimate + last;
            }
            fib
        }
    }
}


#[test]
#[should_panic]
fn test_fib_0_panic() {
    fibonacci_reccursive(0);
}

#[test]
#[should_panic]
fn test_fib_negative_panic() {
    fibonacci_reccursive(-5);
}

#[test]
fn test_fib() {
    let found = fibonacci_reccursive(5);
    assert!(found == 5);
    let found2 = fibonacci_reccursive(20);
    assert!(found2 == fibonacci_reccursive(19) + fibonacci_reccursive(18));
}

