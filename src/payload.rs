use crate::payload::CalculationResult::{NotComputed, NotYetComputed};
use std::num::ParseFloatError;
use std::str::FromStr;

/*
macro_rules! make_error {
    ($( #[doc = $name_doc:expr] )? $name: ident: $msg: literal) => {
        $( #[doc = $name_doc] )?
        #[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
        pub struct $name(pub String);

        impl std::error::Error for $name {}

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {}", $msg, self.0)
            }
        }
    };
}
*/

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// The result of an option calculation.
pub enum CalculationResult {
    /// The computed value.
    Computed(f64),
    /// Indicates that the computation has not been computed yet but will be at some point.
    NotYetComputed,
    /// Indicates that the computation will not be computed.
    NotComputed,
}

impl FromStr for CalculationResult {
    type Err = ParseFloatError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "-1" => NotComputed,
            "-2" => NotYetComputed,
            s => Self::Computed(s.parse()?),
        })
    }
}
