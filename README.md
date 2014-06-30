brundlefly
==========

An unholy marriage of UEFI and Rust. It doesn't actually do much, it's just a PoC of a basic EFI executable in Rust. Inspired by @charliesome's [rustboot](https://github.com/charliesome/rustboot).

Prerequisites
-------------

This build process will only work on OS X. You'll need a couple of tools to build this monstrosity - namely `mtoc` from Apple's `cctools` package, and `GenFw` from the EFI Development Kit.

`cctools` used to pretty easy to build, but it doesn't work for me at the moment. Easiest way I've found is just to install it from `brew`:

    $ brew tap henrik-muehe/homebrew-hyper
	$ brew install cctools

The EDK2 `BaseTools` package doesn't seem to build properly with my current toolchain either. I've had various issues with it in the past, but we can easily patch the Makefile to make it build. Check out the EDK2, patch the Makefile so it works, and build the `BaseTools`:

	$ git clone https://github.com/tianocore/edk2
	$ cd edk2/BaseTools/Source/C
	$ perl -pi -e 's/-fno-merge-constants//g' Makefiles/header.makefile
	$ make

It might error out, but it should have already built the one tool we need.

Whatever the path to the EDK2 is you'll need to note down and pass it to `make`. I have it in `~/code/ext/edk2`.

If you want to run this in `qemu` you'll need to install it:

	$ brew install qemu

Of course, you'll also need the Rust toolchain. I build it from [source](https://github.com/mozilla/rust).

Building
--------

Once you have the pre-reqs ready, you should be able to build with just a `make` in the `brundlefly` dir, passing it the path to the EDK:

	$ make EDK2=~/code/ext/edk2

You should now have `brundlefly.efi` in the `build` directory.

Running
-------

If you have `qemu` installed you can run with:

	$ make run

This uses an included binary of OVMF borrowed from [here](http://people.canonical.com/~jk/ovmf/).
