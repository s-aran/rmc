use std::str::FromStr;

use crate::{
    errors::Pass2Error,
    meta_models::{Code, Command, Pass1Result, Pass2Result, Pass2Working},
    models::PartSymbol,
    part_command::{PartCommand, WrappedPartCommand},
    utils::{is_n, is_sep, ParseUtil},
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
        let mut commands: Vec<WrappedPartCommand> = vec![];

        let mut chars = self.mml.chars();
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
                    // println!("{:?}: {:?}", part, working.tokens);

                    // let w = WrappedPartCommand::new(self.get_code(), part_command.clone());
                    // commands.push(w);
                    let res = self.parse_part_command(&mut working, c);
                    if let Ok(r) = res {
                        match r {
                            PartCommand::Nop => {
                                break 'part_command;
                            }
                            _ => {
                                working.clear();
                                commands.push(WrappedPartCommand::new(self.get_code(), r));
                                if is_n(c) {
                                    result.parts.push((part.clone(), commands.clone()));
                                    commands.clear();
                                }

                                command = Command::Nop;
                            }
                        };
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
            }
        }

        if !working.token.is_empty() {
            working.push();
            println!("end: {:?}", working.tokens);
        }

        Ok(result)
    }

    fn parse_part_command(
        &self,
        working: &mut Pass2Working,
        c: char,
    ) -> Result<PartCommand, Pass2Error> {
        println!(
            "begin: {c}: {:?} {:?} / {}",
            working.token,
            working.tokens,
            working.tokens.first().is_some()
        );

        working.inc();

        if working.tokens.first().is_none() {
            if working.token.is_empty() && is_sep(c) {
                return Ok(PartCommand::Nop);
            }

            if working.token.is_empty() {
                working.eat(c);
            }

            let t = working.token.chars.as_str();
            match t {
                "c" | "d" | "e" | "f" | "g" | "a" | "b" => {
                    working.push();
                    return Ok(PartCommand::Nop);
                }
                "_" => {
                    if working.index != 2 {
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
                    working.push();
                    return Ok(PartCommand::Nop);
                }
                "[" => {
                    working.loop_nest += 1;
                    working.push();
                    return Ok(PartCommand::Nop);
                }
                "]" => {
                    working.loop_nest -= 1;
                    working.push();
                    return Ok(PartCommand::Nop);
                }
                ":" => {
                    // special char in loop
                    if working.loop_nest > 0 {
                        working.eat(c);
                        working.push();
                        // TODO: define pseudo command
                        return Ok(PartCommand::Nop);
                    }
                }
                _ => {
                    panic!("unknwon command: {c}");
                }
            }
        }

        match working.tokens.first().unwrap().chars.as_str() {
            "c" | "d" | "e" | "f" | "g" | "a" | "b" => {
                match c {
                    '=' => {
                        // natural, optional
                        if working.state > 0 {
                            panic!("Note: unexpected {c}");
                        }
                        working.eat(c);
                        working.push();
                        working.jump(1);
                    }
                    '+' | '-' => {
                        // semitone, optional
                        if working.state > 1 {
                            panic!("Note: unexptected {c}");
                        }
                        working.eat(c);
                        working.push();
                        working.jump(2);
                    }
                    '0'..'9' => {
                        // length, optional
                        if working.state > 3 {
                            panic!("Note: unexpected {c}");
                        }

                        working.eat(c);
                        working.jump(3);
                    }
                    '.' => {
                        // dots, optional
                        if working.state > 4 {
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
                        println!("end: {:?}", working.tokens);
                        let instance = Note::From();
                        working
                            .commands
                            .push(PartCimmand::Note(working.command_code, instance));
                        working.clear();

                        // retry
                        return self.parse_part_command(working, c);
                    }
                };
            }
            "_" | "__" => {
                match c {
                    '+' | '-' => {
                        // semitone, required
                        if working.state != 0 {
                            panic!("Absolute Transpose: unexpected {c}");
                        }

                        working.eat(c);
                        working.push();
                        working.jump(1);
                    }
                    '0'..'9' => {
                        // value
                        if working.state > 1 {
                            panic!("Absolute Transpose: unexpected {c}");
                        }

                        working.eat(c);
                        working.jump(2);
                    }
                    _ => {
                        // other command
                        working.push();
                        println!("end: {:?}", working.tokens);
                        working.clear();

                        // retry
                        return self.parse_part_command(working, c);
                    }
                }
            }
            "_{" => {
                match c {
                    '+' | '-' | '=' => {
                        // semitone|natural, required
                        if working.state > 0 {
                            panic!("Ranged Transpose: unexpected {c}");
                        }

                        working.eat(c);
                        working.push();
                        working.jump(1);
                    }
                    'c' | 'd' | 'e' | 'f' | 'g' | 'a' | 'b' => {
                        if !(working.state == 1 || working.state == 2) {
                            panic!("Ranged Transpose: unexpected {c}");
                        }

                        working.eat(c);
                        working.push();
                        working.jump(2);
                    }
                    _ => {
                        // other command
                        working.push();
                        println!("end: {:?}", working.tokens);
                        working.clear();

                        // retry
                        return self.parse_part_command(working, c);
                    }
                }
            }
            "}" => {
                working.push();
                println!("end: {:?}", working.tokens);
                working.clear();

                // retry
                return self.parse_part_command(working, c);
            }
            "_M" => {
                match c {
                    '+' | '-' => {
                        // semitone, required
                        if working.state != 0 {
                            panic!("Master Transpose: unexpected {c}");
                        }

                        working.eat(c);
                        working.push();
                        working.jump(1);
                    }
                    '0'..'9' => {
                        // value
                        if !(working.state == 1 || working.state == 2) {
                            panic!("Master Transpose: unexpected {c}");
                        }

                        working.eat(c);
                        working.jump(2);
                    }
                    _ => {
                        // other command
                        working.push();
                        println!("end: {:?}", working.tokens);
                        working.clear();

                        // retry
                        return self.parse_part_command(working, c);
                    }
                }
            }
            "[" => {
                // other command
                working.push();
                println!("end: {:?}", working.tokens);
                working.clear();

                // retry
                return self.parse_part_command(working, c);
            }
            "]" => {
                match c {
                    '0'..'9' => {
                        // loop count
                        if working.state != 0 {
                            panic!("Loop: unexpected {c}");
                        }

                        working.eat(c);
                        working.push();
                        working.jump(1);
                    }
                    _ => {
                        // other command
                        working.push();
                        println!("end: {:?}", working.tokens);
                        working.clear();

                        // retry
                        return self.parse_part_command(working, c);
                    }
                }
            }
            _ => {
                //
            }
        }

        Ok(PartCommand::Nop)
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::PartSymbol, pass1::Pass1};

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

        assert_eq!(7, result.get_parts(PartSymbol::G).len());
    }
    #[test]
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

        assert_eq!(7, result.get_parts(PartSymbol::G).len());
    }
}
