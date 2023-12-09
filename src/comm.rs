use chrono::NaiveDateTime;
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::io::{Error, Write};

#[derive(Debug)]
pub(crate) struct Writer {
    buf: Vec<u8>,
    offset: Option<usize>,
    inner: tokio::net::tcp::OwnedWriteHalf,
}

impl Writer {
    #[inline]
    /// Create a new `Message` with the default capacity specified as [`constants::OUT_MESSAGE_SIZE`]
    pub(crate) fn new(writer: tokio::net::tcp::OwnedWriteHalf) -> Self {
        Self::with_capacity(writer, crate::constants::OUT_MESSAGE_SIZE)
    }

    #[inline]
    /// Create a new `Message` with the specified capacity.
    pub(crate) fn with_capacity(writer: tokio::net::tcp::OwnedWriteHalf, cap: usize) -> Self {
        let buf = Vec::with_capacity(cap);

        Self {
            buf,
            offset: None,
            inner: writer,
        }
    }

    #[inline]
    pub(crate) fn add_prefix(&mut self, prefix: &str) -> Result<(), Error> {
        self.buf.write_all(prefix.as_bytes())?;
        self.offset = Some(prefix.len());

        Ok(())
    }

    #[inline]
    pub(crate) fn add_body<T: Serialize>(&mut self, body: T) -> Result<(), Error> {
        const LENGTH_PREFIX: &[u8] = b"\0\0\0\0";
        self.buf.write_all(LENGTH_PREFIX)?;

        body.serialize(&mut *self)?;
        let (len, offset) = match self.offset {
            Some(o) => (self.buf.len() - o - LENGTH_PREFIX.len(), o),
            None => (self.buf.len() - LENGTH_PREFIX.len(), 0),
        };

        self.buf.splice(
            offset..LENGTH_PREFIX.len() + offset,
            u32::try_from(len)
                .expect("Overflow: Message length exceeds the max of 2³² - 1 bytes.")
                .to_be_bytes(),
        );

        Ok(())
    }

    #[inline]
    pub(crate) async fn send(&mut self) -> Result<(), Error> {
        tokio::io::AsyncWriteExt::write_all(&mut self.inner, &self.buf).await?;
        self.buf.clear();
        self.offset = None;

        Ok(())
    }

    #[inline]
    pub(crate) async fn flush(&mut self) -> Result<(), Error> {
        tokio::io::AsyncWriteExt::flush(&mut self.inner).await
    }

    #[inline]
    pub(crate) async fn shutdown(&mut self) -> Result<(), Error> {
        tokio::io::AsyncWriteExt::shutdown(&mut self.inner).await
    }
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct SerializeMessageError(String);

impl Display for SerializeMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to serialize message {}", self.0)
    }
}

impl std::error::Error for SerializeMessageError {}

impl serde::ser::Error for SerializeMessageError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        SerializeMessageError(msg.to_string())
    }
}

impl From<Error> for SerializeMessageError {
    fn from(value: Error) -> Self {
        Self(value.to_string())
    }
}

impl From<SerializeMessageError> for Error {
    fn from(value: SerializeMessageError) -> Self {
        Error::new(std::io::ErrorKind::InvalidData, value.0)
    }
}

// Don't worry about the allow. Our serializer doesn't need all of the fields it's given
#[allow(unused_variables)]
pub(crate) mod ser {
    use serde::{
        ser::{
            SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
            SerializeTupleStruct, SerializeTupleVariant,
        },
        Serialize, Serializer,
    };
    use std::io::Write;

    use super::{SerializeMessageError, Writer};

    #[inline]
    fn serialize_int<I: itoa::Integer>(buf: &mut Vec<u8>, int: I) -> Result<(), std::io::Error> {
        let mut temp = itoa::Buffer::new();
        buf.write_all(temp.format(int).as_bytes())?;
        buf.write_all(b"\0")?;

        Ok(())
    }

    #[inline]
    fn serialize_float<F: ryu::Float>(buf: &mut Vec<u8>, float: F) -> Result<(), std::io::Error> {
        let mut temp = ryu::Buffer::new();
        buf.write_all(temp.format(float).as_bytes())?;
        buf.write_all(b"\0")?;

        Ok(())
    }

    impl Serializer for &mut Writer {
        type Ok = ();
        type Error = SerializeMessageError;
        type SerializeSeq = Self;
        type SerializeTuple = Self;
        type SerializeTupleStruct = Self;
        type SerializeTupleVariant = Self;
        type SerializeMap = Self;
        type SerializeStruct = Self;
        type SerializeStructVariant = Self;

        #[inline]
        fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
            self.buf.write_all(if v { b"1\0" } else { b"0\0" })?;

            Ok(())
        }

        #[inline]
        fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
            serialize_int(&mut self.buf, v)?;
            Ok(())
        }

        #[inline]
        fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
            serialize_int(&mut self.buf, v)?;
            Ok(())
        }

        #[inline]
        fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
            serialize_int(&mut self.buf, v)?;
            Ok(())
        }

        #[inline]
        fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
            serialize_int(&mut self.buf, v)?;
            Ok(())
        }

        #[inline]
        fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
            serialize_int(&mut self.buf, v)?;
            Ok(())
        }

        #[inline]
        fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
            serialize_int(&mut self.buf, v)?;
            Ok(())
        }

        #[inline]
        fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
            serialize_int(&mut self.buf, v)?;
            Ok(())
        }

        #[inline]
        fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
            serialize_int(&mut self.buf, v)?;
            Ok(())
        }

        #[inline]
        fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
            serialize_float(&mut self.buf, v)?;
            Ok(())
        }

        #[inline]
        fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
            serialize_float(&mut self.buf, v)?;
            Ok(())
        }

        #[inline]
        fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
            let mut temp = [0; 5];
            v.encode_utf8(&mut temp);
            self.buf.write_all(&temp[..=v.len_utf8()])?;

            Ok(())
        }

        #[inline]
        fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
            self.serialize_bytes(v.as_bytes())
        }

        #[inline]
        fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
            self.buf.write_all(v)?;
            self.buf.write_all(b"\0")?;

            Ok(())
        }

        #[inline]
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            self.buf.write_all(b"\0")?;

            Ok(())
        }

        #[inline]
        fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            value.serialize(self)
        }

        #[inline]
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        #[inline]
        fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
            name.serialize(self)
        }

        #[inline]
        fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            variant.serialize(self)
        }

        #[inline]
        fn serialize_newtype_struct<T: ?Sized>(
            self,
            name: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            value.serialize(self)
        }

        #[inline]
        fn serialize_newtype_variant<T: ?Sized>(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            value.serialize(self)
        }

        #[inline]
        fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            Ok(self)
        }

        #[inline]
        fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
            Ok(self)
        }

        #[inline]
        fn serialize_tuple_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            Ok(self)
        }

        #[inline]
        fn serialize_tuple_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            Ok(self)
        }

        #[inline]
        fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(self)
        }

        #[inline]
        fn serialize_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            Ok(self)
        }

        #[inline]
        fn serialize_struct_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            Ok(self)
        }
    }

    impl SerializeSeq for &mut Writer {
        type Ok = <Self as Serializer>::Ok;
        type Error = <Self as Serializer>::Error;

        #[inline]
        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            value.serialize(&mut **self)?;
            self.buf.splice(self.buf.len() - 1..self.buf.len(), *b",");
            Ok(())
        }

        #[inline]
        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.buf.splice(self.buf.len() - 1..self.buf.len(), *b"\0");
            Ok(())
        }
    }

    impl SerializeTuple for &mut Writer {
        type Ok = <Self as Serializer>::Ok;
        type Error = <Self as Serializer>::Error;

        #[inline]
        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            value.serialize(&mut **self)
        }

        #[inline]
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    impl SerializeTupleStruct for &mut Writer {
        type Ok = <Self as Serializer>::Ok;
        type Error = <Self as Serializer>::Error;

        #[inline]
        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            value.serialize(&mut **self)
        }

        #[inline]
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    impl SerializeTupleVariant for &mut Writer {
        type Ok = <Self as Serializer>::Ok;
        type Error = <Self as Serializer>::Error;

        #[inline]
        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            value.serialize(&mut **self)
        }

        #[inline]
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    impl SerializeMap for &mut Writer {
        type Ok = <Self as Serializer>::Ok;
        type Error = <Self as Serializer>::Error;

        #[inline]
        fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            key.serialize(&mut **self)
        }

        #[inline]
        fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            value.serialize(&mut **self)
        }

        #[inline]
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    impl SerializeStruct for &mut Writer {
        type Ok = <Self as Serializer>::Ok;
        type Error = <Self as Serializer>::Error;

        #[inline]
        fn serialize_field<T: ?Sized>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            value.serialize(&mut **self)
        }

        #[inline]
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    impl SerializeStructVariant for &mut Writer {
        type Ok = <Self as Serializer>::Ok;
        type Error = <Self as Serializer>::Error;

        #[inline]
        fn serialize_field<T: ?Sized>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            key.serialize(&mut **self)?;
            value.serialize(&mut **self)
        }

        #[inline]
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
}

pub(crate) fn serialize_naive_datetime_yyyymmdd_hhcolon_mm_colon_ss<S: Serializer>(
    dt: &NaiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    dt.format("%Y%m%d %T").to_string().serialize(serializer)
}
