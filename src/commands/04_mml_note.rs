use crate::meta_models::{TokenStackTrait, TokenTrait};
use crate::models::Part;
use crate::part_command::{PartCommandParseState, WrappedPartCommand};
use crate::{
    errors::Pass2Error,
    models::{DivisorClock, NegativePositive, NegativePositiveEqual, NoteCommand},
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
        let natural = try_from_get_some_value!(value.pop_and_cast::<String>(1), natural)
            .unwrap_or_default()
            == "=";
        let semitone =
            try_from_get_some_value!(value.pop_and_cast::<NegativePositive>(2), semitone);
        let length = make_some_length(value.pop_by_state_all(3));
        let dots = count_dots(try_from_get_some_value!(
            value.pop_and_cast::<String>(4),
            dots
        ));

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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        command.len() == 1 && "cdefgab".contains(command)
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        match c {
            '=' => {
                // natural, optional
                if working.state > 1 {
                    panic!("Note: unexpected {c}");
                }

                working.eat(c);
                working.jump(1);
                working.push();
            }
            '+' | '-' => {
                // semitone, optional
                if working.state > 2 {
                    panic!("Note: unexptected {c}");
                }

                working.eat(c);
                working.jump(2);
                working.push();
            }
            '%' => {
                if working.state > 4 {
                    panic!("Note: unexpected {c}");
                }

                working.eat(c);
                working.jump(3);
            }
            '0'..='9' => {
                // length, optional
                if working.state > 4 {
                    panic!("Note: unexpected {c}");
                }

                working.eat(c);
                working.jump(3);
            }
            '.' => {
                // dots, optional
                if working.state > 5 {
                    panic!("Note: unexpected {c}");
                }

                if working.state == 3 {
                    working.push();
                }

                working.eat(c);
                working.jump(4);
            }
            _ => {
                // other command
                working.push();

                return PartCommandParseState::Parsed;
            }
        };

        PartCommandParseState::Parsing
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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        vec!["o", "o+", "o-"].contains(&command)
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        match c {
            '0'..='9' => {
                // value
                if working.state > 2 {
                    panic!("Octave: unexpected {c}");
                }

                working.jump(2);

                working.eat(c);
            }
            _ => {
                // other command
                working.push();

                return PartCommandParseState::Parsed;
            }
        }

        PartCommandParseState::Parsing
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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        command == "r"
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        todo!()
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
pub struct Portamento {
    pub begin_command: String,
    pub pitch: Vec<PartCommand>,
    pub end_command: String,
    pub length1: Option<u8>,
    pub dots: u8,
    pub length2: Option<u8>,
}

impl PartCommandStruct for Portamento {
    fn to_variant(self) -> PartCommand {
        PartCommand::Portamento(self)
    }

    fn is_block() -> bool {
        true
    }

    fn is_match(command: &str) -> bool {
        todo!()
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        todo!()
    }
}

impl TryFrom<PartTokenStack> for Portamento {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let begin_command = try_from_get_value!(value.pop_and_cast(0), begin_command);
        let maybe_pitch = if let Some(v) = value.part_command_stack_mut().stack_mut().pop() {
            v
        } else {
            panic!("TryFrom for Portamento: unexpected empty stack");
        };
        let end_command = try_from_get_value!(value.pop_and_cast(1), begin_command);

        let pitch = maybe_pitch
            .iter()
            .map(|e| e.data().clone())
            .filter(|e| match e {
                PartCommand::Note(_)
                | PartCommand::Octave(_)
                | PartCommand::OctaveUp(_)
                | PartCommand::OctaveDown(_) => true,
                _ => panic!("TryFrom for Portament: unexpected command: {:?}", e),
            })
            .collect::<Vec<PartCommand>>();

        let length1 = try_from_get_some_value!(value.pop_and_cast::<u8>(2), length1);
        let dots = count_dots(try_from_get_some_value!(value.pop_and_cast(3), dots));
        let length2 = try_from_get_some_value!(value.pop_and_cast::<u8>(4), length2);

        Ok(Portamento {
            begin_command,
            pitch,
            end_command,
            length1,
            dots,
            length2,
        })
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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        todo!()
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        todo!()
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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        todo!()
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        todo!()
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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        todo!()
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        todo!()
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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        todo!()
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        todo!()
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
    pub length: Option<DivisorClock<u8>>,
    pub dots: u8,
}

impl PartCommandStruct for DefaultLength {
    fn to_variant(self) -> PartCommand {
        PartCommand::DefaultLength(self)
    }

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        todo!()
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        todo!()
    }
}

impl TryFrom<PartTokenStack> for DefaultLength {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);
        let length = make_some_length(value.pop_by_state_all(1));
        let dots = count_dots(try_from_get_some_value!(value.pop_and_cast(2), dots));

        Ok(DefaultLength {
            command,
            length,
            dots,
        })
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProcessLastLengthUpdate {
    pub command: String,
    pub natural: Option<bool>,
    pub length: Option<DivisorClock<u8>>,
    pub dots: u8,
}

impl PartCommandStruct for ProcessLastLengthUpdate {
    fn to_variant(self) -> PartCommand {
        PartCommand::ProcessLastLengthUpdate(self)
    }

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        todo!()
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        todo!()
    }
}

impl TryFrom<PartTokenStack> for ProcessLastLengthUpdate {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);
        let natural =
            try_from_get_some_value!(value.pop_and_cast::<String>(1), natural).map(|v| v == "=");
        let length = make_some_length(value.pop_by_state_all(2));
        let dots = count_dots(try_from_get_some_value!(value.pop_and_cast(3), dots));

        Ok(ProcessLastLengthUpdate {
            command,
            natural,
            length,
            dots,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProcessLastLengthAddSub {
    pub command: String,
    pub length: Option<DivisorClock<u8>>,
    pub dots: u8,
}

impl PartCommandStruct for ProcessLastLengthAddSub {
    fn to_variant(self) -> PartCommand {
        match self.command.as_str() {
            "l+" => PartCommand::ProcessLastLengthAdd(self),
            "l-" => PartCommand::ProcessLastLengthSubtract(self),
            _ => {
                panic!("unexpected command: {}", self.command);
            }
        }
    }

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        todo!()
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        todo!()
    }
}

impl TryFrom<PartTokenStack> for ProcessLastLengthAddSub {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);
        let length = make_some_length(value.pop_by_state_all(3));
        let dots = count_dots(try_from_get_some_value!(value.pop_and_cast(4), dots));

        Ok(ProcessLastLengthAddSub {
            command,
            length,
            dots,
        })
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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        todo!()
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        todo!()
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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        todo!()
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        todo!()
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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        todo!()
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        todo!()
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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        command == "Q"
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        match c {
            '%' => {
                working.eat(c);
                working.push();
                working.jump(2);
            }
            '0'..='9' => {
                working.eat(c);
                working.jump(3);
            }
            _ => {
                // other command
                working.push();

                return PartCommandParseState::Parsed;
            }
        }

        PartCommandParseState::Parsing
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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        command == "q"
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        match c {
            '-' => {
                if !(working.state == 1 || working.state == 3) {
                    panic!("q: unexpected -");
                }

                working.eat(c);
                working.push();
                working.next();
            }
            'l' => {
                if !(working.state == 0 || working.state == 4 || working.state == 7) {
                    panic!("q: unexpected l");
                }

                working.eat(c);
                working.push();
                working.next();
            }
            '0'..='9' => {
                working.eat(c);
            }
            '.' => {
                // dots, optional
                if working.state == 2 || working.state == 8 {
                    working.push();
                }

                working.eat(c);
                match working.state {
                    2 => working.jump(4),
                    8 => working.jump(10),
                    _ => panic!("q: unexpected dot"),
                };
            }
            ',' => {
                working.push();
                working.next();
            }
            _ => {
                // other command
                working.push();

                return PartCommandParseState::Parsed;
            }
        }

        PartCommandParseState::Parsing
    }
}

impl Quantize2 {
    fn command_type_1(mut value: PartTokenStack) -> Result<Self, Pass2Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);
        let value1 = try_from_get_some_value!(value.pop_and_cast(1), value1);
        let value1_dots = 0;

        let has_range =
            try_from_get_some_value!(value.pop_and_cast::<String>(2), range).map(|v| v == "-");
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
            try_from_get_some_value!(value.pop_and_cast::<String>(1), value1_l).map(|v| v != "l");
        let value1 = try_from_get_some_value!(value.pop_and_cast(2), value1);
        if value1.is_some()
            && let Some(v) = has_value1_l
            && !v
        {
            panic!("Quantize2 (format 2): l is not specified in value1");
        }
        let value1_dots = count_dots(try_from_get_some_value!(value.pop_and_cast(3), value1_dots));

        let has_range =
            try_from_get_some_value!(value.pop_and_cast::<String>(4), range).map(|v| v == "l");
        let has_value2_l =
            try_from_get_some_value!(value.pop_and_cast::<String>(5), value1_l).map(|v| v != "l");
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
            try_from_get_some_value!(value.pop_and_cast::<String>(7), value1_l).map(|v| v != "l");
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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        vec!["_", "__"].contains(&command)
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        match c {
            '+' | '-' => {
                // semitone, optional
                if working.state > 1 {
                    panic!("Absolute Transpose: unexpected {c}");
                }

                working.jump(2);

                working.eat(c);
                working.push();
            }
            '0'..='9' => {
                // value
                if working.state > 3 {
                    panic!("Absolute Transpose: unexpected {c}");
                }

                working.jump(3);

                working.eat(c);
            }
            _ => {
                // other command
                working.push();

                return PartCommandParseState::Parsed;
            }
        }

        PartCommandParseState::Parsing
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
pub struct PartTranspose {
    pub command_begin: String,
    pub sign: Option<NegativePositiveEqual>,
    pub notes: Vec<WrappedPartCommand>,
    pub command_end: String,
}

impl PartCommandStruct for PartTranspose {
    fn to_variant(self) -> PartCommand {
        PartCommand::PartTranspose(self)
    }

    fn is_block() -> bool {
        true
    }

    fn is_match(command: &str) -> bool {
        command == "_{"
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        match c {
            '+' | '-' | '=' => {
                // semitone|natural, optional
                if working.state > 2 {
                    panic!("Part Transpose: unexpected {c}");
                }

                working.jump(2);

                working.eat(c);
                working.push();

                working.jump(3);

                // println!("saving:");
                // println!("* token: {:?}", working.token);
                // println!("* tokens: {:?}", working.tokens);
                // println!("* tokens_stack: {:?}", working.tokens_stack);
                // println!("* pc_stack: {:?}", working.part_command_stack);

                working.switch_push_to_stack();
                working.part_command_stack.init_vec();
                working.save_to_stack();
            }
            '}' => {
                if working.state <= 2 {
                    panic!("Part Transpose: unexpecetd {c}");
                }

                // working.eat(c);
                working.push();
            }
            _ => {
                if working.state != 3 {
                    panic!("Part Transpose: unexpected {c}");
                }

                return PartCommandParseState::Parsed;
            }
        }

        PartCommandParseState::Parsing
    }
}

impl TryFrom<PartTokenStack> for PartTranspose {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command_begin = try_from_get_value!(value.pop_and_cast(1), command);
        let sign = try_from_get_some_value!(value.pop_and_cast::<NegativePositiveEqual>(2), sign);
        let notes = match value.part_command_stack_mut().pop_vec() {
            Some(v) => {
                if v.len() > 0 {
                    v
                } else {
                    panic!("TryFrom for PartTranspose (notes) is empty");
                }
            }
            None => panic!("TryFrom for PartTranspose (notes): None"),
        };
        let command_end = try_from_get_value!(value.pop_and_cast(3), command);

        Ok(Self {
            command_begin,
            sign,
            notes,
            command_end,
        })
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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        command == "_M"
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        match c {
            '+' | '-' => {
                // semitone, optional
                if working.state > 1 {
                    panic!("Master Transpose: unexpected {c}");
                }

                working.jump(2);

                working.eat(c);
                working.push();
            }
            '0'..='9' => {
                // value
                if working.state > 3 {
                    panic!("Master Transpose: unexpected {c}");
                }

                working.eat(c);
                working.jump(3);
            }
            _ => {
                // other command
                working.push();

                return PartCommandParseState::Parsed;
            }
        }

        PartCommandParseState::Parsing
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
        meta_models::{Code, TokenStackTrait, TokenTrait},
        models::NegativePositive,
        part_command::{PartToken, PartTokenStack, State, WrappedPartCommand},
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

    #[test]
    fn test_portamento_begin_note_1() {
        let mut tokens = PartTokenStack::default();
        tokens.ez_push(0, "{");
        tokens.ez_push(1, "}");

        let part_commands = vec![
            WrappedPartCommand::new(
                &Code::default(),
                PartCommand::Note(Note {
                    command: "c".to_string(),
                    natural: false,
                    semitone: None,
                    length: None,
                    dots: 0,
                }),
            ),
            WrappedPartCommand::new(
                &Code::default(),
                PartCommand::Note(Note {
                    command: "d".to_string(),
                    natural: false,
                    semitone: None,
                    length: None,
                    dots: 0,
                }),
            ),
        ];
        tokens.part_command_stack_mut().push_vec(part_commands);

        assert_eq!(2, tokens.len());

        let expected = Portamento {
            begin_command: "{".to_string(),
            pitch: vec![
                PartCommand::Note(Note {
                    command: "c".to_string(),
                    natural: false,
                    semitone: None,
                    length: None,
                    dots: 0,
                }),
                PartCommand::Note(Note {
                    command: "d".to_string(),
                    natural: false,
                    semitone: None,
                    length: None,
                    dots: 0,
                }),
            ],
            end_command: "}".to_string(),
            length1: None,
            dots: 0,
            length2: None,
        };

        let actual = Portamento::try_from(tokens).unwrap();

        assert_eq!(expected, actual);
    }
}
