LD=ld
LIBTOOL=libtool
MTOC=mtoc
EDK2:=~/code/ext/edk2
GENFW=$(EDK2)/BaseTools/Source/C/bin/GenFw
RUST_PATH:=$(HOME)/rust
RUSTC=$(RUST_PATH)/bin/rustc
LIBCORE=$(RUST_PATH)/lib/rustlib/x86_64-apple-darwin/lib/libcore-c5ed6fb4-0.11.0-pre.rlib

BUILDDIR=build
VPATH=src

OBJECTS=build/main.o
LINKER_FLAGS=-static -arch x86_64 -u __ModuleEntryPoint -e __ModuleEntryPoint -pie -dead_strip -seg1addr 0x1000 \
				-pagezero_size 0 -macosx_version_min 10.8

all: brundlefly.efi

.PHONY: clean run

$(BUILDDIR)/%.o: %.rs
	mkdir -p build
	$(RUSTC) -Auppercase_variables -Anon_camel_case_types -O --crate-type=lib -o $(BUILDDIR)/brundlefly.lib $< 

brundlefly.efi: $(OBJECTS)
	mkdir -p build
	cd build && \
	$(LD) $(LINKER_FLAGS) -o brundlefly.dll brundlefly.lib $(LIBCORE) && \
	$(MTOC) -subsystem UEFI_APPLICATION -align 0x20 -d brundlefly.dll brundlefly.dll brundlefly.pecoff && \
	$(GENFW) -e UEFI_APPLICATION -o brundlefly.efi brundlefly.pecoff

clean:
	rm -rf build
