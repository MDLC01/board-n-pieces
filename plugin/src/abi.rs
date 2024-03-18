use std::borrow::Cow;
use std::iter;

pub type Result<T> = std::result::Result<T, String>;

pub trait Abi: Sized {
    fn descriptor() -> Cow<'static, str>;

    fn from_bytes(bytes: &mut impl Iterator<Item = u8>) -> Result<Self>;

    fn from_slice(slice: &[u8]) -> Result<Self> {
        let mut iterator = slice.iter().copied();
        let value = Self::from_bytes(&mut iterator)?;
        if iterator.next().is_some() {
            Err(format!(
                "Too many bytes when reading {}",
                Self::descriptor()
            ))?
        }
        Ok(value)
    }

    fn to_bytes(self) -> impl IntoIterator<Item = u8>;

    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().into_iter().collect()
    }
}

pub trait ByteAbi: Sized {
    fn descriptor() -> Cow<'static, str>;

    fn from_byte(byte: u8) -> Result<Self>;

    fn to_byte(self) -> u8;
}

impl<T: ByteAbi> Abi for T {
    fn descriptor() -> Cow<'static, str> {
        T::descriptor()
    }

    fn from_bytes(bytes: &mut impl Iterator<Item = u8>) -> Result<Self> {
        let byte = bytes
            .next()
            .ok_or(format!("Expected byte for {}", Self::descriptor()))?;
        Self::from_byte(byte)
    }

    fn to_bytes(self) -> impl IntoIterator<Item = u8> {
        [self.to_byte()]
    }
}

pub trait Decode<T> {
    fn decode(&self) -> Result<T>;
}

impl<T: Abi> Decode<T> for [u8] {
    fn decode(&self) -> Result<T> {
        T::from_slice(self)
    }
}

impl<T: ByteAbi> Decode<T> for u8 {
    fn decode(&self) -> Result<T> {
        T::from_byte(*self)
    }
}

impl ByteAbi for u8 {
    fn descriptor() -> Cow<'static, str> {
        "u8".into()
    }

    fn from_byte(byte: u8) -> Result<Self> {
        Ok(byte)
    }

    fn to_byte(self) -> u8 {
        self
    }
}

impl Abi for u32 {
    fn descriptor() -> Cow<'static, str> {
        "u32".into()
    }

    fn from_bytes(bytes: &mut impl Iterator<Item = u8>) -> Result<Self> {
        let [b0, b1, b2, b3] = bytes
            .take((Self::BITS / u8::BITS) as usize)
            .collect::<Vec<_>>()[..]
        else {
            Err("Expected 4 bytes for u32")?
        };
        Ok(Self::from_be_bytes([b0, b1, b2, b3]))
    }

    fn to_bytes(self) -> impl IntoIterator<Item = u8> {
        self.to_be_bytes()
    }
}

impl<T: Abi> Abi for Option<T> {
    fn descriptor() -> Cow<'static, str> {
        format!("optional {}", T::descriptor()).into()
    }

    fn from_bytes(bytes: &mut impl Iterator<Item = u8>) -> Result<Self> {
        if bytes.next().ok_or("Expected byte")? == 0 {
            Ok(None)
        } else {
            Ok(Some(T::from_bytes(bytes)?))
        }
    }

    fn to_bytes(self) -> impl IntoIterator<Item = u8> {
        match self {
            None => vec![0],
            Some(v) => iter::once(0).chain(v.to_bytes()).collect(),
        }
    }
}

#[macro_export]
macro_rules! abi {
    (
        $type:ty as $name:literal {
            $( $field:ident ), * $(,)?
        }
    ) => {
        impl $crate::abi::Abi for $type {
            fn descriptor() -> Cow<'static, str> {
                $name.into()
            }

            fn from_bytes(bytes: &mut impl Iterator<Item = u8>) -> abi::Result<Self> {
                Ok(Self {
                    $( $field: <_ as $crate::abi::Abi>::from_bytes(bytes)?, ) +
                })
            }

            fn to_bytes(self) -> impl Iterator<Item = u8> {
                iter::empty()
                    $( .chain($crate::abi::Abi::to_bytes(self.$field)) ) +
            }
        }
    };

    (
        $type:ty {
            $( $field:ident ), * $(,)?
        }
    ) => {
        impl $crate::abi::Abi for $type {
            fn descriptor() -> Cow<'static, str> {
                stringify!($type).into()
            }

            fn from_bytes(bytes: &mut impl Iterator<Item = u8>) -> abi::Result<Self> {
                Ok(Self {
                    $( $field: <_ as $crate::abi::Abi>::from_bytes(bytes)?, ) +
                })
            }

            fn to_bytes(self) -> impl Iterator<Item = u8> {
                iter::empty()
                    $( .chain($crate::abi::Abi::to_bytes(self.$field)) ) +
            }
        }
    }
}
