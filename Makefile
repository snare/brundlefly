LD=ld
RUSTC=rustc
LIBTOOL=libtool
MTOC=mtoc
EDK2:=~/code/ext/edk2
GENFW=$(EDK2)/BaseTools/Source/C/bin/GenFw

OBJECTS=main.o
LINKER_FLAGS=-arch x86_64 -u __ModuleEntryPoint -e __ModuleEntryPoint -preload  -pie -all_load -dead_strip -seg1addr 0x260

all: brundlefly.efi

.PHONY: clean run

%.o : %.rs
	$(RUSTC) -O --crate-type lib -o $@ --emit obj $<

brundlefly.lib: $(OBJECTS)
	$(LIBTOOL) -static -o $@ $(OBJECTS)

brundlefly.dll: brundlefly.lib
	$(LD) $(LINKER_FLAGS) -o $@ $<

brundlefly.pecoff: brundlefly.dll
	$(MTOC) -subsystem UEFI_APPLICATION  -align 0x20 -d $< $< $@

brundlefly.efi: brundlefly.pecoff
	$(GENFW) -e UEFI_APPLICATION -o $@ $<

clean:
	rm -f *.dll *.lib *.efi *.o *.pecoff brundlefly.txt
