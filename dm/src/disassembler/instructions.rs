use crate::StringRef;
use crate::Proc;
use crate::Value;

#[derive(Debug)]
pub enum Variable {
    Null,
	World,
	Usr,
	Src,
	Args,
    Dot,
	Cache,
	Cache2,
	Cache3,
	CurrentProc,
	IndexFromStack,
    Arg(u32),
    Local(u32),
    Global(StringRef),
    Field(Box<Variable>, Vec<StringRef>),
	InitialField(Box<Variable>, Vec<StringRef>),
	StaticProcField(Box<Variable>, Vec<StringRef>, Proc),
	RuntimeProcField(Box<Variable>, Vec<StringRef>, StringRef)
    // TODO: Proc ones
}

#[derive(Debug)]
pub struct Call {
	pub args: ParamCount,
	pub proc: Proc,
}

#[derive(Debug)]
pub struct Switch {
	pub default: Loc,
	pub cases: Vec<(Value, Loc)>,
}

#[derive(Debug)]
pub struct PickSwitch {
	pub default: Loc,
	pub cases: Vec<(u32, Loc)>,
}

#[derive(Debug)]
pub struct SwitchRange {
	pub default: Loc,
	pub cases: Vec<(Value, Loc)>,
	pub range_cases: Vec<(Value, Value, Loc)>,
}

#[derive(Debug)]
pub struct ParamCount(pub u32);

#[derive(Debug)]
pub struct Loc(pub u32);

#[derive(Debug)]
pub enum Instruction {
	End(),
	New(ParamCount),
	// TODO: Pretty format the string
	Format(StringRef, ParamCount),
	Output,
	OutputFormat(),
	Read,
	Stat,
	Link(),
	OutputFTP(),
	OutputRun(),
	Missile(),
	Del,
	Test,
	Not,
	Jmp(Loc),
	Jnz(),
	Jz(u32),
	Ret,
	IsLoc,
	IsMob,
	IsObj,
	IsArea,
	IsTurf,
	Alert,
	EmptyList(),
	NewList(u32),
	View,
	OView(),
	ViewTarget(),
	OViewTarget(),
	Block,
	Prob,
	Rand,
	RandRange,
	Sleep,
	Spawn(),
	BrowseRSC(),
	IsIcon,
	Call(Variable, u32),
	CallNoReturn(Variable, u32),
	CallPath(),
	CallParent,
	CallParentArgs(ParamCount),
	CallSelfArgs(ParamCount),
	CallGlob(Call),
	Log10(),
	Log(),
	GetVar(Variable),
	SetVar(Variable),
	SetVarExpr(Variable),
	GetFlag,
	Teq,
	Tne,
	Tl,
	Tg,
	Tle,
	Tge,
	UnaryNeg,
	Add,
	Sub,
	Mul,
	Div,
	Mod,
	Round,
	RoundN,
	AugAdd(Variable),
	AugSub(Variable),
	AugMul(Variable),
	AugDiv(Variable),
	AugMod(Variable),
	AugBand(Variable),
	AugBor(Variable),
	AugXor(Variable),
	AugLShift(Variable),
	AugRShift(Variable),
	PushInt(i32),
	Pop,
	IterLoad(u32, u32),
	IterNext,
	IterPush,
	IterPop,
	Roll(),
	LocatePos,
	LocateRef,
	Flick(),
	Shutdown(),
	Startup(),
	RollStr(),
	PushVal(Value),
	NewImage(),
	PreInc(Variable),
	PostInc(Variable),
	PreDec(Variable),
	PostDec(Variable),
	Inc(Variable),
	Dec(Variable),
	Abs,
	Sqrt,
	Pow,
	Turn,
	AddText(),
	Length,
	CopyText,
	FindText(),
	FindTextEx(),
	CmpText(),
	SortText(),
	SortTextEx(),
	UpperText,
	LowerText,
	Text2Num,
	Num2Text,
	Num2TextSigFigs,
	Switch(Switch),
	PickSwitch(PickSwitch),
	SwitchRange(SwitchRange),
	ListGet,
	ListSet,
	BeginListSetExpr,
	IsType,
	Band,
	Bor,
	Bxor,
	Bnot,
	LShift,
	RShift,
	DbgFile(StringRef),
	DbgLine(u32),
	Step,
	StepTo,
	StepAway,
	StepTowards,
	StepRand,
	Walk,
	WalkTo,
	WalkAway,
	WalkTowards,
	WalkRand,
	GetStep,
	GetStepTo,
	GetStepAway,
	GetStepTowards,
	GetStepRand,
	GetDist,
	GetDir,
	LocateType,
	Shell,
	Text2File,
	File2Text,
	FCopy,
	IsNull,
	IsNum,
	IsText,
	StatPanel(),
	StatPanelCheck(),
	Min(ParamCount),
	Max(ParamCount),
	TypesOf(ParamCount),
	CKey,
	IsInList,
	Browse(),
	BrowseOpt,
	FList,
	Index,
	JmpOr(Loc),
	JmpAnd(Loc),
	FDel,
	CallName(ParamCount),
	List2Params,
	Params2List,
	CKeyEx(),
	PromptCheck,
	Rgb(),
	HasCall,
	HtmlEncode,
	HtmlDecode,
	Time2Text,
	Input(u32, u32, u32),
	Sin(),
	Cos(),
	ArcSin(),
	ArcCos(),
	Crash,
	NewAssocList(ParamCount),
	CallPathArgList,
	CallNameArgList, // TODO: same as above but without a src?
	CallGlobalArgList(Proc),
	NewArgList,
	MinList(),
	MaxList(),
	Pick,
	NewImageArgList,
	NewImageArgs(ParamCount),
	FCopyRsc,
	RandSeed(),
	Text2Ascii(),
	Ascii2Text(),
	IconStates(),
	IconNew(ParamCount),
	TurnOrFlipIcon {filter_mode: u32, var: Variable},
	IconIntensity(),
	IconSwapColor(),
	ShiftIcon(),
	IsFile(),
	OViewers(),
	Hearers(),
	OHearers(),
	IsPath,
	IsSubPath,
	FExists,
	Jmp2(Loc),
	Jnz2(Loc),
	Jz2(Loc),
	PopN(u32),
	CheckNum,
	Range,
	Orange,
	ForRange(Loc, Variable),
	IconDrawBox(),
	IconInsert(),
	UrlEncode,
	UrlDecode,
	Md5,
	Text2Path,
	WinOutput,
	WinSet,
	WinGet(),
	WinClone(),
	WinShow(),
	IconMapColors(),
	IconScale(),
	IconCrop(),
	Rgba(),
	IconStatesMode,
	IconGetPixel(),
	CallLib(ParamCount),
	CallLibArgList(),
	WinExists(),
	IconBlend(),
	IconSize(),
	Bounds(),
	OBounds(),
	BoundsDist(),
	StepSpeed(),
	StepToSpeed(),
	StepAwaySpeed(),
	StepTowardsSpeed(),
	StepRandSpeed(),
	WalkSpeed(),
	WalkToSpeed(),
	WalkAwaySpeed(),
	WalkTowardsSpeed(),
	WalkRandSpeed(),
	Animate,
	NullAnimate,
	MatrixNew(ParamCount),
	Database(),
	Try(Loc),
	Throw(),
	Catch(Loc),
	ReplaceText,
	ReplaceTextEx(),
	FindLastText(),
	FindLastTextEx(),
	SpanText(),
	NonSpanText(),
	SplitText,
	JoinText,
	JsonEncode,
	JsonDecode,
	RegexNew(ParamCount),
	FilterNewArgList,
	JmpIfNull(Loc),
	PushCache,
	SetCache,
	Tan(),
	ArcTan(),
	ArcTan2(),
	IsList,
	Ref,
	IsMovable,
	Clamp,
	Sha1(),
	LengthChar(),
	CopyTextChar,
	ReplaceTextChar(),
	ReplaceTextExChar(),
	FindLastTextChar(),
	FindLastTextExChar(),
	SpanTextChar(),
	NonSpanTextChar(),
	SplitTextChar(),
	Text2NumRadix(),
	Num2TextRadix(),
}
