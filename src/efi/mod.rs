//
// Types
//

extern crate core;
use core::option::{None, Some};
use core::iter::Iterator;
use core::str::StrSlice;
use core::intrinsics::transmute;
use core::intrinsics::offset;

pub type EfiHandle = *();
pub type EfiPlaceholderFunction = (extern "win64" fn() -> u64);

//
// Errors
//

pub enum EfiStatus {
    EFI_SUCCESS                 = 0,
    EFI_LOAD_ERROR              = (0x80000000 | 1),
    EFI_INVALID_PARAMETER       = (0x80000000 | 2),
    EFI_UNSUPPORTED             = (0x80000000 | 3),
    EFI_BAD_BUFFER_SIZE         = (0x80000000 | 4),
    EFI_BUFFER_TOO_SMALL        = (0x80000000 | 5),
    EFI_NOT_READY               = (0x80000000 | 6),
    EFI_DEVICE_ERROR            = (0x80000000 | 7),
    EFI_WRITE_PROTECTED         = (0x80000000 | 8),
    EFI_OUT_OF_RESOURCES        = (0x80000000 | 9),
    EFI_VOLUME_CORRUPTED        = (0x80000000 | 10),
    EFI_VOLUME_FULL             = (0x80000000 | 11),
    EFI_NO_MEDIA                = (0x80000000 | 12),
    EFI_MEDIA_CHANGED           = (0x80000000 | 13),
    EFI_NOT_FOUND               = (0x80000000 | 14),
    EFI_ACCESS_DENIED           = (0x80000000 | 15),
    EFI_NO_RESPONSE             = (0x80000000 | 16),
    EFI_NO_MAPPING              = (0x80000000 | 17),
    EFI_TIMEOUT                 = (0x80000000 | 18),
    EFI_NOT_STARTED             = (0x80000000 | 19),
    EFI_ALREADY_STARTED         = (0x80000000 | 20),
    EFI_ABORTED                 = (0x80000000 | 21),
    EFI_ICMP_ERROR              = (0x80000000 | 22),
    EFI_TFTP_ERROR              = (0x80000000 | 23),
    EFI_PROTOCOL_ERROR          = (0x80000000 | 24),
    EFI_INCOMPATIBLE_VERSION    = (0x80000000 | 25),
    EFI_SECURITY_VIOLATION      = (0x80000000 | 26),
    EFI_CRC_ERROR               = (0x80000000 | 27),
    EFI_END_OF_MEDIA            = (0x80000000 | 28),
    EFI_END_OF_FILE             = (0x80000000 | 31)
}

//
// Main EFI tables
//

pub struct EfiTableHeader {
    pub Signature:                          u64,
    pub Revision:                           u32,
    pub HeaderSize:                         u32,
    pub Crc32:                              u32,
    pub Reserved:                           u32
}

// All the *u64's are pointers to stuff I don't really need atm and cbf defining structs for
pub struct EfiSystemTable {
    pub Hdr:                                EfiTableHeader,
    pub FirmwareVendor:                     *u16,
    pub Revision:                           u32,

    pub ConsoleInHandle:                    *EfiHandle,
    pub ConIn:                              *EfiSimpleTextInputProtocol,
    pub ConsoleOutHandle:                   *EfiHandle,
    pub ConOut:                             *EfiSimpleTextOutputProtocol,
    pub StandardErrorHandle:                *EfiHandle,
    pub StdErr:                             *EfiSimpleTextOutputProtocol,

    pub RuntimeServices:                    *EfiRuntimeServicesTable,
    pub BootServices:                       *EfiBootServicesTable,

    pub NumberOfTableEntries:               u64,
    pub ConfigurationTable:                 *EfiConfigurationTable
}

// Runtime Services table
pub struct EfiRuntimeServicesTable {
    pub Hdr:                                EfiTableHeader,
    pub GetTime:                            EfiPlaceholderFunction,
    pub SetTime:                            EfiPlaceholderFunction,
    pub GetWakeupTime:                      EfiPlaceholderFunction,
    pub SetWakeupTime:                      EfiPlaceholderFunction,
    pub SetVirtualAddressMap:               EfiPlaceholderFunction,
    pub ConvertPointer:                     EfiPlaceholderFunction,
    pub GetVariable:                        EfiPlaceholderFunction,
    pub GetNextVariableName:                EfiPlaceholderFunction,
    pub SetVariable:                        EfiPlaceholderFunction,
    pub GetNextHighMonotonicCount:          EfiPlaceholderFunction,
    pub ResetSystem:                        EfiPlaceholderFunction,
    pub UpdateCapsule:                      EfiPlaceholderFunction,
    pub QueryCapsuleCapabilities:           EfiPlaceholderFunction,
    pub QueryVariableInfo:                  EfiPlaceholderFunction
}

// Boot Services table
pub struct EfiBootServicesTable {
    pub Hdr:                                EfiTableHeader,
    pub RaiseTPL:                           (extern "win64" fn(newTpl: u64) -> u64),
    pub RestoreTPL:                         (extern "win64" fn(oldTpl: u64) -> u64),
    pub AllocatePages:                      (extern "win64" fn(allocateType: u8, memoryType: u8, pages: u64,
                                                               memory: *u64) -> u64),
    pub FreePages:                          (extern "win64" fn(memory: u64, pages: u64) -> u64),
    pub GetMemoryMap:                       (extern "win64" fn(memoryMapSize: u64, memoryMap: *u64, mapKey: *u64,
                                                               descSize: *u64, descVersion: *u64) -> u64),
    pub AllocatePool:                       (extern "win64" fn(memoryType: u8, size: u64, buffer: **u64) -> u64),
    pub FreePool:                           (extern "win64" fn(buffer: *u64) -> u64),
    pub CreateEvent:                        EfiPlaceholderFunction,
    pub SetTimer:                           EfiPlaceholderFunction,
    pub WaitForEvent:                       EfiPlaceholderFunction,
    pub SignalEvent:                        EfiPlaceholderFunction,
    pub CloseEvent:                         EfiPlaceholderFunction,
    pub CheckEvent:                         EfiPlaceholderFunction,
    pub InstallProtocolInterface:           EfiPlaceholderFunction,
    pub ReinstallProtocolInterface:         EfiPlaceholderFunction,
    pub UninstallProtocolInterface:         EfiPlaceholderFunction,
    pub HandleProtocol:                     (extern "win64" fn(handle: EfiHandle, protocol: *EfiGuid,
                                                               interface: **u64) -> u64),
    pub Reserved:                           u64,
    pub RegisterProtocolNotify:             EfiPlaceholderFunction,
    pub LocateHandle:                       (extern "win64" fn(searchType: u8, protocol: *EfiGuid, searchKey: *u64,
                                                               bufferSize: *u64, buffer: *u64) -> u64),
    pub LocateDevicePath:                   EfiPlaceholderFunction,
    pub InstallConfigurationTable:          EfiPlaceholderFunction,
    pub LoadImage:                          EfiPlaceholderFunction,
    pub StartImage:                         EfiPlaceholderFunction,
    pub Exit:                               EfiPlaceholderFunction,
    pub UnloadImage:                        EfiPlaceholderFunction,
    pub ExitBootServices:                   (extern "win64" fn(imageHandle: EfiHandle, mapKey: u64) -> u64),
    pub GetNextMonotonicCount:              EfiPlaceholderFunction,
    pub Stall:                              (extern "win64" fn(microseconds: u64) -> u64),
    pub SetWatchdogTimer:                   EfiPlaceholderFunction,
    pub ConnectController:                  EfiPlaceholderFunction,
    pub DisconnectController:               EfiPlaceholderFunction,
    pub OpenProtocol:                       (extern "win64" fn(handle: EfiHandle, protocol: *EfiGuid, interface: **u64,
                                                               agentHandle: EfiHandle, controllerHandle: EfiHandle,
                                                               attributes: u32) -> u64),
    pub CloseProtocol:                      (extern "win64" fn(handle: EfiHandle, protocol: *EfiGuid, agentHandle: EfiHandle,
                                                               controllerHandle: EfiHandle) -> u64),
    pub OpenProtocolInformation:            EfiPlaceholderFunction,
    pub ProtocolsPerHandle:                 EfiPlaceholderFunction,
    pub LocateHandleBuffer:                 EfiPlaceholderFunction,
    pub LocateProtocol:                     EfiPlaceholderFunction,
    pub InstallMultipleProtocolInterfaces:  EfiPlaceholderFunction,
    pub UninstallMultipleProtocolInterfaces:EfiPlaceholderFunction,
    pub CalculateCrc32:                     EfiPlaceholderFunction,
    pub CopyMem:                            (extern "win64" fn(destination: *u64, source: *u64) -> u64),
    pub SetMem:                             (extern "win64" fn(buffer: *u64, size: u64, value: u8) -> u64),
    pub CreateEventEx:                      EfiPlaceholderFunction
}

pub struct EfiConfigurationTable;

//
// Some useful protocols and data types
//

pub struct EfiGuid {
    pub Data1:                              u32,
    pub Data2:                              u16,
    pub Data3:                              u16,
    pub Data4:                              u64
}

pub struct EfiSimpleTextOutputProtocol {
    pub Reset:                              (extern "win64" fn(this: *EfiSimpleTextOutputProtocol,
                                                               extendedVerification: bool) -> u64),
    pub OutputString:                       (extern "win64" fn(this: *EfiSimpleTextOutputProtocol,
                                                               string: *u16) -> u64),
    pub TestString:                         (extern "win64" fn(this: *EfiSimpleTextOutputProtocol,
                                                               string: *u16) -> u64),
    pub QueryMode:                          (extern "win64" fn(this: *EfiSimpleTextOutputProtocol,
                                                               modeNumber: u64, columns: &u64, rows: &u64) -> u64),
    pub SetMode:                            (extern "win64" fn(this: *EfiSimpleTextOutputProtocol,
                                                               modeNumber: u64) -> u64),
    pub SetAttribute:                       (extern "win64" fn(this: *EfiSimpleTextOutputProtocol,
                                                               attribute: u64) -> u64),
    pub ClearScreen:                        (extern "win64" fn(this: *EfiSimpleTextOutputProtocol) -> u64),
    pub SetCursorPosition:                  (extern "win64" fn(this: *EfiSimpleTextOutputProtocol,
                                                               column: u64, row: u64) -> u64),
    pub EnableCursor:                       (extern "win64" fn(this: *EfiSimpleTextOutputProtocol,
                                                               visible: bool) -> u64),
    pub Mode:                               *SimpleTextOutputMode
}

pub struct SimpleTextOutputMode {
    pub MaxMode:                            i32,
    pub Mode:                               i32,
    pub Attribute:                          i32,
    pub CursorColumn:                       i32,
    pub CursorRow:                          i32,
    pub CursorVisible:                      bool
}

pub struct EfiSimpleTextInputProtocol;

//
// Main UEFI entry point
//

pub static mut ST: *EfiSystemTable          = 0 as *EfiSystemTable;
pub static mut RT: *EfiRuntimeServicesTable = 0 as *EfiRuntimeServicesTable;
pub static mut BS: *EfiBootServicesTable    = 0 as *EfiBootServicesTable;

#[no_mangle]
#[no_split_stack]
pub extern "win64" fn _ModuleEntryPoint(imageHandle: EfiHandle, systemTable: *EfiSystemTable) -> EfiStatus {
    unsafe {
        ST = systemTable;
        RT = (*systemTable).RuntimeServices;
        BS = (*systemTable).BootServices;
    }

    ::EfiMain(imageHandle, systemTable)
}

//
// Utility functions
//

#[no_split_stack]
pub fn str_to_utf16(string: &str) -> *u16 {
    unsafe {
        let ptr: *u16 = transmute(malloc(string.char_len()*2));

        if ptr as u64 != 0 {
            let mut p = ptr;
            for c in string.chars() {
                *(p as *mut u16) = c as u16;
                p = offset(p, 1);
            }
            *(p as *mut u16) = 0 as u16;
        }

        ptr
    }
}

pub fn print(string: &str) {
    let p = str_to_utf16(string);
    print_utf16(p);
    unsafe { free(transmute(p)); }
}

pub fn print_utf16(string: *u16) {
    unsafe {
        // get the Simple Text Output Protool instance on the console output handle, get the OutputString() function
        let conOut = (*::efi::ST).ConOut;
        let outputString = (*conOut).OutputString;

        // print the string
        outputString(conOut, string);
    }
}

// #[inline]
// #[lang="exchange_malloc"]
pub fn malloc(size: uint) -> *mut u8 {
    unsafe {
        let mut ptr: *u64 = 0 as *u64;
        let status = ((*::efi::BS).AllocatePool)(0, size as u64, &ptr);

        if status != 0 {
            ptr = 0 as *u64;
        }
        ptr as *mut u8
    }
}

// #[inline]
// #[lang="exchange_free"]
pub unsafe fn free(ptr: *u8) {
    unsafe {
        ((*::efi::BS).FreePool)(ptr as *u64);
    }
}


// I have no idea what will happen if these get called. Probably something bad.
#[no_mangle]
#[no_split_stack]
pub fn __morestack() {

}

#[no_mangle]
#[no_split_stack]
pub fn rust_begin_unwind() {
    
}
