//! PS/2 Keyboard Driver for x86 Architecture
//!
//! This module implements a basic PS/2 keyboard driver that handles keyboard
//! input in a bare metal environment. It provides scan code translation to
//! ASCII characters and supports modifier keys (Shift, Ctrl, Alt) for extended
//! input capabilities.
//!
//! The driver interfaces with the keyboard controller through the standard PS/2
//! ports:
//! - Data Port (0x60): Receives scan codes from the keyboard
//! - Status Port (0x64): Reports keyboard controller status
//!
//! # Implementation Details
//! The driver uses scan code set 1 (the standard PC keyboard set) and
//! translates these hardware-level codes into ASCII characters that can be used
//! by higher level software like a shell or text editor. Special consideration
//! is given to key release codes (>0x80) to properly track modifier key states.

use crate::arch::x86::io;
use core::alloc;

#[repr(u8)]
#[allow(missing_docs)]
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

#[doc(hidden)]
pub struct Keyboard {
	shift_pressed: bool,
	ctrl_pressed: bool,
	alt_pressed: bool,
}

impl Default for Keyboard {
	fn default() -> Self {
		return Keyboard {
			shift_pressed: false,
			ctrl_pressed: false,
			alt_pressed: false,
		};
	}
}

impl Keyboard {
	fn get_ascii(&self, scan_code: u8) -> char {
		let shift_pressed = self.shift_pressed;
		let ctrl_pressed = self.ctrl_pressed;
		let alt_pressed = self.alt_pressed;
		let key = unsafe { core::mem::transmute::<u8, KeyboardKey>(scan_code) };

		match (key, shift_pressed, ctrl_pressed, alt_pressed) {
			// === LETTERS ===
			(KeyboardKey::KeyA, false, false, false) => return 'a',
			(KeyboardKey::KeyA, true, false, false) => return 'A',
			(KeyboardKey::KeyA, _, true, false) => return '\x01',
			(KeyboardKey::KeyA, _, false, true) => return '\0',

			(KeyboardKey::KeyB, false, false, false) => return 'b',
			(KeyboardKey::KeyB, true, false, false) => return 'B',
			(KeyboardKey::KeyB, _, true, false) => return '\x02',
			(KeyboardKey::KeyB, _, false, true) => return '\0',

			(KeyboardKey::KeyC, false, false, false) => return 'c',
			(KeyboardKey::KeyC, true, false, false) => return 'C',
			(KeyboardKey::KeyC, _, true, false) => return '\x03',
			(KeyboardKey::KeyC, _, false, true) => return '\0',

			(KeyboardKey::KeyD, false, false, false) => return 'd',
			(KeyboardKey::KeyD, true, false, false) => return 'D',
			(KeyboardKey::KeyD, _, true, false) => return '\x04',
			(KeyboardKey::KeyD, _, false, true) => return '\0',

			(KeyboardKey::KeyE, false, false, false) => return 'e',
			(KeyboardKey::KeyE, true, false, false) => return 'E',
			(KeyboardKey::KeyE, _, true, false) => return '\x05',
			(KeyboardKey::KeyE, _, false, true) => return '\0',

			(KeyboardKey::KeyF, false, false, false) => return 'f',
			(KeyboardKey::KeyF, true, false, false) => return 'F',
			(KeyboardKey::KeyF, _, true, false) => return '\x06',
			(KeyboardKey::KeyF, _, false, true) => return '\0',

			(KeyboardKey::KeyG, false, false, false) => return 'g',
			(KeyboardKey::KeyG, true, false, false) => return 'G',
			(KeyboardKey::KeyG, _, true, false) => return '\x07',
			(KeyboardKey::KeyG, _, false, true) => return '\0',

			(KeyboardKey::KeyH, false, false, false) => return 'h',
			(KeyboardKey::KeyH, true, false, false) => return 'H',
			(KeyboardKey::KeyH, _, true, false) => return '\x08',
			(KeyboardKey::KeyH, _, false, true) => return '\0',

			(KeyboardKey::KeyI, false, false, false) => return 'i',
			(KeyboardKey::KeyI, true, false, false) => return 'I',
			(KeyboardKey::KeyI, _, true, false) => return '\0',
			(KeyboardKey::KeyI, _, false, true) => return '\0',

			(KeyboardKey::KeyJ, false, false, false) => return 'j',
			(KeyboardKey::KeyJ, true, false, false) => return 'J',
			(KeyboardKey::KeyJ, _, true, false) => return '\x0A',
			(KeyboardKey::KeyJ, _, false, true) => return '\0',

			(KeyboardKey::KeyK, false, false, false) => return 'k',
			(KeyboardKey::KeyK, true, false, false) => return 'K',
			(KeyboardKey::KeyK, _, true, false) => return '\x0B',
			(KeyboardKey::KeyK, _, false, true) => return '\0',

			(KeyboardKey::KeyL, false, false, false) => return 'l',
			(KeyboardKey::KeyL, true, false, false) => return 'L',
			(KeyboardKey::KeyL, _, true, false) => return '\x0C',
			(KeyboardKey::KeyL, _, false, true) => return '\0',

			(KeyboardKey::KeyM, false, false, false) => return 'm',
			(KeyboardKey::KeyM, true, false, false) => return 'M',
			(KeyboardKey::KeyM, _, true, false) => return '\x0D',
			(KeyboardKey::KeyM, _, false, true) => return '\0',

			(KeyboardKey::KeyN, false, false, false) => return 'n',
			(KeyboardKey::KeyN, true, false, false) => return 'N',
			(KeyboardKey::KeyN, _, true, false) => return '\x0E',
			(KeyboardKey::KeyN, _, false, true) => return '\0',

			(KeyboardKey::KeyO, false, false, false) => return 'o',
			(KeyboardKey::KeyO, true, false, false) => return 'O',
			(KeyboardKey::KeyO, _, true, false) => return '\x0F',
			(KeyboardKey::KeyO, _, false, true) => return '\0',

			(KeyboardKey::KeyP, false, false, false) => return 'p',
			(KeyboardKey::KeyP, true, false, false) => return 'P',
			(KeyboardKey::KeyP, _, true, false) => return '\0',
			(KeyboardKey::KeyP, _, false, true) => return '\0',

			(KeyboardKey::KeyQ, false, false, false) => return 'q',
			(KeyboardKey::KeyQ, true, false, false) => return 'Q',
			(KeyboardKey::KeyQ, _, true, false) => return '\x11',
			(KeyboardKey::KeyQ, _, false, true) => return '\0',

			(KeyboardKey::KeyR, false, false, false) => return 'r',
			(KeyboardKey::KeyR, true, false, false) => return 'R',
			(KeyboardKey::KeyR, _, true, false) => return '\x12',
			(KeyboardKey::KeyR, _, false, true) => return '\0',

			(KeyboardKey::KeyS, false, false, false) => return 's',
			(KeyboardKey::KeyS, true, false, false) => return 'S',
			(KeyboardKey::KeyS, _, true, false) => return '\x13',
			(KeyboardKey::KeyS, _, false, true) => return '\0',

			(KeyboardKey::KeyT, false, false, false) => return 't',
			(KeyboardKey::KeyT, true, false, false) => return 'T',
			(KeyboardKey::KeyT, _, true, false) => return '\x14',
			(KeyboardKey::KeyT, _, false, true) => return '\0',

			(KeyboardKey::KeyU, false, false, false) => return 'u',
			(KeyboardKey::KeyU, true, false, false) => return 'U',
			(KeyboardKey::KeyU, _, true, false) => return '\x15',
			(KeyboardKey::KeyU, _, false, true) => return '\0',

			(KeyboardKey::KeyV, false, false, false) => return 'v',
			(KeyboardKey::KeyV, true, false, false) => return 'V',
			(KeyboardKey::KeyV, _, true, false) => return '\x16',
			(KeyboardKey::KeyV, _, false, true) => return '\0',

			(KeyboardKey::KeyW, false, false, false) => return 'w',
			(KeyboardKey::KeyW, true, false, false) => return 'W',
			(KeyboardKey::KeyW, _, true, false) => return '\x17',
			(KeyboardKey::KeyW, _, false, true) => return '\0',

			(KeyboardKey::KeyX, false, false, false) => return 'x',
			(KeyboardKey::KeyX, true, false, false) => return 'X',
			(KeyboardKey::KeyX, _, true, false) => return '\x18',
			(KeyboardKey::KeyX, _, false, true) => return '\0',

			(KeyboardKey::KeyY, false, false, false) => return 'y',
			(KeyboardKey::KeyY, true, false, false) => return 'Y',
			(KeyboardKey::KeyY, _, true, false) => return '\x19',
			(KeyboardKey::KeyY, _, false, true) => return '\0',

			(KeyboardKey::KeyZ, false, false, false) => return 'z',
			(KeyboardKey::KeyZ, true, false, false) => return 'Z',
			(KeyboardKey::KeyZ, _, true, false) => return '\x1A',
			(KeyboardKey::KeyZ, _, false, true) => return '\0',

			// === NUMBERS ===
			(KeyboardKey::Key1, false, false, false) => return '1',
			(KeyboardKey::Key1, true, false, false) => return '!',
			(KeyboardKey::Key1, _, true, false) => return '\x11',
			(KeyboardKey::Key1, _, false, true) => return '\0',

			(KeyboardKey::Key2, false, false, false) => return '2',
			(KeyboardKey::Key2, true, false, false) => return '@',
			(KeyboardKey::Key2, _, true, false) => return '\x12',
			(KeyboardKey::Key2, _, false, true) => return '\0',

			(KeyboardKey::Key3, false, false, false) => return '3',
			(KeyboardKey::Key3, true, false, false) => return '#',
			(KeyboardKey::Key3, _, true, false) => return '\x13',
			(KeyboardKey::Key3, _, false, true) => return '\0',

			(KeyboardKey::Key4, false, false, false) => return '4',
			(KeyboardKey::Key4, true, false, false) => return '$',
			(KeyboardKey::Key4, _, true, false) => return '\x14',
			(KeyboardKey::Key4, _, false, true) => return '\0',

			(KeyboardKey::Key5, false, false, false) => return '5',
			(KeyboardKey::Key5, true, false, false) => return '%',
			(KeyboardKey::Key5, _, true, false) => return '\x15',
			(KeyboardKey::Key5, _, false, true) => return '\0',

			(KeyboardKey::Key6, false, false, false) => return '6',
			(KeyboardKey::Key6, true, false, false) => return '^',
			(KeyboardKey::Key6, _, true, false) => return '\x16',
			(KeyboardKey::Key6, _, false, true) => return '\0',

			(KeyboardKey::Key7, false, false, false) => return '7',
			(KeyboardKey::Key7, true, false, false) => return '&',
			(KeyboardKey::Key7, _, true, false) => return '\x17',
			(KeyboardKey::Key7, _, false, true) => return '\0',

			(KeyboardKey::Key8, false, false, false) => return '8',
			(KeyboardKey::Key8, true, false, false) => return '*',
			(KeyboardKey::Key8, _, true, false) => return '\x18',
			(KeyboardKey::Key8, _, false, true) => return '\0',

			(KeyboardKey::Key9, false, false, false) => return '9',
			(KeyboardKey::Key9, true, false, false) => return '(',
			(KeyboardKey::Key9, _, true, false) => return '\x19',
			(KeyboardKey::Key9, _, false, true) => return '\0',

			(KeyboardKey::Key0, false, false, false) => return '0',
			(KeyboardKey::Key0, true, false, false) => return ')',
			(KeyboardKey::Key0, _, true, false) => return '\x10',
			(KeyboardKey::Key0, _, false, true) => return '\0',

			// === SPECIAL CHARACTERS ===
			(KeyboardKey::KeyMinus, false, false, false) => return '-',
			(KeyboardKey::KeyMinus, true, false, false) => return '_',
			(KeyboardKey::KeyMinus, _, true, false) => return '\x1F',
			(KeyboardKey::KeyMinus, _, false, true) => return '\0',

			// === WHITESPACE AND CONTROL ===
			(KeyboardKey::KeySpace, _, false, false) => return ' ',
			(KeyboardKey::KeySpace, _, true, false) => return '\0',
			(KeyboardKey::KeySpace, _, false, true) => return '\0',

			(KeyboardKey::KeyEnter, _, false, false) => return '\n',
			(KeyboardKey::KeyEnter, _, true, false) => return '\n',
			(KeyboardKey::KeyEnter, _, false, true) => return '\0',

			(KeyboardKey::KeyTab, _, false, false) => return '\t',
			(KeyboardKey::KeyTab, _, true, false) => return '\t',
			(KeyboardKey::KeyTab, _, false, true) => return '\0',

			(KeyboardKey::KeyBackspace, _, false, false) => return '\x08',
			(KeyboardKey::KeyBackspace, _, true, false) => return '\x08',
			(KeyboardKey::KeyBackspace, _, false, true) => return '\0',

			// === FUNCTION KEYS (no ASCII output) ===
			(KeyboardKey::KeyF1, ..) => return '\0',
			(KeyboardKey::KeyF2, ..) => return '\0',
			(KeyboardKey::KeyF3, ..) => return '\0',
			(KeyboardKey::KeyF4, ..) => return '\0',
			(KeyboardKey::KeyF5, ..) => return '\0',
			(KeyboardKey::KeyF6, ..) => return '\0',
			(KeyboardKey::KeyF7, ..) => return '\0',
			(KeyboardKey::KeyF8, ..) => return '\0',
			(KeyboardKey::KeyF9, ..) => return '\0',
			(KeyboardKey::KeyF10, ..) => return '\0',
			(KeyboardKey::KeyF11, ..) => return '\0',
			(KeyboardKey::KeyF12, ..) => return '\0',

			// === KEYPAD KEYS ===
			(KeyboardKey::KeyKeypad0, false, false, false) => return '0',
			(KeyboardKey::KeyKeypad1, false, false, false) => return '1',
			(KeyboardKey::KeyKeypad2, false, false, false) => return '2',
			(KeyboardKey::KeyKeypad3, false, false, false) => return '3',
			(KeyboardKey::KeyKeypad4, false, false, false) => return '4',
			(KeyboardKey::KeyKeypad5, false, false, false) => return '5',
			(KeyboardKey::KeyKeypad6, false, false, false) => return '6',
			(KeyboardKey::KeyKeypad7, false, false, false) => return '7',
			(KeyboardKey::KeyKeypad8, false, false, false) => return '8',
			(KeyboardKey::KeyKeypad9, false, false, false) => return '9',
			(KeyboardKey::KeyKeypadDot, false, false, false) => return '.',
			(KeyboardKey::KeyKeypadStar, false, false, false) => return '*',
			(KeyboardKey::KeyKeypadMinus, false, false, false) => return '-',
			(KeyboardKey::KeyKeypadPlus, false, false, false) => return '+',
			(KeyboardKey::KeyKeypadSlash, false, false, false) => return '/',
			(KeyboardKey::KeyKeypadEnter, false, false, false) => return '\n',

			// === MODIFIER KEYS (no output) ===
			(KeyboardKey::KeyLeftShift, ..) => return '\0',
			(KeyboardKey::KeyRightShift, ..) => return '\0',
			(KeyboardKey::KeyLeftControl, ..) => return '\0',
			(KeyboardKey::KeyRightControl, ..) => return '\0',
			(KeyboardKey::KeyLeftAlt, ..) => return '\0',
			(KeyboardKey::KeyRightAlt, ..) => return '\0',
			(KeyboardKey::KeyLeftGUI, ..) => return '\0',
			(KeyboardKey::KeyRightGUI, ..) => return '\0',

			// === SPECIAL KEYS (no output) ===
			(KeyboardKey::KeyPrintScreen, ..) => return '\0',
			(KeyboardKey::KeyScrollLock, ..) => return '\0',
			(KeyboardKey::KeyPause, ..) => return '\0',
			(KeyboardKey::KeyInsert, ..) => return '\0',
			(KeyboardKey::KeyDelete, ..) => return '\0',
			(KeyboardKey::KeyHome, ..) => return '\0',
			(KeyboardKey::KeyEnd, ..) => return '\0',
			(KeyboardKey::KeyPageUp, ..) => return '\0',
			(KeyboardKey::KeyPageDown, ..) => return '\0',
			(KeyboardKey::KeyCursorUp, ..) => return '\0',
			(KeyboardKey::KeyCursorDown, ..) => return '\0',
			(KeyboardKey::KeyCursorLeft, ..) => return '\0',
			(KeyboardKey::KeyCursorRight, ..) => return '\0',

			// Catch any unhandled combinations
			_ => return '\0',
		}
	}

	// TODO: Clean up code
	pub fn input(&mut self) -> Option<char> {
		const KEYBOARD_DATA_PORT: u16 = 0x60;
		const KEYBOARD_STATUS_PORT: u16 = 0x64;

		if io::inb(KEYBOARD_STATUS_PORT) & 1 == 0 {
			return None;
		}

		let scan_code = io::inb(KEYBOARD_DATA_PORT);

		// Alt Pressed
		if scan_code == 56 {
			self.alt_pressed = true;
			return None;
		}

		// Alt Released
		if scan_code == 184 {
			self.alt_pressed = false;
			return None;
		}

		// Ctrl Pressed
		if scan_code == 29 {
			self.ctrl_pressed = true;
			return None;
		}

		// Ctrl Released
		if scan_code == 157 {
			self.ctrl_pressed = false;
			return None;
		}

		// Shift Pressed
		if scan_code == 42 {
			self.shift_pressed = true;
			return None;
		}

		// Shift Released
		if scan_code == 170 {
			self.shift_pressed = false;
			return None;
		}

		if scan_code >= 0x80 {
			return None;
		}

		let c = self.get_ascii(scan_code);

		return Some(c);
	}
}
