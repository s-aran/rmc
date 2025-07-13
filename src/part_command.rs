use std::str::FromStr;

use crate::{
    errors::Pass2Error,
    meta_models::{Code, MetaData, Token, TokenStackTrait, TokenTrait},
    models::{
        DivisorClock, NegativePositive, NegativePositiveEqual, NoteCommand, NoteOctaveCommand,
    },
    utils::get_type_name,
};

macro_rules! try_from_get_value {
    ($expr:expr, $field:ident) => {
        match $expr {
            Ok(Some(v)) => v,
            Ok(None) => panic!(
                "TryFrom for {} ({}): None",
                stringify!(Self),
                stringify!($field)
            ),
            Err(e) => panic!(
                "TryFrom for {} ({}): {}",
                stringify!($typ),
                stringify!($field),
                e
            ),
        }
    };
}

macro_rules! try_from_get_some_value {
    ($expr:expr, $field:ident) => {
        match $expr {
            Ok(v) => v,
            Err(e) => panic!(
                "TryFrom for {} ({}): {}",
                stringify!(Self),
                stringify!($field),
                e
            ),
        }
    };
}

pub(crate) type State = u8;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub(crate) struct PartToken {
    state: State,
    code: Code,
    token: Token,
}

impl TokenTrait for PartToken {
    fn chars(&self) -> &String {
        self.token.chars()
    }

    fn chars_mut(&mut self) -> &mut String {
        self.token.chars_mut()
    }

    fn begin(&self) -> &usize {
        self.token.begin()
    }

    fn begin_mut(&mut self) -> &mut usize {
        self.token.begin_mut()
    }

    fn end(&self) -> &usize {
        self.token.end()
    }

    fn end_mut(&mut self) -> &mut usize {
        self.token.end_mut()
    }
}

impl PartToken {
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn set_code(&mut self, code: &Code) {
        self.code = code.clone();
    }

    pub fn get_code(&self) -> &Code {
        &self.code
    }
}
#[derive(Debug, Clone, Default)]
pub(crate) struct PartTokenStack {
    stack: Vec<PartToken>,
}

impl TokenStackTrait<PartToken> for PartTokenStack {
    fn stack(&self) -> &Vec<PartToken> {
        &self.stack
    }

    fn stack_mut(&mut self) -> &mut Vec<PartToken> {
        &mut self.stack
    }
}

impl PartTokenStack {
    pub fn first(&self) -> Option<&PartToken> {
        self.stack.first()
    }

    pub fn dequeue(&mut self) -> Option<PartToken> {
        if self.stack.len() > 0 {
            return Some(self.stack.remove(0));
        }

        None
    }

    pub fn get(&self, index: usize) -> Option<&PartToken> {
        self.stack.get(index)
    }

    pub fn find_by_state(&self, state: State) -> Vec<&PartToken> {
        self.stack.iter().filter(|&e| e.state == state).collect()
    }

    pub fn get_by_state(&self, state: State) -> Option<&PartToken> {
        let r = self.find_by_state(state);
        if r.len() > 1 {
            panic!("get_by_state({})", state);
        }

        match r.get(0) {
            Some(e) => Some(*e),
            None => None,
        }
    }

    pub fn get_and_cast<T>(&self, state: State) -> Result<Option<T>, <T as FromStr>::Err>
    where
        T: FromStr + Clone,
    {
        let t = self.get_by_state(state);
        match t {
            Some(e) => match T::from_str(e.token.chars.as_str()) {
                Ok(v) => Ok(Some(v)),
                Err(e) => Err(e),
            },
            None => Ok(None),
        }
    }

    pub fn pop_by_state_all(&mut self, state: State) -> Vec<PartToken> {
        let (removed, kept) = self.stack.drain(..).partition(|e| e.state == state);
        self.stack = kept;

        removed
    }

    pub fn pop_by_state(&mut self, state: State) -> Option<PartToken> {
        let r = self.find_by_state(state);
        if r.len() > 1 {
            panic!("pop_by_state({}): len > 1", state);
        }

        match r.get(0) {
            Some(e) => {
                let t = self
                    .stack
                    .remove(self.stack.iter().position(|x| x == *e).unwrap());
                Some(t)
            }
            None => None,
        }
    }

    pub fn pop_and_cast_vec<T>(&mut self, state: State) -> Result<Vec<T>, <T as FromStr>::Err>
    where
        T: FromStr + Clone,
    {
        let t = self.pop_by_state_all(state);
        Ok(t.iter()
            .map(|e| {
                if let Ok(v) = T::from_str(e.token.chars.as_str()) {
                    v
                } else {
                    panic!("pop_and_cast_vec<{}>", get_type_name::<T>());
                }
            })
            .collect())
    }

    pub fn pop_and_cast<T>(&mut self, state: State) -> Result<Option<T>, <T as FromStr>::Err>
    where
        T: FromStr + Clone,
    {
        let t = self.pop_by_state(state);
        match t {
            Some(e) => match T::from_str(e.token.chars.as_str()) {
                Ok(v) => Ok(Some(v)),
                Err(e) => Err(e),
            },
            None => Ok(None),
        }
    }
}

pub trait PartCommandStruct: std::fmt::Debug {
    fn to_variant(self) -> PartCommand;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Note {
    pub command: String,
    pub natural: bool,
    pub semitone: Option<NegativePositive>,
    pub length: Option<u8>,
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
        let length = try_from_get_some_value!(value.pop_and_cast::<u8>(3), length);
        let dots = if let Some(e) = try_from_get_some_value!(value.pop_and_cast::<String>(4), dots)
        {
            e.chars().filter(|&c| c == '.').count() as u8
        } else {
            0
        };

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
    pub length: Option<u8>,
    pub dots: Option<String>,
}

impl PartCommandStruct for NoteX {
    fn to_variant(self) -> PartCommand {
        PartCommand::NoteX(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoteR {
    pub length: Option<u8>,
    pub dots: Option<String>,
}

impl PartCommandStruct for NoteR {
    fn to_variant(self) -> PartCommand {
        PartCommand::NoteR(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PortamentoBegin {
    pub pitch1: Option<NoteOctaveCommand>,
    pub pitch2: Option<NoteOctaveCommand>,
    pub length1: Option<u8>,
    pub dots: Option<String>,
    pub length2: Option<u8>,
}

impl PartCommandStruct for PortamentoBegin {
    fn to_variant(self) -> PartCommand {
        PartCommand::Portamento(self)
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
    pub value_type: Option<DivisorClock>,
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
    pub value_type: Option<DivisorClock>,
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
    pub value_type: Option<DivisorClock>,
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
    pub value_type: Option<DivisorClock>,
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
    pub value: u8,
}

impl PartCommandStruct for ProcessLastLengthMultiply {
    fn to_variant(self) -> PartCommand {
        PartCommand::ProcessLastLengthMultiply(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tie {
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
    pub length: Option<u8>,
    pub dots: Option<String>,
}

impl PartCommandStruct for Slur {
    fn to_variant(self) -> PartCommand {
        PartCommand::Slur(self)
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalLoopBegin {
    pub command: String,
}

impl PartCommandStruct for LocalLoopBegin {
    fn to_variant(self) -> PartCommand {
        PartCommand::LocalLoopBegin(self)
    }
}

impl TryFrom<PartTokenStack> for LocalLoopBegin {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);

        Ok(Self { command })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalLoopFinalBreak {
    pub command: String,
}

impl PartCommandStruct for LocalLoopFinalBreak {
    fn to_variant(self) -> PartCommand {
        PartCommand::LocalLoopFinalBreak(self)
    }
}

impl TryFrom<PartTokenStack> for LocalLoopFinalBreak {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);

        Ok(Self { command })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalLoopEnd {
    pub command: String,
    pub count: Option<u8>,
}

impl PartCommandStruct for LocalLoopEnd {
    fn to_variant(self) -> PartCommand {
        PartCommand::LocalLoopEnd(self)
    }
}

impl TryFrom<PartTokenStack> for LocalLoopEnd {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);
        let count = try_from_get_some_value!(value.pop_and_cast(1), count);

        Ok(Self { command, count })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SsgPcmSoftwareEnvelope {
    pub command: String,
    pub value1: u8,         // 0..255 || 0..31
    pub value2: i8,         // -15..+15 || 0..31
    pub value3: u8,         // 0..255 || 0..31
    pub value4: u8,         // 0.255 || 0..15
    pub value5: Option<u8>, // None || 0..15
    pub value6: Option<u8>, // None || 0..15
}

impl PartCommandStruct for SsgPcmSoftwareEnvelope {
    fn to_variant(self) -> PartCommand {
        PartCommand::SsgPcmSoftwareEnvelope(self)
    }
}

impl TryFrom<PartTokenStack> for SsgPcmSoftwareEnvelope {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);
        let value1 = try_from_get_value!(value.pop_and_cast(1), value1);

        let value2_sign = try_from_get_some_value!(value.pop_and_cast(2), value2_sign);
        let value2_value = try_from_get_value!(value.pop_and_cast(3), value2);
        let value2 = to_some_i8(value2_sign, Some(value2_value)).unwrap();

        let value3 = try_from_get_value!(value.pop_and_cast(4), value3);
        let value4 = try_from_get_value!(value.pop_and_cast(5), value4);
        let value5 = try_from_get_some_value!(value.pop_and_cast(6), value5);
        let value6 = try_from_get_some_value!(value.pop_and_cast(7), value6);

        Ok(Self {
            command,
            value1,
            value2,
            value3,
            value4,
            value5,
            value6,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Volume {
    pub command: String,
    pub value: u8,
}

impl PartCommandStruct for Volume {
    fn to_variant(self) -> PartCommand {
        match self.command.as_str() {
            "v" => PartCommand::Volume1(self),
            "V" => PartCommand::Volume2(self),
            "v+" => PartCommand::GlobalVolume1Positive(self),
            "v-" => PartCommand::GlobalVolume1Negative(self),
            "v)" => PartCommand::GlobalVolume2Positive(self),
            "v(" => PartCommand::GlobalVolume2Negative(self),
            _ => {
                panic!("unexpected command: {}", self.command);
            }
        }
    }
}

impl TryFrom<PartTokenStack> for Volume {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(1), command);
        let value = try_from_get_value!(value.pop_and_cast(2), value);

        Ok(Self { command, value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quantize1 {
    pub command: String,
    pub divisor: Option<DivisorClock>,
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
        let divisor = try_from_get_some_value!(value.pop_and_cast::<DivisorClock>(2), divisor);
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
pub enum PartCommand {
    Nop,

    Note(Note),
    NoteX(NoteX),
    NoteR(NoteR),

    Portamento(PortamentoBegin),

    Octave(Octave),
    OctaveUp(OctaveUp),
    OctaveDown(OctaveDown),
    OctaveReverse(OctaveReverse),
    PartOctaveChangePositive(Octave),
    PartOctaveChangeNegative(Octave),

    DefaultLength(DefaultLength),

    ProcessLastLengthUpdate(ProcessLastLengthUpdate),
    ProcessLastLengthAdd(ProcessLastLengthAdd),
    ProcessLastLengthSubtract(ProcessLastLengthSubtract),
    ProcessLastLengthMultiply(ProcessLastLengthMultiply),

    Tie(Tie),
    Slur(Slur),

    Quantize1(Quantize1),
    Quantize2(Quantize2),
    AbsoluteTranspose(TemporaryTranspose),
    RelativeTranspose(TemporaryTranspose),
    PartTransposeBegin(PartTransposeBegin),
    PartTransposeEnd(PartTransposeEnd),
    MasterTranspose(MasterTranspose),

    LocalLoopBegin(LocalLoopBegin),
    LocalLoopFinalBreak(LocalLoopFinalBreak),
    LocalLoopEnd(LocalLoopEnd),

    SsgPcmSoftwareEnvelope(SsgPcmSoftwareEnvelope),

    Volume1(Volume),
    Volume2(Volume),
    GlobalVolume1Positive(Volume),
    GlobalVolume1Negative(Volume),
    GlobalVolume2Positive(Volume),
    GlobalVolume2Negative(Volume),
}

pub trait IsPartCommand {}
impl IsPartCommand for PartCommand {}

pub type WrappedPartCommand = MetaData<PartCommand>;

fn to_some_i8(sign: Option<NegativePositive>, value: Option<u8>) -> Option<i8> {
    if value.is_none() {
        return None;
    }

    Some(match sign {
        Some(NegativePositive::Positive) | None => value.unwrap() as i8,
        Some(NegativePositive::Negative) => -(value.unwrap() as i8),
    })
}

fn count_dots(dots: Option<String>) -> u8 {
    if let Some(dots) = dots {
        dots.chars().filter(|&c| c == '.').count() as u8
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_1() {}
}
