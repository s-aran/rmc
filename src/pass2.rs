use crate::{
    errors::Pass1Error,
    meta_models::{Code, Command, Pass1Result, Pass2Result, Token, TokenStack, VariantValue},
    models::{Comment1, Comment2, FmToneDefine, Macro, Variable},
    utils::{is_n, is_sep, split, ParseUtil},
};

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
                return Command::Comment1(self.get_code().clone());
            }
            '`' => {
                return Command::Comment2(self.get_code().clone());
            }
            '@' => {
                if self.get_code().chars == 0 {
                    return Command::FmToneDefine(self.get_code().clone());
                }
            }
            '#' => {
                if self.get_code().chars == 0 {
                    return Command::Macro(self.get_code().clone());
                }
            }
            '!' => {
                if self.get_code().chars == 0 {
                    return Command::Variable(self.get_code().clone());
                }
            }
            'A'..'Z' => {
                if self.get_code().chars == 0 {
                    //
                }
            }
            'a'..'z' => {
                if self.get_code().chars == 0 {
                    //
                }
            }
            _ => {
                eprintln!("unsupported command: {}", c);
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

    pub fn parse(&mut self) -> Result<Pass2Result, Pass1Error> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::pass1::Pass1;

    use super::*;

    #[test]
    fn test_1() {
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
        let result = pass2.parse();
    }
}
