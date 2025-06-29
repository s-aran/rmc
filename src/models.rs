use std::{collections::HashMap, str::FromStr};

use strum::VariantNames;

use crate::{
    meta_models::{Code, FileName, Pass1Result, Token, TokenStack, VariantValue},
    utils::is_sep,
};

pub type MeasureType = u16;

#[derive(Debug, Clone, PartialEq, Eq, strum::EnumString, strum::VariantNames, strum::EnumIter)]
pub enum PartSymbol {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    R,
}

#[derive(Debug, Clone, PartialEq, Eq, strum::EnumString, strum::VariantNames, strum::EnumIter)]
#[allow(non_camel_case_types)]
pub enum ExtendPartSymbol {
    L,
    M,
    N,
    O,
    P,
    Q,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    a,
    b,
    c,
    d,
    e,
    f,
    g,
    h,
    i,
    j,
    k,
    l,
    m,
    n,
    o,
    p,
    q,
    r,
    s,
    t,
    u,
    v,
    w,
    x,
    y,
    z,
}

#[derive(Debug, Clone, PartialEq, Eq, strum::EnumString, strum::VariantNames, strum::EnumIter)]
pub enum InstrumentsCategorySymbol {
    F, // FM
    S, // SSG
    P, // PCM
    R, // Rhythm
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelativeAbsolute8 {
    Relative(i16),
    Absolute(u8),
}

impl From<&str> for RelativeAbsolute8 {
    fn from(value: &str) -> Self {
        if value.starts_with('+') || value.starts_with('-') {
            // NOTE: allow +128
            // #Volumedown	FR+16,P+128,S+32 ==> C0 FA 80
            // #Volumedown	FR+16,P+127,S+32 ==> C0 FA 7F
            // #Volumedown	FR+16,P128,S+32  ==> C0 FB 80
            let value = value.parse::<i16>().unwrap();
            RelativeAbsolute8::Relative(value)
        } else {
            let value = value.parse::<u8>().unwrap();
            RelativeAbsolute8::Absolute(value)
        }
    }
}

#[derive(Debug)]
pub struct Flags {
    opl_flag: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, strum::EnumString)]
pub enum OnOffOption {
    #[strum(serialize = "on")]
    On,
    #[strum(serialize = "off")]
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReverseNormalOption {
    Reverse,
    Normal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtendNormalOption {
    Extend,
    Normal,
}

// ===============================================================================
// §16-5	コメント指定
// 	;
// -------------------------------------------------------------------------------
// [書式]	;
// -------------------------------------------------------------------------------
// [音源]	FM / SSG / PCM / R選択 / R定義
// -------------------------------------------------------------------------------
// 	その行のCompileを、そこで打ち切り、残りはコメントとして無視します。
#[derive(Debug, Clone)]
pub struct Comment1 {
    pub code: Code,
    pub comment: String,
}

// ===============================================================================
// §16-6	コメント指定２
// 	`
// -------------------------------------------------------------------------------
// [書式]	`
// -------------------------------------------------------------------------------
// [音源]	FM / SSG / PCM / R選択 / R定義 または 行の頭
// -------------------------------------------------------------------------------
// 	１度指定されたら、もう１度指定するまで、その間のＭＭＬを
// 	総てコメントとし、無視します。
//
// 	行の頭に表記した場合は、総てのパートでコメントかどうかのフラグが
// 	反転されます。
// 	また、間にある # @ ! 各種定義命令も無視されます。
//
// 	必ず、MML中か、行頭にある必要があります。
//
// 	MML中にドキュメント等を書きたい場合や、
// 	MML行の中にコメントを表記したい場合に使用すると便利です。
#[derive(Debug, Clone)]
pub struct Comment2 {
    pub code: Code,
    pub comment: String,
}

// ====================================================================
// *******************************************************************************
// §2	[[[[ 全体制御コマンド ]]]]
// *******************************************************************************
// ===============================================================================
//
// 	以下のコマンドは、行頭に指定し、全パートに影響する制御や
// 	曲自体の各種定義を行います。
//
// 	"Filename" "Composer" "Extend" 等のアルファベット文字列は、
// 	小文字でも大文字でも構いません。
//
// 	コマンド名と文字列及び数値は、１つ以上の SPACE 又は TAB で区切られて
// 	いる必要があります。
//
// 	1PASS目に処理されるため、MMLファイルのどの位置にあっても構いませんが、
// 	#Memo以外のコマンドを重複指定した場合は、後ろの行にあるものが有効と
// 	なります。
#[derive(Debug, Clone)]
pub struct Macro {
    pub code: Code,
    pub key: String,
    pub value: VariantValue,
}

// ===============================================================================
// §3-1	FM音色定義
// 	@
// -------------------------------------------------------------------------------
// [書式1]	@ 音色番号 ＡＬＧ ＦＢ
// 	 ＡＲ ＤＲ ＳＲ ＲＲ ＳＬ ＴＬ ＫＳ ＭＬ ＤＴ ＡＭＳ
// 	 ＡＲ ＤＲ ＳＲ ＲＲ ＳＬ ＴＬ ＫＳ ＭＬ ＤＴ ＡＭＳ
// 	 ＡＲ ＤＲ ＳＲ ＲＲ ＳＬ ＴＬ ＫＳ ＭＬ ＤＴ ＡＭＳ
// 	 ＡＲ ＤＲ ＳＲ ＲＲ ＳＬ ＴＬ ＫＳ ＭＬ ＤＴ ＡＭＳ
// [書式2]	@ 音色番号 ＡＬＧ ＦＢ
// 	 ＡＲ ＤＲ ＳＲ ＲＲ ＳＬ ＴＬ ＫＳ ＭＬ ＤＴ ＤＴ２ ＡＭＳ
// 	 ＡＲ ＤＲ ＳＲ ＲＲ ＳＬ ＴＬ ＫＳ ＭＬ ＤＴ ＤＴ２ ＡＭＳ
// 	 ＡＲ ＤＲ ＳＲ ＲＲ ＳＬ ＴＬ ＫＳ ＭＬ ＤＴ ＤＴ２ ＡＭＳ
// 	 ＡＲ ＤＲ ＳＲ ＲＲ ＳＬ ＴＬ ＫＳ ＭＬ ＤＴ ＤＴ２ ＡＭＳ
// [書式3]	@ 音色番号 ＡＬＧ ＦＢ
// 	 ＡＲ ＤＲ ＲＲ ＳＬ ＴＬ ＫＳＬ ＭＬ ＫＳＲ ＥＧＴ ＶＩＢ ＡＭ
// 	 ＡＲ ＤＲ ＲＲ ＳＬ ＴＬ ＫＳＬ ＭＬ ＫＳＲ ＥＧＴ ＶＩＢ ＡＭ
// -------------------------------------------------------------------------------
// [備考]	任意の位置に、= 音色名 を表記する事が出来る。
// -------------------------------------------------------------------------------
// [範囲]	[書式1および2]	ＡＬＧ	････	0～7
// 			ＦＢ	････	0～7
// 			ＡＲ	････	0～31
// 			ＤＲ	････	0～31
// 			ＳＲ	････	0～31
// 			ＲＲ	････	0～15
// 			ＳＬ	････	0～15
// 			ＴＬ	････	0～127
// 			ＫＳ	････	0～3
// 			ＭＬ	････	0～15
// 			ＤＴ	････	-3～3 または 0～7
// 			ＡＭＳ	････	0～1
// 	[書式2]		ＤＴ２	････	0～3
// 	[書式3]		ＡＬＧ	････	0～1
// 			ＦＢ	････	0～7
// 			ＡＲ	････	0～15
// 			ＤＲ	････	0～15
// 			ＲＲ	････	0～15
// 			ＳＬ	････	0～15
// 			ＴＬ	････	0～63
// 			ＫＳＬ	････	0～3
// 			ＭＬ	････	0～15
// 			ＫＳＲ	････	0～1
// 			ＥＧＴ	････	0～1
// 			ＶＩＢ	････	0～1
// 			ＡＭ	････	0～1
// -------------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FmToneDefine {
    pub code: Code,

    pub tone_number: u8,
    pub algorism: u8, // 0..7
    pub feedback: u8, // 0..7

    pub name: Option<String>, // 任意の位置に、= 音色名 を表記する事が出来る。
}

#[derive(Debug, PartialEq, Eq)]
pub struct FmToneDefineDetails {
    code: Code,
    name: String,
    // 0～255
    value: u8,
}

// ===============================================================================
// §3-2	MML変数定義
// 	!
// -------------------------------------------------------------------------------
// [書式1]	!文字列		MML文字列
// [書式2]	!数値		MML文字列
// -------------------------------------------------------------------------------
// [文字列]	文字種・文字数は任意。先頭から半角３０文字まで判別。
// [範囲]		0～255
// -------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Variable {
    pub code: Code,
    pub name: String,
    pub value: String,
}

// ===============================================================================
// §2-1	出力ファイル名指定
// 	#Filename
// -------------------------------------------------------------------------------
// [書式1]	#Filename	ファイル名
// [書式2]	#Filename	.拡張子
// -------------------------------------------------------------------------------
// 	MC.EXEの出力するコンパイル後の曲データのファイル名を変更します。
//
// 	デフォルトでは、MMLのファイル名の拡張子を".M"に変更したものに
// 	なります。
//
// 	拡張子のみ指定されていた場合は、拡張子のみ、デフォルトの".M"から
// 	変更します。
//
// 	ファイル名はTAB/ESC以外のCTRLコードが来るまで定義されます。
// 	(通常は行の終わり(CR)迄です。)
//
// [注意]	後ろに ; をつけてコメント等は記せません。
#[derive(Debug, Clone)]
pub struct FileNameMacro {
    pub code: Code,
    pub value: FileName,
}

impl From<Macro> for FileNameMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::String(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Filename");
    }
}

// ===============================================================================
// §2-2	使用SSGPCMファイル名指定
// 	#PPSFile
// -------------------------------------------------------------------------------
// [書式]	#PPSFile	ファイル名
// -------------------------------------------------------------------------------
// 	使用するPPS(SSGPCM)File名を定義します。(->PPS.DOC)
//
// 	ファイル名はTAB/ESC以外のCTRLコードが来るまで定義されます。
// 	(通常は行の終わり(CR)迄です。)
//
// [注意1]	後ろに ; をつけてコメント等は記せません。
//
// [注意2]	拡張子は省略出来ません。
//
// [注意3]	MC.EXEを使用する際は、音色データを曲データ中に定義するように(->MC.DOC)
// 	しないと、MC.EXEの /P または /S オプションで即演奏した場合にしか
// 	ファイルが読み込まれません。
#[derive(Debug, Clone)]
pub struct PpsFileMacro {
    pub code: Code,
    pub value: FileName,
}

impl From<Macro> for PpsFileMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::String(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Ppsfile");
    }
}

// ===============================================================================
// §2-3	使用PCMファイル名指定
// 	#PCMFile , #PPCFile
// -------------------------------------------------------------------------------
// [書式1]	#PCMFile	ファイル名
// [書式2]	#PPCFile	ファイル名
// -------------------------------------------------------------------------------
// 	Jパートで使用するPCM(.PPC/.PVI/.P86)ファイル名を定義します。
// 	(->PMDPCM.DOC)
// 	PMDPPZEを使用する場合は .PVI/.PZI ファイル名を指示します。
//
// 	#PCMFileと#PPCFileは全く同じです。
//
// 	ファイル名はTAB/ESC以外のCTRLコードが来るまで定義されます。
// 	(通常は行の終わり(CR)迄です。)
//
// [注意1]	後ろに ; をつけてコメント等は記せません。
//
// [注意2]	拡張子は省略出来ません。
//
// [注意3]	MC.EXEを使用する際は、音色データを曲データ中に定義するように(->MC.DOC)
// 	しないと、MC.EXEの /P または /S オプションで即演奏した場合にしか
// 	ファイルが読み込まれません。
#[derive(Debug, Clone)]
pub struct PcmFileMacro {
    pub code: Code,
    pub value: FileName,
}

impl From<Macro> for PcmFileMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::String(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Pcmfile");
    }
}

// ===============================================================================
// §2-4	使用音色ファイル名指定
// 	#FFFile
// -------------------------------------------------------------------------------
// [書式]	#FFFile		ファイル名[.FF/.FFL]
// -------------------------------------------------------------------------------
// 	使用する音色(.FF/.FFL)ファイル名を定義します。
//
// 	ファイル名はTAB/ESC以外のCTRLコードが来るまで定義されます。
// 	(通常は行の終わり(CR)迄です。)
//
// 	MC.EXEのコマンドラインに音色ファイル名を書いた場合(->MC.DOC)と全く
// 	同じ動作をし、
// 	ファイルが存在する場合は、同時に /V オプションも自動付加され、
// 	ファイルが存在しない場合は、/VWオプションの効果で書き込む際に、
// 	指定したファイル名が使用されます。
//
// 	拡張子を省略した場合、OPLオプション(/L)がついている場合は .FFL、
// 	ついていない場合は .FF になります。
//
// [注意1]	音色定義コマンド(@)と併用する場合は、必ずFFFile -> 音色定義の順に
// 	表記して下さい。
// 	後ろに書くと定義した音色に上書きしてFILEを読み込んでしまう
// 	可能性があります。
//
// [注意2]	もしコマンドラインで音色を指定していても、このコマンドが指定されている
// 	場合はこちらが有効となります。
#[derive(Debug, Clone)]
pub struct FfFileMacro {
    pub code: Code,
    pub value: FileName,
}

impl From<Macro> for FfFileMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::String(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Fffile");
    }
}

// ===============================================================================
// §2-5	コンパイラオプション設定
// 	#Option
// -------------------------------------------------------------------------------
// [書式]	#Option		文字列
// -------------------------------------------------------------------------------
// 	コンパイラのオプションを設定します。
//
// 	指定する文字列は、コマンドラインに表記するものと全く同一です。
// 	(->MC.DOC)
//
// [注意1]	コマンドラインに設定したオプションに、付加される形で追加されます。
// 	/N と /L といった、同時に設定出来ないオプションがかち合った場合は、
// 	後に設定したものが有効になります。
//
// [注意2]	/N、/M、/Lといった、ＦＭ音色定義に関係するオプションを設定する場合は、
// 	必ず音色定義(@)前に設定して下さい。
#[derive(Debug, Clone)]
pub struct OptionMacro {
    pub code: Code,
    pub values: Vec<String>,
}

impl From<Macro> for OptionMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::String(options) = m.value {
            // TODO: panic when unknwon or invalid option

            let values = options
                .split("/")
                .map(|e| e.trim().to_string())
                .filter(|e| e.len() > 0)
                .collect::<Vec<String>>();

            return Self {
                code: m.code,
                values,
            };
        }

        panic!("Option");
    }
}

// ===============================================================================
// §2-6	タイトル定義
// 	#Title
// -------------------------------------------------------------------------------
// [書式]	#Title		タイトル文字列
// -------------------------------------------------------------------------------
// 	曲のタイトルを定義します。
//
// 	文字列はTAB/ESC以外のCTRLコードが来るまで定義されます。
// 	(通常は行の終わり(CR)迄です。)
//
// [注意1]	後ろに ; をつけてコメント等は記せません。
//
// [注意2]	MC.EXEを使用する際は、音色データを曲データ中に定義するように(->MC.DOC)
// 	しないと、MC.EXEの /P または /S オプションで即演奏した場合にしか
// 	この文字列は表示されません。
#[derive(Debug, Clone)]
pub struct TitleMacro {
    pub code: Code,
    pub value: String,
}

impl From<Macro> for TitleMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::String(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Title");
    }
}

//===============================================================================
//§2-7	作曲者定義
//	#Composer
//-------------------------------------------------------------------------------
//[書式]	#Composer	作曲者名文字列
//-------------------------------------------------------------------------------
//	作曲者名を定義します。
//
//	デフォルトでは、環境変数 "COMPOSER=" または "USER=" が定義されている
//	場合は、定義されている文字列を使用します。(->MC.DOC)
//
//	文字列はTAB/ESC以外のCTRLコードが来るまで定義されます。
//	(通常は行の終わり(CR)迄です。)
//
//[注意1]	後ろに ; をつけてコメント等は記せません。
//
//[注意2]	MC.EXEを使用する際は、音色データを曲データ中に定義するように(->MC.DOC)
//	しないと、MC.EXEの /P または /S オプションで即演奏した場合にしか
//	この文字列は表示されません。
#[derive(Debug, Clone)]
pub struct ComposerMacro {
    pub code: Code,
    pub value: String,
}

impl From<Macro> for ComposerMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::String(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Composer");
    }
}

// ===============================================================================
// §2-8	編曲者定義
// 	#Arranger
// -------------------------------------------------------------------------------
// [書式]	#Arranger	編曲者名文字列
// -------------------------------------------------------------------------------
// 	編曲者名を定義します。
//
// 	デフォルトでは、環境変数 "ARRANGER=" または "USER=" が定義されている
// 	場合は、定義されている文字列を使用します。(->MC.DOC)
//
// 	文字列はTAB/ESC以外のCTRLコードが来るまで定義されます。
// 	(通常は行の終わり(CR)迄です。)
//
// [注意1]	後ろに ; をつけてコメント等は記せません。
//
// [注意2]	MC.EXEを使用する際は、音色データを曲データ中に定義するように(->MC.DOC)
// 	しないと、MC.EXEの /P または /S オプションで即演奏した場合にしか
// 	この文字列は表示されません。
#[derive(Debug, Clone)]
pub struct ArrangerMacro {
    pub code: Code,
    pub value: String,
}

impl From<Macro> for ArrangerMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::String(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Arranger");
    }
}

// ===============================================================================
// §2-9	メモ定義
// 	#Memo
// -------------------------------------------------------------------------------
// [書式]	#Memo		メモ文字列
// -------------------------------------------------------------------------------
// 	メモ文字列を定義します。
//
// 	複数指定が可能で、順に定義されます。最大は 128行 までです。
//
// [注意1]	後ろに ; をつけてコメント等は記せません。
//
// [注意2]	MC.EXEを使用する際は、音色データを曲データ中に定義するように(->MC.DOC)
// 	しないと、MC.EXEの /P または /S オプションで即演奏した場合にしか
// 	この文字列は表示されません。
#[derive(Debug, Clone)]
pub struct MemoMacro {
    pub code: Code,
    pub value: String,
}

impl From<Macro> for MemoMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::String(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Memo");
    }
}

// ===============================================================================
// §2-10	テンポ指定
// 	#Tempo #Timer
// -------------------------------------------------------------------------------
// [書式1]	#Tempo		数値
// [書式2]	#Timer		数値
// -------------------------------------------------------------------------------
// [範囲]	[書式1]		18～255
// 	[書式2]		0～250
// -------------------------------------------------------------------------------
// 	テンポを指定します。
//
// 	#Tempoの場合、内部クロック 48 が１分間に何回になるかを指定します。
// 	内部クロック 48 は、デフォルトでは２分音符ですが、#Zenlen コマンド
// 	または C コマンドで変更されます。
//
// 	#Timerの場合、TimerB が いくつになるかを直接指定します。
//
// 	このコマンドを指定するとパート G の頭に t または T コマンドを自動的に
// 	発行します。
#[derive(Debug, Clone)]
pub struct TempoMacro {
    pub code: Code,
    pub value: u8,
}

impl From<Macro> for TempoMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::Unsigned(value) = m.value {
            // TODO: 255 or 250
            // if value < 18 {
            //     panic!("Tempo value is too large");
            // }

            return Self {
                code: m.code,
                value: value as u8,
            };
        };

        panic!("Tempo");
    }
}

// ===============================================================================
// §2-11	全音符長指定
// 	#Zenlen
// -------------------------------------------------------------------------------
// [書式]	#Zenlen		数値
// -------------------------------------------------------------------------------
// [範囲]	1～255
// -------------------------------------------------------------------------------
// 	全音符長を指定します。
//
// 	MMLのCコマンドと同様で、96以外の値を指定するとパート G の頭に
// 	C コマンドを自動的に発行します。
//
// 	指定するすべての音符長は、この数値の約数でなくてはなりません。
//
// 	デフォルトは 96 ですので、使用出来る音符長は、
// 	1,2,3,4,6,8,12,16,24,32,48,96
// 	になります。
//
// [注意]	変更した場合、テンポの値が、１分間にどの音符がテンポ指定値になるかが
// 	変化します。
#[derive(Debug, Clone)]
pub struct ZenLenMacro {
    pub code: Code,
    pub value: u8,
}

impl From<Macro> for ZenLenMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::Unsigned(value) = m.value {
            if value < 1 {
                panic!("Zenlen value is too small");
            }

            return Self {
                code: m.code,
                value: value,
            };
        };

        panic!("Zenlen");
    }
}

// ===============================================================================
// §2-12	オクターブ上下記号の機能指定
// 	#Octave
// -------------------------------------------------------------------------------
// [書式1]	#Octave		Reverse
// [書式2]	#Octave		Normal
// -------------------------------------------------------------------------------
// 	>、< コマンドの役割をreverseで反対に、normalで標準にします。
//
// 	MMLのXコマンドと同様で、指定するとパート A の頭に X コマンドを
// 	自動的に発行します。
#[derive(Clone, Debug)]
pub struct OctaveMacro {
    pub code: Code,
    pub value: ReverseNormalOption,
}

impl From<Macro> for OctaveMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::ReverseNormal(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Octave");
    }
}

// ===============================================================================
// §2-13	デフォルトループ回数の設定
// 	#LoopDefault
// -------------------------------------------------------------------------------
// [書式]	#LoopDefault	数値
// -------------------------------------------------------------------------------
// [範囲]	0～255
// -------------------------------------------------------------------------------
// 	ループ終了コマンド ] で、数値を省略した場合に指定される値を
// 	設定します。
//
// 	デフォルトは０（無限ループ）です。
#[derive(Debug, Clone)]
pub struct LoopDefaultMacro {
    pub code: Code,
    pub value: u8,
}

impl From<Macro> for LoopDefaultMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::Unsigned(value) = m.value {
            return Self {
                code: m.code,
                value: value,
            };
        };

        panic!("LoopDefault");
    }
}

// ===============================================================================
// §2-14	音色データにDT2を含めるかどうかの設定
// 	#DT2Flag
// -------------------------------------------------------------------------------
// [書式1]	#DT2Flag	on
// [書式2]	#DT2Flag	off
// -------------------------------------------------------------------------------
// 	音色データフォーマットにDT2が含まれるかどうかを設定します。
//
// 	デフォルトは、MCのオプションによって変化し、
// 	/m が指定されていた場合		-> on
// 	/m が指定されていない場合	-> off
// 	となります。
//
// 	DT2が必要とされる音源で、このフラグがoffで指定されていない場合は、
// 	DT2は全スロット 0 となります。
#[derive(Debug, Clone)]
pub struct Dt2FlagMacro {
    pub code: Code,
    pub value: OnOffOption,
}

impl From<Macro> for Dt2FlagMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::OnOff(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Dt2Flag");
    }
}

// ===============================================================================
// §2-15	ベンドレンジ設定
// 	#Bendrange
// -------------------------------------------------------------------------------
// [書式]	#Bendrange	数値
// -------------------------------------------------------------------------------
// [範囲]	0～255
// -------------------------------------------------------------------------------
// 	ベンド幅を設定します。デフォルトは0です。
//
// 	MMLの B コマンドと同様で、指定するとパート A の頭に B コマンドを
// 	自動的に発行します。
//
// 	0以外の値を指定すると、I コマンドが有効になり、
// 	I±8192 でベンド幅 × 半音分ずれるようになります。
//
// [注意]	ベンド指定は、I コマンドに関するかなりの注意(->§7-5)を要する上、
// 	MMLが見づらくなることもあり、MIDIからのコンバート以外では使用しない
// 	方が無難です。
#[derive(Debug, Clone)]
pub struct BendRangeMacro {
    pub code: Code,
    pub value: u8,
}

impl From<Macro> for BendRangeMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::Unsigned(value) = m.value {
            return Self {
                code: m.code,
                value: value as u8,
            };
        };

        panic!("Bendrange")
    }
}

// ===============================================================================
// §2-16	SSG音程 拡張/ノーマル選択
// 	#Detune
// -------------------------------------------------------------------------------
// [書式1]	#Detune		Extend
// [書式2]	#Detune		Normal
// -------------------------------------------------------------------------------
// 	SSGのDetune/LFOを拡張仕様にするかノーマル仕様にするかを選択します。
//
// 	Extendにした場合、SSGパート(G,H,I)の頭全てに"DX1"コマンドを指定した
// 	のと同じになります。
//
// 	ノーマル仕様の場合、DETUNE/LFOは1ずれると、音源に送る音程値も必ず
// 	1ずれるようになります。(同じDETUNE値でも高音域になるとずれが大きくなる)
//
// 	拡張仕様の場合は、高音域にいけばいくほどDETUNE値が小さくなるようにし、
// 	同じDETUNE値ならどの音程でも同程度ずれるように補正します。
#[derive(Debug, Clone)]
pub struct DetuneMacro {
    pub code: Code,
    pub value: ExtendNormalOption,
}

impl From<Macro> for DetuneMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::ExtendNormal(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Detune");
    }
}

// ===============================================================================
// §2-17	ソフトウエアLFO速度 拡張/ノーマル選択
// 	#LFOSpeed
// -------------------------------------------------------------------------------
// [書式1]	#LFOSpeed	Extend
// [書式2]	#LFOSpeed	Normal
// -------------------------------------------------------------------------------
// 	ソフトウエアLFOの速度をテンポ依存しない拡張仕様にするかどうかを
// 	選択します。
//
// 	Extendにした場合、FM,SSG,ADPCMパート(A～J)の頭全てに "MXA1 MXB1"
// 	とコマンドを指定したのと同じになります。
//
// 	２つあるＬＦＯのうち、片方のみ変更したいといった場合は、MXコマンドを
// 	使用して下さい。
#[derive(Debug, Clone)]
pub struct LfoSpeedMacro {
    pub code: Code,
    pub value: ExtendNormalOption,
}

impl From<Macro> for LfoSpeedMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::ExtendNormal(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Lfospeed");
    }
}

// ===============================================================================
// §2-18	ソフトウエアエンベロープ速度 拡張/ノーマル選択
// 	#EnvelopeSpeed
// -------------------------------------------------------------------------------
// [書式1]	#EnvelopeSpeed	Extend
// [書式2]	#EnvelopeSpeed	Normal
// -------------------------------------------------------------------------------
// 	SSG/PCMのソフトウエアエンベロープの速度をテンポ依存しない拡張仕様に
// 	するかどうかを選択します。
//
// 	Extendにした場合、SSG,PCMパート(G～J)の頭全てに "EX1"コマンドを指定
// 	したのと同じになります。
#[derive(Debug, Clone)]
pub struct EnvelopeSpeedMacro {
    pub code: Code,
    pub value: ExtendNormalOption,
}

impl From<Macro> for EnvelopeSpeedMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::ExtendNormal(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Envelopespeed");
    }
}

// ===============================================================================
// §2-19	PCM音量値 拡張/ノーマル選択
// 	#PCMVolume
// -------------------------------------------------------------------------------
// [書式1]	#PCMVolume	Extend
// [書式2]	#PCMVolume	Normal
// -------------------------------------------------------------------------------
// 	PCMパートの"v"値->"V"値の変換方法を、
// 	Normal:	V = v×16
// 	Extend: V = v×v
// 	のどちらにするかを選択します。
//
// 	PMDB2,PMDVAでは Extend、PMD86では Normalにすると、"v"値での音量カーブ
// 	が、ほぼ直線に近くなるようです。
#[derive(Debug, Clone)]
pub struct PcmVolumeMacro {
    pub code: Code,
    pub value: ExtendNormalOption,
}

impl From<Macro> for PcmVolumeMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::ExtendNormal(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Pcmvolume");
    }
}

// ===============================================================================
// §2-20	FM音源３チャネル目のパート拡張
// 	#FM3Extend
// -------------------------------------------------------------------------------
// [書式]	#FM3Extend	パート記号１[パート記号２[パート記号３]]]
// -------------------------------------------------------------------------------
// [記号]	LMNOPQSTUVWXYZabcdefghijklmnopqrstuvwxyz のうちのいずれか
// -------------------------------------------------------------------------------
// 	FM音源3 のパートを、指定したパート記号で拡張します。
// 	最大３ｃｈ分設定可能です。
//
// 	FM音源3チャネル目は、独立して最大４つまでのパートを演奏する事が
// 	可能ですが、デフォルトでは PMD/PMDVA1 を除き、１パートしか定義されて
// 	いませんので、このコマンドで新たにパート記号を定義します。
//
// 	PMD/PMDVA1 では、D E F パートがFM3chの拡張パートとしてデフォルトで
// 	定義されていますが、このコマンドはそれを変更します。
#[derive(Debug, Clone)]
pub struct Fm3ExtendMacro {
    pub code: Code,
    pub value: Vec<ExtendPartSymbol>,
}

impl From<Macro> for Fm3ExtendMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::String(parts_str) = m.value {
            let value = parts_str
                .chars()
                .map(|c| c.to_string())
                .map(|s| {
                    if ExtendPartSymbol::VARIANTS.contains(&s.as_str()) {
                        println!("{}", s);
                        ExtendPartSymbol::from_str(s.as_str()).unwrap()
                    } else {
                        panic!("Fm3Extend: invalid part symbol: {}", s);
                    }
                })
                .collect::<Vec<ExtendPartSymbol>>();

            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Fm3extend");
    }
}

// ===============================================================================
// §2-21	MMLファイルの挿入
// 	#Include
// -------------------------------------------------------------------------------
// [書式]	#Include	ファイル名
// -------------------------------------------------------------------------------
// 	MMLの#Include行～次の行の間にMMLファイルを読み込んで追加します。
// 	拡張子は省略出来ません。
//
// 	デフォルトの #命令、@音色、!変数などを定義したMMLを外部から読み込む
// 	と非常に便利です。ただし、MMLの長さ制限（61KB）には充分注意して下さい。
//
// 	INCLUDEファイル中にINCLUDEファイルを読み込む(ネストする)事も可能です。
#[derive(Debug, Clone)]
pub struct IncludeMacro {
    pub code: Code,
}

impl From<Macro> for IncludeMacro {
    fn from(m: Macro) -> Self {
        todo!();
    }
}

// ===============================================================================
// §2-22	音源別音量ダウン設定
// 	#Volumedown
// -------------------------------------------------------------------------------
// [書式]	#Volumedown	[F[S[P[R]]]][±]数値[,[F[S[P[R]]]][±]数値,]････
// -------------------------------------------------------------------------------
// [範囲]	[±がない場合]	0～255
// 	[±がある場合]	-128～+127
// -------------------------------------------------------------------------------
// 	音源別音量ダウン値を変更します。
//
// 	F がＦＭ音源、
// 	S がＳＳＧ音源、
// 	P がＰＣＭ音源、
// 	R がリズム音源をそれぞれ表します。
//
// 	設定された場合、Gパートの頭に自動的に DF,DS,DP,DRコマンドが
// 	発行される形になります。
//
// 	数値の前に + - が付いた場合は、PMDの/DF,/DS,/DP,/DRオプションの値から
// 	相対的に変化されます。
// 	+で音量が下がる点に注意して下さい。
//
// [注意]	ここで設定される音量ダウン値は、その曲のみに有効の局所的な値であり、
// 	再度演奏開始する際に、PMDの/DF,/DS,/DP,/DRオプションで指定した値が
// 	再設定され、元に戻ります。
#[derive(Clone, Debug)]
pub struct VolumeDownMacro {
    pub code: Code,
    pub value: Vec<(InstrumentsCategorySymbol, RelativeAbsolute8)>,
}

impl From<Macro> for VolumeDownMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::String(value) = m.value {
            let mut tokens = TokenStack::new();
            {
                let mut token = Token::new();
                let mut current_inst_symbol = false;

                for c in value.chars() {
                    if c == ',' {
                        tokens.push(&token);
                        token.clear();

                        current_inst_symbol = false;
                        continue;
                    }

                    if ('0'..('9' as u8 + 1) as char).contains(&c) || c == '+' || c == '-' {
                        if current_inst_symbol {
                            tokens.push(&token);
                            token.clear();
                        }

                        current_inst_symbol = false;

                        token.eat(c);

                        continue;
                    }

                    if InstrumentsCategorySymbol::VARIANTS.contains(&c.to_string().as_str()) {
                        tokens.push(&token);
                        token.clear();

                        current_inst_symbol = true;
                        token.eat(c);
                        continue;
                    }
                }

                if !token.is_empty() {
                    tokens.push(&token);
                }
            }

            let mut values = vec![];
            {
                let mut value = if let Some(t) = tokens.pop() {
                    Self::token_to_relative_absolue_8(&t.chars.as_str()).unwrap()
                } else {
                    panic!("VolumeDown: no parameters");
                };

                while tokens.len() > 0 {
                    let poped = tokens.pop().unwrap();
                    let s = poped.chars.as_str();
                    if InstrumentsCategorySymbol::VARIANTS.contains(&s) {
                        values.push((
                            InstrumentsCategorySymbol::from_str(s).unwrap(),
                            value.clone(),
                        ));
                        continue;
                    }

                    value = Self::token_to_relative_absolue_8(s).unwrap();
                }
            }

            values.reverse();

            return Self {
                code: m.code,
                value: values,
            };
        };

        panic!("Volumedown");
    }
}

impl VolumeDownMacro {
    fn token_to_relative_absolue_8(value: &str) -> Result<RelativeAbsolute8, ()> {
        match RelativeAbsolute8::try_from(value) {
            Ok(v) => Ok(v),
            Err(_) => panic!("VolumeDown: invalid value {}", value),
        }
    }
}

// ===============================================================================
// §2-23	PCMパートの仕様設定
// 	#ADPCM
// -------------------------------------------------------------------------------
// [書式1]	#ADPCM		on
// [書式2]	#ADPCM		off
// -------------------------------------------------------------------------------
// 	PMD86で演奏した際にのみ有効で、
//
// 	onの場合	ADPCMに音量・ループを合わせる	(/s オプションあり)
// 	offの場合	ADPCMに音量・ループを合わせない	(/s オプション無し)
//
// 	とします。
//
// 	/sオプションの状態に関らず、状態をそのように変更します。
//
// 	その曲のみに有効な、局所的な設定で、他の曲を演奏する際には
// 	元に戻ります。
//
// 	このコマンドが使用されると、Jパートの頭に、A コマンドを発行します。
#[derive(Debug, Clone)]
pub struct AdpcmMacro {
    pub code: Code,
    pub value: OnOffOption,
}

impl From<Macro> for AdpcmMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::OnOff(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Adpcm");
    }
}

// ===============================================================================
// §2-24	演奏開始小節指定
// 	#Jump
// -------------------------------------------------------------------------------
// [書式1]	#Jump		小節番号
// -------------------------------------------------------------------------------
// [範囲]	0～65535
// -------------------------------------------------------------------------------
// 	MCH.EXE または MC.EXEの /P,/S オプションで演奏を開始する場合のみに
// 	有効で、演奏を開始する小節番号を指定します。
//
// 	作成される .M ファイルには影響を及ぼさないので、PMP.COM 等で再演奏
// 	させた場合は、曲の頭から再生されますので注意して下さい。
//
// 	また、CPUが遅い機種で、大きな小節番号を指定した場合は、演奏開始まで
// 	しばらく時間がかかる事があります。
#[derive(Debug, Clone)]
pub struct JumpMacro {
    pub code: Code,
    pub value: MeasureType,
}

impl From<Macro> for JumpMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::UnsignedShort(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Jump");
    }
}

// ===============================================================================
// §2-25	PPZ8用パート拡張
// 	#PPZExtend
// -------------------------------------------------------------------------------
// [書式]	#PPZExtend	パート記号１[パート記号２[パート記号３]････(８つまで)]]
// -------------------------------------------------------------------------------
// [記号]	LMNOPQSTUVWXYZabcdefghijklmnopqrstuvwxyz のうちのいずれか
// -------------------------------------------------------------------------------
// 	PPZ8用のパートを、指定したパート記号で拡張します。
// 	最大８ｃｈ分設定可能です。
#[derive(Debug, Clone)]
pub struct PpzExtendMacro {
    pub code: Code,
    pub value: Vec<ExtendPartSymbol>,
}

impl From<Macro> for PpzExtendMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::String(parts_str) = m.value {
            let value = parts_str
                .chars()
                .map(|c| c.to_string())
                .map(|s| {
                    if ExtendPartSymbol::VARIANTS.contains(&s.as_str()) {
                        ExtendPartSymbol::from_str(s.as_str()).unwrap()
                    } else {
                        panic!("PpzExtend: invalid part symbol: {}", s);
                    }
                })
                .collect::<Vec<ExtendPartSymbol>>();

            if value.len() > 8 {
                panic!("Ppzextend: too many part symbols");
            }

            return Self {
                code: m.code,
                value,
            };
        };

        panic!("PpzExtend");
    }
}

// ===============================================================================
// §2-26	使用PPZ用PCMファイル名指定
// 	#PPZFile
// -------------------------------------------------------------------------------
// [書式]	#PPZFile	ファイル名[.PVI/.PZI][,ファイル名[.PVI/.PZI]]
// -------------------------------------------------------------------------------
// 	PPZ拡張パートで使用するPCM(.PVI/.PZI)ファイル名を定義します。
// 	拡張子が省略された場合、通常は読み込み時にPZI->PVIの順で検索されます。
//
// 	ファイルは２つまで定義できます。
// 	２つ目に定義したPCMファイルは、音色番号に 128が加算され、
// 	@128～@255 に定義されます。
//
// 	ファイル名はTAB/ESC以外のCTRLコードが来るまで定義されます。
// 	(通常は行の終わり(CR)迄です。)
//
// [注意1]	後ろに ; をつけてコメント等は記せません。
//
// [注意2]	２つ目のPCMファイルを定義する際に、カンマ [,] 記号の左右に
// 	スペース、タブなどは入れられません。
//
// [注意3]	MC.EXEを使用する際は、音色データを曲データ中に定義するように(->MC.DOC)
// 	しないと、MC.EXEの /P または /S オプションで即演奏した場合にしか
// 	ファイルが読み込まれません。
#[derive(Debug, Clone)]
pub struct PpzFileMacro {
    pub code: Code,
    pub value: Vec<FileName>,
}

impl From<Macro> for PpzFileMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::String(value) = m.value {
            let mut tokens = TokenStack::new();
            let mut token = Token::new();

            for c in value.chars() {
                if is_sep(c) && (tokens.len() > 0 || !token.is_empty()) {
                    panic!("PpzFile");
                }

                if c == ',' {
                    tokens.push(&token);
                    token.clear();
                    continue;
                }

                token.eat(c);
            }

            if !token.is_empty() {
                tokens.push(&token);
                token.clear();
            }

            let mut files = vec![];
            while tokens.len() > 0 {
                files.push(tokens.pop().unwrap().chars.to_owned());
            }

            files.reverse();

            return Self {
                code: m.code,
                value: files,
            };
        };

        panic!("PpzFile");
    }
}

// ===============================================================================
// §2-27	全体転調設定
// 	#Transpose
// -------------------------------------------------------------------------------
// [書式]	#Transpose	数値
// -------------------------------------------------------------------------------
// [範囲]	-128～+127
// -------------------------------------------------------------------------------
// 	曲全体の転調値を設定します。
//
// 	指定した場合、Rhythm以外の全パートの頭全てに "_M"コマンドを指定
// 	したのと同じになります。
//
// 	リズムに使用しているトラック等で、転調したくないパートがある場合は、
// 	当該トラック先頭に "_M0" コマンドを発行してキャンセルしてください。
#[derive(Debug, Clone)]
pub struct TransposeMacro {
    pub code: Code,
    pub value: i8,
}

impl From<Macro> for TransposeMacro {
    fn from(m: Macro) -> Self {
        if let VariantValue::Signed(value) = m.value {
            return Self {
                code: m.code,
                value,
            };
        };

        panic!("Transpose");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename_success() {
        {
            let m = FileNameMacro::from(Macro {
                code: Code::default(),
                key: "Filename".to_owned(),
                value: VariantValue::String("test".to_owned()),
            });

            assert_eq!("test", m.value);
        }

        {
            let m = FileNameMacro::from(Macro {
                code: Code::default(),
                key: "FileName".to_owned(),
                value: VariantValue::String("🍣🍣".to_owned()),
            });

            assert_eq!("🍣🍣", m.value);
        }

        {
            let m = FileNameMacro::from(Macro {
                code: Code::default(),
                key: "fILENAME".to_owned(),
                value: VariantValue::String(".M".to_owned()),
            });

            assert_eq!(".M", m.value);
        }
    }

    #[test]
    fn test_ppsfile_success() {
        {
            let m = PpsFileMacro::from(Macro {
                code: Code::default(),
                key: "PPSFile".to_owned(),
                value: VariantValue::String("test".to_owned()),
            });

            assert_eq!("test", m.value);
        }

        {
            let m = PpsFileMacro::from(Macro {
                code: Code::default(),
                key: "PpsFile".to_owned(),
                value: VariantValue::String("🍣🍣".to_owned()),
            });

            assert_eq!("🍣🍣", m.value);
        }

        {
            let m = PpsFileMacro::from(Macro {
                code: Code::default(),
                key: "pPSfILE".to_owned(),
                value: VariantValue::String("ツツジ".to_owned()),
            });

            assert_eq!("ツツジ", m.value);
        }
    }

    #[test]
    fn test_pcmfile_success() {
        {
            let m = PpsFileMacro::from(Macro {
                code: Code::default(),
                key: "PCMFile".to_owned(),
                value: VariantValue::String("test".to_owned()),
            });

            assert_eq!("test", m.value);
        }

        {
            let m = PpsFileMacro::from(Macro {
                code: Code::default(),
                key: "PcmFile".to_owned(),
                value: VariantValue::String("🍣🍣".to_owned()),
            });

            assert_eq!("🍣🍣", m.value);
        }

        {
            let m = PpsFileMacro::from(Macro {
                code: Code::default(),
                key: "pCMfILE".to_owned(),
                value: VariantValue::String("ツツジ".to_owned()),
            });

            assert_eq!("ツツジ", m.value);
        }
    }

    #[test]
    fn test_fffile_success() {
        {
            let m = PpsFileMacro::from(Macro {
                code: Code::default(),
                key: "FFFile".to_owned(),
                value: VariantValue::String("test".to_owned()),
            });

            assert_eq!("test", m.value);
        }

        {
            let m = PpsFileMacro::from(Macro {
                code: Code::default(),
                key: "FfFile".to_owned(),
                value: VariantValue::String("🍣🍣".to_owned()),
            });

            assert_eq!("🍣🍣", m.value);
        }

        {
            let m = PpsFileMacro::from(Macro {
                code: Code::default(),
                key: "fffILE".to_owned(),
                value: VariantValue::String("ツツジ.FF".to_owned()),
            });

            assert_eq!("ツツジ.FF", m.value);
        }
    }

    #[test]
    fn test_option_success() {
        {
            let m = OptionMacro::from(Macro {
                code: Code::default(),
                key: "Option".to_owned(),
                value: VariantValue::String("/L".to_owned()),
            });

            assert_eq!(vec!["L"], m.values);
        }

        {
            let m = OptionMacro::from(Macro {
                code: Code::default(),
                key: "Option".to_owned(),
                value: VariantValue::String("/L/S/A/O".to_owned()),
            });

            assert_eq!(vec!["L", "S", "A", "O"], m.values);
        }

        {
            let m = OptionMacro::from(Macro {
                code: Code::default(),
                key: "Option".to_owned(),
                value: VariantValue::String("/L /S /A /O".to_owned()),
            });

            assert_eq!(vec!["L", "S", "A", "O"], m.values);
        }
    }

    #[test]
    fn test_title_success() {
        {
            let m = TitleMacro::from(Macro {
                code: Code::default(),
                key: "Title".to_owned(),
                value: VariantValue::String("ツツジ".to_owned()),
            });

            assert_eq!("ツツジ", m.value);
        }

        {
            let m = TitleMacro::from(Macro {
                code: Code::default(),
                key: "tITLE".to_owned(),
                value: VariantValue::String("🍣🍣".to_owned()),
            });

            assert_eq!("🍣🍣", m.value);
        }
    }

    #[test]
    fn test_composer_success() {
        {
            let m = ComposerMacro::from(Macro {
                code: Code::default(),
                key: "Composer".to_owned(),
                value: VariantValue::String("ツツジ".to_owned()),
            });

            assert_eq!("ツツジ", m.value);
        }

        {
            let m = ComposerMacro::from(Macro {
                code: Code::default(),
                key: "Title".to_owned(),
                value: VariantValue::String("🍣🍣".to_owned()),
            });

            assert_eq!("🍣🍣", m.value);
        }
    }

    #[test]
    fn test_arranger_success() {
        {
            let m = ArrangerMacro::from(Macro {
                code: Code::default(),
                key: "Arranger".to_owned(),
                value: VariantValue::String("ツツジ".to_owned()),
            });

            assert_eq!("ツツジ", m.value);
        }

        {
            let m = ArrangerMacro::from(Macro {
                code: Code::default(),
                key: "aRRANGER".to_owned(),
                value: VariantValue::String("🍣🍣".to_owned()),
            });

            assert_eq!("🍣🍣", m.value);
        }
    }

    #[test]
    fn test_memo_success() {
        {
            let m = MemoMacro::from(Macro {
                code: Code::default(),
                key: "Title".to_owned(),
                value: VariantValue::String("ツツジ".to_owned()),
            });

            assert_eq!("ツツジ", m.value);
        }

        {
            let m = MemoMacro::from(Macro {
                code: Code::default(),
                key: "Title".to_owned(),
                value: VariantValue::String("🍣🍣".to_owned()),
            });

            assert_eq!("🍣🍣", m.value);
        }
    }

    #[test]
    fn test_tempo_success() {
        {
            let m = TempoMacro::from(Macro {
                code: Code::default(),
                key: "Tempo".to_owned(),
                value: VariantValue::Unsigned(18),
            });

            assert_eq!(18, m.value);
        }

        {
            let m = TempoMacro::from(Macro {
                code: Code::default(),
                key: "tEMPO".to_owned(),
                value: VariantValue::Unsigned(255),
            });

            assert_eq!(255, m.value);
        }

        {
            let m = TempoMacro::from(Macro {
                code: Code::default(),
                key: "tEMPO".to_owned(),
                value: VariantValue::Unsigned(0),
            });

            assert_eq!(0, m.value);
        }

        {
            let m = TempoMacro::from(Macro {
                code: Code::default(),
                key: "tEMPO".to_owned(),
                value: VariantValue::Unsigned(250),
            });

            assert_eq!(250, m.value);
        }
    }

    #[test]
    fn test_zenlen_success() {
        {
            let m = ZenLenMacro::from(Macro {
                code: Code::default(),
                key: "Zenlen".to_owned(),
                value: VariantValue::Unsigned(1),
            });

            assert_eq!(1, m.value);
        }

        {
            let m = ZenLenMacro::from(Macro {
                code: Code::default(),
                key: "zENLEN".to_owned(),
                value: VariantValue::Unsigned(255),
            });

            assert_eq!(255, m.value);
        }
    }

    #[test]
    #[should_panic(expected = "Zenlen")]
    fn test_zenlen_failed_less() {
        {
            let _ = ZenLenMacro::from(Macro {
                code: Code::default(),
                key: "Zenlen".to_owned(),
                value: VariantValue::Unsigned(0),
            });
        }
    }

    #[test]
    fn test_octave_success() {
        {
            let m = OctaveMacro::from(Macro {
                code: Code::default(),
                key: "Octave".to_owned(),
                value: VariantValue::ReverseNormal(ReverseNormalOption::Normal),
            });

            assert_eq!(ReverseNormalOption::Normal, m.value);
        }

        {
            let m = OctaveMacro::from(Macro {
                code: Code::default(),
                key: "oCTAVE".to_owned(),
                value: VariantValue::ReverseNormal(ReverseNormalOption::Reverse),
            });

            assert_eq!(ReverseNormalOption::Reverse, m.value);
        }
    }

    #[test]
    fn test_loop_default_success() {
        {
            let m = LoopDefaultMacro::from(Macro {
                code: Code::default(),
                key: "LoopDefault".to_owned(),
                value: VariantValue::Unsigned(0),
            });

            assert_eq!(0, m.value);
        }

        {
            let m = LoopDefaultMacro::from(Macro {
                code: Code::default(),
                key: "LoopDefault".to_owned(),
                value: VariantValue::Unsigned(255),
            });

            assert_eq!(255, m.value);
        }
    }

    #[test]
    fn test_dt2flag_success() {
        {
            let m = Dt2FlagMacro::from(Macro {
                code: Code::default(),
                key: "DT2Flag".to_owned(),
                value: VariantValue::OnOff(OnOffOption::On),
            });

            assert_eq!(OnOffOption::On, m.value);
        }

        {
            let m = Dt2FlagMacro::from(Macro {
                code: Code::default(),
                key: "dt2fLAG".to_owned(),
                value: VariantValue::OnOff(OnOffOption::Off),
            });

            assert_eq!(OnOffOption::Off, m.value);
        }
    }

    #[test]
    fn test_bendrange_success() {
        {
            let m = BendRangeMacro::from(Macro {
                code: Code::default(),
                key: "Bendrange".to_owned(),
                value: VariantValue::Unsigned(0),
            });

            assert_eq!(0, m.value);
        }

        {
            let m = BendRangeMacro::from(Macro {
                code: Code::default(),
                key: "bENDRANGE".to_owned(),
                value: VariantValue::Unsigned(255),
            });

            assert_eq!(255, m.value);
        }
    }

    #[test]
    fn test_detune_success() {
        {
            let m = DetuneMacro::from(Macro {
                code: Code::default(),
                key: "Detune".to_owned(),
                value: VariantValue::ExtendNormal(ExtendNormalOption::Normal),
            });

            assert_eq!(ExtendNormalOption::Normal, m.value);
        }

        {
            let m = DetuneMacro::from(Macro {
                code: Code::default(),
                key: "dETUNE".to_owned(),
                value: VariantValue::ExtendNormal(ExtendNormalOption::Extend),
            });

            assert_eq!(ExtendNormalOption::Extend, m.value);
        }
    }

    #[test]
    fn test_lfospeed_success() {
        {
            let m = LfoSpeedMacro::from(Macro {
                code: Code::default(),
                key: "LFOSpeed".to_owned(),
                value: VariantValue::ExtendNormal(ExtendNormalOption::Normal),
            });

            assert_eq!(ExtendNormalOption::Normal, m.value);
        }

        {
            let m = DetuneMacro::from(Macro {
                code: Code::default(),
                key: "lfosPEED".to_owned(),
                value: VariantValue::ExtendNormal(ExtendNormalOption::Extend),
            });

            assert_eq!(ExtendNormalOption::Extend, m.value);
        }
    }

    #[test]
    fn test_envelopespeed_success() {
        {
            let m = EnvelopeSpeedMacro::from(Macro {
                code: Code::default(),
                key: "EnvelopeSpeed".to_owned(),
                value: VariantValue::ExtendNormal(ExtendNormalOption::Normal),
            });

            assert_eq!(ExtendNormalOption::Normal, m.value);
        }

        {
            let m = EnvelopeSpeedMacro::from(Macro {
                code: Code::default(),
                key: "eNVELOPEsPEED".to_owned(),
                value: VariantValue::ExtendNormal(ExtendNormalOption::Extend),
            });

            assert_eq!(ExtendNormalOption::Extend, m.value);
        }
    }

    #[test]
    fn test_pcmvolume_success() {
        {
            let m = PcmVolumeMacro::from(Macro {
                code: Code::default(),
                key: "PCMVolue".to_owned(),
                value: VariantValue::ExtendNormal(ExtendNormalOption::Normal),
            });

            assert_eq!(ExtendNormalOption::Normal, m.value);
        }

        {
            let m = PcmVolumeMacro::from(Macro {
                code: Code::default(),
                key: "pcmvOLUE".to_owned(),
                value: VariantValue::ExtendNormal(ExtendNormalOption::Extend),
            });

            assert_eq!(ExtendNormalOption::Extend, m.value);
        }
    }

    #[test]
    fn test_fm3extend_success() {
        {
            let m = Fm3ExtendMacro::from(Macro {
                code: Code::default(),
                key: "FM3Extend".to_owned(),
                value: VariantValue::String("XYZ".to_owned()),
            });

            assert_eq!(
                vec![
                    ExtendPartSymbol::X,
                    ExtendPartSymbol::Y,
                    ExtendPartSymbol::Z
                ],
                m.value
            );
        }

        {
            let m = Fm3ExtendMacro::from(Macro {
                code: Code::default(),
                key: "fm3eXTEND".to_owned(),
                value: VariantValue::String("xyz".to_owned()),
            });

            assert_eq!(
                vec![
                    ExtendPartSymbol::x,
                    ExtendPartSymbol::y,
                    ExtendPartSymbol::z
                ],
                m.value
            );
        }

        {
            let m = Fm3ExtendMacro::from(Macro {
                code: Code::default(),
                key: "fm3extend".to_owned(),
                value: VariantValue::String("a".to_owned()),
            });

            assert_eq!(vec![ExtendPartSymbol::a], m.value);
        }
    }

    // TODO: test #Include

    #[test]
    fn test_volumedown_success() {
        {
            let m = VolumeDownMacro::from(Macro {
                code: Code::default(),
                key: "Volumedown".to_owned(),
                value: VariantValue::String("FR+16,P+128,S+32".to_owned()),
            });

            assert_eq!(
                vec![
                    (
                        InstrumentsCategorySymbol::F,
                        RelativeAbsolute8::Relative(16)
                    ),
                    (
                        InstrumentsCategorySymbol::R,
                        RelativeAbsolute8::Relative(16)
                    ),
                    (
                        InstrumentsCategorySymbol::P,
                        RelativeAbsolute8::Relative(128)
                    ),
                    (
                        InstrumentsCategorySymbol::S,
                        RelativeAbsolute8::Relative(32)
                    ),
                ],
                m.value
            );
        }

        {
            let m = VolumeDownMacro::from(Macro {
                code: Code::default(),
                key: "Volumedown".to_owned(),
                value: VariantValue::String("F-16".to_owned()),
            });

            assert_eq!(
                vec![(
                    InstrumentsCategorySymbol::F,
                    RelativeAbsolute8::Relative(-16)
                ),],
                m.value
            );
        }

        {
            let m = VolumeDownMacro::from(Macro {
                code: Code::default(),
                key: "Volumedown".to_owned(),
                value: VariantValue::String("P96".to_owned()),
            });

            assert_eq!(
                vec![(
                    InstrumentsCategorySymbol::P,
                    RelativeAbsolute8::Absolute(96)
                ),],
                m.value
            );
        }
    }

    #[test]
    fn test_adpcm_success() {
        {
            let m = AdpcmMacro::from(Macro {
                code: Code::default(),
                key: "ADPCM".to_owned(),
                value: VariantValue::OnOff(OnOffOption::On),
            });

            assert_eq!(OnOffOption::On, m.value);
        }

        {
            let m = AdpcmMacro::from(Macro {
                code: Code::default(),
                key: "adpcm".to_owned(),
                value: VariantValue::OnOff(OnOffOption::Off),
            });

            assert_eq!(OnOffOption::Off, m.value);
        }
    }

    #[test]
    fn test_jump_success() {
        {
            let m = JumpMacro::from(Macro {
                code: Code::default(),
                key: "Jump".to_owned(),
                value: VariantValue::UnsignedShort(0),
            });

            assert_eq!(0, m.value);
        }

        {
            let m = JumpMacro::from(Macro {
                code: Code::default(),
                key: "jUMP".to_owned(),
                value: VariantValue::UnsignedShort(65535),
            });

            assert_eq!(65535, m.value);
        }
    }

    #[test]
    fn test_ppzextend_success() {
        {
            let m = PpzExtendMacro::from(Macro {
                code: Code::default(),
                key: "PPZExtend".to_owned(),
                value: VariantValue::String("abcdefgh".to_owned()),
            });

            assert_eq!(
                vec![
                    ExtendPartSymbol::a,
                    ExtendPartSymbol::b,
                    ExtendPartSymbol::c,
                    ExtendPartSymbol::d,
                    ExtendPartSymbol::e,
                    ExtendPartSymbol::f,
                    ExtendPartSymbol::g,
                    ExtendPartSymbol::h,
                ],
                m.value
            );
        }

        {
            let m = PpzExtendMacro::from(Macro {
                code: Code::default(),
                key: "ppzeXTEND".to_owned(),
                value: VariantValue::String("a".to_owned()),
            });

            assert_eq!(vec![ExtendPartSymbol::a,], m.value);
        }
    }

    #[test]
    fn test_ppzfile_success() {
        {
            let m = PpzFileMacro::from(Macro {
                code: Code::default(),
                key: "PPZFile".to_owned(),
                value: VariantValue::String("SAMPLE.PZI".to_owned()),
            });

            assert_eq!(vec!["SAMPLE.PZI"], m.value);
        }

        {
            let m = PpzFileMacro::from(Macro {
                code: Code::default(),
                key: "ppzfILE".to_owned(),
                value: VariantValue::String("BASEPCM.PZI,EXTEND.PVI".to_owned()),
            });

            assert_eq!(vec!["BASEPCM.PZI", "EXTEND.PVI"], m.value);
        }
    }

    #[test]
    #[should_panic]
    fn test_ppzfile_error() {
        {
            let _ = PpzFileMacro::from(Macro {
                code: Code::default(),
                key: "PPZFile".to_owned(),
                value: VariantValue::String("BASEPCM.PZI , EXTEND.PVI".to_owned()),
            });
        }
    }

    #[test]
    fn test_transpose_success() {
        {
            let m = TransposeMacro::from(Macro {
                code: Code::default(),
                key: "Transpose".to_owned(),
                value: VariantValue::Signed(-128),
            });

            assert_eq!(-128, m.value);
        }

        {
            let m = TransposeMacro::from(Macro {
                code: Code::default(),
                key: "tRANSPOSE".to_owned(),
                value: VariantValue::Signed(127),
            });

            assert_eq!(127, m.value);
        }
    }
}
