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
    pub natural: Option<bool>,
    pub semitone: Option<NegativePositive>,
    pub length: Option<u8>,
    pub dots: Option<String>,
}

impl TryFrom<PartTokenStack> for Note {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);
        let natural = match value.pop_and_cast::<String>(1) {
            Ok(v) => {
                if let Some(w) = v {
                    Some(w == "=")
                } else {
                    None
                }
            }
            Err(e) => panic!("TryFrom for Note (natural): {}", e),
        };

        let semitone =
            try_from_get_some_value!(value.pop_and_cast::<NegativePositive>(2), semitone);
        let length = try_from_get_some_value!(value.pop_and_cast::<u8>(3), length);
        let dots = try_from_get_some_value!(value.pop_and_cast::<String>(4), dots);

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
    pub value: u8,
}

impl PartCommandStruct for Octave {
    fn to_variant(self) -> PartCommand {
        PartCommand::Octave(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OctaveUp;

impl PartCommandStruct for OctaveUp {
    fn to_variant(self) -> PartCommand {
        PartCommand::OctaveUp
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OctaveDown;

impl PartCommandStruct for OctaveDown {
    fn to_variant(self) -> PartCommand {
        PartCommand::OctaveDown
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OctaveReverse;

impl PartCommandStruct for OctaveReverse {
    fn to_variant(self) -> PartCommand {
        PartCommand::OctaveReverse
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PartOctaveChangePositive;

impl PartCommandStruct for PartOctaveChangePositive {
    fn to_variant(self) -> PartCommand {
        PartCommand::PartOctaveChangePositive
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PartOctaveChangeNegative;

impl PartCommandStruct for PartOctaveChangeNegative {
    fn to_variant(self) -> PartCommand {
        PartCommand::PartOctaveChangeNegative
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
    command: String,
    sign: Option<NegativePositiveEqual>,
    notes: Vec<NoteCommand>,
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
    command: String,
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
    command: String,
    sign: Option<NegativePositive>,
    value: u8,
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
    command: String,
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
    command: String,
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
    command: String,
    count: Option<u8>,
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PartCommand {
    Nop,

    Note(Note),
    NoteX(NoteX),
    NoteR(NoteR),

    Portamento(PortamentoBegin),

    Octave(Octave),
    OctaveUp,
    OctaveDown,
    OctaveReverse,
    PartOctaveChangePositive,
    PartOctaveChangeNegative,

    DefaultLength(DefaultLength),

    ProcessLastLengthUpdate(ProcessLastLengthUpdate),
    ProcessLastLengthAdd(ProcessLastLengthAdd),
    ProcessLastLengthSubtract(ProcessLastLengthSubtract),
    ProcessLastLengthMultiply(ProcessLastLengthMultiply),

    Tie(Tie),
    Slur(Slur),

    AbsoluteTranspose(TemporaryTranspose),
    RelativeTranspose(TemporaryTranspose),
    PartTransposeBegin(PartTransposeBegin),
    PartTransposeEnd(PartTransposeEnd),
    MasterTranspose(MasterTranspose),

    LocalLoopBegin(LocalLoopBegin),
    LocalLoopFinalBreak(LocalLoopFinalBreak),
    LocalLoopEnd(LocalLoopEnd),
}

pub trait IsPartCommand {}
impl IsPartCommand for PartCommand {}

pub type WrappedPartCommand = MetaData<PartCommand>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_1() {}
}
