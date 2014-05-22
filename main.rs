#![no_std]
#![no_main]

// Most of these function defs are not right, only OutputString() is real
struct EfiSimpleTextOutputProtocol {
    reset:              (extern "win64" fn() -> u64),
    outputString:       (extern "win64" fn(this: *EfiSimpleTextOutputProtocol, string: &str) -> u64),
    testString:         (extern "win64" fn() -> u64),
    queryMode:          (extern "win64" fn() -> u64),
    setMode:            (extern "win64" fn() -> u64),
    setAttribute:       (extern "win64" fn() -> u64),
    clearScreen:        (extern "win64" fn() -> u64),
    setCursorPosition:  (extern "win64" fn() -> u64),
    enableCursor:       (extern "win64" fn() -> u64),
    mode:               *u64
}

struct EfiTableHeader {
    signature:  u64,
    revision:   u32,
    headerSize: u32,
    crc32:      u32,
    reserved:   u32
}

// All the *u64's are pointers to stuff I don't really need atm and cbf defining structs for
pub struct EfiSystemTable {
    hdr:                    EfiTableHeader,
    firmwareVendor:         *u64,
    revision:               u32,

    consoleInHandle:        *u64,
    conIn:                  *u64,
    consoleOutHandle:       *u64,
    conOut:                 *EfiSimpleTextOutputProtocol,
    standardErrorHandle:    *u64,
    stdErr:                 *u64,

    numberOfTableEntries:   *u64,
    configurationTable:     *u64
}

#[no_mangle]
#[no_split_stack]
pub extern "win64" fn _ModuleEntryPoint(imageHandle: u64, systemTable: *EfiSystemTable) -> int {
    unsafe {
        // get the Simple Text Output Protool instance on the console output handle
        let conOut = (*systemTable).conOut;
        // get the OutputString function
        let outputString = (*conOut).outputString;
        // make a terrible string (lol no utf-16 in the rust core)
        let string = "r\x00u\x00s\x00t\x00y\x00b\x00u\x00t\x00t\x00s\x00\n\x00";
        // print it
        outputString(conOut, string);
    }
    0
}
