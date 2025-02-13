# KFS 01 - Grub, boot & screen

## Introduction

Our first `KFS` project! I was quite nervous myself at first looking at this. My previous experience with kernels was limited to Linux from Scratch, which barely scratches the surface compared to KFS.

For this project, I chose Rust as my programming language. My experience is limited. I just did the tutorial called [Rustlings](https://github.com/rust-lang/rustlings/). I do have a few years of experience in C/C++ & Assembly, which will definitly be useful.

## Goals

- A kernel that can boot via Grub
- An `ASM` bootable base
- A basic Kernel Library
- Show "42" on screen

Simple enough, right? (_It wasn't_)

## Technical Approach & Implimentation

My approach was quite straightforward for this project. Read, read & READ! I primarily started reading [OSDev](https://wiki.osdev.org/Expanded_Main_Page). It offers good guidance on kernel development.

I started following OSDev's straightforward tutorial for OS booting in C. Having my own `libc` implementation made this phase quite smooth. With ease, I was able to make a system bootable.

After that, I had some basic knowledge on booting up a system via GRUB. Now was the challange to convert it to Rust. Luckily, [Philipp Oppermann's blog](https://os.phil-opp.com/) helped me immensily! It gave me more insight on how to setup a Rust envirnment. I just had to figure out how to change that to `x86_32`, since their tutorial is meant for `x86_64`.

After that I noticed Mr. Oppermann having a second tutorial on VGA; how to set it up and print to it, which is one of the requirements. I finished it and mostly finished `KFS_01`. I just had to put the dots on the i, and cross the t's.

## Challenges

The biggest of this project was understanding the `nix-shell`, the targeting system of Rust & booting up the Rust kernel in `x86_32`.

You may have noticed that I am using `nix-shell`. The reason is simply to make it easier for a developer to start in the correct environment. Once `nix-shell` is setup it ensures you are always on the correct version with the correct programs. You will have less of the _"It works on my machine"_. The main challenge was to setup the `nix-shell`, since the documentation of Nix is quite limited. It was just trying a lot of things until it worked.

Secondly, the Rust targeting system was quite vague for me. The main challenge for me was to understand you need a `target.json` for your own specifications. I do think this is the superior approach, but I was used to `gcc`, where you have to compile it yourself. It took me a bit of time until I understood that you do not have to compile `rustc` from scratch, but give it a `target.json` to give your bare-metal code.

Lastly, booting up in 32-bit was such an ass. In the end, I am still not sure what went entirely wrong. The Rust code itself worked, but there was something wrong with my `boot.asm` & `linker.ld`. It was not correctly setup by the Linker to let the BIOS know where to find my `kernel_main()`. In the end, I just had to change something of my `boot.asm`. Which worked out.

## Conclusion & Lesson Learned

In the end, it went much smoother than expected. There were plenty of tutorials and understandable documentation to get me through the first project.

The lesson I learned was to not assume the same approach for each compiler. Each programming language has a different approach. I am still happy with my choice to use `nix-shell`. It will definitely avoid headaches in the future.
