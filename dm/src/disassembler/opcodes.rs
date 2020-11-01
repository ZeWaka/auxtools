#[repr(u32)]
#[derive(PartialEq, Eq, Copy, Clone)]
#[non_exhaustive]
#[allow(dead_code)]
#[derive(Debug)]
pub enum OpCode {
	End = 0x00,
	New = 0x01,
	Format = 0x02,
	Output = 0x03,
	OutputFormat = 0x04,
	Stat = 0x05,
	Unk_06 = 0x06,
	Link = 0x07,
	OutputFtp = 0x08,
	OutputRun = 0x09,
	Unk_0A = 0x0A,
	Missile = 0x0B,
	Del = 0x0C,
	Test = 0x0D,
	Not = 0x0E,
	Jmp = 0x0F,
	Jnz = 0x10,
	Jz = 0x11,
	Ret = 0x12,
	IsLoc = 0x13,
	IsMob = 0x14,
	IsObj = 0x15,
	IsArea = 0x16,
	IsTurf = 0x17,
	Alert = 0x18,
	EmptyList = 0x19,
	NewList = 0x1A,
	View = 0x1B,
	OView = 0x1C,
	ViewTarget = 0x1D,
	OViewTarget = 0x1E,
	Block = 0x1F,
	Unk_20 = 0x20,
	Prob = 0x21,
	Rand = 0x22,
	RandRange = 0x23,
	Sleep = 0x24,
	Spawn = 0x25,
	Unk_26 = 0x26,
	BrowseRsc = 0x27,
	IsIcon = 0x28,
	Call = 0x29,
	CallNoReturn = 0x2A,
	CallPath = 0x2B,
	CallParent = 0x2C,
	CallParentArgs = 0x2D,
	CallSelf = 0x2E,
	CallSelfArgs = 0x2F,
	CallGlob = 0x30,
	Log10 = 0x31,
	Log = 0x32,
	GetVar = 0x33,
	SetVar = 0x34,
	SetVarExpr = 0x35,
	GetFlag = 0x36,
	Teq = 0x37,
	Tne = 0x38,
	Tl = 0x39,
	Tg = 0x3A,
	Tle = 0x3B,
	Tge = 0x3C,
	UnaryNeg = 0x3D,
	Add = 0x3E,
	Sub = 0x3F,
	Mul = 0x40,
	Div = 0x41,
	Mod = 0x42,
	Round = 0x43,
	RoundN = 0x44,
	AugAdd = 0x45,
	AugSub = 0x46,
	AugMul = 0x47,
	AugDiv = 0x48,
	AugMod = 0x49,
	AugBand = 0x4A,
	AugBor = 0x4B,
	AugXor = 0x4C,
	AugLShift = 0x4D,
	AugRShift = 0x4E,
	PushInt = 0x50,
	Pop = 0x51,
	IterLoad = 0x52,
	IterNext = 0x53,
	IterPush = 0x54,
	IterPop = 0x55,
	Num2TextSigFigs = 0x56,
	Roll = 0x57,
	Unk_58 = 0x58,
	Range = 0x59,
	LocatePos = 0x5A,
	LocateRef = 0x5B,
	Flick = 0x5C,
	Shutdown = 0x5D,
	Startup = 0x5E,
	RollStr = 0x5F,
	PushVal = 0x60,
	NewImage = 0x61,
	PreInc = 0x62,
	PostInc = 0x63,
	PreDec = 0x64,
	PostDec = 0x65,
	Inc = 0x66,
	Dec = 0x67,
	Abs = 0x68,
	Sqrt = 0x69,
	Pow = 0x6A,
	Turn = 0x6B,
	AddText = 0x6C,
	Length = 0x6D,
	CopyText = 0x6E,
	FindText = 0x6F,
	FindTextEx = 0x70,
	CmpText = 0x71,
	SortText = 0x72,
	SortTextEx = 0x73,
	UpperText = 0x74,
	LowerText = 0x75,
	Text2Num = 0x76,
	Num2Text = 0x77,
	Switch = 0x78,
	PickSwitch = 0x79,
	SwitchRange = 0x7A,
	ListGet = 0x7B,
	ListSet = 0x7C,
	IsType = 0x7D,
	Band = 0x7E,
	Bor = 0x7F,
	Bxor = 0x80,
	Bnot = 0x81,
	LShift = 0x82,
	RShift = 0x83,
	DbgFile = 0x84,
	DbgLine = 0x85,
	Step = 0x86,
	StepTo = 0x87,
	StepAway = 0x88,
	StepTowards = 0x89,
	StepRand = 0x8A,
	Walk = 0x8B,
	WalkTo = 0x8C,
	WalkAway = 0x8D,
	WalkTowards = 0x8E,
	WalkRand = 0x8F,
	GetStep = 0x90,
	GetStepTo = 0x91,
	GetStepAway = 0x92,
	GetStepTowards = 0x93,
	GetStepRand = 0x94,
	GetDist = 0x95,
	GetDir = 0x96,
	LocateType = 0x97,
	Shell = 0x98,
	Text2File = 0x99,
	File2Text = 0x9A,
	FCopy = 0x9B,
	IsNull = 0x9E,
	IsNum = 0x9F,
	IsText = 0xA0,
	StatPanel = 0xA1,
	StatPanelCheck = 0xA2,
	Min = 0xA5,
	Max = 0xA6,
	TypesOf = 0xA7,
	CKey = 0xA8,
	IsIn = 0xA9,
	Browse = 0xAA,
	BrowseOpt = 0xAB,
	FList = 0xAC,
	Orange = 0xAD,
	Unk_0xAE = 0xAE,
	Read = 0xAF,
	Index = 0xB0,
	JmpOr = 0xB2,
	JmpAnd = 0xB3,
	FDel = 0xB4,
	CallName = 0xB5,
	List2Params = 0xB7,
	Params2List = 0xB8,
	CKeyEx = 0xB9,
	PromptCheck = 0xBa,
	Rgb = 0xBB,
	HasCall = 0xBC,
	HtmlEncode = 0xBE,
	HtmlDecode = 0xBF,
	Time2Text = 0xC0,
	Input = 0xC1,
	Sin = 0xC2,
	Cos = 0xC3,
	ArcSin = 0xC4,
	ArcCos = 0xC5,
	InputColor = 0xC6,
	Crash = 0xC7,
	NewAssocList = 0xC8,
	CallParentArgList = 0xC9,
	CallSelfArgList = 0xCA,
	CallPathArgList = 0xCB,
	CallNameArgList = 0xCC,
	CallGlobalArgList = 0xCD,
	NewArgList = 0xCF,
	MinList = 0xD0,
	MaxList = 0xD1,
	Pick = 0xD2,
	NewImageArgList = 0xD3,
	NewImageArgs = 0xD4,
	Unk_D5 = 0xD5,
	Unk_D6 = 0xD6,
	FCopyRsc = 0xD7,
	RandSeed = 0xDA,
	Text2Ascii = 0xDB,
	Ascii2Text = 0xDC,
	IconStates = 0xDD,
	IconNew = 0xDE,
	TurnOrFlipIcon = 0xDF,
	Unk_E0 = 0xE0,
	IconIntensity = 0xE1,
	IconSwapColor = 0xE2,
	ShiftIcon = 0xE3,
	IsFile = 0xE4,
	Viewers = 0xE5,
	OViewers = 0xE6,
	Hearers = 0xE7,
	OHearers = 0xE8,
	Unk_E9 = 0xE9,
	Unk_EA = 0xEA,
	Unk_EB = 0xEB,
	Unk_EC = 0xEC,
	Unk_ED = 0xED,
	Unk_EF = 0xEF,
	Unk_F0 = 0xF0,
	Unk_F1 = 0xF1,
	Unk_F2 = 0xF2,
	Unk_F3 = 0xF3,
	Unk_F4 = 0xF4,
	IsPath = 0xF5,
	IsSubPath = 0xF6,
	FExists = 0xF7,
	Jmp2 = 0xF8,
	Jnz2 = 0xF9,
	Jz2 = 0xFA,
	PopN = 0xFB,
	CheckNum = 0xFc,
	ForRange = 0xFD,
	ForRangeStepSetup = 0xFE,
	ForRangeStep = 0xFF,
	Unk_100 = 0x100,
	Unk_101 = 0x101,
	Unk_102 = 0x102,
	Unk_103 = 0x103,
	Unk_104 = 0x104,
	IconDrawBox = 0x105,
	IconInsert = 0x106,
	UrlEncode = 0x107,
	UrlDecode = 0x108,
	Md5 = 0x109,
	Text2Path = 0x10A,
	WinOutput = 0x10B,
	WinSet = 0x10C,
	WinGet = 0x10D,
	WinClone = 0x10E,
	WinShow = 0x10F,
	IconMapColors = 0x110,
	IconScale = 0x111,
	IconCrop = 0x112,
	Rgba = 0x113,
	IconStatesMode = 0x114,
	IconGetPixel = 0x115,
	CallLib = 0x116,
	CallLibArgList = 0x117,
	WinExists = 0x118,
	IconBlend = 0x119,
	IconSize = 0x11A,
	Bounds = 0x11B,
	OBounds = 0x11C,
	BoundsDist = 0x11D,
	StepSpeed = 0x11E,
	StepToSpeed = 0x11F,
	StepAwaySpeed = 0x120,
	StepTowardsSpeed = 0x121,
	StepRandSpeed = 0x122,
	WalkSpeed = 0x123,
	WalkToSpeed = 0x124,
	WalkAwaySpeed = 0x125,
	WalkTowardsSpeed = 0x126,
	WalkRandSpeed = 0x127,
	Animate = 0x128,
	NullAnimate = 0x129,
	MatrixNew = 0x12A,
	Database = 0x12B,
	Try = 0x12C,
	Throw = 0x12D,
	Catch = 0x12E,
	Unk_12F = 0x12F,	
	ReplaceText = 0x130,
	ReplaceTextEx = 0x131,
	FindLastText = 0x132,
	FindLastTextEx = 0x133,
	SpanText = 0x134,
	NonSpanText = 0x135,
	SplitText = 0x136,
	JoinText = 0x137,
	JsonEncode = 0x138,
	JsonDecode = 0x139,
	RegexNew = 0x13A,
	FilterNewArgList = 0x13B,
	BeginListSetExpr = 0x13C,
	JmpIfNull = 0x13D,
	JmpIfNull2 = 0x13E,
	NullCacheMaybe = 0x13F,
	Unk_140 = 0x140,
	TestNotEquiv = 0x141,
	PushToCache = 0x142,
	PopFromCache = 0x143,
	Tan = 0x144,
	ArcTan = 0x145,
	ArcTan2 = 0x146,
	IsList = 0x147,
	Ref = 0x148,
	IsMovable = 0x149,
	Clamp = 0x14A,
	Sha1 = 0x14B,
	LengthChar = 0x14D,
	CopyTextChar = 0x14E,
	FindTextChar = 0x14F,
	ReplaceTextChar = 0x151,
	ReplaceTextExChar = 0x152,
	FindLastTextChar = 0x153,
	FindLastTextExChar = 0x154,
	SpanTextChar = 0x155,
	NonSpanTextChar = 0x156,
	SplitTextChar = 0x157,
	Text2NumRadix = 0x158,
	Num2TextRadix = 0x159,
	Unk_15A = 0x15A,
	Unk_15B = 0x15B,
	Unk_15C = 0x15C,
	Unk_15D = 0x15D,
	Unk_15E = 0x15E,
	Unk_15F = 0x15F,
	Unk_160 = 0x160,
	Unk_161 = 0x161,
	Unk_162 = 0x162,
	Unk_163 = 0x163,
	Unk_164 = 0x164,
	Unk_165 = 0x165,
	Unk_166 = 0x166,
	Unk_167 = 0x167,
	Unk_168 = 0x168,
	Unk_169 = 0x169,
	Unk_16A = 0x16A,
	Unk_16B = 0x16B,
	Unk_16C = 0x16C,
	Unk_16D = 0x16D,
	Unk_16E = 0x16E,
	Unk_16F = 0x16F,
	Unk_170 = 0x170,
	Unk_171 = 0x171,
	Unk_172 = 0x172,
	Unk_173 = 0x173,
	Unk_174 = 0x174,
	Unk_175 = 0x175,
	Unk_176 = 0x176,
	Unk_177 = 0x177,
	Unk_178 = 0x178,
	Unk_179 = 0x179,
	Unk_17A = 0x17A,
	Unk_17B = 0x17B,
	Unk_17C = 0x17C,
	Unk_17D = 0x17D,
	Unk_17E = 0x17E,
	Unk_17F = 0x17F,
}

#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum AccessModifier {
	Usr = 0xFFCD,			// 65485
	Src = 0xFFCE,			// 65486
	Args = 0xFFCF,
	Dot = 0xFFD0,
	Cache = 0xFFD8,
	Arg = 0xFFD9,			// 65497
	Local = 0xFFDA,			// 65498
	Global = 0xFFDB,		// 65499
	Field = 0xFFDC,			// 65500
	SrcProc2 = 0xFFDD,
	SrcProc = 0xFFDE,
	Proc = 0xFFDF,			// 65503
	Proc2 = 0xFFE0,
	// TODO: What are these?
	// 0xFFE1
	// 0xFFE2
	Cache2 = 0xFFE3,
	Cache3 = 0xFFE4,
	World = 0xFFE5,
	Null = 0xFFE6,
	Initial = 0xFFE7,		// 65511
}

impl AccessModifier {
	pub fn in_range(value: u32) -> bool {
		value >= (Self::Usr as u32) && value <= (Self::Initial as u32)
	}
}