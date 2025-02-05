#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86;

/// The name of the current CPU architecture.
#[allow(dead_code)]
pub const ARCH: &str = {
	#[cfg(target_arch = "x86")]
	{
		"x86"
	}
	#[cfg(target_arch = "x86_64")]
	{
		"x86_64"
	}
};
