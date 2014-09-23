LD=ld
LIBTOOL=libtool
MTOC=mtoc
EDK2:=~/code/ext/edk2
GENFW=$(EDK2)/BaseTools/Source/C/bin/GenFw
RUST_PATH?=$(HOME)/rust
RUSTC=$(RUST_PATH)/bin/rustc
RUSTC_FLAGS=-Anon_camel_case_types -Aunused_variable -Aunused_imports -O --crate-type=lib
RUST_LIB_DIR=$(RUST_PATH)/lib/rustlib/x86_64-apple-darwin/lib
RUST_LIBS=$(RUST_LIB_DIR)/libcore-c5ed6fb4-0.11.0-pre.rlib $(RUST_LIB_DIR)/libcompiler-rt.a

BUILDDIR=build
RUNDIR=run
BOOTDIR=$(RUNDIR)/efi/boot
VPATH=src

OBJECTS=build/main.o
LINKER_FLAGS=-static -arch x86_64 -u __ModuleEntryPoint -e __ModuleEntryPoint -pie -dead_strip -seg1addr 0x1000 \
				-pagezero_size 0 -macosx_version_min 10.8

all: brundlefly.efi

.PHONY: clean run

$(BUILDDIR)/%.o: %.rs
	mkdir -p build
	$(RUSTC) $(RUSTC_FLAGS) -o $(BUILDDIR)/brundlefly.lib $<

brundlefly.efi: $(OBJECTS)
	mkdir -p build
	cd build && \
	$(LD) $(LINKER_FLAGS) -o brundlefly.dll brundlefly.lib $(RUST_LIBS) && \
	$(MTOC) -subsystem UEFI_APPLICATION -align 0x20 -d brundlefly.dll brundlefly.dll brundlefly.pecoff && \
	$(GENFW) -e UEFI_APPLICATION -o brundlefly.efi brundlefly.pecoff

clean:
	rm -rf build run

run:
	mkdir -p $(BOOTDIR)
	cp $(BUILDDIR)/brundlefly.efi $(BOOTDIR)/bootx64.efi
	qemu-system-x86_64 -bios bios.bin -k en-us -hda fat:$(RUNDIR)
