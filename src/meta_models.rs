use crate::models::{
    Comment1, Comment2, ExtendNormalOption, FmToneDefine, Macro, OnOffOption, PartSymbol,
    ReverseNormalOption, Variable,
};
use crate::part_command::{PartToken, PartTokenStack, State, WrappedPartCommand};

pub type FileName = String;
pub type LineNumber = usize;
pub type CharacterNumber = usize;

pub type CommandName = String;
pub type CommandParameter = String;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Default)]
pub struct Code {
    pub file_name: FileName,
    pub lines: LineNumber,
    pub chars: CharacterNumber,
}

impl Code {
    pub fn inc_lines(&mut self) {
        self.lines += 1;
        self.chars = 0;
    }

    pub fn inc_chars(&mut self) {
        self.chars += 1;
    }
}

#[derive(Debug, Clone)]
pub struct MetaData<T> {
    code: Code,
    data: T,
}

impl<T> MetaData<T> {
    pub fn new(code: &Code, data: T) -> Self {
        Self {
            code: code.clone(),
            data,
        }
    }

    pub fn code(&self) -> &Code {
        &self.code
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Token {
    pub begin: usize,
    pub end: usize,
    pub chars: String,
}

pub(crate) trait TokenTrait {
    fn eat(&mut self, c: char) {
        self.chars_mut().push(c);
        *self.end_mut() += 1;
    }

    fn clear(&mut self) {
        *self.begin_mut() = *self.end();
        self.chars_mut().clear();
    }

    fn chars(&self) -> &String;
    fn chars_mut(&mut self) -> &mut String;

    fn begin(&self) -> &usize;
    fn begin_mut(&mut self) -> &mut usize;
    fn end(&self) -> &usize;
    fn end_mut(&mut self) -> &mut usize;

    fn len(&self) -> usize {
        self.end() - self.begin()
    }

    fn range(&self) -> std::ops::Range<usize> {
        *self.begin()..*self.end()
    }

    fn skip(&mut self) {
        *self.begin_mut() += 1;
    }

    fn token(&self) -> String {
        self.chars().to_owned()
    }

    fn is_empty(&self) -> bool {
        self.begin() == self.end()
    }
}

impl TokenTrait for Token {
    fn chars(&self) -> &String {
        &self.chars
    }

    fn chars_mut(&mut self) -> &mut String {
        &mut self.chars
    }

    fn begin(&self) -> &usize {
        &self.begin
    }

    fn begin_mut(&mut self) -> &mut usize {
        &mut self.begin
    }

    fn end(&self) -> &usize {
        &self.end
    }

    fn end_mut(&mut self) -> &mut usize {
        &mut self.end
    }
}

impl Token {
    pub fn new() -> Self {
        Self {
            begin: 0,
            end: 0,
            chars: String::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TokenStack {
    stack: Vec<Token>,
}

impl TokenStackTrait<Token> for TokenStack {
    fn stack(&self) -> &Vec<Token> {
        &self.stack
    }

    fn stack_mut(&mut self) -> &mut Vec<Token> {
        &mut self.stack
    }
}

pub(crate) trait TokenStackTrait<T>
where
    T: Clone + TokenTrait,
{
    fn stack(&self) -> &Vec<T>;
    fn stack_mut(&mut self) -> &mut Vec<T>;

    fn push(&mut self, token: &T) {
        if !token.is_empty() {
            self.stack_mut().push(token.clone());
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack_mut().pop()
    }

    fn len(&self) -> usize {
        self.stack().len()
    }

    fn clear(&mut self) {
        self.stack_mut().clear();
    }
}

impl TokenStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
}

#[derive(Debug)]
pub enum Command {
    Nop,
    Comment1(Code), // ;
    Comment2(Code), // `
    Macro(Code),    // #
    // ===============================================================================
    // §3-2	MML変数定義
    //     !
    // -------------------------------------------------------------------------------
    // [書式1]	!文字列		MML文字列
    // [書式2]	!数値		MML文字列
    // -------------------------------------------------------------------------------
    // [文字列]	文字種・文字数は任意。先頭から半角３０文字まで判別。
    // [範囲]		0～255
    // -------------------------------------------------------------------------------
    //     MML変数を定義します。

    //     文字列定義は最大２５６種類、数値での定義も２５６種類、
    //     それぞれ独立して定義が可能です。

    //     文字列は、先頭が数字でなければ、どんな文字でも使用出来ます。
    //     ( !スネア 等の全角での指定も可能 )

    //     文字列、数値と、MML文字列の間には、必ず１つ以上のスペースまたは
    //     タブが必要です。

    //     変数のネストも可能ですが、絶対に再帰させないで下さい。

    //     変数をMML中で使用するのは、! コマンド(MML) です。

    // [注意1]	文字列の認識長は半角３０文字です。それ以上の文字数を定義した場合は、
    //     残りの文字列は無視されます。

    // [注意2]	再帰させて永久ループになった場合、最悪の場合暴走します。
    //     充分注意してください。

    // [例１]
    // !A	cde
    // !1	!A fga
    // A	l8 !1 b
    // [結果]
    // A	l8 cdefgab	と同等。

    // [例２]
    // !BassDrum	@0v12
    // !SnareDrum	@1v14
    // A	!BassDrum cc !SnareDrum g !BassDrum c
    // [結果]
    // A	@0v12 cc @1v14 g @0v12 c

    // [失敗例]
    // !A	cde !B
    // !B	cde !A
    // A	!A
    // [結果]	!Aと!Bがお互いを参照し続けるので、スタックが溢れ、最悪の場合暴走します｡

    // [関連]	! コマンド(MML) (->§16-1)
    Variable(Code), // !

    // ===============================================================================
    // *******************************************************************************
    // §6	[[[[ MMLコマンド・音色指定部 ]]]]
    // *******************************************************************************
    // ===============================================================================

    //     このセクションでは、音色番号指定及び、音色定義に関するコマンド解説を
    //     まとめています。

    // ===============================================================================
    // §6-1	音色番号指定
    //     @
    // -------------------------------------------------------------------------------
    // [書式1]	@[@] 音色番号
    // [書式2]	@[@] 音色番号[,数値1[,数値2[,数値3]]]
    // -------------------------------------------------------------------------------
    // [範囲]	音色番号	FM,PCM	0～255
    //     音色番号	SSG	0～9
    //     音色番号	SSGﾘｽﾞﾑ	0～16383
    //     数値1		PCM	-32768～+32767
    //     数値2		PCM	-32768～+32767
    //     数値3		PCM	-32768～+32767
    // -------------------------------------------------------------------------------
    // [音源]	FM / SSG / PCM / R定義
    // -------------------------------------------------------------------------------
    //     以後、指定された音色番号の音色を使用するように設定します。
    //     @@ と表記された場合は、音色番号に128が加算されます。
    //     (２つ目のPPZFile音色選択用の便宜)

    // -------------------------------------------------------------------------------
    // §6-1-1	音色番号指定/FM音源パートの場合
    // -------------------------------------------------------------------------------
    //     @ 音色番号 で使用する音色を指示します。

    // [例]	@1 cde @2 fga
    // [結果]	音色番号1 の音色で ドレミ、音色番号2 の音色で ファソラ、を演奏します。

    // [注意]	s コマンドで スロットをマスクしてある場合、そのパートでは
    //     指定されたスロットにしか音色は定義されません。
    //     s コマンドを使用した場合は、なるべく音色を再定義した方が安全です。

    // [関連]	s コマンド (->§6-2)
    //     FM音色定義(@) (->§3-1)

    // -------------------------------------------------------------------------------
    // §6-1-2	音色番号指定/SSG音源パートの場合
    // -------------------------------------------------------------------------------
    //     @ 音色番号 で、MMLコンパイラ内部に用意してある、10種類のSSGソフトウエア
    //     エンベロープからセレクトして、E コマンドとして展開されます。

    //     展開される内容は以下の通りです。
    // @0	E0,0,0,0	標準
    // @1	E2,-1,0,1	Synth type 1
    // @2	E2,-2,0,1	Synth type 2
    // @3	E2,-2,0,8	Synth type 3
    // @4	E2,-1,24,1	Piano type 1
    // @5	E2,-2,24,1	Piano type 2
    // @6	E2,-2,4,1	Glocken/Malimba type
    // @7	E2,1,0,1	Strings Type
    // @8	E1,2,0,1	Brass type 1
    // @9	E1,2,24,1	Brass type 2
    // @10	以降は指定しないで下さい。

    // [例]	@6v10l8 cegb>c<gec
    // [結果]	打楽器タイプのソフトウエアエンベロープでＣM7のアルペジオを演奏する。

    // [関連]	E コマンド (->§8-1)

    // -------------------------------------------------------------------------------
    // §6-1-3	音色番号指定/SSGリズム定義パート(PPSDRV無し)の場合
    // -------------------------------------------------------------------------------
    //     R パートでは、PMD内部に定義してあるSSGドラム音色のどれを使用するかを
    //     選択します。

    //     音色番号とリズム音の対応は以下の通りです。
    // @1	Bass Drum
    // @2	Snare Drum 1
    // @4	Low Tom
    // @8	Middle Tom
    // @16	High Tom
    // @32	Rim Shot
    // @64	Snare Drum 2
    // @128	Hi-Hat Close
    // @256	Hi-Hat Open
    // @512	Crash Cymbal
    // @1024	Ride Cymbal

    //     PMDB2/PMDVA/PMD86では、/N オプション(->PMD.DOC)が指定されていなければ、
    //     同時にリズム音源で適当な音が発音されるようになっています。
    //     その場合、各値を足し算すれば、いくつか同時に鳴っているように
    //     聞こえる風になります。（SSGリズム音は小さい番号の音色が優先されます。）

    // [例1]	@2c
    // [結果]	スネアドラムを鳴らします。

    // [例2]	@129c
    // [結果]	PMD.COMの場合は、@1のバスドラムが鳴ります。
    //     PMDB2/VA/86の場合は、上に加え、リズム音源のハイハットが同時に鳴ります。

    // -------------------------------------------------------------------------------
    // §6-1-4	音色番号指定/SSGリズム定義パート(PPSDRVあり)の場合
    // -------------------------------------------------------------------------------
    //     PDR,PPSDRVが常駐している場合は、SSGPCM音色を選択します。

    //     指定される音色番号は§6-1-3のSSGリズムの場合に加え、
    //     @2048 / @4096 / @8192 の３つが追加されます。

    //     音色番号とリズム音色の関係は、.PPS ファイルの内容によって変化します。

    //     PDRを #Double モードで再生している場合は、同時に２つまでの音を
    //     加算する事によって発音させる事が可能です。

    // [例]	@4c
    // [結果]	.PPSの３番目に定義されているSSGPCM音を発音します。

    // -------------------------------------------------------------------------------
    // §6-1-5	音色番号指定/PCM音源パートの場合
    // -------------------------------------------------------------------------------
    //     @ 音色番号 に加え、PMDB2/PMDVA/PMD86/PMDPPZでは、その後ろに、音色の
    //     リピートアドレスの設定を追加する事ができます。( ->§6-1 [書式2] )

    //     各値は、
    // 数値1	リピート開始位置
    // 数値2	リピート終了位置 (デフォルト0)
    // 数値3	リリース開始位置 (デフォルト$8000)	※PMDPPZ,PMDPPZEでは無効
    //     となっていて、-32768～+32767までの値が指定出来ます。

    //     各値とも、正の値(+)を指定した場合、音色開始アドレスから加算、
    //               負の値(-)を指定した場合、音色終了アドレスから減算
    //     されます。

    //     数値1が無指定の場合は、リピート設定されません。(ワンショットPCM)

    //     数値2が0or無指定の場合は、リピート開始位置～音色終了アドレス でのloopに
    //     なります。

    //     数値3が$8000or無指定の場合は、keyoffされても、
    //     リピート開始位置～リピート終了位置 のloopを保持します。

    //     各値は、
    //     PMD86/PMDPPZの場合	  １byte単位、
    //     PMDB2/PMDVAの場合	１６byte単位で計算されます。

    //     ただし、PMD86に/Sオプション(->PMD.DOC)を指定していた場合は、
    //     ３２byte単位で計算されます。(ADPCM 16byte = PCM 32byteに対応するため)

    //     この場合、内部で32倍しているだけなので、各数値は-1024～+1023の範囲内で
    //     指定して下さい。

    // [例]	PMDB2の場合
    //     @0の音色の大きさが4000bytesだったとすると、１６byte単位で考えると
    //     音色開始アドレス = 0　／ 音色終了アドレス = 250 という値になります。
    // J	@0,100,-50,-50 g1
    //     の場合、以下の順に再生されます。
    // 1)	先頭位置から、+200 (250 -|数値2|)の位置まで再生。
    // 2)	+100(数値1)から、+200(250 -|数値2|)の範囲がリピート再生。
    // 3)	keyoffされた瞬間に、+200(250 -|数値3|)～+250までの範囲が再生されて
    //     発声終了。

    // [補足]	YM2608(PMDB2/PMDVA)のADPCMでリピート再生する場合、リピートする瞬間、
    //     次のデータの電圧差予測値が 0 にクリアされてしまうようです。
    //     これはハードの仕様のようで、どうすることもできません。

    //     このため、リピートさせた場合、リピートした瞬間に極端に音量が小さく
    //     なってしまう現象がしばしば起こります。

    //     その場合は、リピートアドレスをいろいろ変化させてみて下さい。

    //     コツとしては、まず、目的の音を鳴らしている最中にいきなり録音開始した
    //     データを使って、リピート時に音色の頭に戻すようにすれば、リピートの際
    //     音が小さくなる現象が出なくなります。
    //     その場合は、音色の音量の時間的変化はソフトウエアエンベロープを使用して
    //     表現して下さい。

    //     さらに、PCM波形を見ながら編集できるツールをお持ちでしたら、
    //     リピート開始位置と終了位置を、電圧±０に近い位置にし、さらに電圧変化が
    //     おとなしい場所を選ぶようにすれば、リピートする瞬間のノイズが
    //     軽減されるようですのでお試し下さい。
    FmToneDefine(Code), // @

    Part(Code, PartSymbol),

    Unknown(CommandName, CommandParameter, Code),
}

impl Default for Command {
    fn default() -> Self {
        Self::Nop
    }
}

#[derive(Debug, Clone)]
pub enum VariantValue {
    Unsigned(u8),
    Signed(i8),
    UnsignedShort(u16),
    String(String),
    OnOff(OnOffOption),
    ReverseNormal(ReverseNormalOption),
    ExtendNormal(ExtendNormalOption),
}

#[derive(Default, Debug, Clone)]
pub struct Pass1Result {
    pub macros: Vec<Macro>,
    pub variables: Vec<Variable>,
    pub fm_tones: Vec<FmToneDefine>,
    pub comment1s: Vec<Comment1>,
    pub comment2s: Vec<Comment2>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Pass2Working {
    pub tokens: PartTokenStack,
    pub token: PartToken,
    pub code: Code,
    pub state: State,
    pub loop_nest: u8,
    pub commands: Vec<WrappedPartCommand>,
}

impl Pass2Working {
    pub fn eat(&mut self, c: char) {
        self.token.eat(c);
    }

    pub fn push(&mut self) {
        self.token.set_code(&self.code);
        self.token.set_state(self.state);
        self.tokens.push(&self.token);
        self.token.clear();
    }

    pub fn clear(&mut self) {
        self.tokens.clear();
        self.token.clear();
        self.state = 0;
    }

    pub fn next(&mut self) {
        self.jump(self.state + 1);
    }

    pub fn jump(&mut self, state: State) {
        self.state = state;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Pass2Result {
    pub macros: Vec<Macro>,
    pub variables: Vec<Variable>,
    pub fm_tones: Vec<FmToneDefine>,
    pub comment1s: Vec<Comment1>,
    pub comment2s: Vec<Comment2>,

    pub parts: Vec<(PartSymbol, Vec<WrappedPartCommand>)>,
}

impl Pass2Result {
    pub fn get_parts(&self, part: &PartSymbol) -> Vec<&Vec<WrappedPartCommand>> {
        self.parts
            .iter()
            .filter(|(s, _)| s == part)
            .map(|(_, l)| l)
            .collect::<Vec<&Vec<WrappedPartCommand>>>()
    }
}
