use std::process::Command;

pub enum ParamKind {
    Note,
    Length,
    Int,
    Comma,
    Dots,
}

pub struct CommandSpec {
    pub command: &'static str,
    pub min_args: usize,
    pub max_args: usize,
    pub kinds: &'static [ParamKind],
}

// ================================================================================

pub const PORTAMENTO_BEGIN_SPEC: CommandSpec = CommandSpec {
    command: "{",
    min_args: 1,
    max_args: 1,
    kinds: &[
        ParamKind::Note,
        ParamKind::Note,
        ParamKind::Length,
        ParamKind::Dots,
        ParamKind::Length,
    ],
};

pub const PORTAMENTO_END_SPEC: CommandSpec = CommandSpec {
    command: "}",
    min_args: 1,
    max_args: 3,
    kinds: &[ParamKind::Int, ParamKind::Dots, ParamKind::Int],
};

pub const LOCAL_LOOP_BEGIN_SPEC: CommandSpec = CommandSpec {
    command: "[",
    min_args: 0,
    max_args: 0,
    kinds: &[],
};

pub const LOCAL_LOOP_SEPARATOR_SPEC: CommandSpec = CommandSpec {
    command: ":",
    min_args: 0,
    max_args: 0,
    kinds: &[],
};

pub const LOCAL_LOOP_END_SPEC: CommandSpec = CommandSpec {
    command: "]",
    min_args: 0,
    max_args: 1,
    kinds: &[ParamKind::Int],
};

pub const NOTE_C_SPEC: CommandSpec = CommandSpec {
    command: "c",
    min_args: 0,
    max_args: 4,
    kinds: &[ParamKind::Int, ParamKind::Length, ParamKind::Dots],
};
