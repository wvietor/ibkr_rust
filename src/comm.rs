/// This macro has two possible invocations:
///     1) [`make_body!(field1, field2, ...)`]
///         This invocation creates a body by appending all the fields to a string and adding a
///         null character '\0' between fields.
///     2) [`make_body!(field1, field2, ...; suffix)`]
///         This invocation performs the same process as the first invocation with the fields but
///         adds the suffix variable to the end of the message. Note that it does not add a null
///         terminator to the suffix.
#[macro_export]
macro_rules! make_body {
    ( $( $field:expr ),+ ) => {
        {
            let mut body = String::new();
            $(
                body.push_str(&*$field.to_string());
                body.push('\0');
            )+
            body
        }
    };
    ( $( $field:expr ),+; $suffix:expr ) => {
        {
            let mut body = String::new();
            $(
                body.push_str(&*$field.to_string());
                body.push('\0');
            )+
            body.push_str(&*$suffix.to_string());
            body
        }
    }
}

/// Unsafe code due to unchecked read to prefix. The correctness of this code has been verified
/// though.
///
/// This macro has two possible invocations:
///     1) [`make_msg!(field1, field2, ...)`]
///         This invocation creates a message from the given fields by concatenating them into a
///         string with a null character '\0' terminating each field. Additionally, it adds a
///         big-endian encoded length to the front of the message.
///     2) [`make_msg!(prefix; field1, field2, ...)`]
///         This invocation performs the same process as the first invocation with the fields, but
///         it adds the prefix variable to the front of the message. Note that it does not add a
///         null terminator to the prefix.
#[macro_export]
macro_rules! make_msg {
    ( $( $field:expr ),+ ) => {
        {
            let mut msg = String::from("\0\0\0\0");
            $(
                msg.push_str(&*$field.to_string());
                msg.push('\0');
            )+
            let len = (
                u32::try_from(msg.len())
                    .expect("Outgoing message to IBKR is too long (length must be less than 2^32)")
                -
                4_u32
            )
            .to_be_bytes();
            // Safety
            // This will "always" be safe unless something is really broken
            unsafe {
                let prefix = core::str::from_utf8_unchecked(len.as_slice());
                msg.replace_range(..core::mem::size_of::<u32>(), prefix);
            }
            msg
        }
    };
    ( $prefix:literal; $( $field:expr ),+ ) => {
        {
            let mut msg = String::from($prefix);
            msg.push_str("\0\0\0\0");
            let len1 = msg.len();
            $(
                msg.push_str(&*$field.to_string());
                msg.push('\0');
            )+
            let len = (
                u32::try_from(msg.len())
                    .expect("Outgoing message to IBKR is too long (length must be less than 2^32)")
                -
                u32::try_from(len1)
                    .expect("Outgoing message to IBKR is too long (length must be less than 2^32)")
                )
            .to_be_bytes();
            // Safety
            // This will "always" be safe unless something is really broken
            unsafe {
                let prefix = core::str::from_utf8_unchecked(len.as_slice());
                msg.replace_range(len1 - core::mem::size_of::<u32>()..len1, prefix);
            }
            msg
        }
    }
}
