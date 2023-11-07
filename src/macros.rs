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
macro_rules! registers {
    ($(#[$($name_attrss:tt)*])* $base:path => $name:ident {
		$(
			$(#[$($reg_attrss:tt)*])*
			$v:vis $reg:ident: $t:ty,
		)*
	}) => {
		$(#[$($name_attrss)*])*
		pub struct $name(core::marker::PhantomData<()>);

		impl $name {
			pub unsafe fn new() -> Self {
				Self(core::marker::PhantomData)
			}

			pub fn ptr() -> *mut Registers {
				$base as *mut _
			}
		}

		impl core::ops::Deref for $name {
			type Target = Registers;

			fn deref(&self) -> &Self::Target {
				unsafe { &*Self::ptr() }
			}
		}

		impl core::ops::DerefMut for $name {
			fn deref_mut(&mut self) -> &mut Self::Target {
				unsafe { &mut *Self::ptr() }
			}
		}

		/// # Register block
		#[repr(C)]
		pub struct Registers {
			$(
				$(#[$($reg_attrss)*])*
				$v $reg: $t,
			)*
		}
	};
}

#[macro_export]
macro_rules! fields {
    [$($(#[$($attrss:tt)*])* $size:path => $name:ident,)*] => {
		$(
			$(#[$($attrss)*])*
			#[derive(Debug)]
			pub struct $name(pub $size);

			impl core::ops::Deref for $name {
				type Target = $size;

				fn deref(&self) -> &Self::Target {
					&self.0
				}
			}

			impl From<$name> for u8 {
				fn from(value: $name) -> Self {
					value.into()
				}
			}

			impl From<$name> for u16 {
				fn from(value: $name) -> Self {
					value.into()
				}
			}

			impl From<$name> for u32 {
				fn from(value: $name) -> Self {
					value.into()
				}
			}

			impl From<u8> for $name {
				fn from(value: u8) -> Self {
					value.into()
				}
			}

			impl From<u16> for $name {
				fn from(value: u16) -> Self {
					value.into()
				}
			}

			impl From<u32> for $name {
				fn from(value: u32) -> Self {
					value.into()
				}
			}
		)*
    };
}

#[macro_export]
macro_rules! enums {
    [$($(#[$($attrss:tt)*])* $size:path => $name:ident {
		$($value:expr => $key:ident,)*
	},)*] => {
		$(
			$(#[$($attrss)*])*
			#[derive(Debug)]
			pub enum $name { $($key,)* }

			impl From<$size> for $name {
				fn from(value: $size) -> Self {
					match value {
						$($value => <$name>::$key,)*
						#[allow(unreachable_patterns)]
						_ => panic!(),
					}
				}
			}

			impl From<$name> for $size {
				fn from(value: $name) -> Self {
					match value {
						$(<$name>::$key => $value,)*
					}
				}
			}
		)*
	};
}
