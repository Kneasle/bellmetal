use crate::Number;

// Imports used only for the documentation
#[allow(unused_imports)]
use crate::Bell;

/// The maximum stage allowed before the masking code causes undefined behaviour.
pub const MAX_STAGE: usize = 64;

/// A string containing all the [Bell] names in order.
pub static BELL_NAMES: &str = "1234567890ETABCDFGHJKLMNPRSUVWYZ";

/// An array of char ASCII values to their index in [BELL_NAMES].
static BELL_NAME_LOOKUP_TABLE: [i8; 91] = [
    -1, -1, -1, -1, -1, // 0..5
    -1, -1, -1, -1, -1, // 5..10
    -1, -1, -1, -1, -1, // 10..15
    -1, -1, -1, -1, -1, // 15..20
    -1, -1, -1, -1, -1, // 20..25
    -1, -1, -1, -1, -1, // 25..30
    -1, -1, -1, -1, -1, // 30..35
    -1, -1, -1, -1, -1, // 35..40
    -1, -1, -1, -1, -1, // 40..45
    -1, -1, -1, // 45..48
    9,  // 48 = '0'
    0, 1, 2, 3, 4, 5, 6, 7, 8, // 49..58 = '1'..'9'
    -1, -1, // 58..60
    -1, -1, -1, -1, -1, // 60..65
    12, 13, 14, 15, 10, // 65..70 = 'A'-'D'
    16, 17, 18, -1, 19, // 70..75 = 'E'-'J'
    20, 21, 22, 23, -1, // 75..80 = 'K'-'O'
    24, -1, 25, 26, 11, // 80..85 = 'P'-'T'
    27, 28, 29, -1, 30, 31, // 85..91 = 'U'-'Z'
];

/// Given a [char], returns `true` if it is a valid [Bell] name (but without searching through the
/// entirety of [BELL_NAMES] every time).
///
/// # Example
/// ```
/// use bellmetal::is_bell_name;
///
/// assert!(is_bell_name('4'));
/// assert!(is_bell_name('0'));
/// assert!(is_bell_name('E'));
/// assert!(!is_bell_name('I'));
/// assert!(!is_bell_name(' '));
/// ```
pub fn is_bell_name(c: char) -> bool {
    ((c >= '0' && c <= '9') || (c >= 'A' && c <= 'Z'))
        && c != 'I'
        && c != 'O'
        && c != 'Q'
        && c != 'X'
}

/// Converts a [char] into either a valid [Bell] number or `-1`, even if the [char] points to outside
/// the range of [BELL_NAME_LOOKUP_TABLE].
fn get_number(name: char) -> i8 {
    // Return `-1` if outside the range of [BELL_NAME_LOOKUP_TABLE]
    if name as usize >= BELL_NAME_LOOKUP_TABLE.len() {
        return -1;
    }

    // Since the index is guarunteed to be inside (by the first if statement, we can skip the
    // bounds check
    BELL_NAME_LOOKUP_TABLE[name as usize]
}

/// Convert a [char] representing a [Bell] into the [Number] that represents it (where `0`
/// represents the treble).
///
/// # Example
/// ```
/// use bellmetal::name_to_number;
///
/// assert_eq!(name_to_number('1'), 0);
/// assert_eq!(name_to_number('4'), 3);
/// assert_eq!(name_to_number('T'), 11);
/// assert_eq!(name_to_number('0'), 9);
/// ```
pub fn name_to_number(name: char) -> Number {
    let n = get_number(name);

    if n == -1 {
        panic!("Unknown bell name '{}'.", name);
    }

    n as Number
}

#[cfg(test)]
mod tests {
    use crate::consts::{get_number, is_bell_name};
    use crate::{name_to_number, Bell, BELL_NAMES};

    macro_rules! name_to_number_panic_test {
        ($n : ident, $e : expr) => {
            #[test]
            #[should_panic]
            fn $n() {
                name_to_number($e);
            }
        };
    }

    name_to_number_panic_test!(name_to_number_null, '\0');
    name_to_number_panic_test!(name_to_number_newline, '\n');
    name_to_number_panic_test!(name_to_number_space, ' ');
    name_to_number_panic_test!(name_to_number_nonascii, '★');

    #[test]
    fn lookup_table() {
        fn get_from_names(name: char) -> i8 {
            for (i, c) in BELL_NAMES.chars().enumerate() {
                if c == name {
                    return i as i8;
                }
            }

            -1
        }

        for i in 0..127u8 {
            let c = i as char;

            print!("{}", c);

            assert_eq!(get_from_names(c), get_number(c));
        }
    }

    #[test]
    fn bell_name_tests() {
        assert!(is_bell_name('1'));
        assert!(!is_bell_name('X'));
        assert!(!is_bell_name('I'));
        assert!(is_bell_name('F'));
        assert!(!is_bell_name('\n'));
        assert!(!is_bell_name('\0'));
        assert!(!is_bell_name('!'));
    }

    #[test]
    fn char_conversion() {
        for c in BELL_NAMES.chars() {
            assert_eq!(Bell::from(name_to_number(c)).as_char(), c);
        }
    }
}
