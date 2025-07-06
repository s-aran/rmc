use std::collections::HashMap;

use crate::{
    errors::Pass2Error,
    meta_models::{Code, MetaData, Pass2Working, Token, TokenStack},
    models::{DivisorClock, NegativePositive, NoteCommand},
    pass2::Pass2,
};

const SINGLE_COMMANDS: &[&str] = &[
    // ノート・休符
    "c", "d", "e", "f", "g", "a", "b", "x", "r", // オクターブ
    "o", ">", "<", "X", // デフォルト長・加工
    "l", "&", "C", "Q", "q", // テンポ・音量
    "t", "T", "v", "V", "(", ")", // 装飾・擬似エコー
    "S", "W", // 音色・FM/SSG/PCM
    "@", "s", "O", "D", "M", // LFO, ループ, その他
    "#", "[", "]", "L", "*", "N", // ポルタメント
    "{", "}",
];

const MULTI_COMMANDS: &[&str] = &[
    // グリッサンド
    "_{",
    "_M",
    // Pass1 ディレクティブ
    "##",
    "#Include",
    "#FM3Extend",
    "#PPZExtend",
    "#Zenlen",
    "#Tempo",
    "#Timer",
    "#Octave",
    "#LoopDefault",
    "#DT2Flag",
    "#Transpose",
    "#Detune",
    "#LFOSpeed",
    "#PCMVolume",
    "#EnvelopeSpeed",
    "#Volumedown",
    // 複合コマンド
    "DX",
    "DM",
    "DD",
    "EX",
    "MA",
    "MB",
    "sd",
    "sdd",
    "__",
    // 符号付きコマンド
    "o+",
    "o-",
    "l=",
    "l+",
    "l-",
    "l^",
    "v+",
    "v-",
    "v)",
    "v(",
];

trait ParsePartCommand {
    fn parse(&mut self, pass2: &Pass2, working: &mut Pass2Working, c: char);
    fn next();
}

pub(crate) type State = u8;

pub(crate) trait ParseState {
    fn get_initialized() -> State {
        0
    }

    fn get_completed() -> State;

    fn get_abort() -> State {
        State::MAX
    }

    fn next_states(current_state: &State) -> Vec<State>;
    fn eval(&mut self, state: State, token: &Token) -> Result<(), Pass2Error>;

    fn is_allowed(current_state: &State) -> bool {
        Self::next_states(current_state).contains(&&current_state)
    }
}

#[derive(Debug)]
pub(crate) enum MachineState {
    Nop,
    Command,
    Parameter,
    Completed(PartCommand),
}

#[derive(Debug)]
pub(crate) struct StateMachine<'a> {
    pass2: &'a Pass2,
    working: &'a mut Pass2Working,

    state: State,
    machine_state: MachineState,
    command: Token,
}

impl<'a> StateMachine<'a> {
    pub fn new(pass2: &'a Pass2, working: &'a mut Pass2Working) -> Self {
        Self {
            pass2,
            command: working.token.clone(),
            working,
            state: State::default(),
            machine_state: MachineState::Nop,
        }
    }

    pub fn next(&mut self, c: char) -> State {
        0
    }

    pub fn get(&self) -> Result<PartCommand, Pass2Error> {
        if self.command.is_empty() {
            panic!("StateMachine: token is empty");
        }
        panic!("StateMachine: token is empty");
    }
}

pub(crate) fn is_command(token: &Token) -> bool {
    let s = &token.chars.as_str();
    MULTI_COMMANDS.contains(s) || SINGLE_COMMANDS.contains(s)
}

#[derive(Debug, Clone)]
pub struct Note {
    pub command: String,
    pub natural: Option<bool>,
    pub semitone: Option<NegativePositive>,
    pub length: Option<u8>,
    pub dots: Option<String>,
}

impl ParseState for Note {
    fn get_completed() -> State {
        5
    }

    fn eval(&mut self, state: State, token: &Token) -> Result<(), Pass2Error> {
        let t = &token.chars;

        match state {
            0 => {
                if t.len() == 1 && "cdefgab".contains(t) {
                    self.command = t.to_owned();
                    return Ok(());
                }
                panic!("Note: note");
            }
            1 => {
                if t.len() == 1 && "=".contains(t) {
                    self.natural = Some(true);
                    return Ok(());
                }
                panic!("Note: natural");
            }
            2 => {
                if t.len() == 1 && "+-".contains(t) {
                    return Ok(());
                }
                panic!("Note: semitone");
            }
            3 => {
                if let Ok(parsed) = t.parse::<u8>() {
                    self.length = Some(parsed);
                    return Ok(());
                }
                panic!("Note: length");
            }
            4 => {
                if t.chars().all(|c| c == '.') {
                    self.dots = Some(t.to_owned());
                    return Ok(());
                }
                panic!("Note: dots");
            }
            _ => {
                panic!("Note: unknown")
            }
        }
    }

    fn next_states(current_state: &State) -> Vec<State> {
        match current_state {
            0 => vec![1],          // c/d/e/f/g/a/b
            1 => vec![2, 3, 4, 5], // [=]
            2 => vec![3, 4, 5],    // [+/-]
            3 => vec![4, 5],       // [音長]
            4 => vec![5],          // [.]
            _ => vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoteX {
    pub length: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NoteR {
    pub length: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PortamentoBegin {
    pub pitch1: Option<NoteCommand>,
    pub pitch2: Option<NoteCommand>,
    pub length1: Option<u8>,
    pub dots: Option<String>,
    pub length2: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct Octave {
    pub value: u8,
}

#[derive(Debug, Clone)]
pub struct OctaveUp;

#[derive(Debug, Clone)]
pub struct OctaveDown;

#[derive(Debug, Clone)]
pub struct OctaveReverse;

#[derive(Debug, Clone)]
pub struct PartOctaveChangePositive;

#[derive(Debug, Clone)]
pub struct PartOctaveChangeNegative;

#[derive(Debug, Clone)]
pub struct DefaultLength {
    pub value_type: Option<DivisorClock>,
    pub value: u8,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProcessLastLengthUpdate {
    pub value_type: Option<DivisorClock>,
    pub value: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProcessLastLengthAdd {
    pub value_type: Option<DivisorClock>,
    pub value: u8,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProcessLastLengthSubtract {
    pub value_type: Option<DivisorClock>,
    pub value: u8,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProcessLastLengthMultiply {
    pub value: u8,
}

#[derive(Debug, Clone)]
pub struct Tie {
    pub length: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Slur {
    pub length: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub enum PartCommand {
    Nop,

    NoteC(Code, Note),
    NoteD(Code, Note),
    NoteE(Code, Note),
    NoteF(Code, Note),
    NoteG(Code, Note),
    NoteA(Code, Note),
    NoteB(Code, Note),
    NoteX(Code, NoteX),
    NoteR(Code, NoteR),

    Portamento(Code, PortamentoBegin),

    Octave(Code, Octave),
    OctaveUp(Code),
    OctaveDown(Code),
    OctaveReverse(Code),
    PartOctaveChangePositive(Code),
    PartOctaveChangeNegative(Code),

    DefaultLength(Code, DefaultLength),

    ProcessLastLengthUpdate(Code, ProcessLastLengthUpdate),
    ProcessLastLengthAdd(Code, ProcessLastLengthAdd),
    ProcessLastLengthSubtract(Code, ProcessLastLengthSubtract),
    ProcessLastLengthMultiply(Code, ProcessLastLengthMultiply),

    Tie(Code, Tie),
    Slur(Code, Slur),
}

pub type WrappedPartCommand = MetaData<PartCommand>;

#[cfg(test)]
mod tests {
    use crate::meta_models::Pass1Result;

    use super::*;

    #[test]
    fn test_note_1() {
        const MML: &str = "c+4";
        let mut code = Code::default();
        let pass1_result = Pass1Result::default();

        let pass2 = Pass2::new(code, MML.to_string(), pass1_result);
        let mut working = Pass2Working::default();

        let mut state_machine = StateMachine::new(&pass2, &mut working);
        for c in MML.chars() {
            println!("{c}");
            state_machine.next(c);
        }

        let command = state_machine.get();
    }
}
