use crate::consts::BELL_NAMES;
use std::convert::From;
use std::error;
use std::fmt;
use std::ops::{Mul, Not};
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Parity {
    Even = 0,
    Odd = 1,
}

impl Mul for Parity {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, other: Self) -> Self {
        match (self as usize) ^ (other as usize) {
            0 => Parity::Even,
            1 => Parity::Odd,
            _ => panic!("Unknown parity found"),
        }
    }
}

impl Not for Parity {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Parity::Even => Parity::Odd,
            Parity::Odd => Parity::Even,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Stroke {
    Back = 0,
    Hand = 1,
}

impl Stroke {
    pub fn from_index(index: usize) -> Stroke {
        if index % 2 == 0 {
            Stroke::Back
        } else {
            Stroke::Hand
        }
    }
}

impl Not for Stroke {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Stroke::Hand => Stroke::Back,
            Stroke::Back => Stroke::Hand,
        }
    }
}

type MaskType = u64;

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct MaskStruct {
    mask: MaskType,
}

pub type Mask = MaskStruct;

pub trait MaskMethods {
    fn empty() -> Self;
    fn limit() -> Number;

    fn from_bitmask(value: u64) -> Mask;

    fn get(&self, value: Number) -> bool;
    fn del(&mut self, value: Number);
    fn add(&mut self, value: Number);
}

impl fmt::Debug for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(Mask::limit() as usize);

        for i in 0..Mask::limit() {
            s.push(if self.get(i) { '1' } else { '0' });
        }

        write!(f, "{}", s)
    }
}

impl MaskMethods for MaskStruct {
    fn empty() -> MaskStruct {
        MaskStruct {
            mask: 0 as MaskType,
        }
    }

    fn limit() -> Number {
        64
    }

    fn from_bitmask(value: u64) -> Mask {
        Mask { mask: value }
    }

    fn get(&self, value: Number) -> bool {
        self.mask & ((1 as MaskType) << value) != 0
    }

    fn del(&mut self, value: Number) {
        self.mask &= !(1 as MaskType) << value
    }

    fn add(&mut self, value: Number) {
        self.mask |= (1 as MaskType) << value
    }
}

pub type Number = u32;

macro_rules! define_int_synonymn {
    ($type:ident) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
        pub struct $type(Number);

        impl From<Number> for $type {
            fn from(x: Number) -> $type {
                $type(x)
            }
        }

        impl From<i32> for $type {
            fn from(x: i32) -> $type {
                if x < 0 {
                    panic!("Can't convert a negative number");
                }

                $type(x as Number)
            }
        }

        impl From<usize> for $type {
            fn from(x: usize) -> $type {
                $type(x as Number)
            }
        }

        impl $type {
            pub fn as_number(&self) -> Number {
                self.0 as Number
            }

            pub fn as_i32(&self) -> i32 {
                self.0 as i32
            }

            pub fn as_u32(&self) -> u32 {
                self.0 as u32
            }

            pub fn as_isize(&self) -> isize {
                self.as_u32() as isize
            }

            pub fn as_usize(&self) -> usize {
                self.as_u32() as usize
            }

            pub fn as_char(&self) -> char {
                if self.0 >= BELL_NAMES.len() as Number {
                    panic!("Bell name '{}' too big to convert to char", self.0);
                }

                BELL_NAMES.as_bytes()[self.as_usize()] as char
            }
        }
    };
}

define_int_synonymn!(Place);
define_int_synonymn!(Bell);
define_int_synonymn!(Stage);

impl Stage {
    pub const ZERO: Stage = Stage(0);
    pub const ONE: Stage = Stage(1);
    pub const TWO: Stage = Stage(2);

    pub const SINGLES: Stage = Stage(3);
    pub const MINIMUS: Stage = Stage(4);

    pub const DOUBLES: Stage = Stage(5);
    pub const MINOR: Stage = Stage(6);
    pub const TRIPLES: Stage = Stage(7);
    pub const MAJOR: Stage = Stage(8);

    pub const CATERS: Stage = Stage(9);
    pub const ROYAL: Stage = Stage(10);
    pub const CINQUES: Stage = Stage(11);
    pub const MAXIMUS: Stage = Stage(12);

    pub const SEXTUPLES: Stage = Stage(13);
    pub const FOURTEEN: Stage = Stage(14);
    pub const SEPTUPLES: Stage = Stage(15);
    pub const SIXTEEN: Stage = Stage(16);

    pub const OCTUPLES: Stage = Stage(17);
    pub const EIGHTEEN: Stage = Stage(18);
    pub const NONUPLES: Stage = Stage(19);
    pub const TWENTY: Stage = Stage(20);

    pub const DECUPLES: Stage = Stage(21);
    pub const TWENTY_TWO: Stage = Stage(22);
}

static STAGE_NAMES: [&str; 23] = [
    "Zero",
    "One",
    "Two",
    "Singles",
    "Minimus",
    "Doubles",
    "Minor",
    "Triples",
    "Major",
    "Caters",
    "Royal",
    "Cinques",
    "Maximus",
    "Sextuples",
    "Fourteen",
    "Septuples",
    "Sixteen",
    "Octuples",
    "Eighteen",
    "Nonuples",
    "Twenty",
    "Decuples",
    "Twenty-Two",
];

impl Stage {
    pub fn write_to_string(&self, string: &mut String) {
        if self.0 as usize >= STAGE_NAMES.len() {
            string.push_str("<stage ");
            string.push_str(&self.0.to_string());
            string.push('>');
        } else {
            string.push_str(STAGE_NAMES[self.0 as usize]);
        }
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(20);

        self.write_to_string(&mut s);

        write!(f, "{}", s)
    }
}

impl FromStr for Stage {
    type Err = UnknownStageError;

    fn from_str(string: &str) -> Result<Stage, Self::Err> {
        for (i, s) in STAGE_NAMES.iter().enumerate() {
            if *s == string {
                return Ok(Stage::from(i));
            }
        }

        Err(UnknownStageError)
    }
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub struct UnknownStageError;

impl fmt::Display for UnknownStageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown stage name")
    }
}

impl error::Error for UnknownStageError {}

impl From<char> for Bell {
    fn from(c: char) -> Bell {
        match BELL_NAMES.find(c) {
            Some(i) => Bell::from(i),
            None => panic!("Illegal bell name '{}'", c),
        }
    }
}

#[cfg(test)]
mod stage_tests {
    use crate::types::UnknownStageError;
    use crate::Stage;
    use std::str::FromStr;

    #[test]
    fn stage_from_invalid_string() {
        assert_eq!(Stage::from_str("").err(), Some(UnknownStageError));
        assert_eq!(Stage::from_str("Sixteens").err(), Some(UnknownStageError));
        assert_eq!(
            Stage::from_str("ahlagskhdioghapsodihg").err(),
            Some(UnknownStageError)
        );
        assert_eq!(Stage::from_str("\n\n\n\n\n").err(), Some(UnknownStageError));
    }

    #[test]
    fn string_conversions() {
        for i in 0..23 {
            let s = Stage::from(i);

            assert_eq!(Stage::from_str(&s.to_string()).ok(), Some(s));
        }

        assert_eq!(Stage::from(100).to_string(), "<stage 100>");
    }
}

#[cfg(test)]
mod parity_tests {
    use crate::Parity;

    #[test]
    fn not() {
        assert_eq!(!Parity::Even, Parity::Odd);
        assert_eq!(!Parity::Odd, Parity::Even);
    }

    #[test]
    fn multiply() {
        assert_eq!(Parity::Even * Parity::Even, Parity::Even);
        assert_eq!(Parity::Even * Parity::Odd, Parity::Odd);
        assert_eq!(Parity::Odd * Parity::Even, Parity::Odd);
        assert_eq!(Parity::Odd * Parity::Odd, Parity::Even);
    }
}

#[cfg(test)]
mod stroke_tests {
    use crate::Stroke;

    #[test]
    fn not() {
        assert_eq!(!Stroke::Hand, Stroke::Back);
        assert_eq!(!Stroke::Back, Stroke::Hand);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Bell, Place, Stage};

    macro_rules! panic_negative_conversion {
        ($name : ident, $type : ident, $val : expr) => {
            #[test]
            #[should_panic]
            fn $name() {
                $type::from($val);
            }
        };
    }

    panic_negative_conversion!(negative_conversion_bell, Bell, -1);
    panic_negative_conversion!(negative_conversion_stage, Stage, -1);
    panic_negative_conversion!(negative_conversion_place, Place, -1);

    macro_rules! panic_too_large_string_conversion {
        ($name : ident, $type : ident, $val : expr) => {
            #[test]
            #[should_panic]
            fn $name() {
                $type::from($val).as_char();
            }
        };
    }

    panic_too_large_string_conversion!(too_large_conversion_place, Place, 10000);
    panic_too_large_string_conversion!(too_large_conversion_bell, Bell, 10000);
    panic_too_large_string_conversion!(too_large_conversion_stage, Stage, 10000);
}

#[cfg(test)]
mod mask_tests {
    use crate::{Mask, MaskMethods};

    #[test]
    fn empty_limit() {
        let mask = Mask::empty();

        for i in 0..Mask::limit() {
            assert!(!mask.get(i));
        }
    }

    #[test]
    fn get() {
        let mask = Mask::from_bitmask(0b0001_0001_1000u64);

        assert!(!mask.get(0));
        assert!(mask.get(3));
        assert!(mask.get(4));
        assert!(!mask.get(25));
    }

    #[test]
    fn add() {
        let mut mask = Mask::from_bitmask(0b0001_0001_1000u64);

        assert!(!mask.get(0));
        assert!(mask.get(3));
        assert!(mask.get(4));
        assert!(!mask.get(25));

        mask.add(25);
        mask.add(4);

        assert!(mask.get(4));
        assert!(mask.get(25));
        assert!(!mask.get(26));
    }

    #[test]
    fn del() {
        let mut mask = Mask::from_bitmask(0b1001_1000u64);

        assert!(!mask.get(0));
        assert!(mask.get(3));
        assert!(mask.get(4));
        assert!(!mask.get(25));

        mask.del(3);
        mask.del(0);

        assert!(!mask.get(3));
        assert!(!mask.get(0));
    }

    #[test]
    fn debug_print() {
        assert_eq!(
            format!("{:?}", Mask::empty()),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(
            format!("{:?}", Mask::from_bitmask(0b1001_1000u64)),
            "0001100100000000000000000000000000000000000000000000000000000000"
        );
    }
}
