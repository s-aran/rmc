use std::{f32::consts::E, str::FromStr, sync::Arc};

use crate::{
    commands::{
        commands_envelope::SsgPcmSoftwareEnvelope,
        commands_loop::LocalLoop,
        commands_mml::{
            MasterTranspose, Note, Octave, PartTranspose, Portamento, Quantize1, Quantize2,
            TemporaryTranspose,
        },
        commands_volume::Volume,
    },
    errors::Pass2Error,
    meta_models::{
        Code, Command, Pass1Result, Pass2Result, Pass2Working, TokenStackTrait, TokenTrait,
    },
    models::PartSymbol,
    part_command::{
        PartCommand, PartCommandParseState, PartCommandStruct, PartToken, PartTokenStack,
        WrappedPartCommand,
    },
    utils::{ParseUtil, get_type_name, is_n, is_sep},
};

#[derive(Debug, Clone)]
pub struct Pass2 {
    code: Code,
    mml: String,
    pass1: Pass1Result,
}

impl ParseUtil for Pass2 {
    fn get_mml(&self) -> &String {
        &self.mml
    }

    fn get_code(&self) -> &Code {
        &self.code
    }

    fn parse_command(&self, c: char) -> Command {
        if is_sep(c) || is_n(c) {
            return Command::Nop;
        }

        match c {
            ';' => {
                return Command::Comment1(self.clone_code());
            }
            '`' => {
                return Command::Comment2(self.clone_code());
            }
            '@' => {
                if self.get_code().chars == 0 {
                    return Command::FmToneDefine(self.clone_code());
                }
            }
            '#' => {
                if self.get_code().chars == 0 {
                    return Command::Macro(self.clone_code());
                }
            }
            '!' => {
                if self.get_code().chars == 0 {
                    return Command::Variable(self.clone_code());
                }
            }
            'A'..'Z' => {
                if self.get_code().chars == 0 {
                    return Command::Part(
                        self.clone_code(),
                        PartSymbol::from_str(&c.to_string().as_str()).unwrap(),
                    );
                }
            }
            'a'..'z' => {
                if self.get_code().chars == 0 {
                    return Command::Part(
                        self.clone_code(),
                        PartSymbol::from_str(&c.to_string().as_str()).unwrap(),
                    );
                }
            }
            _ => {
                // eprintln!("unsupported command: {}", c);
            }
        }

        return Command::Nop;
    }
}

impl Pass2 {
    pub fn new(code: Code, mml: String, pass1_result: Pass1Result) -> Self {
        Self {
            code,
            mml,
            pass1: pass1_result,
        }
    }

    pub fn parse(&mut self) -> Result<Pass2Result, Pass2Error> {
        let mut result = Pass2Result::default();

        let mut working = Pass2Working::default();

        let mut command = Command::Nop;

        let new_lined_mml = format!("{}\n", self.mml);
        let mut chars = new_lined_mml.chars();
        let mut maybe_c = chars.next();
        while maybe_c.is_some() {
            let c = maybe_c.unwrap();

            match command {
                Command::Nop => 'nop: {
                    command = self.parse_command(c);
                    break 'nop;
                }
                Command::Comment1(_) => 'comment1_command: {
                    if !is_n(c) {
                        break 'comment1_command;
                    }

                    command = Command::Nop;
                }
                Command::Comment2(_) => 'comment2_command: {
                    if c != '`' {
                        break 'comment2_command;
                    }

                    command = Command::Nop;
                }
                Command::Part(_, ref part) => 'part_command: {
                    // let w = WrappedPartCommand::new(self.get_code(), part_command.clone());
                    // commands.push(w);
                    working.code = self.code.clone();
                    let res = self.parse_part_command(&mut working, c);
                    if is_n(c) {
                        if !working.token.is_empty() {
                            working.push();
                        }

                        // println!("end ==> working.tokens = {:?}", working.tokens);

                        working.clear();

                        result.parts.push((part.clone(), working.commands.clone()));
                        println!("result.parts: {:?}", result.parts);
                    }

                    if let Ok(r) = res {
                        // match r {
                        //     PartCommand::Nop => {
                        //         break 'part_command;
                        //     }
                        //     _ => {
                        //         working.clear();
                        //         commands.push(WrappedPartCommand::new(self.get_code(), r));
                        //         if is_n(c) {
                        //             result.parts.push((part.clone(), commands.clone()));
                        //             commands.clear();
                        //         }

                        //         command = Command::Nop;
                        //     }
                        // };
                    }
                }
                _ => {
                    // nop
                }
            }

            self.code.inc_chars();
            maybe_c = chars.next();
            if is_n(c) {
                self.code.inc_lines();
                command = Command::Nop;
            }
        }

        Ok(result)
    }

    fn parse_part_command(
        &self,
        working: &mut Pass2Working,
        c: char,
    ) -> Result<PartCommand, Pass2Error> {
        // println!(
        //     "begin: {c}: {:?} {:?} / {}",
        //     working.token,
        //     working.tokens,
        //     working.tokens.first().is_some()
        // );

        // println!("tokens: {:?} // {:?}", working.tokens, working.token);
        if working.tokens.first().is_none() {
            if working.token.is_empty() && is_sep(c) {
                return Ok(PartCommand::Nop);
            }

            if working.token.is_empty() {
                working.eat(c);
            }

            let t = working.token.chars().as_str();
            match t {
                "c" | "d" | "e" | "f" | "g" | "a" | "b" | "E" | "V" | "q" | "Q" => {
                    working.push();
                    working.jump(1);
                    return Ok(PartCommand::Nop);
                }
                "o" => {
                    if working.state <= 0 {
                        working.jump(1);
                        return Ok(PartCommand::Nop);
                    }

                    match c {
                        '+' | '-' => {
                            working.eat(c);
                            working.push();
                            return Ok(PartCommand::Nop);
                        }
                        _ => {
                            working.push();
                            // fall
                        }
                    }
                }
                "_" => {
                    if working.state <= 0 {
                        working.jump(1);
                        return Ok(PartCommand::Nop);
                    }

                    match c {
                        '_' | '{' | 'M' => {
                            working.eat(c);
                            working.push();

                            return Ok(PartCommand::Nop);
                        }
                        _ => {
                            working.push();
                            // fall
                        }
                    }
                }
                "}" => {
                    working.load_from_stack();
                    working.jump(3);
                    working.switch_push_to_commands();

                    // println!("loaded:");
                    // println!("* token: {:?}", working.token);
                    // println!("* tokens: {:?}", working.tokens);
                    // println!("* tokens_stack: {:?}", working.tokens_stack);
                    // println!("* pc_stack: {:?}", working.part_command_stack);

                    // fall througbh
                }
                "[" => {
                    working.loop_nest += 1;

                    working.push();
                    working.jump(2);

                    // println!("saving:");
                    // println!("* token: {:?}", working.token);
                    // println!("* tokens: {:?}", working.tokens);
                    // println!("* tokens_stack: {:?}", working.tokens_stack);
                    // println!("* pc_stack: {:?}", working.part_command_stack);

                    working.switch_push_to_stack();
                    working.part_command_stack.init_vec();
                    working.save_to_stack();

                    return Ok(PartCommand::Nop);
                }
                "]" => {
                    // println!("LocalLoop end {c}:");

                    working.load_from_stack();
                    working.jump(5);
                    working.switch_push_to_commands();
                    working.loop_nest -= 1;

                    // println!("loaded:");
                    // println!("* token: {:?}", working.token);
                    // println!("* tokens: {:?}", working.tokens);
                    // println!("* tokens_stack: {:?}", working.tokens_stack);
                    // println!("* pc_stack: {:?}", working.part_command_stack);

                    // fall through
                }
                ":" => {
                    // determine before token
                    working.tokens = if let Some(v) = working.tokens_stack.pop() {
                        // prepare stack for post part commands
                        working.part_command_stack.init_vec();
                        v
                    } else {
                        panic!("LocalLoop: invalid stack");
                    };
                    working.state = working.state_stack.pop().unwrap();
                    working.jump(3);

                    working.part_command_stack.init_vec();
                    working.state_stack.push(working.state);
                    return Ok(PartCommand::Nop);
                }
                "v" => {
                    if working.state <= 0 {
                        working.jump(1);
                        return Ok(PartCommand::Nop);
                    }

                    match c {
                        '+' | '-' | ')' | '(' => {
                            working.eat(c);
                            working.push();
                            return Ok(PartCommand::Nop);
                        }
                        _ => {
                            working.push();
                            // fall
                        }
                    }
                }
                _ => {
                    panic!("unknwon command: {c}");
                }
            }
        }

        // println!("tokens ==> {:?} // c={c}", working.tokens);
        let first_token = working.tokens.first().unwrap().chars().as_str();
        match first_token {
            "c" | "d" | "e" | "f" | "g" | "a" | "b" => {
                self.__parse_part_command::<Note>(working, c)
            }
            "o" | "o+" | "o-" => self.__parse_part_command::<Octave>(working, c),
            "_{" => self.__parse_part_command::<PartTranspose>(working, c),
            "_" | "__" => self.__parse_part_command::<TemporaryTranspose>(working, c),
            "_M" => self.__parse_part_command::<MasterTranspose>(working, c),
            "[" => self.__parse_part_command::<LocalLoop>(working, c),
            "E" => self.__parse_part_command::<SsgPcmSoftwareEnvelope>(working, c),
            "v" | "V" | "v+" | "v-" | "v)" | "v(" => {
                self.__parse_part_command::<Volume>(working, c)
            }
            "Q" => self.__parse_part_command::<Quantize1>(working, c),
            "q" => self.__parse_part_command::<Quantize2>(working, c),
            _ => {
                panic!("unknown command: {first_token}");
            }
        }
    }

    fn __parse_part_command<T>(
        &self,
        working: &mut Pass2Working,
        c: char,
    ) -> Result<PartCommand, Pass2Error>
    where
        T: TryFrom<PartTokenStack, Error = Pass2Error> + PartCommandStruct,
    {
        if T::parse(working, c) == PartCommandParseState::Parsed {
            if T::is_block() {
                Self::push_block_part_command::<T>(working);
            } else {
                Self::push_part_command::<T>(working);
            }

            // retry
            return self.parse_part_command(working, c);
        }

        return Ok(PartCommand::Nop);
    }

    fn push_part_command<T>(working: &mut Pass2Working)
    where
        T: TryFrom<PartTokenStack, Error = Pass2Error> + PartCommandStruct,
    {
        let code = working.tokens.first().unwrap().get_code().clone();
        let tokens = working.tokens.drain();
        let command = match T::try_from(tokens) {
            Ok(v) => v,
            Err(e) => panic!("{}: {} // {:?}", get_type_name::<T>(), e, working.tokens),
        };

        // println!("==> {}: {:?}", get_type_name::<T>(), command);
        // println!("==> {:?}", working);

        let w = WrappedPartCommand::new(&code, command.to_variant());

        // println!("{}", working.push_to_working_stack);
        if working.push_to_working_stack {
            println!("==> push to part_command_stack: {:?}", w);
            working.part_command_stack.push_token(w);
        } else {
            println!("==> push commands: {:?}", w);
            working.commands.push(w);
        }

        working.clear();
    }

    fn push_block_part_command<T>(working: &mut Pass2Working)
    where
        T: TryFrom<PartTokenStack, Error = Pass2Error> + PartCommandStruct,
    {
        // working.tokens.part_command_stack_mut().init_vec();
        if let Some(v) = working.part_command_stack.pop_vec() {
            working.tokens.part_command_stack_mut().push_vec(v);
        } else {
            panic!("{}: part command stack is empty", get_type_name::<T>());
        }
        // for _ in 0..pops {
        //     if let Some(v) = working.part_command_stack.pop_vec() {
        //         working.tokens.part_command_stack_mut().push_vec(v);
        //     } else {
        //         panic!("{}: part command stack is empty", get_type_name::<T>());
        //     }
        // }

        Self::push_part_command::<T>(working);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{DivisorClock, NegativePositive, NegativePositiveEqual, PartSymbol},
        pass1::Pass1,
    };

    use super::*;

    #[test]
    fn test_1() {
        let mml = r#"G	c+4d-12e8f.g=a..b4...._-2[e__+1]8_0_{-eab}_M+120"#;

        let code = Code::default();
        let mut pass1 = Pass1::new(code, mml.to_owned());
        let pass1_result = pass1.parse().unwrap();

        // moved
        let code = Code::default();
        let mut pass2 = Pass2::new(code, mml.to_owned(), pass1_result);
        let result: Pass2Result = pass2.parse().unwrap();

        let part_g_list = result.get_parts(&PartSymbol::G);
        assert_eq!(1, part_g_list.len());

        println!("{:?}", part_g_list);

        let g_commands = part_g_list.get(0).unwrap();
        assert_eq!(12, g_commands.len());

        // c+4
        let expected = Note {
            command: "c".to_string(),
            natural: false,
            semitone: Some(NegativePositive::Positive),
            length: Some(DivisorClock::Divisor(4)),
            dots: 0,
        };
        let actual = g_commands.get(0).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // d-12
        let expected = Note {
            command: "d".to_string(),
            natural: false,
            semitone: Some(NegativePositive::Negative),
            length: Some(DivisorClock::Divisor(12)),
            dots: 0,
        };
        let actual = g_commands.get(1).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // e8
        let expected = Note {
            command: "e".to_string(),
            natural: false,
            semitone: None,
            length: Some(DivisorClock::Divisor(8)),
            dots: 0,
        };
        let actual = g_commands.get(2).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // f.
        let expected = Note {
            command: "f".to_string(),
            natural: false,
            semitone: None,
            length: None,
            dots: 1,
        };
        let actual = g_commands.get(3).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // g=
        let expected = Note {
            command: "g".to_string(),
            natural: true,
            semitone: None,
            length: None,
            dots: 0,
        };
        let actual = g_commands.get(4).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // a..
        let expected = Note {
            command: "a".to_string(),
            natural: false,
            semitone: None,
            length: None,
            dots: 2,
        };
        let actual = g_commands.get(5).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // b4....
        let expected = Note {
            command: "b".to_string(),
            natural: false,
            semitone: None,
            length: Some(DivisorClock::Divisor(4)),
            dots: 4,
        };
        let actual = g_commands.get(6).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // _-2
        let expected = TemporaryTranspose {
            command: "_".to_string(),
            semitone: Some(NegativePositive::Negative),
            value: 2,
        };
        let actual = g_commands.get(7).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::AbsoluteTranspose(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // [e__+1]8
        let expected = LocalLoop {
            begin_command: "[".to_string(),
            body_pre: vec![
                WrappedPartCommand::new(
                    &Code {
                        file_name: "".to_string(),
                        lines: 0,
                        chars: 28,
                    },
                    // e
                    (Note {
                        command: "e".to_string(),
                        natural: false,
                        semitone: None,
                        length: None,
                        dots: 0,
                    })
                    .to_variant(),
                ),
                WrappedPartCommand::new(
                    &Code {
                        file_name: "".to_string(),
                        lines: 0,
                        chars: 30,
                    },
                    // __+1
                    (TemporaryTranspose {
                        command: "__".to_string(),
                        semitone: Some(NegativePositive::Positive),
                        value: 1,
                    })
                    .to_variant(),
                ),
            ],
            separator: None,
            body_post: vec![],
            end_command: "]".to_string(),
            count: Some(8),
        };
        let actual = g_commands.get(8).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::LocalLoop(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // _0
        let expected = TemporaryTranspose {
            command: "_".to_string(),
            semitone: None,
            value: 0,
        };
        let actual = g_commands.get(9).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::AbsoluteTranspose(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // _{-eab}
        let expected = PartTranspose {
            command_begin: "_{".to_string(),
            sign: Some(NegativePositiveEqual::Negative),
            notes: vec![
                WrappedPartCommand::new(
                    &Code {
                        file_name: "".to_string(),
                        lines: 0,
                        chars: 40,
                    },
                    PartCommand::Note(Note {
                        command: "e".to_string(),
                        natural: false,
                        semitone: None,
                        length: None,
                        dots: 0,
                    }),
                ),
                WrappedPartCommand::new(
                    &Code {
                        file_name: "".to_string(),
                        lines: 0,
                        chars: 41,
                    },
                    PartCommand::Note(Note {
                        command: "a".to_string(),
                        natural: false,
                        semitone: None,
                        length: None,
                        dots: 0,
                    }),
                ),
                WrappedPartCommand::new(
                    &Code {
                        file_name: "".to_string(),
                        lines: 0,
                        chars: 42,
                    },
                    PartCommand::Note(Note {
                        command: "b".to_string(),
                        natural: false,
                        semitone: None,
                        length: None,
                        dots: 0,
                    }),
                ),
            ],
            command_end: "}".to_string(),
        };
        let actual = g_commands.get(10).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::PartTranspose(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // _M+120
        let expected = MasterTranspose {
            command: "_M".to_string(),
            sign: Some(NegativePositive::Positive),
            value: 120,
        };
        let actual = g_commands.get(11).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::MasterTranspose(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );
    }

    #[test]
    #[ignore]
    fn test_2() {
        let mml = r#";{{ 音階1 [音階2 …音階16まで] }} [音長] [ ,1音の長さ(def=%1) [ ,タイon(1,def)/off(0)
;				         [ ,gate(def=0) [ ,1loopで変化する音量±(def=0) ]]]]
;音量は変化させたら戻らない手抜き仕様です >_<

G	E1,-2,24,0 v15 q0
G	o4l8[{{eg>c<}}2. {{eg>c<}},,,2 {{fa>c<}}%108
G	o4l8 {{fa->c<}}%60 {{fa->c<}}r {{eg>c<}}%108]2
G	o5l8 {{eg>c<}}2. {{eg>c<}},,,2 {{fa>c<}}%108
G	o5l8 {{fa->c<}}%60 {{fa->c<}}r {{eg>c<}}%108
G	o5l8 {{eg>c<}}2. {{eg>c<}},,,2 {{fa>c<}}%108
G	o5l8[{{fb>d<}},,,2]3rr {{fb>d<}}r {{eg>c<}} r1
H	E2,-2,6,0 v14o2l8 [c>Q6cQ8<]56 <gggrrgr>c r1
!b	E1,-4,1,0v15q99,3P1o3c16q0
!h	E1,-3,1,0v15q99,4P2w0c16q0
!s	E1,-2,1,0v15P1o3a%1P2w10v13c16-%1
!c4	E0,-0,2,0v15P1o3e%1P2w04c4-%1
I	l16MP-128 [!b[!h]3!s[!h]3]15 !b!h!b!b!s!h!b!b
I	!c4!s[!h]3[!b[!h]3!s[!h]3]11 !sr!sr!sr !b!b!br !brr8 !br r1
"#;

        let code = Code::default();
        let mut pass1 = Pass1::new(code, mml.to_owned());
        let pass1_result = pass1.parse().unwrap();

        // moved
        let code = Code::default();
        let mut pass2 = Pass2::new(code, mml.to_owned(), pass1_result);
        let result: Pass2Result = pass2.parse().unwrap();

        assert_eq!(7, result.get_parts(&PartSymbol::G).len());
    }
}
