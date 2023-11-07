#[macro_export]
macro_rules! interface {
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
macro_rules! fields {
    [$($(#[$($attrss:tt)*])* $t:path => $f:ident,)*] => {
		$(
			$(#[$($attrss)*])*
			#[derive(Debug)]
			pub struct $f(pub $t);

			impl core::ops::Deref for $f {
				type Target = $t;

				fn deref(&self) -> &Self::Target {
					&self.0
				}
			}

			impl From<$f> for u8 {
				fn from(value: $f) -> Self {
					value.into()
				}
			}

			impl From<$f> for u16 {
				fn from(value: $f) -> Self {
					value.into()
				}
			}

			impl From<$f> for u32 {
				fn from(value: $f) -> Self {
					value.into()
				}
			}

			impl From<u8> for $f {
				fn from(value: u8) -> Self {
					value.into()
				}
			}

			impl From<u16> for $f {
				fn from(value: u16) -> Self {
					value.into()
				}
			}

			impl From<u32> for $f {
				fn from(value: u32) -> Self {
					value.into()
				}
			}
		)*
    };
}

#[macro_export]
macro_rules! enums {
    [$($(#[$($attrss:tt)*])* $t:path => $f:ident { $($v:expr => $k:ident,)* },)*] => {
		$(
			$(#[$($attrss)*])*
			#[derive(Debug)]
			pub enum $f { $($k,)* }

			impl From<$t> for $f {
				fn from(value: $t) -> Self {
					match value {
						$($v => <$f>::$k,)*
						#[allow(unreachable_patterns)]
						_ => panic!(),
					}
				}
			}

			impl From<$f> for $t {
				fn from(value: $f) -> Self {
					match value {
						$(<$f>::$k => $v,)*
					}
				}
			}
		)*
	};
}
