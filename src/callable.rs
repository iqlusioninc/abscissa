//! Callables are callbacks used commands

/// Something which can be called
pub trait Callable {
    /// Call this callable (i.e. command), running its behavior
    fn call(&self);
}

#[cfg(test)]
mod tests {
    use crate::Callable;
    use std::sync::Mutex;

    #[allow(dead_code)]
    #[derive(Callable)]
    enum TestEnum {
        A(VariantA),
        B(VariantB),
    }

    #[allow(dead_code)]
    struct VariantA {}

    impl Callable for VariantA {
        fn call(&self) {
            panic!("don't call this!")
        }
    }

    #[derive(Default)]
    struct VariantB {
        called: Mutex<bool>,
    }

    impl VariantB {
        fn was_called(&self) -> bool {
            let called = self.called.lock().unwrap();
            *called
        }
    }

    impl Callable for VariantB {
        fn call(&self) {
            let mut called = self.called.lock().unwrap();
            *called = true;
        }
    }

    #[test]
    fn custom_derive_test() {
        let variant_b = VariantB::default();
        assert!(!variant_b.was_called());

        let ex = TestEnum::B(variant_b);
        ex.call();

        let variant_b = match ex {
            TestEnum::A(_) => panic!("this shouldn't be!"),
            TestEnum::B(b) => b,
        };
        assert!(variant_b.was_called());
    }
}
