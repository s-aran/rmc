use crate::{
    errors::Pass2Error,
    models::{
        DivisorClock, NegativePositive, NegativePositiveEqual, NoteCommand, NoteOctaveCommand,
    },
    part_command::{PartCommand, PartCommandStruct, PartTokenStack, count_dots, make_some_length},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Note {
    pub command: String,
    pub natural: bool,
    pub semitone: Option<NegativePositive>,
    pub length: Option<DivisorClock<u8>>,
    pub dots: u8,
}

impl TryFrom<PartTokenStack> for Note {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);
        let natural =
            if let Some(v) = try_from_get_some_value!(value.pop_and_cast::<String>(1), natural) {
                v == "="
            } else {
                false
            };

        let semitone =
            try_from_get_some_value!(value.pop_and_cast::<NegativePositive>(2), semitone);
        let length = make_some_length(value.pop_by_state_all(3));
        let dots = count_dots(try_from_get_some_value!(value.pop_and_cast(4), dots));

        Ok(Note {
            command,
            natural,
            semitone,
            length,
            dots,
        })
    }
}

impl PartCommandStruct for Note {
    fn to_variant(self) -> PartCommand {
        PartCommand::Note(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoteX {
    pub command: String,
    pub length: Option<DivisorClock<u8>>,
    pub dots: u8,
}

impl PartCommandStruct for NoteX {
    fn to_variant(self) -> PartCommand {
        PartCommand::NoteX(self)
    }
}

impl TryFrom<PartTokenStack> for NoteX {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);
        let length = make_some_length(value.pop_by_state_all(1));
        let dots = count_dots(try_from_get_some_value!(value.pop_and_cast(2), dots));

        Ok(NoteX {
            command,
            length,
            dots,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoteR {
    pub command: String,
    pub length: Option<DivisorClock<u8>>,
    pub dots: u8,
}

impl PartCommandStruct for NoteR {
    fn to_variant(self) -> PartCommand {
        PartCommand::NoteR(self)
    }
}

impl TryFrom<PartTokenStack> for NoteR {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);
        let length = make_some_length(value.pop_by_state_all(1));
        let dots = count_dots(try_from_get_some_value!(value.pop_and_cast(2), dots));

        Ok(NoteR {
            command,
            length,
            dots,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PortamentoBegin {
    pub command: String,
    pub pitch1: Option<NoteOctaveCommand>,
    pub pitch2: Option<NoteOctaveCommand>,
    pub length1: Option<u8>,
    pub dots: u8,
    pub length2: Option<u8>,
}

impl PartCommandStruct for PortamentoBegin {
    fn to_variant(self) -> PartCommand {
        PartCommand::PortamentoBegin(self)
    }
}

impl TryFrom<PartTokenStack> for PortamentoBegin {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast::<String>(0), command);
        let pitch1 = try_from_get_some_value!(value.pop_and_cast::<NoteOctaveCommand>(1), pitch1);
        let pitch2 = try_from_get_some_value!(value.pop_and_cast::<NoteOctaveCommand>(2), pitch2);
        let length1 = try_from_get_some_value!(value.pop_and_cast::<u8>(3), length1);
        let dots = count_dots(try_from_get_some_value!(
            value.pop_and_cast::<String>(4),
            dots
        ));
        let length2 = try_from_get_some_value!(value.pop_and_cast::<u8>(5), length2);

        Ok(PortamentoBegin {
            command,
            pitch1,
            pitch2,
            length1,
            dots,
            length2,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PortamentoEnd {
    pub command: String,
}

impl PartCommandStruct for PortamentoEnd {
    fn to_variant(self) -> PartCommand {
        PartCommand::PortamentoEnd(self)
    }
}

impl TryFrom<PartTokenStack> for PortamentoEnd {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast::<String>(0), command);
        Ok(PortamentoEnd { command })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Octave {
    pub command: String,
    pub value: u8,
}

impl PartCommandStruct for Octave {
    fn to_variant(self) -> PartCommand {
        match self.command.as_str() {
            "o" => PartCommand::Octave(self),
            "o+" => PartCommand::PartOctaveChangePositive(self),
            "o-" => PartCommand::PartOctaveChangeNegative(self),
            _ => panic!("PartOctaveChange: unexpected command {}", self.command),
        }
    }
}

impl TryFrom<PartTokenStack> for Octave {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast::<String>(1), value);
        let value = try_from_get_value!(value.pop_and_cast::<u8>(2), value);

        Ok(Octave { command, value })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OctaveUp {
    pub command: String,
}

impl PartCommandStruct for OctaveUp {
    fn to_variant(self) -> PartCommand {
        PartCommand::OctaveUp(self)
    }
}

impl TryFrom<PartTokenStack> for OctaveUp {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast::<String>(0), value);

        Ok(OctaveUp { command })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OctaveDown {
    pub command: String,
}

impl PartCommandStruct for OctaveDown {
    fn to_variant(self) -> PartCommand {
        PartCommand::OctaveDown(self)
    }
}

impl TryFrom<PartTokenStack> for OctaveDown {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast::<String>(0), value);

        Ok(OctaveDown { command })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OctaveReverse {
    pub command: String,
}

impl PartCommandStruct for OctaveReverse {
    fn to_variant(self) -> PartCommand {
        PartCommand::OctaveReverse(self)
    }
}

impl TryFrom<PartTokenStack> for OctaveReverse {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast::<String>(0), value);

        Ok(OctaveReverse { command })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefaultLength {
    pub command: String,
    pub value_type: Option<DivisorClock<u8>>,
    pub value: u8,
    pub dots: Option<String>,
}

impl PartCommandStruct for DefaultLength {
    fn to_variant(self) -> PartCommand {
        PartCommand::DefaultLength(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProcessLastLengthUpdate {
    pub command: String,
    pub value_type: Option<DivisorClock<u8>>,
    pub value: Option<u8>,
    pub dots: Option<String>,
}

impl PartCommandStruct for ProcessLastLengthUpdate {
    fn to_variant(self) -> PartCommand {
        PartCommand::ProcessLastLengthUpdate(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProcessLastLengthAdd {
    pub command: String,
    pub value_type: Option<DivisorClock<u8>>,
    pub value: u8,
    pub dots: Option<String>,
}

impl PartCommandStruct for ProcessLastLengthAdd {
    fn to_variant(self) -> PartCommand {
        PartCommand::ProcessLastLengthAdd(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProcessLastLengthSubtract {
    pub command: String,
    pub value_type: Option<DivisorClock<u8>>,
    pub value: u8,
    pub dots: Option<String>,
}

impl PartCommandStruct for ProcessLastLengthSubtract {
    fn to_variant(self) -> PartCommand {
        PartCommand::ProcessLastLengthSubtract(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProcessLastLengthMultiply {
    pub command: String,
    pub value: u8,
}

impl PartCommandStruct for ProcessLastLengthMultiply {
    fn to_variant(self) -> PartCommand {
        PartCommand::ProcessLastLengthMultiply(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tie {
    pub command: String,
    pub length: Option<u8>,
    pub dots: Option<String>,
}

impl PartCommandStruct for Tie {
    fn to_variant(self) -> PartCommand {
        PartCommand::Tie(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Slur {
    pub command: String,
    pub length: Option<u8>,
    pub dots: Option<String>,
}

impl PartCommandStruct for Slur {
    fn to_variant(self) -> PartCommand {
        PartCommand::Slur(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quantize1 {
    pub command: String,
    pub divisor: Option<DivisorClock<u8>>,
    pub value: u8,
}

impl PartCommandStruct for Quantize1 {
    fn to_variant(self) -> PartCommand {
        PartCommand::Quantize1(self)
    }
}

impl TryFrom<PartTokenStack> for Quantize1 {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(1), command);
        // let divisor = try_from_get_some_value!(value.pop_and_cast::<DivisorClock<u8>>(2), divisor);
        let divisor = None;
        let value = try_from_get_value!(value.pop_and_cast(3), value);

        Ok(Self {
            command,
            divisor,
            value,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quantize2 {
    pub command: String,
    pub value1: Option<u8>,
    pub value1_dots: u8,
    pub value2: Option<u8>,
    pub value3: Option<u8>,
    pub value3_dots: u8,
}

impl PartCommandStruct for Quantize2 {
    fn to_variant(self) -> PartCommand {
        PartCommand::Quantize2(self)
    }
}

impl Quantize2 {
    fn command_type_1(mut value: PartTokenStack) -> Result<Self, Pass2Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);
        let value1 = try_from_get_some_value!(value.pop_and_cast(1), value1);
        let value1_dots = 0;

        let has_range =
            if let Some(v) = try_from_get_some_value!(value.pop_and_cast::<String>(2), range) {
                Some(v == "-")
            } else {
                None
            };

        let value2 = try_from_get_some_value!(value.pop_and_cast(3), value2);
        if value2.is_some()
            && let Some(v) = has_range
        {
            if !v {
                panic!("Quantize2 (format 1): unexpected range");
            }
        }

        let value3 = try_from_get_some_value!(value.pop_and_cast(4), value3);
        let value3_dots = 0;

        Ok(Self {
            command,
            value1,
            value1_dots,
            value2,
            value3,
            value3_dots,
        })
    }

    fn command_type_2(mut value: PartTokenStack) -> Result<Self, Pass2Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);

        let has_value1_l =
            if let Some(v) = try_from_get_some_value!(value.pop_and_cast::<String>(1), value1_l) {
                Some(v != "l")
            } else {
                None
            };

        let value1 = try_from_get_some_value!(value.pop_and_cast(2), value1);
        if value1.is_some()
            && let Some(v) = has_value1_l
            && !v
        {
            panic!("Quantize2 (format 2): l is not specified in value1");
        }
        let value1_dots = count_dots(try_from_get_some_value!(value.pop_and_cast(3), value1_dots));

        let has_range =
            if let Some(v) = try_from_get_some_value!(value.pop_and_cast::<String>(4), range) {
                Some(v == "-")
            } else {
                None
            };

        let has_value2_l =
            if let Some(v) = try_from_get_some_value!(value.pop_and_cast::<String>(5), value1_l) {
                Some(v != "l")
            } else {
                None
            };

        let value2 = try_from_get_some_value!(value.pop_and_cast(6), value2);
        if value2.is_some() {
            if let Some(v) = has_range
                && !v
            {
                panic!("Quantize2 (format 2): unexpected range");
            }

            if let Some(v) = has_value2_l
                && !v
            {
                panic!("Quantize2 (format 2): l is not specified in value2");
            }
        }

        let has_value3_l =
            if let Some(v) = try_from_get_some_value!(value.pop_and_cast::<String>(7), value1_l) {
                Some(v != "l")
            } else {
                None
            };

        let value3 = try_from_get_some_value!(value.pop_and_cast(8), value3);
        if value1.is_some()
            && let Some(v) = has_value3_l
            && !v
        {
            panic!("Quantize2 (format 2): l is not specified in value3");
        }
        let value3_dots = count_dots(try_from_get_some_value!(
            value.pop_and_cast(10),
            value3_dots
        ));

        Ok(Self {
            command,
            value1,
            value1_dots,
            value2,
            value3,
            value3_dots,
        })
    }
}

impl TryFrom<PartTokenStack> for Quantize2 {
    type Error = Pass2Error;

    fn try_from(value: PartTokenStack) -> Result<Self, Self::Error> {
        if let Ok(Some(v)) = value.get_and_cast::<String>(1) {
            if v == "l" {
                Self::command_type_2(value)
            } else {
                Self::command_type_1(value)
            }
        } else {
            Self::command_type_1(value)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TemporaryTranspose {
    pub command: String,
    pub semitone: Option<NegativePositive>,
    pub value: u8,
}

impl PartCommandStruct for TemporaryTranspose {
    fn to_variant(self) -> PartCommand {
        match self.command.as_str() {
            "_" => PartCommand::AbsoluteTranspose(self),
            "__" => PartCommand::RelativeTranspose(self),
            _ => panic!("ConvertPartCommand: unexpected command {}", self.command),
        }
    }
}

impl TryFrom<PartTokenStack> for TemporaryTranspose {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(1), command);
        let semitone =
            try_from_get_some_value!(value.pop_and_cast::<NegativePositive>(2), semitone);
        let value = try_from_get_value!(value.pop_and_cast(3), value);

        Ok(Self {
            command,
            semitone,
            value,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PartTransposeBegin {
    pub command: String,
    pub sign: Option<NegativePositiveEqual>,
    pub notes: Vec<NoteCommand>,
}

impl PartCommandStruct for PartTransposeBegin {
    fn to_variant(self) -> PartCommand {
        PartCommand::PartTransposeBegin(self)
    }
}

impl TryFrom<PartTokenStack> for PartTransposeBegin {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(1), command);
        let sign = try_from_get_some_value!(value.pop_and_cast::<NegativePositiveEqual>(2), sign);
        let notes = match value.pop_and_cast_vec::<NoteCommand>(3) {
            Ok(v) => {
                if v.len() > 0 {
                    v
                } else {
                    panic!("TryFrom for PartTranspose (notes) is empty");
                }
            }
            Err(e) => panic!("TryFrom for PartTranspose (notes): {}", e),
        };

        Ok(Self {
            command,
            sign,
            notes,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PartTransposeEnd {
    pub command: String,
}

impl PartCommandStruct for PartTransposeEnd {
    fn to_variant(self) -> PartCommand {
        PartCommand::PartTransposeEnd(self)
    }
}

impl TryFrom<PartTokenStack> for PartTransposeEnd {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);

        Ok(Self { command })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MasterTranspose {
    pub command: String,
    pub sign: Option<NegativePositive>,
    pub value: u8,
}

impl PartCommandStruct for MasterTranspose {
    fn to_variant(self) -> PartCommand {
        PartCommand::MasterTranspose(self)
    }
}

impl TryFrom<PartTokenStack> for MasterTranspose {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(1), command);
        let sign = try_from_get_some_value!(value.pop_and_cast::<NegativePositive>(2), sign);
        let value = try_from_get_value!(value.pop_and_cast::<u8>(3), value);

        Ok(Self {
            command,
            sign,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        meta_models::{TokenStackTrait, TokenTrait},
        models::NegativePositive,
        part_command::{PartToken, PartTokenStack, State},
    };

    use super::*;

    impl PartTokenStack {
        fn ez_push(&mut self, state: State, token: impl Into<String>) {
            let v = token.into();
            let mut t = PartToken::default();
            *t.begin_mut() = if let Some(e) = self.stack().last() {
                e.end() + 1
            } else {
                0
            };
            *t.end_mut() = t.begin() + v.len();

            t.set_state(state);
            *t.chars_mut() = v;

            println!("{:?}", t);

            self.push(&t);
        }
    }

    #[test]
    fn test_note_command_1() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "c");

        assert_eq!(1, tokens.len());

        let expected = Note {
            command: "c".to_string(),
            natural: false,
            semitone: None,
            length: None,
            dots: 0,
        };

        let actual = Note::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_command_2() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "b");

        assert_eq!(1, tokens.len());

        let expected = Note {
            command: "b".to_string(),
            natural: false,
            semitone: None,
            length: None,
            dots: 0,
        };

        let actual = Note::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_natural_1() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "d");
        tokens.ez_push(1, "=");

        assert_eq!(2, tokens.len());

        let expected = Note {
            command: "d".to_string(),
            natural: true,
            semitone: None,
            length: None,
            dots: 0,
        };

        let actual = Note::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_semitone_1() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "e");
        tokens.ez_push(2, "+");

        assert_eq!(2, tokens.len());

        let expected = Note {
            command: "e".to_string(),
            natural: false,
            semitone: Some(NegativePositive::Positive),
            length: None,
            dots: 0,
        };

        let actual = Note::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_semitone_2() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "e");
        tokens.ez_push(2, "-");

        assert_eq!(2, tokens.len());

        let expected = Note {
            command: "e".to_string(),
            natural: false,
            semitone: Some(NegativePositive::Negative),
            length: None,
            dots: 0,
        };

        let actual = Note::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_length_1() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "f");
        tokens.ez_push(3, "4");

        assert_eq!(2, tokens.len());

        let expected = Note {
            command: "f".to_string(),
            natural: false,
            semitone: None,
            length: Some(DivisorClock::Divisor(4)),
            dots: 0,
        };

        let actual = Note::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_length_2() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "f");
        tokens.ez_push(3, "12");

        assert_eq!(2, tokens.len());

        let expected = Note {
            command: "f".to_string(),
            natural: false,
            semitone: None,
            length: Some(DivisorClock::Divisor(12)),
            dots: 0,
        };

        let actual = Note::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_length_3() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "f");
        tokens.ez_push(3, "%");
        tokens.ez_push(3, "4");

        assert_eq!(3, tokens.len());

        let expected = Note {
            command: "f".to_string(),
            natural: false,
            semitone: None,
            length: Some(DivisorClock::Clock(4)),
            dots: 0,
        };

        let actual = Note::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_length_4() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "f");
        tokens.ez_push(3, "%");
        tokens.ez_push(3, "12");

        assert_eq!(3, tokens.len());

        let expected = Note {
            command: "f".to_string(),
            natural: false,
            semitone: None,
            length: Some(DivisorClock::Clock(12)),
            dots: 0,
        };

        let actual = Note::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_dots_1() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "g");
        tokens.ez_push(4, ".");

        assert_eq!(2, tokens.len());

        let expected = Note {
            command: "g".to_string(),
            natural: false,
            semitone: None,
            length: None,
            dots: 1,
        };

        let actual = Note::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_dots_2() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "g");
        tokens.ez_push(4, "....");

        assert_eq!(2, tokens.len());

        let expected = Note {
            command: "g".to_string(),
            natural: false,
            semitone: None,
            length: None,
            dots: 4,
        };

        let actual = Note::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_all_1() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "a");
        tokens.ez_push(2, "-");
        tokens.ez_push(3, "%");
        tokens.ez_push(3, "20");
        tokens.ez_push(4, "...");

        assert_eq!(5, tokens.len());

        let expected = Note {
            command: "a".to_string(),
            natural: false,
            semitone: Some(NegativePositive::Negative),
            length: Some(DivisorClock::Clock(20)),
            dots: 3,
        };

        let actual = Note::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_x_length_1() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "x");
        tokens.ez_push(1, "4");

        assert_eq!(2, tokens.len());

        let expected = NoteX {
            command: "x".to_string(),
            length: Some(DivisorClock::Divisor(4)),
            dots: 0,
        };

        let actual = NoteX::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_x_length_2() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "x");
        tokens.ez_push(1, "12");

        assert_eq!(2, tokens.len());

        let expected = NoteX {
            command: "x".to_string(),
            length: Some(DivisorClock::Divisor(12)),
            dots: 0,
        };

        let actual = NoteX::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_x_length_3() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "x");
        tokens.ez_push(1, "%");
        tokens.ez_push(1, "4");

        assert_eq!(3, tokens.len());

        let expected = NoteX {
            command: "x".to_string(),
            length: Some(DivisorClock::Clock(4)),
            dots: 0,
        };

        let actual = NoteX::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_x_length_4() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "x");
        tokens.ez_push(1, "%");
        tokens.ez_push(1, "12");

        assert_eq!(3, tokens.len());

        let expected = NoteX {
            command: "x".to_string(),
            length: Some(DivisorClock::Clock(12)),
            dots: 0,
        };

        let actual = NoteX::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_x_dots_1() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "x");
        tokens.ez_push(2, "..");

        assert_eq!(2, tokens.len());

        let expected = NoteX {
            command: "x".to_string(),
            length: None,
            dots: 2,
        };

        let actual = NoteX::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_x_all_1() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "x");
        tokens.ez_push(1, "12");
        tokens.ez_push(2, ".");

        assert_eq!(3, tokens.len());

        let expected = NoteX {
            command: "x".to_string(),
            length: Some(DivisorClock::Divisor(12)),
            dots: 1,
        };

        let actual = NoteX::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_x_all_2() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "x");
        tokens.ez_push(1, "%");
        tokens.ez_push(1, "192");
        tokens.ez_push(2, "...");

        assert_eq!(4, tokens.len());

        let expected = NoteX {
            command: "x".to_string(),
            length: Some(DivisorClock::Clock(192)),
            dots: 3,
        };

        let actual = NoteX::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_r_length_1() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "r");
        tokens.ez_push(1, "4");

        assert_eq!(2, tokens.len());

        let expected = NoteR {
            command: "r".to_string(),
            length: Some(DivisorClock::Divisor(4)),
            dots: 0,
        };

        let actual = NoteR::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_r_length_2() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "r");
        tokens.ez_push(1, "12");

        assert_eq!(2, tokens.len());

        let expected = NoteR {
            command: "r".to_string(),
            length: Some(DivisorClock::Divisor(12)),
            dots: 0,
        };

        let actual = NoteR::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_r_length_3() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "r");
        tokens.ez_push(1, "%");
        tokens.ez_push(1, "4");

        assert_eq!(3, tokens.len());

        let expected = NoteR {
            command: "r".to_string(),
            length: Some(DivisorClock::Clock(4)),
            dots: 0,
        };

        let actual = NoteR::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_r_length_4() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "r");
        tokens.ez_push(1, "%");
        tokens.ez_push(1, "12");

        assert_eq!(3, tokens.len());

        let expected = NoteR {
            command: "r".to_string(),
            length: Some(DivisorClock::Clock(12)),
            dots: 0,
        };

        let actual = NoteR::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_r_dots_1() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "r");
        tokens.ez_push(2, "..");

        assert_eq!(2, tokens.len());

        let expected = NoteR {
            command: "r".to_string(),
            length: None,
            dots: 2,
        };

        let actual = NoteR::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_r_all_1() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "r");
        tokens.ez_push(1, "12");
        tokens.ez_push(2, ".");

        assert_eq!(3, tokens.len());

        let expected = NoteR {
            command: "r".to_string(),
            length: Some(DivisorClock::Divisor(12)),
            dots: 1,
        };

        let actual = NoteR::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_note_r_all_2() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "r");
        tokens.ez_push(1, "%");
        tokens.ez_push(1, "192");
        tokens.ez_push(2, "...");

        assert_eq!(4, tokens.len());

        let expected = NoteR {
            command: "r".to_string(),
            length: Some(DivisorClock::Clock(192)),
            dots: 3,
        };

        let actual = NoteR::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }
}
