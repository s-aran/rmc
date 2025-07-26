use crate::{
    errors::Pass2Error,
    part_command::{PartCommand, PartCommandParseState, PartCommandStruct, PartTokenStack},
};

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

    fn is_block() -> bool {
        false
    }

    fn is_match(command: &str) -> bool {
        vec!["v", "V", "v+", "v-", "v)", "v("].contains(&command)
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        match c {
            '0'..='9' => {
                working.eat(c);
                working.jump(2);
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

impl TryFrom<PartTokenStack> for Volume {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(1), command);
        let value = try_from_get_value!(value.pop_and_cast(2), value);

        Ok(Self { command, value })
    }
}
