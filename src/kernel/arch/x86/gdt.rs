pub struct Gate(pub u64);

impl Gate {
	pub fn new(base: u32, limit: u32, access: u8, flags: u8) -> Self {
		let mut c = Self(0);
		c.set_base(base);
		c.set_limit(limit);
		c.set_access(access);
		c.set_flags(flags);

		c
	}

	pub fn get_base(&mut self) -> u32 {
		(((self.0 >> 16) & 0xffffff) | ((self.0 >> 56) & 0xff) << 24) as u32
	}

	pub fn set_base(&mut self, base: u32) {
		self.0 &= !(0xffffff << 16);
		self.0 &= !(0xff << 56);

		self.0 |= (base as u64 & 0xffffff) << 16;
		self.0 |= ((base as u64 >> 24) & 0xff) << 56;
	}

	pub fn get_limit(&mut self) -> u32 {
		((self.0 & 0xffff) | (((self.0 >> 48) & 0xf) << 16)) as u32
	}

	pub fn set_limit(&mut self, limit: u32) {
		self.0 &= !0xffff;
		self.0 &= !(0xf << 48);

		self.0 |= limit as u64 & 0xffff;
		self.0 |= ((limit as u64 >> 16) & 0xf) << 48;
	}

	pub fn get_access(&mut self) -> u8 {
		(self.0 >> 40) as u8
	}

	pub fn set_access(&mut self, access: u8) {
		self.0 &= !(0xff << 40);
		self.0 |= (access as u64) << 40;
	}

	pub fn get_flags(&mut self) -> u8 {
		((self.0 >> 52) & 0x0f) as u8
	}

	pub fn set_flags(&mut self, flags: u8) {
		self.0 &= !(0xf << 52);
		self.0 |= (flags as u64) << 52;
	}
}
