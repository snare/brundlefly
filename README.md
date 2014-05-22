brundlefly
==========

An unholy marriage of UEFI and Rust.

Prerequisites
-------------

This build process will only work on OS X. You'll need a couple of tools to build this monstrosity - namely `mtoc` from Apple's `cctools` package, and `GenFw` from the EFI Development Kit.

`cctools` used to pretty easy to build, but it doesn't work for me at the moment. Easiest way I've found is just to install it from `brew`:

	$ brew install cctools

The EDK2 `BaseTools` package doesn't seem to build properly with my current toolchain either. I've had various issues with it in the past, but we can easily patch the Makefile to make it build. Check out the EDK2, patch the Makefile so it works, and build the `BaseTools`:

	$ git clone https://github.com/tianocore/edk2
	$ cd edk2/BaseTools/Source/C
	$ perl -pi -e 's/-fno-merge-constants//g' Makefiles/header.makefile
	$ make

It might error out, but it should have already built the one tool we need.

Whatever the path to the EDK2 is you'll need to note down and pass it to `make`. I have it in `~/code/ext/edk2`.

Building
--------

Once you have the pre-reqs ready, you should be able to build with just a `make` in the `brundlefly` dir, passing it the path to the EDK:

	$ make EDK2=~/code/ext/edk2
	rustc -O --crate-type lib -o main.o --emit obj main.rs
	main.rs:45:41: 45:52 warning: unused variable: `imageHandle`, #[warn(unused_variable)] on by default
	main.rs:45 pub extern "win64" fn _ModuleEntryPoint(imageHandle: u64, systemTable: *EfiSystemTable) -> int {
	                                                   ^~~~~~~~~~~
	libtool -static -o brundlefly.lib main.o
	ld -arch x86_64 -u __ModuleEntryPoint -e __ModuleEntryPoint -preload  -pie -all_load -dead_strip -seg1addr 0x260 -o brundlefly.dll brundlefly.lib
	ld: warning: -seg1addr not 4096 byte aligned, rounding up
	mtoc -subsystem UEFI_APPLICATION  -align 0x20 -d brundlefly.dll brundlefly.dll brundlefly.pecoff
	~/code/ext/edk2/BaseTools/Source/C/bin/GenFw -e UEFI_APPLICATION -o brundlefly.efi brundlefly.pecoff

Running
-------

You should now have `brundlefly.efi` which you can run from the UEFI shell. If you care enough to try this, you probably already know how that works.