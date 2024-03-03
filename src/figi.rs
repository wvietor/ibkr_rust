use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
enum Consonant {
    B = 11,
    C = 12,
    D = 13,
    F = 15,
    G = 16,
    H = 17,
    J = 19,
    K = 20,
    L = 21,
    M = 22,
    N = 23,
    P = 25,
    Q = 26,
    R = 27,
    S = 28,
    T = 29,
    V = 31,
    W = 32,
    X = 33,
    Y = 34,
    Z = 35,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
enum ConsonantOrNumeric {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    B = 11,
    C = 12,
    D = 13,
    F = 15,
    G = 16,
    H = 17,
    J = 19,
    K = 20,
    L = 21,
    M = 22,
    N = 23,
    P = 25,
    Q = 26,
    R = 27,
    S = 28,
    T = 29,
    V = 31,
    W = 32,
    X = 33,
    Y = 34,
    Z = 35,
}

impl std::convert::TryFrom<char> for Consonant {
    type Error = InvalidConsonant;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            'F' => Self::F,
            'G' => Self::G,
            'H' => Self::H,
            'J' => Self::J,
            'K' => Self::K,
            'L' => Self::L,
            'M' => Self::M,
            'N' => Self::N,
            'P' => Self::P,
            'Q' => Self::Q,
            'R' => Self::R,
            'S' => Self::S,
            'T' => Self::T,
            'V' => Self::V,
            'W' => Self::W,
            'X' => Self::X,
            'Y' => Self::Y,
            'Z' => Self::Z,
            _ => return Err(InvalidConsonant),
        })
    }
}

impl std::convert::TryFrom<char> for ConsonantOrNumeric {
    type Error = InvalidConsonantOrNumeric;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            'F' => Self::F,
            'G' => Self::G,
            'H' => Self::H,
            'J' => Self::J,
            'K' => Self::K,
            'L' => Self::L,
            'M' => Self::M,
            'N' => Self::N,
            'P' => Self::P,
            'Q' => Self::Q,
            'R' => Self::R,
            'S' => Self::S,
            'T' => Self::T,
            'V' => Self::V,
            'W' => Self::W,
            'X' => Self::X,
            'Y' => Self::Y,
            'Z' => Self::Z,
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            _ => return Err(InvalidConsonantOrNumeric),
        })
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct InvalidConsonant;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct InvalidConsonantOrNumeric;

impl std::fmt::Display for InvalidConsonant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid consonant. Must be an uppercase English consonant."
        )
    }
}

impl std::fmt::Display for InvalidConsonantOrNumeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid consonant/number. Must be an uppercase English consonant or a digit 0,1,...,9.")
    }
}

impl std::error::Error for InvalidConsonant {}

impl std::error::Error for InvalidConsonantOrNumeric {}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
struct G;

impl From<G> for u8 {
    fn from(_: G) -> u8 {
        16
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(clippy::struct_field_names)]
/// A valid FIGI code. See the module level documentation for a link to the official standard.
pub struct Figi {
    pos_1: Consonant,
    pos_2: Consonant,
    pos_3: G,
    pos_4_12: [ConsonantOrNumeric; 9],
}

impl Serialize for Figi {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_seq(Some(12))?;
        s.serialize_element(&self.pos_1)?;
        s.serialize_element(&self.pos_2)?;
        s.serialize_element(&self.pos_3)?;
        for ref c in self.pos_4_12 {
            s.serialize_element(c)?;
        }
        s.end()
    }
}

impl std::str::FromStr for Figi {
    type Err = InvalidFigi;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b: [u8; 12] = s.as_bytes().try_into().map_err(|_| InvalidFigi::Length(s.to_owned()))?;
        let s = b.map(|c| c as char);

        Self::from_chars(&s)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(clippy::module_name_repetitions)]
/// Represents all the possible ways a FIGI code could be invalid
pub enum InvalidFigi {
    /// The checksum is invalid
    Checksum(String),
    /// The first two characters are BS, BM, GG, GB, GH, KY, or VG
    FirstTwo(String),
    /// The third character is not G.
    Third(String),
    /// One of the first two characters is not an uppercase English consonant
    Consonant(String),
    /// One of the fourth through eleventh characters is not an uppercase English consonant or digit 0 through 9.
    ConsonantOrNumeric(String),
    /// The provided code is not exactly twelve characters.
    Length(String),
}

impl std::fmt::Display for InvalidFigi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let msg = match self {
            Self::Checksum(s) => format!("Invalid checksum for: {s}"),
            Self::FirstTwo(s) => format!("Invalid first two characters for {s}. First two characters cannot be BS, BM, GG, GB, GH, KY, or VG."),
            Self::Third(s) => format!("Invalid third character for {s}. Third character must be G"),
            Self::Consonant(s) => format!("Invalid consonant found for {s}. {InvalidConsonant}"),
            Self::ConsonantOrNumeric(s) => format!("Invalid consonant or numeric found for {s}. {InvalidConsonantOrNumeric}"),
            Self::Length(s) => format!("Invalid length. A FIGI code is exactly 12 characters long. {s}"),
        };
        write!(f, "Invalid FIGI. {}", &msg)
    }
}

impl std::error::Error for InvalidFigi {}

impl Figi {
    #[inline]
    /// Construct a new [`Figi`] from a sequence of 12 characters.
    ///
    /// # Returns
    /// A new, valid [`Figi`]
    ///
    /// # Errors
    /// Will error if the provided charaters are not a valid FIGI code.
    pub fn from_chars(s: &[char; 12]) -> Result<Self, InvalidFigi> {
        let (pos_1, pos_2) = match (s[0], s[1]) {
            ('B', 'S' | 'M') | ('G', 'G' | 'B' | 'H') | ('K', 'Y') | ('V', 'G') => {
                return Err(InvalidFigi::FirstTwo(s.iter().collect()))
            }
            (c1, c2) => (
                Consonant::try_from(c1).map_err(|_| InvalidFigi::Consonant(s.iter().collect()))?,
                Consonant::try_from(c2).map_err(|_| InvalidFigi::Consonant(s.iter().collect()))?,
            ),
        };
        let pos_3 = if s[2] == 'G' {
            G
        } else {
            return Err(InvalidFigi::Third(s.iter().collect()));
        };
        let pos_4_12 = [
            ConsonantOrNumeric::try_from(s[3])
                .map_err(|_| InvalidFigi::ConsonantOrNumeric(s.iter().collect()))?,
            ConsonantOrNumeric::try_from(s[4])
                .map_err(|_| InvalidFigi::ConsonantOrNumeric(s.iter().collect()))?,
            ConsonantOrNumeric::try_from(s[5])
                .map_err(|_| InvalidFigi::ConsonantOrNumeric(s.iter().collect()))?,
            ConsonantOrNumeric::try_from(s[6])
                .map_err(|_| InvalidFigi::ConsonantOrNumeric(s.iter().collect()))?,
            ConsonantOrNumeric::try_from(s[7])
                .map_err(|_| InvalidFigi::ConsonantOrNumeric(s.iter().collect()))?,
            ConsonantOrNumeric::try_from(s[8])
                .map_err(|_| InvalidFigi::ConsonantOrNumeric(s.iter().collect()))?,
            ConsonantOrNumeric::try_from(s[9])
                .map_err(|_| InvalidFigi::ConsonantOrNumeric(s.iter().collect()))?,
            ConsonantOrNumeric::try_from(s[10])
                .map_err(|_| InvalidFigi::ConsonantOrNumeric(s.iter().collect()))?,
            ConsonantOrNumeric::try_from(s[11])
                .map_err(|_| InvalidFigi::ConsonantOrNumeric(s.iter().collect()))?,
        ];

        let out = Self {
            pos_1,
            pos_2,
            pos_3,
            pos_4_12,
        };
        if out.is_valid() {
            Ok(out)
        } else {
            Err(InvalidFigi::Checksum(s.iter().collect()))
        }
    }

    #[inline]
    fn is_valid(&self) -> bool {
        let mut sum = sum_digits_sub_100(self.pos_1 as u8)
            + sum_digits_sub_100(self.pos_2 as u8 * 2)
            + sum_digits_sub_100(G.into());

        for (i, c) in self.pos_4_12[..self.pos_4_12.len() - 1].iter().enumerate() {
            if i % 2 == 0 {
                sum += sum_digits_sub_100(2 * *c as u8);
            } else {
                sum += sum_digits_sub_100(*c as u8);
            }
        }
        self.pos_4_12[self.pos_4_12.len() - 1] as u8 == 10 - sum % 10
    }
}

#[inline]
const fn sum_digits_sub_100(n: u8) -> u8 {
    let rem = n % 10;
    rem + (n - rem) / 10
}
