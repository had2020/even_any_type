pub trait IsEven {
    fn is_even(&self) -> bool;
}

// unsigned ints

impl IsEven for usize {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
impl IsEven for u8 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
impl IsEven for u16 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
impl IsEven for u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
impl IsEven for u64 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
impl IsEven for u128 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

// signed ints

impl IsEven for isize {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
impl IsEven for i8 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
impl IsEven for i16 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
impl IsEven for i64 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
impl IsEven for i128 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

// floats

impl IsEven for f32 {
    fn is_even(&self) -> bool {
        self % 2.0 == 0.0
    }
}
impl IsEven for f64 {
    fn is_even(&self) -> bool {
        self % 2.0 == 0.0
    }
}

#[cfg(test)]
mod tests {
    use crate::IsEven;

    // int tests
    #[test]
    fn test_even_number() {
        assert!(4.is_even());
    }

    #[test]
    fn test_odd_number() {
        assert!(!3.is_even());
    }

    // float tests
    #[test]
    fn test_even_number_float() {
        assert!(4.0.is_even());
    }

    #[test]
    fn test_odd_number_float() {
        assert!(!3.5.is_even());
    }
}
