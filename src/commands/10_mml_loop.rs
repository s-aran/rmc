use crate::{
    errors::Pass2Error,
    part_command::{
        PartCommand, PartCommandParseState, PartCommandStruct, PartTokenStack, WrappedPartCommand,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalLoop {
    pub begin_command: String,
    pub body_pre: Vec<WrappedPartCommand>,
    pub separator: Option<String>,
    pub body_post: Vec<WrappedPartCommand>,
    pub end_command: String,
    pub count: Option<u8>,
}

impl PartCommandStruct for LocalLoop {
    fn to_variant(self) -> PartCommand {
        PartCommand::LocalLoop(self)
    }

    fn is_block() -> bool {
        true
    }

    fn is_match(command: &str) -> bool {
        "[" == command
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        match c {
            '0'..='9' => {
                if working.state < 5 {
                    panic!("LocalLoop: unexpected {c}");
                }

                if working.state == 5 {
                    //
                }

                working.eat(c);
                working.jump(6);
            }
            ']' => {
                if working.state != 5 {
                    panic!("LocalLoop: unexpected {c}");
                }

                working.push();
                working.jump(6);
            }
            _ => {
                if working.state != 6 {
                    panic!("LocalLoop: unexpected {c}");
                }

                // other command
                working.push();

                return PartCommandParseState::Parsed;
            }
        }

        PartCommandParseState::Parsing
    }
}

impl TryFrom<PartTokenStack> for LocalLoop {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        println!("TryFrom LocalLoop: {:?}", value);

        let begin_command = try_from_get_value!(value.pop_and_cast::<String>(0), begin_command);
        let body_pre = value.part_command_stack_mut().pop_vec().unwrap_or_default();
        let end_command = try_from_get_value!(value.pop_and_cast::<String>(5), end_comamnd);
        let body_post = value.part_command_stack_mut().pop_vec().unwrap_or_default();
        let count = try_from_get_some_value!(value.pop_and_cast::<u8>(6), count);

        Ok(LocalLoop {
            begin_command,
            body_pre,
            separator: None,
            body_post,
            end_command,
            count,
        })
    }
}
