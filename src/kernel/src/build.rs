use std::{
	env, fs,
	path::Path,
	process::{exit, Command},
};

fn compile_asm(out_dir: &String) {
	let arch_dir = Path::new("../arch/x86");
	let asm_files = fs::read_dir(arch_dir).unwrap_or_else(|e| {
		eprint!("Failed to read directory: {}", e);
		exit(1);
	});

	for entry in asm_files {
		let path = entry.unwrap().path();

		if path.extension().and_then(|s| s.to_str()) == Some("asm") {
			let file_stem = path.file_stem().unwrap().to_str().unwrap();
			let output = format!("{}/{}.o", out_dir, file_stem);

			println!("cargo:warning=Compiling {}", path.display());

			Command::new("nasm")
				.args(&["-f", "elf32", path.to_str().unwrap(), "-o", &output])
				.status()
				.expect("Could not compile NASM correctly");

			println!("cargo:rustc-link-arg={}", output);
		}
	}
}

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap_or_else(|e| {
		eprint!("{}", e);
		exit(1);
	});

	compile_asm(&out_dir);

	// Tell cargo where to find our objects
	println!("cargo:rustc-link-search={}", out_dir);

	// Linker arguments
	println!("cargo:rustc-link-arg=-m");
	println!("cargo:rustc-link-arg=elf_i386");
	println!("cargo:rustc-link-arg=--no-dynamic-linker");
	println!("cargo:rustc-link-arg=-static");
	println!("cargo:rustc-link-arg=-T../arch/x86/x86.ld");

	// Watch for changes
	println!("cargo:rerun-if-changed=../arch/x86/test_gdt.asm");
	println!("cargo:rerun-if-changed=../arch/x86/gdt.asm");
	println!("cargo:rerun-if-changed=../arch/x86/boot.asm");
	println!("cargo:rerun-if-changed=../arch/x86/x86.ld");
}
