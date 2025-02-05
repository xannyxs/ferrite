#[cfg(not(any(target_arch = "x86")))]
compile_error!("This code needs to be compiled for i686/x86!");

#[cfg(not(target_pointer_width = "32"))]
compile_error!("This code needs to be compiled for 32-bit architecture!");
