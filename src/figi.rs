use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeSeq;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
struct G;

impl G {
    #[inline]
    const fn as_u8(&self) -> u8 {
        16
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Figi {
    pos_1: Consonant,
    pos_2: Consonant,
    pos_3: G,
    pos_4_11: [ConsonantOrNumeric; 8],
    check: u8,
}

impl Serialize for Figi {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_seq(Some(12))?;
        s.serialize_element(&self.pos_1)?;
        s.serialize_element(&self.pos_2)?;
        s.serialize_element(&self.pos_3)?;
        for ref c in self.pos_4_11 {
            s.serialize_element(c)?;
        }
        s.serialize_element(&self.check)?;
        s.end()
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum InvalidFigi {
    InvalidChecksum(String),
    InvalidFirstTwo(String),
    InvalidThird(String),
}

impl std::fmt::Display for InvalidFigi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let msg = match self {
            Self::InvalidChecksum(s) => format!("Invalid checksum for: {s}"),
            Self::InvalidFirstTwo(s) => format!("Invalid first two characters for {s}. First two characters cannot be BS, BM, GG, GB, GH, KY, or VG."),
            Self::InvalidThird(s) => format!("Invalid third character for {s}. Third character must be G"),
        };
        write!(f, "Invalid FIGI. {}", &msg)
    }
}

impl std::error::Error for InvalidFigi {}

impl std::str::FromStr for Figi {
    type Err = InvalidFigi;

    fn from_str(s: &str) -> Result<Figi, InvalidFigi> {
        todo!()
    }
}

impl Figi {
    pub fn from_chars(s: &[char; 11]) {
        todo!()
    }

    fn is_valid(&self) -> bool {
        let mut sum = sum_digits_sub_100(self.pos_1 as u8) + 
            sum_digits_sub_100(self.pos_2 as u8 * 2) +
            sum_digits_sub_100(G.as_u8());

        for (i, c) in self.pos_4_11.iter().enumerate() {
            if i % 2 == 0 {
                sum += sum_digits_sub_100(2 * *c as u8);
            } else {
                sum += sum_digits_sub_100(*c as u8);
            }
        }
        self.check == 10 - sum % 10
    }
}

const fn sum_digits_sub_100(n: u8) -> u8 {
    let rem = n % 10;
    rem + (n - rem) / 10
}
