use crate::consts;
use crate::types::*;
use crate::{Change, ChangeAccumulator, MaskMethods};
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PlaceNotation {
    pub places: Mask,
    pub stage: Stage,
}

impl PlaceNotation {
    pub fn is_cross(&self) -> bool {
        let mut count = 0;

        for i in 0..self.stage.as_usize() {
            if self.places.get(i as Number) {
                count += 1;
            }
        }

        count == 0
    }

    pub fn iter(&self) -> PlaceNotationIterator {
        PlaceNotationIterator::new(self)
    }

    pub fn places_made<'a>(&'a self) -> impl Iterator<Item = Place> + 'a {
        (0..self.stage.as_usize())
            .filter(move |x| self.places.get(*x as Number))
            .map(Place::from)
    }

    // Returns the place notation that represents 'self' but with the places reversed
    // (for example 14 -> 58 in Major, 1 -> 7 in Triples, etc)
    pub fn reversed(&self) -> PlaceNotation {
        let stage = self.stage.as_usize();

        let mut places = Mask::empty();

        for i in 0..stage {
            if self.places.get(i as Number) {
                places.add((stage - i - 1) as Number);
            }
        }

        PlaceNotation {
            places,
            stage: self.stage,
        }
    }

    pub fn shares_places_with(&self, other: &PlaceNotation) -> bool {
        if other.stage != self.stage {
            panic!("Can't figure out if two place notations of different stages share a place");
        }

        for p in 0..self.stage.as_number() {
            if self.places.get(p) && other.places.get(p) {
                return true;
            }
        }

        false
    }

    pub fn transposition(&self) -> Change {
        Change::from_iterator(self.iter())
    }

    pub fn write_to_string_compact(&self, string: &mut String) {
        let mut count = 0;
        let mut is_1sts_made = false;
        let mut is_nths_made = false;
        let mut internal_place_count = 0;

        let stage = self.stage.as_usize();

        for i in 0..stage {
            // Don't cover implicit places
            if self.places.get(i as Number) {
                if i == 0 {
                    is_1sts_made = true;
                } else if i == stage - 1 {
                    is_nths_made = true;
                } else {
                    internal_place_count += 1;

                    string.push(Bell::from(i).as_char());
                }

                count += 1;
            }
        }

        if count == 0 {
            string.push('x');
        } else {
            if internal_place_count > 0 {
                return;
            }

            if is_1sts_made {
                string.push(Bell::from(0).as_char());
            } else if is_nths_made {
                string.push(Bell::from(stage - 1).as_char());
            }
        }
    }

    pub fn write_to_string_full(&self, string: &mut String) {
        let mut count = 0;

        for i in 0..self.stage.as_usize() {
            if self.places.get(i as Number) {
                string.push(Bell::from(i).as_char());

                count += 1;
            }
        }

        if count == 0 {
            string.push('x');
        }
    }
}

impl PlaceNotation {
    pub fn is_cross_notation(notation: char) -> bool {
        notation == 'X' || notation == 'x' || notation == '-'
    }

    pub fn cross(stage: Stage) -> PlaceNotation {
        if stage.as_u32() & 1u32 != 0 {
            panic!("Non-even stage used with a cross notation");
        }

        PlaceNotation {
            places: Mask::empty(),
            stage,
        }
    }

    pub fn from_str(notation: &str, stage: Stage) -> PlaceNotation {
        let mut places = Mask::empty();

        if notation == "" || notation == "X" || notation == "x" || notation == "-" {
            if stage.as_u32() & 1u32 != 0 {
                panic!("Non-even stage used with a cross notation");
            }

        // Nothing to be done here, since places defaults to 0
        } else {
            // Should decode bell names as places
            for c in notation.chars() {
                places.add(consts::name_to_number(c));
            }

            // Add implicit places (lower place)
            let mut lowest_place = 0 as Number;

            while !places.get(lowest_place) {
                lowest_place += 1;
            }

            if lowest_place & 1 == 1 {
                places.add(0 as Number);
            }

            // Add implicit places (higher place)
            let mut highest_place = stage.as_number();

            while !places.get(highest_place) {
                highest_place -= 1;
            }

            if (stage.as_number() - highest_place) & 1 == 0 {
                places.add(stage.as_number() - 1);
            }
        }

        PlaceNotation { places, stage }
    }

    pub fn notations_to_string_short(place_notations: &[PlaceNotation]) -> String {
        let mut string = String::with_capacity(200);

        PlaceNotation::write_notations_to_string_compact(place_notations, &mut string);

        string
    }

    pub fn write_notations_to_string_compact(
        place_notations: &[PlaceNotation],
        string: &mut String,
    ) {
        let len = place_notations.len();

        let is_symmetrical = |i: usize| -> bool {
            for j in 0..i >> 1 {
                if place_notations[j] != place_notations[i - j - 1] {
                    return false;
                }
            }
            for j in 0..(len - i) >> 1 {
                if place_notations[i + j] != place_notations[len - j - 1] {
                    return false;
                }
            }

            true
        };

        // Decide on the location, if any, of the comma
        let mut comma_index: Option<usize> = None;

        if place_notations.len() % 2 == 0 && place_notations.len() > 2 {
            if is_symmetrical(len - 1) {
                comma_index = Some(len - 1);
            } else {
                for i in (1..len - 1).step_by(2) {
                    if is_symmetrical(i) {
                        comma_index = Some(i);
                        break;
                    }
                }
            }
        }

        // Generate string
        let mut was_last_place_notation_cross = true; // Used to decide whether to insert a dot

        match comma_index {
            Some(x) => {
                // Before comma
                for p in &place_notations[..x / 2 + 1] {
                    if p.is_cross() {
                        string.push('x');

                        was_last_place_notation_cross = true;
                    } else {
                        if !was_last_place_notation_cross {
                            string.push('.');
                        }

                        p.write_to_string_compact(string);

                        was_last_place_notation_cross = false;
                    }
                }

                string.push(',');
                was_last_place_notation_cross = true;

                // After comma
                for p in &place_notations[x..x + (len - x) / 2 + 1] {
                    if p.is_cross() {
                        string.push('x');

                        was_last_place_notation_cross = true;
                    } else {
                        if !was_last_place_notation_cross {
                            string.push('.');
                        }

                        p.write_to_string_compact(string);

                        was_last_place_notation_cross = false;
                    }
                }
            }
            None => {
                for p in place_notations {
                    if p.is_cross() {
                        string.push('x');

                        was_last_place_notation_cross = true;
                    } else {
                        if !was_last_place_notation_cross {
                            string.push('.');
                        }

                        p.write_to_string_compact(string);

                        was_last_place_notation_cross = false;
                    }
                }
            }
        }
    }

    pub fn notations_to_string_full(place_notations: &[PlaceNotation]) -> String {
        let mut string = String::with_capacity(200);

        PlaceNotation::write_notations_to_string_full(place_notations, &mut string);

        string
    }

    pub fn write_notations_to_string_full(place_notations: &[PlaceNotation], string: &mut String) {
        let mut was_last_place_notation_cross = true; // Used to decide whether to insert a dot

        for p in place_notations {
            if p.is_cross() {
                string.push('x');

                was_last_place_notation_cross = true;
            } else {
                if !was_last_place_notation_cross {
                    string.push('.');
                }

                p.write_to_string_full(string);

                was_last_place_notation_cross = false;
            }
        }
    }

    pub fn from_multiple_string(string: &str, stage: Stage) -> Vec<PlaceNotation> {
        let mut string_buff = String::with_capacity(Mask::limit() as usize);
        let mut place_notations: Vec<PlaceNotation> = Vec::with_capacity(string.len());
        let mut comma_index: Option<usize> = None;

        macro_rules! add_place_not {
            () => {
                if string_buff.len() != 0 {
                    place_notations.push(PlaceNotation::from_str(&string_buff, stage));
                    string_buff.clear();
                }
            };
        }

        for c in string.chars() {
            if c == '.' || c == ' ' {
                add_place_not!();
            } else if c == ',' {
                add_place_not!();

                comma_index = Some(place_notations.len());
            } else if PlaceNotation::is_cross_notation(c) {
                if !string_buff.is_empty() {
                    add_place_not!();
                }

                place_notations.push(PlaceNotation::cross(stage));
            } else {
                string_buff.push(c);
            }
        }

        add_place_not!();

        // Deal with strings with comma in them
        if let Some(ind) = comma_index {
            // Disappoiningly, the handwritten implementation is faster than iterator magic,
            // and so despite clippy's continued complaints, I'm keeping it.
            if false {
                // The notations before the comma forwards
                place_notations
                    .iter()
                    .take(ind)
                    // The notations before the comma backwards
                    .chain(place_notations.iter().take(ind).rev().skip(1))
                    // The notations after the comma forwards
                    .chain(place_notations.iter().skip(ind))
                    // The notations after the comma backwards
                    .chain(place_notations.iter().skip(ind).rev().skip(1))
                    // Cloned and put into a vector
                    .cloned()
                    .collect::<Vec<PlaceNotation>>()
            } else {
                let mut reordered_place_notations: Vec<PlaceNotation> =
                    Vec::with_capacity(ind * 2 + (place_notations.len() - ind) * 2 - 2);

                macro_rules! add {
                    ($x : expr) => {
                        reordered_place_notations.push(place_notations[$x].clone());
                    };
                }

                // Before the comma forwards
                for i in 0..ind {
                    add!(i);
                }

                // Before the comma backwards
                for i in 0..ind - 1 {
                    add!(ind - 2 - i);
                }

                // After the comma forwards
                for i in ind..place_notations.len() {
                    add!(i);
                }

                // After the comma backwards
                for i in 0..place_notations.len() - ind - 1 {
                    add!(place_notations.len() - 2 - i);
                }

                reordered_place_notations
            }
        } else {
            place_notations
        }
    }

    pub fn overall_transposition(pns: &[PlaceNotation]) -> Change {
        if pns.is_empty() {
            panic!("Can't find overall transposition of empty PlaceNotation list");
        }

        let mut accum = ChangeAccumulator::new(pns[0].stage);

        for pn in pns {
            accum.accumulate_iterator(pn.iter());
        }

        accum.total().clone()
    }
}

impl fmt::Display for PlaceNotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(Mask::limit() as usize);

        self.write_to_string_full(&mut s);

        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PlaceNotationIterator<'a> {
    place_notation: &'a PlaceNotation,
    index: usize,
    should_hunt_up: bool,
}

impl PlaceNotationIterator<'_> {
    fn new(place_notation: &PlaceNotation) -> PlaceNotationIterator {
        PlaceNotationIterator {
            place_notation,
            index: 0,
            should_hunt_up: false,
        }
    }
}

impl<'a> Iterator for PlaceNotationIterator<'a> {
    type Item = Bell;

    fn next(&mut self) -> Option<Bell> {
        if self.index == self.place_notation.stage.as_usize() {
            return None;
        }

        let output;

        if self.place_notation.places.get(self.index as Number) {
            output = self.index;

            self.should_hunt_up = false;
        } else {
            if self.should_hunt_up {
                output = self.index - 1;
            } else if self.place_notation.places.get((self.index + 1) as Number) {
                output = self.index;
            } else {
                output = self.index + 1;
            }

            self.should_hunt_up = !self.should_hunt_up;
        }

        self.index += 1;

        Some(Bell::from(output))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{Change, ChangeAccumulator, PlaceNotation, Stage, Touch};

    #[test]
    fn is_cross() {
        assert!(PlaceNotation::from_str("x", Stage::MAXIMUS).is_cross());
        assert!(PlaceNotation::from_str("-", Stage::MAJOR).is_cross());
        assert!(PlaceNotation::from_str("X", Stage::MINOR).is_cross());
        assert!(PlaceNotation::from_str("", Stage::ROYAL).is_cross());
        assert!(!PlaceNotation::from_str("1", Stage::TRIPLES).is_cross());
        assert!(!PlaceNotation::from_str("18", Stage::MAJOR).is_cross());
        assert!(!PlaceNotation::from_str("3", Stage::SINGLES).is_cross());
    }

    #[test]
    #[should_panic]
    fn parser_cross_odd_stage_before_comma() {
        PlaceNotation::from_multiple_string("1.7.9x,1", Stage::CINQUES);
    }

    #[test]
    #[should_panic]
    fn parser_cross_odd_stage_internal() {
        PlaceNotation::from_multiple_string("1.7.9x9.1", Stage::CINQUES);
    }

    #[test]
    #[should_panic]
    fn parser_cross_odd_stage_cross() {
        PlaceNotation::from_str("x", Stage::CINQUES);
    }

    #[test]
    #[should_panic]
    fn single_parser_odd_stage_cross() {
        PlaceNotation::cross(Stage::CINQUES);
    }

    #[test]
    #[should_panic]
    fn cross_odd_stage() {
        PlaceNotation::cross(Stage::CINQUES);
    }

    #[test]
    fn multiple_string_conversion_long() {
        for (input, stage, expansion) in &[
            (
                "x4x4x7x7x7.36.7.8x,2",
                Stage::ROYAL,
                "x14x14x70x70x70.36.70.18x18.70.36.70x70x70x14x14x12",
            ), // Hurricane Jack Differential Royal
            ("x1", Stage::MINOR, "x16"), // Original Minor
            (
                "3.4.5.1.5.1.5.1.5.1",
                Stage::DOUBLES,
                "3.145.5.1.5.1.5.1.5.1",
            ), // Gnu Bob Doubles
            (
                "3.1.7.1.5.1.7.1.7.5.1.7.1.7.1.7.1.7.1.5.1.5.1.7.1.7.1.7.1.7",
                Stage::TRIPLES,
                "3.1.7.1.5.1.7.1.7.5.1.7.1.7.1.7.1.7.1.5.1.5.1.7.1.7.1.7.1.7",
            ), // Scientific Triples
            ("x2,1", Stage::MINOR, "x12x16"), // Bastow Minor
            ("1,2x", Stage::MINOR, "16.12x12"), // Unnamed (for good reason) Minor
            (
                "3,1.E.1.E.1.E.1.E.1.E.1",
                Stage::CINQUES,
                "3.1.E.1.E.1.E.1.E.1.E.1.E.1.E.1.E.1.E.1.E.1",
            ), // Grandsire Cinques
        ] {
            assert_eq!(
                PlaceNotation::notations_to_string_full(&PlaceNotation::from_multiple_string(
                    input, *stage
                )),
                *expansion
            );
        }
    }
    #[test]
    fn multiple_string_conversion_short() {
        for (string, length, leadhead) in &[
            ("x4x4x7x7x7.36.7.8x,2", 28, "8756341290"), // Hurricane Jack Differential Royal
            ("x1", 2, "241635"),                        // Original Minor
            ("3.4.5.1.5.1.5.1.5.1", 10, "12435"),       // Gnu Bob Doubles
            (
                "3.1.7.1.5.1.7.1.7.5.1.7.1.7.1.7.1.7.1.5.1.5.1.7.1.7.1.7.1.7",
                30,
                "4623751",
            ), // Scientific Triples
            ("x2,1", 4, "142635"),                      // Bastow Minor
            ("1,2x", 4, "315264"),                      // Bastow Minor
            ("3,1.E.1.E.1.E.1.E.1.E.1", 22, "12537496E80"), // Grandsire Cinques
        ] {
            let lh = Change::from(*leadhead);
            let pns = PlaceNotation::from_multiple_string(string, lh.stage());

            let touch = Touch::from(&pns[..]);

            assert_eq!(PlaceNotation::notations_to_string_short(&pns), *string);

            assert_eq!(touch.length, *length);
            assert_eq!(touch.leftover_change, lh);
        }
    }

    #[test]
    fn single_string_conversions() {
        for (pn, stage, exp) in &[
            ("x", Stage::MAJOR, "x"),
            ("123", Stage::SINGLES, "123"),
            ("149", Stage::CINQUES, "149"),
            ("189", Stage::CATERS, "189"),
            ("45", Stage::MAJOR, "1458"),
            ("2", Stage::TRIPLES, "127"),
            ("", Stage::ROYAL, "x"),
            ("4", Stage::SIXTEEN, "14"),
        ] {
            assert_eq!(format!("{}", PlaceNotation::from_str(pn, *stage)), *exp);
        }
    }

    #[test]
    #[should_panic]
    fn overall_transposition_empty_panic() {
        PlaceNotation::overall_transposition(&[]);
    }

    #[test]
    #[should_panic]
    fn shares_places_with_panic() {
        PlaceNotation::from_str("14", Stage::MAJOR)
            .shares_places_with(&PlaceNotation::from_str("14", Stage::ROYAL));
    }

    #[test]
    fn shares_places_with() {
        for (lhs, rhs, stage, expected_value) in &[
            ("x", "x", Stage::MINIMUS, false),
            ("147", "1", Stage::TRIPLES, true),
            ("1", "7", Stage::TRIPLES, false),
            ("14", "34", Stage::MAJOR, true),
            ("12", "18", Stage::MAJOR, true),
            ("x", "18", Stage::MAJOR, false),
            ("1490", "1270", Stage::ROYAL, true),
        ] {
            assert_eq!(
                PlaceNotation::from_str(lhs, *stage)
                    .shares_places_with(&PlaceNotation::from_str(rhs, *stage)),
                *expected_value
            );
        }
    }

    #[test]
    fn reversal() {
        for (original, reversed, stage) in &[
            ("x", "x", Stage::MINIMUS),
            ("147", "147", Stage::TRIPLES),
            ("1", "7", Stage::TRIPLES),
            ("14", "58", Stage::MAJOR),
            ("1490", "1270", Stage::ROYAL),
        ] {
            assert_eq!(
                PlaceNotation::from_str(original, *stage).reversed(),
                PlaceNotation::from_str(reversed, *stage)
            );
        }
    }

    #[test]
    fn equality() {
        assert!(
            PlaceNotation::from_str("14", Stage::MINIMUS)
                == PlaceNotation::from_str("14", Stage::MINIMUS)
        );

        assert!(
            PlaceNotation::from_str("14", Stage::MINIMUS)
                != PlaceNotation::from_str("14", Stage::DOUBLES)
        );

        assert!(
            PlaceNotation::from_str("14", Stage::MAJOR)
                != PlaceNotation::from_str("1458", Stage::MAJOR)
        );
    }

    #[test]
    fn implicit_places() {
        for (lhs, rhs, stage) in &[
            ("4", "147", Stage::TRIPLES),
            ("47", "147", Stage::CATERS),
            ("45", "1458", Stage::MAJOR),
            ("1", "10", Stage::ROYAL),
        ] {
            assert_eq!(
                PlaceNotation::from_str(lhs, *stage),
                PlaceNotation::from_str(rhs, *stage)
            );
        }
    }

    #[test]
    fn transpositions() {
        for (lhs, rhs) in &[
            ("4", "1324657"),
            ("x", "2143658709"),
            ("x", "21436587"),
            ("135", "12345"),
        ] {
            assert_eq!(
                PlaceNotation::from_str(lhs, Stage::from(rhs.len())).transposition(),
                Change::from(*rhs)
            );
        }
    }

    #[test]
    fn implicit_places_removal() {
        let mut s = String::with_capacity(10);

        for (from, stage, to) in &[
            ("1", Stage::SINGLES, "1"),
            ("3", Stage::SINGLES, "3"),
            ("123", Stage::SINGLES, "2"),
            ("1", Stage::DOUBLES, "1"),
            ("3", Stage::DOUBLES, "3"),
            ("5", Stage::DOUBLES, "5"),
            ("125", Stage::DOUBLES, "2"),
            ("x", Stage::MINOR, "x"),
            ("14", Stage::MINOR, "4"),
            ("16", Stage::MINOR, "1"),
            ("1456", Stage::MINOR, "45"),
            ("14", Stage::SIXTEEN, "4"),
        ] {
            PlaceNotation::from_str(from, *stage).write_to_string_compact(&mut s);

            println!("{}", from);

            assert_eq!(s, *to);

            s.clear();
        }
    }

    #[test]
    fn split_many_and_change_accum() {
        fn test(string: &str, stage: Stage, result: Change) {
            let split_notation = PlaceNotation::from_multiple_string(string, stage);

            // Naive and extremely ineffecient accumulation
            let mut accum: Change = Change::rounds(stage);

            for c in &split_notation {
                accum = accum * c.transposition();
            }

            assert_eq!(accum, result);

            // Much faster accumulation function
            let mut change_accum = ChangeAccumulator::new(stage);

            for c in &split_notation {
                change_accum.accumulate_iterator(c.iter());
            }

            assert_eq!(*change_accum.total(), result);

            // Built-in accum function
            assert_eq!(
                PlaceNotation::overall_transposition(&split_notation),
                result
            );
        }

        test("x16", Stage::MINOR, Change::from("241635")); // Original Minor
        test(
            "3.145.5.1.5.1.5.1.5.1",
            Stage::DOUBLES,
            Change::from("12435"),
        ); // Gnu Bob Doubles
        test(
            "3.1.7.1.5.1.7.1.7.5.1.7.1.7.1.7.1.7.1.5.1.5.1.7.1.7.1.7.1.7",
            Stage::TRIPLES,
            Change::from("4623751"),
        ); // Scientific Triples
        test("x12,16", Stage::MINOR, Change::from("142635")); // Bastow Minor
        test(
            "3,1.E.1.E.1.E.1.E.1.E.1",
            Stage::CINQUES,
            Change::from("12537496E80"),
        ); // Grandsire Cinques
    }
}
