use std::str::FromStr;

use crate::{
    errors::Pass2Error,
    meta_models::{Code, MetaData, Token, TokenStack, TokenStackTrait, TokenTrait},
    models::{DivisorClock, NegativePositive, NoteCommand},
};

pub(crate) type State = u8;

#[derive(Debug, Clone, Default)]
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

    fn try_from(value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = match value.get_and_cast(0) {
            Ok(v) => {
                if let Some(w) = v {
                    w
                } else {
                    panic!("TryFrom for Note (command): command is None");
                }
            }
            Err(e) => panic!("TryFrom for Note (command): {}", e),
        };

        let natural = match value.get_and_cast::<String>(1) {
            Ok(v) => {
                if let Some(w) = v {
                    Some(w == "=")
                } else {
                    None
                }
            }
            Err(e) => panic!("TryFrom for Note (natural): {}", e),
        };

        let semitone = match value.get_and_cast::<NegativePositive>(2) {
            Ok(v) => v,
            Err(e) => panic!("TryFrom for Note (semitone): {}", e),
        };

        let length = match value.get_and_cast::<u8>(3) {
            Ok(v) => v,
            Err(e) => panic!("TryFrom for Note (lengh): {}", e),
        };

        let dots = match value.get_and_cast::<String>(4) {
            Ok(v) => v,
            Err(e) => panic!("TryFrom for Note (dots): {}", e),
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoteX {
    pub length: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoteR {
    pub length: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PortamentoBegin {
    pub pitch1: Option<NoteCommand>,
    pub pitch2: Option<NoteCommand>,
    pub length1: Option<u8>,
    pub dots: Option<String>,
    pub length2: Option<u8>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Octave {
    pub value: u8,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OctaveUp;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OctaveDown;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OctaveReverse;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PartOctaveChangePositive;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PartOctaveChangeNegative;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefaultLength {
    pub value_type: Option<DivisorClock>,
    pub value: u8,
    pub dots: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProcessLastLengthUpdate {
    pub value_type: Option<DivisorClock>,
    pub value: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProcessLastLengthAdd {
    pub value_type: Option<DivisorClock>,
    pub value: u8,
    pub dots: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProcessLastLengthSubtract {
    pub value_type: Option<DivisorClock>,
    pub value: u8,
    pub dots: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProcessLastLengthMultiply {
    pub value: u8,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tie {
    pub length: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Slur {
    pub length: Option<u8>,
    pub dots: Option<String>,
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
}

pub type WrappedPartCommand = MetaData<PartCommand>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_1() {}
}
