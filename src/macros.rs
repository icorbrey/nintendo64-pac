#[macro_export]
macro_rules! impl_interface {
    ($interface:ty, $registers:ty, $base:expr) => {
        impl $interface {
            pub fn ptr() -> *const $registers {
                $base as *const _
            }
        }

        unsafe impl Sync for $interface {}

        impl Deref for $interface {
            type Target = $registers;

            fn deref(&self) -> &Self::Target {
                unsafe { &*Self::ptr() }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref {
    ($t:ty, $inner:ty) => {
        impl core::ops::Deref for $t {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_get {
    ($to:ty, $from:ty) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                Self(value)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_set {
    ($from:ident, $to:ident, $low:literal .. $high:literal) => {
        impl $from {
            #[allow(arithmetic_overflow)]
            const MASK: $to = if $high - $low != <$to>::BITS as $to {
                !((1 << ($high - $low)) - 1)
            } else {
                0
            };
        }

        impl TryFrom<$from> for $to {
            type Error = ();

            fn try_from(value: $from) -> Result<Self, Self::Error> {
                if value.0 & <$from>::MASK == 0 {
                    Ok(value.0)
                } else {
                    Err(())
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    pub struct A(pub u8);
    pub struct B(pub u8);
    pub struct C(pub u8);
    pub struct D(pub u8);

    impl_set!(A, u8, 0..3);
    impl_set!(B, u8, 3..5);
    impl_set!(C, u8, 5..8);
    impl_set!(D, u8, 0..8);

    #[test]
    fn test_a() {
        let ok: Result<u8, ()> = A(0b111).try_into();
        let err: Result<u8, ()> = A(0b1111).try_into();

        assert!(ok.is_ok());
        assert!(err.is_err());
    }

    #[test]
    fn test_b() {
        let ok: Result<u8, ()> = B(0b11).try_into();
        let err: Result<u8, ()> = B(0b111).try_into();

        assert!(ok.is_ok());
        assert!(err.is_err());
    }

    #[test]
    fn test_c() {
        let ok: Result<u8, ()> = C(0b111).try_into();
        let err: Result<u8, ()> = C(0b1111).try_into();

        assert!(ok.is_ok());
        assert!(err.is_err());
    }

    #[test]
    fn test_d() {
        let ok: Result<u8, ()> = D(0b11111111).try_into();

        assert!(ok.is_ok());
    }
}
