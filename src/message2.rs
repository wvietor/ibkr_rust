#![allow(dead_code)]

use bytes::{BufMut, BytesMut};
use serde::Serialize;

const DELIMITER: u8 = b'\0';
type Out = crate::message::Out;
// type In = crate::message::In;


#[derive(Debug, Clone)]
pub(crate) struct Message {
    /// The number of bytes in the message body. Stored as usize; converted to u32 when serializing.
    length: usize,
    /// The message body.
    body: BytesMut,
}

impl Message {
    #[inline]
    pub(crate) fn new<T>(code: Out, t: T) -> Self
    where
        T: Serialize
    {
        let mut body= BytesMut::with_capacity(4);
        body.put_bytes(0, 4);
        let mut msg = Self {
            length: 0,
            body,
        };

        code.serialize(&mut msg).expect("Somehow, you managed to create an invalid message code. Get a grip.");
        t.serialize(&mut msg).expect("You should never see this message."); // Should always be safe, no errors possible in serialization as defined here

        msg
    }

    #[inline]
    pub(crate) fn output_bytes(&mut self) -> &[u8] {
        for (b, l) in self.body[..4 ]
            .iter_mut()
            .zip(u32::try_from(self.length).expect("Somehow, you created a message exceeding 4GB. Get a grip.").to_be_bytes())
        {
            *b = l;
        }
        &self.body
    }
}

#[allow(unused_variables)]
mod ser {
    use super::{Message, DELIMITER};
    use bytes::BufMut;
    use serde::ser::{self, Serialize, Serializer};

    #[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
    pub(crate) struct SerializeMessageError(String);

    impl std::fmt::Display for SerializeMessageError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Failed to serialize message: {}", self.0)
        }
    }

    impl std::error::Error for SerializeMessageError {}

    impl ser::Error for SerializeMessageError {
        fn custom<T>(msg: T) -> Self
        where
            T: std::fmt::Display,
        {
            Self(msg.to_string())
        }
    }

    #[inline]
    fn serialize_int<I: itoa::Integer>(
        msg: &mut Message,
        int: I,
    ) -> Result<<&mut Message as Serializer>::Ok, <&mut Message as Serializer>::Error> {
        let mut buf = itoa::Buffer::new();
        let val = buf.format(int);

        msg.serialize_bytes(val.as_bytes())
    }

    #[inline]
    fn serialize_float<F: ryu::Float>(
        msg: &mut Message,
        float: F,
    ) -> Result<<&mut Message as Serializer>::Ok, <&mut Message as Serializer>::Error> {
        let mut buf = ryu::Buffer::new();
        let val = buf.format(float);

        msg.serialize_bytes(val.as_bytes())
    }

    impl Serializer for &mut Message {
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
            self.body.put_u8(if v { b'1' } else { b'0' });
            self.body.put_u8(DELIMITER);
            self.length += 2;

            Ok(())
        }

        #[inline]
        fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
            serialize_int(self, v)
        }

        #[inline]
        fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
            serialize_int(self, v)
        }

        #[inline]
        fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
            serialize_int(self, v)
        }

        #[inline]
        fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
            serialize_int(self, v)
        }

        #[inline]
        fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
            serialize_int(self, v)
        }

        #[inline]
        fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
            serialize_int(self, v)
        }

        #[inline]
        fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
            serialize_int(self, v)
        }

        #[inline]
        fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
            serialize_int(self, v)
        }

        #[inline]
        fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
            serialize_float(self, v)
        }

        #[inline]
        fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
            serialize_float(self, v)
        }

        #[inline]
        fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
            let mut buf = [0; 4];
            let val = char::encode_utf8(v, &mut buf);

            self.serialize_bytes(val.as_bytes())
        }

        #[inline]
        fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
            self.serialize_bytes(v.as_bytes())
        }

        #[inline]
        fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
            self.body.put(v);
            self.body.put_u8(DELIMITER);
            self.length += 1 + v.len();

            Ok(())
        }

        #[inline]
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            self.serialize_str("")
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
            self.serialize_str(name)
        }

        #[inline]
        fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            self.serialize_str(variant)
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
            self.serialize_seq(Some(len))
        }

        #[inline]
        fn serialize_tuple_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            self.serialize_seq(Some(len))
        }

        #[inline]
        fn serialize_tuple_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            self.serialize_seq(Some(len))
        }

        #[inline]
        fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            self.serialize_seq(len)
        }

        #[inline]
        fn serialize_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            self.serialize_seq(Some(len))
        }

        #[inline]
        fn serialize_struct_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            self.serialize_seq(Some(len))
        }
    }

    impl ser::SerializeSeq for &mut Message
    
    {
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

    impl ser::SerializeTuple for &mut Message
    
    {
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

    impl ser::SerializeTupleStruct for &mut Message
    
    {
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

    impl ser::SerializeTupleVariant for &mut Message
    
    {
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

    impl ser::SerializeMap for &mut Message
    
    {
        type Ok = <Self as Serializer>::Ok;
        type Error = <Self as Serializer>::Error;

        #[inline]
        fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            Ok(())
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

    impl ser::SerializeStruct for &mut Message
    
    {
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

    impl ser::SerializeStructVariant for &mut Message
    
    {
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
}
