use crate::{arch::x86::io, print};

#[repr(u8)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum KeyboardKey {
	KeyEsc = 0x01,
	Key1 = 0x02,
	Key2 = 0x03,
	Key3 = 0x04,
	Key4 = 0x05,
	Key5 = 0x06,
	Key6 = 0x07,
	Key7 = 0x08,
	Key8 = 0x09,
	Key9 = 0x0a,
	Key0 = 0x0b,
	KeyMinus = 0x0c,
	KeyEqual = 0x0d,
	KeyBackspace = 0x0e,
	KeyTab = 0x0f,
	KeyQ = 0x10,
	KeyW = 0x11,
	KeyE = 0x12,
	KeyR = 0x13,
	KeyT = 0x14,
	KeyY = 0x15,
	KeyU = 0x16,
	KeyI = 0x17,
	KeyO = 0x18,
	KeyP = 0x19,
	KeyOpenBrace = 0x1a,
	KeyCloseBrace = 0x1b,
	KeyEnter = 0x1c,
	KeyLeftControl = 0x1d,
	KeyA = 0x1e,
	KeyS = 0x1f,
	KeyD = 0x20,
	KeyF = 0x21,
	KeyG = 0x22,
	KeyH = 0x23,
	KeyJ = 0x24,
	KeyK = 0x25,
	KeyL = 0x26,
	KeySemiColon = 0x27,
	KeySingleQuote = 0x28,
	KeyBackTick = 0x29,
	KeyLeftShift = 0x2a,
	KeyBackslash = 0x2b,
	KeyZ = 0x2c,
	KeyX = 0x2d,
	KeyC = 0x2e,
	KeyV = 0x2f,
	KeyB = 0x30,
	KeyN = 0x31,
	KeyM = 0x32,
	KeyComma = 0x33,
	KeyDot = 0x34,
	KeySlash = 0x35,
	KeyRightShift = 0x36,
	KeyKeypadStar = 0x37,
	KeyLeftAlt = 0x38,
	KeySpace = 0x39,
	KeyCapsLock = 0x3a,
	KeyF1 = 0x3b,
	KeyF2 = 0x3c,
	KeyF3 = 0x3d,
	KeyF4 = 0x3e,
	KeyF5 = 0x3f,
	KeyF6 = 0x40,
	KeyF7 = 0x41,
	KeyF8 = 0x42,
	KeyF9 = 0x43,
	KeyF10 = 0x44,
	KeyNumberLock = 0x45,
	KeyScrollLock = 0x46,
	KeyKeypad7 = 0x47,
	KeyKeypad8 = 0x48,
	KeyKeypad9 = 0x49,
	KeyKeypadMinus = 0x4a,
	KeyKeypad4 = 0x4b,
	KeyKeypad5 = 0x4c,
	KeyKeypad6 = 0x4d,
	KeyKeypadPlus = 0x4e,
	KeyKeypad1 = 0x4f,
	KeyKeypad2 = 0x50,
	KeyKeypad3 = 0x51,
	KeyKeypad0 = 0x52,
	KeyKeypadDot = 0x53,
	KeyF11 = 0x57,
	KeyF12 = 0x58,
	KeyPreviousTrack = 0x90,
	KeyNextTrack = 0x99,
	KeyKeypadEnter = 0x9c,
	KeyRightControl = 0x9d,
	KeyMute = 0xa0,
	KeyCalculator = 0xa1,
	KeyPlay = 0xa2,
	KeyStop = 0xa4,
	KeyVolumeDown = 0xae,
	KeyVolumeUp = 0xb0,
	KeyWWWHome = 0xb2,
	KeyKeypadSlash = 0xb5,
	KeyRightAlt = 0xb8,
	KeyHome = 0xc7,
	KeyCursorUp = 0xc8,
	KeyPageUp = 0xc9,
	KeyCursorLeft = 0xcb,
	KeyCursorRight = 0xcd,
	KeyEnd = 0xcf,
	KeyCursorDown = 0xd0,
	KeyPageDown = 0xd1,
	KeyInsert = 0xd2,
	KeyDelete = 0xd3,
	KeyLeftGUI = 0xdb,
	KeyRightGUI = 0xdc,
	KeyApps = 0xdd,
	KeyACPIPower = 0xde,
	KeyACPISleep = 0xdf,
	KeyACPIWake = 0xe3,
	KeyWWWSearch = 0xe5,
	KeyWWWFavorites = 0xe6,
	KeyWWWRefresh = 0xe7,
	KeyWWWStop = 0xe8,
	KeyWWWForward = 0xe9,
	KeyWWWBack = 0xea,
	KeyMyComputer = 0xeb,
	KeyEmail = 0xec,
	KeyMediaSelect = 0xed,
	KeyPrintScreen = 0xee,
	KeyPause = 0xef,
}

pub struct Keyboard {
	shift_pressed: bool,
	ctrl_pressed: bool,
	alt_pressed: bool,
}

impl Keyboard {
	pub fn new() -> Keyboard {
		Keyboard {
			shift_pressed: false,
			ctrl_pressed: false,
			alt_pressed: false,
		}
	}

	fn get_ascii(&self, scan_code: u8) -> char {
		let shift_pressed = self.shift_pressed;
		let ctrl_pressed = self.ctrl_pressed;
		let alt_pressed = self.alt_pressed;
		let key = unsafe { core::mem::transmute::<u8, KeyboardKey>(scan_code) };

		match (key, shift_pressed, ctrl_pressed, alt_pressed) {
			// === LETTERS ===
			(KeyboardKey::KeyA, false, false, false) => 'a',
			(KeyboardKey::KeyA, true, false, false) => 'A',
			(KeyboardKey::KeyA, _, true, false) => '\x01',
			(KeyboardKey::KeyA, _, false, true) => '\0',

			(KeyboardKey::KeyB, false, false, false) => 'b',
			(KeyboardKey::KeyB, true, false, false) => 'B',
			(KeyboardKey::KeyB, _, true, false) => '\x02',
			(KeyboardKey::KeyB, _, false, true) => '\0',

			(KeyboardKey::KeyC, false, false, false) => 'c',
			(KeyboardKey::KeyC, true, false, false) => 'C',
			(KeyboardKey::KeyC, _, true, false) => '\x03',
			(KeyboardKey::KeyC, _, false, true) => '\0',

			(KeyboardKey::KeyD, false, false, false) => 'd',
			(KeyboardKey::KeyD, true, false, false) => 'D',
			(KeyboardKey::KeyD, _, true, false) => '\x04',
			(KeyboardKey::KeyD, _, false, true) => '\0',

			(KeyboardKey::KeyE, false, false, false) => 'e',
			(KeyboardKey::KeyE, true, false, false) => 'E',
			(KeyboardKey::KeyE, _, true, false) => '\x05',
			(KeyboardKey::KeyE, _, false, true) => '\0',

			(KeyboardKey::KeyF, false, false, false) => 'f',
			(KeyboardKey::KeyF, true, false, false) => 'F',
			(KeyboardKey::KeyF, _, true, false) => '\x06',
			(KeyboardKey::KeyF, _, false, true) => '\0',

			(KeyboardKey::KeyG, false, false, false) => 'g',
			(KeyboardKey::KeyG, true, false, false) => 'G',
			(KeyboardKey::KeyG, _, true, false) => '\x07',
			(KeyboardKey::KeyG, _, false, true) => '\0',

			(KeyboardKey::KeyH, false, false, false) => 'h',
			(KeyboardKey::KeyH, true, false, false) => 'H',
			(KeyboardKey::KeyH, _, true, false) => '\x08',
			(KeyboardKey::KeyH, _, false, true) => '\0',

			(KeyboardKey::KeyI, false, false, false) => 'i',
			(KeyboardKey::KeyI, true, false, false) => 'I',
			(KeyboardKey::KeyI, _, true, false) => '\0',
			(KeyboardKey::KeyI, _, false, true) => '\0',

			(KeyboardKey::KeyJ, false, false, false) => 'j',
			(KeyboardKey::KeyJ, true, false, false) => 'J',
			(KeyboardKey::KeyJ, _, true, false) => '\x0A',
			(KeyboardKey::KeyJ, _, false, true) => '\0',

			(KeyboardKey::KeyK, false, false, false) => 'k',
			(KeyboardKey::KeyK, true, false, false) => 'K',
			(KeyboardKey::KeyK, _, true, false) => '\x0B',
			(KeyboardKey::KeyK, _, false, true) => '\0',

			(KeyboardKey::KeyL, false, false, false) => 'l',
			(KeyboardKey::KeyL, true, false, false) => 'L',
			(KeyboardKey::KeyL, _, true, false) => '\x0C',
			(KeyboardKey::KeyL, _, false, true) => '\0',

			(KeyboardKey::KeyM, false, false, false) => 'm',
			(KeyboardKey::KeyM, true, false, false) => 'M',
			(KeyboardKey::KeyM, _, true, false) => '\x0D',
			(KeyboardKey::KeyM, _, false, true) => '\0',

			(KeyboardKey::KeyN, false, false, false) => 'n',
			(KeyboardKey::KeyN, true, false, false) => 'N',
			(KeyboardKey::KeyN, _, true, false) => '\x0E',
			(KeyboardKey::KeyN, _, false, true) => '\0',

			(KeyboardKey::KeyO, false, false, false) => 'o',
			(KeyboardKey::KeyO, true, false, false) => 'O',
			(KeyboardKey::KeyO, _, true, false) => '\x0F',
			(KeyboardKey::KeyO, _, false, true) => '\0',

			(KeyboardKey::KeyP, false, false, false) => 'p',
			(KeyboardKey::KeyP, true, false, false) => 'P',
			(KeyboardKey::KeyP, _, true, false) => '\0',
			(KeyboardKey::KeyP, _, false, true) => '\0',

			(KeyboardKey::KeyQ, false, false, false) => 'q',
			(KeyboardKey::KeyQ, true, false, false) => 'Q',
			(KeyboardKey::KeyQ, _, true, false) => '\x11',
			(KeyboardKey::KeyQ, _, false, true) => '\0',

			(KeyboardKey::KeyR, false, false, false) => 'r',
			(KeyboardKey::KeyR, true, false, false) => 'R',
			(KeyboardKey::KeyR, _, true, false) => '\x12',
			(KeyboardKey::KeyR, _, false, true) => '\0',

			(KeyboardKey::KeyS, false, false, false) => 's',
			(KeyboardKey::KeyS, true, false, false) => 'S',
			(KeyboardKey::KeyS, _, true, false) => '\x13',
			(KeyboardKey::KeyS, _, false, true) => '\0',

			(KeyboardKey::KeyT, false, false, false) => 't',
			(KeyboardKey::KeyT, true, false, false) => 'T',
			(KeyboardKey::KeyT, _, true, false) => '\x14',
			(KeyboardKey::KeyT, _, false, true) => '\0',

			(KeyboardKey::KeyU, false, false, false) => 'u',
			(KeyboardKey::KeyU, true, false, false) => 'U',
			(KeyboardKey::KeyU, _, true, false) => '\x15',
			(KeyboardKey::KeyU, _, false, true) => '\0',

			(KeyboardKey::KeyV, false, false, false) => 'v',
			(KeyboardKey::KeyV, true, false, false) => 'V',
			(KeyboardKey::KeyV, _, true, false) => '\x16',
			(KeyboardKey::KeyV, _, false, true) => '\0',

			(KeyboardKey::KeyW, false, false, false) => 'w',
			(KeyboardKey::KeyW, true, false, false) => 'W',
			(KeyboardKey::KeyW, _, true, false) => '\x17',
			(KeyboardKey::KeyW, _, false, true) => '\0',

			(KeyboardKey::KeyX, false, false, false) => 'x',
			(KeyboardKey::KeyX, true, false, false) => 'X',
			(KeyboardKey::KeyX, _, true, false) => '\x18',
			(KeyboardKey::KeyX, _, false, true) => '\0',

			(KeyboardKey::KeyY, false, false, false) => 'y',
			(KeyboardKey::KeyY, true, false, false) => 'Y',
			(KeyboardKey::KeyY, _, true, false) => '\x19',
			(KeyboardKey::KeyY, _, false, true) => '\0',

			(KeyboardKey::KeyZ, false, false, false) => 'z',
			(KeyboardKey::KeyZ, true, false, false) => 'Z',
			(KeyboardKey::KeyZ, _, true, false) => '\x1A',
			(KeyboardKey::KeyZ, _, false, true) => '\0',

			// === NUMBERS ===
			(KeyboardKey::Key1, false, false, false) => '1',
			(KeyboardKey::Key1, true, false, false) => '!',
			(KeyboardKey::Key1, _, true, false) => '\x11',
			(KeyboardKey::Key1, _, false, true) => '\0',

			(KeyboardKey::Key2, false, false, false) => '2',
			(KeyboardKey::Key2, true, false, false) => '@',
			(KeyboardKey::Key2, _, true, false) => '\x12',
			(KeyboardKey::Key2, _, false, true) => '\0',

			(KeyboardKey::Key3, false, false, false) => '3',
			(KeyboardKey::Key3, true, false, false) => '#',
			(KeyboardKey::Key3, _, true, false) => '\x13',
			(KeyboardKey::Key3, _, false, true) => '\0',

			(KeyboardKey::Key4, false, false, false) => '4',
			(KeyboardKey::Key4, true, false, false) => '$',
			(KeyboardKey::Key4, _, true, false) => '\x14',
			(KeyboardKey::Key4, _, false, true) => '\0',

			(KeyboardKey::Key5, false, false, false) => '5',
			(KeyboardKey::Key5, true, false, false) => '%',
			(KeyboardKey::Key5, _, true, false) => '\x15',
			(KeyboardKey::Key5, _, false, true) => '\0',

			(KeyboardKey::Key6, false, false, false) => '6',
			(KeyboardKey::Key6, true, false, false) => '^',
			(KeyboardKey::Key6, _, true, false) => '\x16',
			(KeyboardKey::Key6, _, false, true) => '\0',

			(KeyboardKey::Key7, false, false, false) => '7',
			(KeyboardKey::Key7, true, false, false) => '&',
			(KeyboardKey::Key7, _, true, false) => '\x17',
			(KeyboardKey::Key7, _, false, true) => '\0',

			(KeyboardKey::Key8, false, false, false) => '8',
			(KeyboardKey::Key8, true, false, false) => '*',
			(KeyboardKey::Key8, _, true, false) => '\x18',
			(KeyboardKey::Key8, _, false, true) => '\0',

			(KeyboardKey::Key9, false, false, false) => '9',
			(KeyboardKey::Key9, true, false, false) => '(',
			(KeyboardKey::Key9, _, true, false) => '\x19',
			(KeyboardKey::Key9, _, false, true) => '\0',

			(KeyboardKey::Key0, false, false, false) => '0',
			(KeyboardKey::Key0, true, false, false) => ')',
			(KeyboardKey::Key0, _, true, false) => '\x10',
			(KeyboardKey::Key0, _, false, true) => '\0',

			// === SPECIAL CHARACTERS ===
			(KeyboardKey::KeyMinus, false, false, false) => '-',
			(KeyboardKey::KeyMinus, true, false, false) => '_',
			(KeyboardKey::KeyMinus, _, true, false) => '\x1F',
			(KeyboardKey::KeyMinus, _, false, true) => '\0',

			// === WHITESPACE AND CONTROL ===
			(KeyboardKey::KeySpace, _, false, false) => ' ',
			(KeyboardKey::KeySpace, _, true, false) => '\0',
			(KeyboardKey::KeySpace, _, false, true) => '\0',

			(KeyboardKey::KeyEnter, _, false, false) => '\n',
			(KeyboardKey::KeyEnter, _, true, false) => '\n',
			(KeyboardKey::KeyEnter, _, false, true) => '\0',

			(KeyboardKey::KeyTab, _, false, false) => '\t',
			(KeyboardKey::KeyTab, _, true, false) => '\t',
			(KeyboardKey::KeyTab, _, false, true) => '\0',

			(KeyboardKey::KeyBackspace, _, false, false) => '\x08',
			(KeyboardKey::KeyBackspace, _, true, false) => '\x08',
			(KeyboardKey::KeyBackspace, _, false, true) => '\0',

			// === FUNCTION KEYS (no ASCII output) ===
			(KeyboardKey::KeyF1, ..) => '\0',
			(KeyboardKey::KeyF2, ..) => '\0',
			(KeyboardKey::KeyF3, ..) => '\0',
			(KeyboardKey::KeyF4, ..) => '\0',
			(KeyboardKey::KeyF5, ..) => '\0',
			(KeyboardKey::KeyF6, ..) => '\0',
			(KeyboardKey::KeyF7, ..) => '\0',
			(KeyboardKey::KeyF8, ..) => '\0',
			(KeyboardKey::KeyF9, ..) => '\0',
			(KeyboardKey::KeyF10, ..) => '\0',
			(KeyboardKey::KeyF11, ..) => '\0',
			(KeyboardKey::KeyF12, ..) => '\0',

			// === KEYPAD KEYS ===
			(KeyboardKey::KeyKeypad0, false, false, false) => '0',
			(KeyboardKey::KeyKeypad1, false, false, false) => '1',
			(KeyboardKey::KeyKeypad2, false, false, false) => '2',
			(KeyboardKey::KeyKeypad3, false, false, false) => '3',
			(KeyboardKey::KeyKeypad4, false, false, false) => '4',
			(KeyboardKey::KeyKeypad5, false, false, false) => '5',
			(KeyboardKey::KeyKeypad6, false, false, false) => '6',
			(KeyboardKey::KeyKeypad7, false, false, false) => '7',
			(KeyboardKey::KeyKeypad8, false, false, false) => '8',
			(KeyboardKey::KeyKeypad9, false, false, false) => '9',
			(KeyboardKey::KeyKeypadDot, false, false, false) => '.',
			(KeyboardKey::KeyKeypadStar, false, false, false) => '*',
			(KeyboardKey::KeyKeypadMinus, false, false, false) => '-',
			(KeyboardKey::KeyKeypadPlus, false, false, false) => '+',
			(KeyboardKey::KeyKeypadSlash, false, false, false) => '/',
			(KeyboardKey::KeyKeypadEnter, false, false, false) => '\n',

			// === MODIFIER KEYS (no output) ===
			(KeyboardKey::KeyLeftShift, ..) => '\0',
			(KeyboardKey::KeyRightShift, ..) => '\0',
			(KeyboardKey::KeyLeftControl, ..) => '\0',
			(KeyboardKey::KeyRightControl, ..) => '\0',
			(KeyboardKey::KeyLeftAlt, ..) => '\0',
			(KeyboardKey::KeyRightAlt, ..) => '\0',
			(KeyboardKey::KeyLeftGUI, ..) => '\0',
			(KeyboardKey::KeyRightGUI, ..) => '\0',

			// === SPECIAL KEYS (no output) ===
			(KeyboardKey::KeyPrintScreen, ..) => '\0',
			(KeyboardKey::KeyScrollLock, ..) => '\0',
			(KeyboardKey::KeyPause, ..) => '\0',
			(KeyboardKey::KeyInsert, ..) => '\0',
			(KeyboardKey::KeyDelete, ..) => '\0',
			(KeyboardKey::KeyHome, ..) => '\0',
			(KeyboardKey::KeyEnd, ..) => '\0',
			(KeyboardKey::KeyPageUp, ..) => '\0',
			(KeyboardKey::KeyPageDown, ..) => '\0',
			(KeyboardKey::KeyCursorUp, ..) => '\0',
			(KeyboardKey::KeyCursorDown, ..) => '\0',
			(KeyboardKey::KeyCursorLeft, ..) => '\0',
			(KeyboardKey::KeyCursorRight, ..) => '\0',

			// Catch any unhandled combinations
			_ => '\0',
		}
	}

	pub fn input(&mut self) {
		const KEYBOARD_DATA_PORT: u16 = 0x60;
		const KEYBOARD_STATUS_PORT: u16 = 0x64;

		if io::inb(KEYBOARD_STATUS_PORT) & 1 == 0 {
			return;
		}

		let scan_code = io::inb(KEYBOARD_DATA_PORT);

		// Alt Pressed
		if scan_code == 56 {
			self.alt_pressed = true;
			return;
		}

		// Alt Released
		if scan_code == 184 {
			self.alt_pressed = false;
			return;
		}

		// Ctrl Pressed
		if scan_code == 29 {
			self.ctrl_pressed = true;
			return;
		}

		// Ctrl Released
		if scan_code == 157 {
			self.ctrl_pressed = false;
			return;
		}

		// Shift Pressed
		if scan_code == 42 {
			self.shift_pressed = true;
			return;
		}

		// Shift Released
		if scan_code == 170 {
			self.shift_pressed = false;
			return;
		}

		if scan_code >= 0x80 {
			return;
		}

		let c = self.get_ascii(scan_code);

		print!("{}", c);
	}
}
