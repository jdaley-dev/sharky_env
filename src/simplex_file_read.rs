use std::io::{BufRead, BufReader, Read, Seek};

pub trait SimplexReadable {
    fn le_read<T>(reader: &mut BufReader<T>) -> Option<Self>
    where
        Self: Sized,
        T: Seek,
        T: Read;

    fn be_read<T>(reader: &mut BufReader<T>) -> Option<Self>
    where
        Self: Sized,
        T: Seek,
        T: Read;
}

macro_rules! impl_simplex_readable {
    ($type:ty, $size:expr) => {
        impl SimplexReadable for $type {
            fn le_read<T>(reader: &mut BufReader<T>) -> Option<Self>
            where
                Self: Sized,
                T: Seek,
                T: Read,
            {
                let mut buf = [0u8; $size];
                reader.read(&mut buf).ok()?;
                Some(<$type>::from_le_bytes(buf))
            }

            fn be_read<T>(reader: &mut BufReader<T>) -> Option<Self>
            where
                Self: Sized,
                T: Seek,
                T: Read,
            {
                let mut buf = [0u8; $size];
                reader.read(&mut buf).ok()?;
                Some(<$type>::from_be_bytes(buf))
            }
        }
    };
}

pub fn simplex_le_read<T: SimplexReadable, U>(buf: &mut BufReader<U>) -> Option<T>
where
    U: Seek,
    U: Read,
{
    T::le_read(buf)
}

pub fn simplex_be_read<T: SimplexReadable, U>(buf: &mut BufReader<U>) -> Option<T>
where
    U: Seek,
    U: Read,
{
    T::be_read(buf)
}

pub fn simplex_string_read<T>(buf: &mut BufReader<T>) -> Option<String>
where
    T: Seek,
    T: Read,
{
    let mut buffer: Vec<u8> = Vec::new();
    buf.read_until(0, &mut buffer).ok()?;
    Some(String::from_utf8(buffer).ok()?)
}

impl_simplex_readable!(u16, 2);
impl_simplex_readable!(i16, 2);
impl_simplex_readable!(u32, 4);
impl_simplex_readable!(i32, 4);
impl_simplex_readable!(u64, 8);
impl_simplex_readable!(i64, 8);
impl_simplex_readable!(u128, 16);
impl_simplex_readable!(i128, 16);
impl_simplex_readable!(f32, 4);
impl_simplex_readable!(f64, 8);
