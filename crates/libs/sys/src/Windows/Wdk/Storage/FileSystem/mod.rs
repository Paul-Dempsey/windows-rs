::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn AcceptSecurityContext(phcredential : *const SecHandle, phcontext : *const SecHandle, pinput : *const SecBufferDesc, fcontextreq : u32, targetdatarep : u32, phnewcontext : *mut SecHandle, poutput : *mut SecBufferDesc, pfcontextattr : *mut u32, ptsexpiry : *mut i64) -> ::windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn AcquireCredentialsHandleW(pprincipal : *const super::super::super::Win32::Foundation:: UNICODE_STRING, ppackage : *const super::super::super::Win32::Foundation:: UNICODE_STRING, fcredentialuse : u32, pvlogonid : *const ::core::ffi::c_void, pauthdata : *const ::core::ffi::c_void, pgetkeyfn : SEC_GET_KEY_FN, pvgetkeyargument : *const ::core::ffi::c_void, phcredential : *mut SecHandle, ptsexpiry : *mut i64) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn AddCredentialsA(hcredentials : *const SecHandle, pszprincipal : ::windows_sys::core::PCSTR, pszpackage : ::windows_sys::core::PCSTR, fcredentialuse : u32, pauthdata : *const ::core::ffi::c_void, pgetkeyfn : SEC_GET_KEY_FN, pvgetkeyargument : *const ::core::ffi::c_void, ptsexpiry : *mut i64) -> ::windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn AddCredentialsW(hcredentials : *const SecHandle, pprincipal : *const super::super::super::Win32::Foundation:: UNICODE_STRING, ppackage : *const super::super::super::Win32::Foundation:: UNICODE_STRING, fcredentialuse : u32, pauthdata : *const ::core::ffi::c_void, pgetkeyfn : SEC_GET_KEY_FN, pvgetkeyargument : *const ::core::ffi::c_void, ptsexpiry : *mut i64) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn ApplyControlToken(phcontext : *const SecHandle, pinput : *const SecBufferDesc) -> ::windows_sys::core::HRESULT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcAsyncCopyRead(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *mut ::core::ffi::c_void, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, ioissuerthread : *const super::super::Foundation:: KTHREAD, asyncreadcontext : *const CC_ASYNC_READ_CONTEXT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcCanIWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, bytestowrite : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, retrying : u8) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcCoherencyFlushAndPurgeCache(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, fileoffset : *const i64, length : u32, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, flags : u32) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcCopyRead(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *mut ::core::ffi::c_void, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcCopyReadEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *mut ::core::ffi::c_void, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, ioissuerthread : *const super::super::Foundation:: KTHREAD) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcCopyWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcCopyWriteEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *const ::core::ffi::c_void, ioissuerthread : *const super::super::Foundation:: KTHREAD) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcCopyWriteWontFlush(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcDeferWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, postroutine : PCC_POST_DEFERRED_WRITE, context1 : *const ::core::ffi::c_void, context2 : *const ::core::ffi::c_void, bytestowrite : u32, retrying : super::super::super::Win32::Foundation:: BOOLEAN) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn CcErrorCallbackRoutine(context : *const CC_ERROR_CALLBACK_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcFastCopyRead(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : u32, length : u32, pagecount : u32, buffer : *mut ::core::ffi::c_void, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcFastCopyWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : u32, length : u32, buffer : *const ::core::ffi::c_void) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcFlushCache(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, fileoffset : *const i64, length : u32, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcGetDirtyPages(loghandle : *const ::core::ffi::c_void, dirtypageroutine : PDIRTY_PAGE_ROUTINE, context1 : *const ::core::ffi::c_void, context2 : *const ::core::ffi::c_void) -> i64);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcGetFileObjectFromBcb(bcb : *const ::core::ffi::c_void) -> *mut super::super::Foundation:: FILE_OBJECT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcGetFileObjectFromSectionPtrs(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS) -> *mut super::super::Foundation:: FILE_OBJECT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcGetFileObjectFromSectionPtrsRef(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS) -> *mut super::super::Foundation:: FILE_OBJECT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn CcGetFlushedValidData(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, bcblistheld : super::super::super::Win32::Foundation:: BOOLEAN) -> i64);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcInitializeCacheMap(fileobject : *const super::super::Foundation:: FILE_OBJECT, filesizes : *const CC_FILE_SIZES, pinaccess : super::super::super::Win32::Foundation:: BOOLEAN, callbacks : *const CACHE_MANAGER_CALLBACKS, lazywritecontext : *const ::core::ffi::c_void) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcInitializeCacheMapEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, filesizes : *const CC_FILE_SIZES, pinaccess : super::super::super::Win32::Foundation:: BOOLEAN, callbacks : *const CACHE_MANAGER_CALLBACKS, lazywritecontext : *const ::core::ffi::c_void, flags : u32) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn CcIsCacheManagerCallbackNeeded(status : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcIsThereDirtyData(vpb : *const super::super::Foundation:: VPB) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcIsThereDirtyDataEx(vpb : *const super::super::Foundation:: VPB, numberofdirtypages : *const u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcMapData(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, flags : u32, bcb : *mut *mut ::core::ffi::c_void, buffer : *mut *mut ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcMdlRead(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, mdlchain : *mut *mut super::super::super::Win32::Graphics::DirectDraw:: MDL, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcMdlReadComplete(fileobject : *const super::super::Foundation:: FILE_OBJECT, mdlchain : *const super::super::super::Win32::Graphics::DirectDraw:: MDL) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcMdlWriteAbort(fileobject : *const super::super::Foundation:: FILE_OBJECT, mdlchain : *const super::super::super::Win32::Graphics::DirectDraw:: MDL) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcMdlWriteComplete(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, mdlchain : *const super::super::super::Win32::Graphics::DirectDraw:: MDL) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcPinMappedData(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, flags : u32, bcb : *mut *mut ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcPinRead(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, flags : u32, bcb : *mut *mut ::core::ffi::c_void, buffer : *mut *mut ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcPrepareMdlWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, mdlchain : *mut *mut super::super::super::Win32::Graphics::DirectDraw:: MDL, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcPreparePinWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, zero : super::super::super::Win32::Foundation:: BOOLEAN, flags : u32, bcb : *mut *mut ::core::ffi::c_void, buffer : *mut *mut ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn CcPurgeCacheSection(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, fileoffset : *const i64, length : u32, flags : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn CcRemapBcb(bcb : *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn CcRepinBcb(bcb : *const ::core::ffi::c_void) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcScheduleReadAhead(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcScheduleReadAheadEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, ioissuerthread : *const super::super::Foundation:: KTHREAD) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcSetAdditionalCacheAttributes(fileobject : *const super::super::Foundation:: FILE_OBJECT, disablereadahead : super::super::super::Win32::Foundation:: BOOLEAN, disablewritebehind : super::super::super::Win32::Foundation:: BOOLEAN) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcSetAdditionalCacheAttributesEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, flags : u32) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn CcSetBcbOwnerPointer(bcb : *const ::core::ffi::c_void, ownerpointer : *const ::core::ffi::c_void) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcSetDirtyPageThreshold(fileobject : *const super::super::Foundation:: FILE_OBJECT, dirtypagethreshold : u32) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn CcSetDirtyPinnedData(bcbvoid : *const ::core::ffi::c_void, lsn : *const i64) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcSetFileSizes(fileobject : *const super::super::Foundation:: FILE_OBJECT, filesizes : *const CC_FILE_SIZES) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcSetFileSizesEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, filesizes : *const CC_FILE_SIZES) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcSetLogHandleForFile(fileobject : *const super::super::Foundation:: FILE_OBJECT, loghandle : *const ::core::ffi::c_void, flushtolsnroutine : PFLUSH_TO_LSN) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcSetParallelFlushFile(fileobject : *const super::super::Foundation:: FILE_OBJECT, enableparallelflush : super::super::super::Win32::Foundation:: BOOLEAN) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcSetReadAheadGranularity(fileobject : *const super::super::Foundation:: FILE_OBJECT, granularity : u32) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcUninitializeCacheMap(fileobject : *const super::super::Foundation:: FILE_OBJECT, truncatesize : *const i64, uninitializeevent : *const CACHE_UNINITIALIZE_EVENT) -> super::super::super::Win32::Foundation:: BOOLEAN);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn CcUnpinData(bcb : *const ::core::ffi::c_void) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn CcUnpinDataForThread(bcb : *const ::core::ffi::c_void, resourcethreadid : usize) -> ());
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcUnpinRepinnedBcb(bcb : *const ::core::ffi::c_void, writethrough : super::super::super::Win32::Foundation:: BOOLEAN, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn CcWaitForCurrentLazyWriterActivity() -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn CcZeroData(fileobject : *const super::super::Foundation:: FILE_OBJECT, startoffset : *const i64, endoffset : *const i64, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn CompleteAuthToken(phcontext : *const SecHandle, ptoken : *const SecBufferDesc) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn DecryptMessage(phcontext : *const SecHandle, pmessage : *const SecBufferDesc, messageseqno : u32, pfqop : *mut u32) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn DeleteSecurityContext(phcontext : *const SecHandle) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn EncryptMessage(phcontext : *const SecHandle, fqop : u32, pmessage : *const SecBufferDesc, messageseqno : u32) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn EnumerateSecurityPackagesW(pcpackages : *mut u32, pppackageinfo : *mut *mut SecPkgInfoW) -> ::windows_sys::core::HRESULT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn ExDisableResourceBoostLite(resource : *const super::super::Foundation:: ERESOURCE) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ExQueryPoolBlockSize(poolblock : *const ::core::ffi::c_void, quotacharged : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> usize);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn ExportSecurityContext(phcontext : *const SecHandle, fflags : u32, ppackedcontext : *mut SecBuffer, ptoken : *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FreeContextBuffer(pvcontextbuffer : *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FreeCredentialsHandle(phcredential : *const SecHandle) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlAcknowledgeEcp(ecpcontext : *const ::core::ffi::c_void) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlAcquireFileExclusive(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlAddBaseMcbEntry(mcb : *mut BASE_MCB, vbn : i64, lbn : i64, sectorcount : i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlAddBaseMcbEntryEx(mcb : *mut BASE_MCB, vbn : i64, lbn : i64, sectorcount : i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlAddLargeMcbEntry(mcb : *mut LARGE_MCB, vbn : i64, lbn : i64, sectorcount : i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlAddMcbEntry(mcb : *mut MCB, vbn : u32, lbn : u32, sectorcount : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlAddToTunnelCache(cache : *mut TUNNEL, directorykey : u64, shortname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, longname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, keybyshortname : super::super::super::Win32::Foundation:: BOOLEAN, datalength : u32, data : *const ::core::ffi::c_void) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlAddToTunnelCacheEx(cache : *mut TUNNEL, directorykey : u64, shortname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, longname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, flags : u32, datalength : u32, data : *const ::core::ffi::c_void) -> ());
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn FsRtlAllocateAePushLock(pooltype : super::super::Foundation:: POOL_TYPE, tag : u32) -> *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlAllocateExtraCreateParameter(ecptype : *const ::windows_sys::core::GUID, sizeofcontext : u32, flags : u32, cleanupcallback : PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, pooltag : u32, ecpcontext : *mut *mut ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlAllocateExtraCreateParameterFromLookasideList(ecptype : *const ::windows_sys::core::GUID, sizeofcontext : u32, flags : u32, cleanupcallback : PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, lookasidelist : *mut ::core::ffi::c_void, ecpcontext : *mut *mut ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn FsRtlAllocateExtraCreateParameterList(flags : u32, ecplist : *mut *mut super::super::Foundation:: ECP_LIST) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlAllocateFileLock(completelockirproutine : PCOMPLETE_LOCK_IRP_ROUTINE, unlockroutine : PUNLOCK_ROUTINE) -> *mut FILE_LOCK);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlAllocateResource() -> *mut super::super::Foundation:: ERESOURCE);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlAreNamesEqual(constantnamea : *const super::super::super::Win32::Foundation:: UNICODE_STRING, constantnameb : *const super::super::super::Win32::Foundation:: UNICODE_STRING, ignorecase : super::super::super::Win32::Foundation:: BOOLEAN, upcasetable : *const u16) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlAreThereCurrentOrInProgressFileLocks(filelock : *const FILE_LOCK) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlAreThereWaitingFileLocks(filelock : *const FILE_LOCK) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlAreVolumeStartupApplicationsComplete() -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlBalanceReads(targetdevice : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlCancellableWaitForMultipleObjects(count : u32, objectarray : *const *const ::core::ffi::c_void, waittype : super::super::super::Win32::System::Kernel:: WAIT_TYPE, timeout : *const i64, waitblockarray : *const super::super::Foundation:: KWAIT_BLOCK, irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlCancellableWaitForSingleObject(object : *const ::core::ffi::c_void, timeout : *const i64, irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlChangeBackingFileObject(currentfileobject : *const super::super::Foundation:: FILE_OBJECT, newfileobject : *const super::super::Foundation:: FILE_OBJECT, changebackingtype : FSRTL_CHANGE_BACKING_TYPE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlCheckLockForOplockRequest(filelock : *const FILE_LOCK, allocationsize : *const i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlCheckLockForReadAccess(filelock : *const FILE_LOCK, irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlCheckLockForWriteAccess(filelock : *const FILE_LOCK, irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlCheckOplock(oplock : *const *const ::core::ffi::c_void, irp : *const super::super::Foundation:: IRP, context : *const ::core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine : POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlCheckOplockEx(oplock : *const *const ::core::ffi::c_void, irp : *const super::super::Foundation:: IRP, flags : u32, context : *const ::core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine : POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlCheckOplockEx2(oplock : *const *const ::core::ffi::c_void, irp : *const super::super::Foundation:: IRP, flags : u32, flagsex2 : u32, completionroutinecontext : *const ::core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine : POPLOCK_FS_PREPOST_IRP, timeout : u64, notifycontext : *const ::core::ffi::c_void, notifyroutine : POPLOCK_NOTIFY_ROUTINE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlCheckUpperOplock(oplock : *const *const ::core::ffi::c_void, newloweroplockstate : u32, completionroutinecontext : *const ::core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, prependroutine : POPLOCK_FS_PREPOST_IRP, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlCopyRead(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, lockkey : u32, buffer : *mut ::core::ffi::c_void, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlCopyWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, wait : super::super::super::Win32::Foundation:: BOOLEAN, lockkey : u32, buffer : *const ::core::ffi::c_void, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlCreateSectionForDataScan(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, sectionobject : *mut *mut ::core::ffi::c_void, sectionfilesize : *mut i64, fileobject : *const super::super::Foundation:: FILE_OBJECT, desiredaccess : u32, objectattributes : *const super::super::super::Win32::System::WindowsProgramming:: OBJECT_ATTRIBUTES, maximumsize : *const i64, sectionpageprotection : u32, allocationattributes : u32, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlCurrentBatchOplock(oplock : *const *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlCurrentOplock(oplock : *const *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlCurrentOplockH(oplock : *const *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlDeleteExtraCreateParameterLookasideList(lookaside : *mut ::core::ffi::c_void, flags : u32) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlDeleteKeyFromTunnelCache(cache : *mut TUNNEL, directorykey : u64) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlDeleteTunnelCache(cache : *mut TUNNEL) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlDeregisterUncProvider(handle : super::super::super::Win32::Foundation:: HANDLE) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlDismountComplete(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, dismountstatus : super::super::super::Win32::Foundation:: NTSTATUS) -> ());
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlDissectDbcs(path : super::super::super::Win32::System::Kernel:: STRING, firstname : *mut super::super::super::Win32::System::Kernel:: STRING, remainingname : *mut super::super::super::Win32::System::Kernel:: STRING) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlDissectName(path : super::super::super::Win32::Foundation:: UNICODE_STRING, firstname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, remainingname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> ());
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlDoesDbcsContainWildCards(name : *const super::super::super::Win32::System::Kernel:: STRING) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlDoesNameContainWildCards(name : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlFastCheckLockForRead(filelock : *const FILE_LOCK, startingbyte : *const i64, length : *const i64, key : u32, fileobject : *const super::super::Foundation:: FILE_OBJECT, processid : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlFastCheckLockForWrite(filelock : *const FILE_LOCK, startingbyte : *const i64, length : *const i64, key : u32, fileobject : *const ::core::ffi::c_void, processid : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlFastUnlockAll(filelock : *const FILE_LOCK, fileobject : *const super::super::Foundation:: FILE_OBJECT, processid : *const super::super::Foundation:: KPROCESS, context : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlFastUnlockAllByKey(filelock : *const FILE_LOCK, fileobject : *const super::super::Foundation:: FILE_OBJECT, processid : *const super::super::Foundation:: KPROCESS, key : u32, context : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlFastUnlockSingle(filelock : *const FILE_LOCK, fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : *const i64, processid : *const super::super::Foundation:: KPROCESS, key : u32, context : *const ::core::ffi::c_void, alreadysynchronized : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn FsRtlFindExtraCreateParameter(ecplist : *const super::super::Foundation:: ECP_LIST, ecptype : *const ::windows_sys::core::GUID, ecpcontext : *mut *mut ::core::ffi::c_void, ecpcontextsize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlFindInTunnelCache(cache : *const TUNNEL, directorykey : u64, name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, shortname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, longname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, datalength : *mut u32, data : *mut ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlFindInTunnelCacheEx(cache : *const TUNNEL, directorykey : u64, name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, shortname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, longname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, flags : u32, datalength : *mut u32, data : *mut ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlFreeAePushLock(aepushlock : *mut ::core::ffi::c_void) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlFreeExtraCreateParameter(ecpcontext : *const ::core::ffi::c_void) -> ());
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn FsRtlFreeExtraCreateParameterList(ecplist : *const super::super::Foundation:: ECP_LIST) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlFreeFileLock(filelock : *const FILE_LOCK) -> ());
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlGetCurrentProcessLoaderList() -> *mut super::super::super::Win32::System::Kernel:: LIST_ENTRY);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlGetEcpListFromIrp(irp : *const super::super::Foundation:: IRP, ecplist : *mut *mut super::super::Foundation:: ECP_LIST) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlGetFileSize(fileobject : *const super::super::Foundation:: FILE_OBJECT, filesize : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlGetNextBaseMcbEntry(mcb : *const BASE_MCB, runindex : u32, vbn : *mut i64, lbn : *mut i64, sectorcount : *mut i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn FsRtlGetNextExtraCreateParameter(ecplist : *const super::super::Foundation:: ECP_LIST, currentecpcontext : *const ::core::ffi::c_void, nextecptype : *mut ::windows_sys::core::GUID, nextecpcontext : *mut *mut ::core::ffi::c_void, nextecpcontextsize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlGetNextFileLock(filelock : *const FILE_LOCK, restart : super::super::super::Win32::Foundation:: BOOLEAN) -> *mut FILE_LOCK_INFO);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlGetNextLargeMcbEntry(mcb : *const LARGE_MCB, runindex : u32, vbn : *mut i64, lbn : *mut i64, sectorcount : *mut i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlGetNextMcbEntry(mcb : *const MCB, runindex : u32, vbn : *mut u32, lbn : *mut u32, sectorcount : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlGetSectorSizeInformation(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, sectorsizeinfo : *mut FILE_FS_SECTOR_SIZE_INFORMATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlGetSupportedFeatures(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, supportedfeatures : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlGetVirtualDiskNestingLevel(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, nestinglevel : *mut u32, nestingflags : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlIncrementCcFastMdlReadWait() -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlIncrementCcFastReadNoWait() -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlIncrementCcFastReadNotPossible() -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlIncrementCcFastReadResourceMiss() -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlIncrementCcFastReadWait() -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlInitExtraCreateParameterLookasideList(lookaside : *mut ::core::ffi::c_void, flags : u32, size : usize, tag : u32) -> ());
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn FsRtlInitializeBaseMcb(mcb : *mut BASE_MCB, pooltype : super::super::Foundation:: POOL_TYPE) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn FsRtlInitializeBaseMcbEx(mcb : *mut BASE_MCB, pooltype : super::super::Foundation:: POOL_TYPE, flags : u16) -> super::super::super::Win32::Foundation:: BOOLEAN);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlInitializeExtraCreateParameter(ecp : *mut ECP_HEADER, ecpflags : u32, cleanupcallback : PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, totalsize : u32, ecptype : *const ::windows_sys::core::GUID, listallocatedfrom : *const ::core::ffi::c_void) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn FsRtlInitializeExtraCreateParameterList(ecplist : *mut super::super::Foundation:: ECP_LIST) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlInitializeFileLock(filelock : *mut FILE_LOCK, completelockirproutine : PCOMPLETE_LOCK_IRP_ROUTINE, unlockroutine : PUNLOCK_ROUTINE) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlInitializeLargeMcb(mcb : *mut LARGE_MCB, pooltype : super::super::Foundation:: POOL_TYPE) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlInitializeMcb(mcb : *mut MCB, pooltype : super::super::Foundation:: POOL_TYPE) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlInitializeOplock(oplock : *mut *mut ::core::ffi::c_void) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlInitializeTunnelCache(cache : *mut TUNNEL) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn FsRtlInsertExtraCreateParameter(ecplist : *mut super::super::Foundation:: ECP_LIST, ecpcontext : *mut ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlInsertPerFileContext(perfilecontextpointer : *const *const ::core::ffi::c_void, ptr : *const FSRTL_PER_FILE_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlInsertPerFileObjectContext(fileobject : *const super::super::Foundation:: FILE_OBJECT, ptr : *const FSRTL_PER_FILEOBJECT_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlInsertPerStreamContext(perstreamcontext : *const FSRTL_ADVANCED_FCB_HEADER, ptr : *const FSRTL_PER_STREAM_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn FsRtlIs32BitProcess(process : *const super::super::Foundation:: KPROCESS) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlIsDaxVolume(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlIsDbcsInExpression(expression : *const super::super::super::Win32::System::Kernel:: STRING, name : *const super::super::super::Win32::System::Kernel:: STRING) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlIsEcpAcknowledged(ecpcontext : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlIsEcpFromUserMode(ecpcontext : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlIsExtentDangling(startpage : u32, numberofpages : u32, flags : u32) -> u32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlIsFatDbcsLegal(dbcsname : super::super::super::Win32::System::Kernel:: STRING, wildcardspermissible : super::super::super::Win32::Foundation:: BOOLEAN, pathnamepermissible : super::super::super::Win32::Foundation:: BOOLEAN, leadingbackslashpermissible : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlIsHpfsDbcsLegal(dbcsname : super::super::super::Win32::System::Kernel:: STRING, wildcardspermissible : super::super::super::Win32::Foundation:: BOOLEAN, pathnamepermissible : super::super::super::Win32::Foundation:: BOOLEAN, leadingbackslashpermissible : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlIsMobileOS() -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlIsNameInExpression(expression : *const super::super::super::Win32::Foundation:: UNICODE_STRING, name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, ignorecase : super::super::super::Win32::Foundation:: BOOLEAN, upcasetable : ::windows_sys::core::PCWSTR) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlIsNameInUnUpcasedExpression(expression : *const super::super::super::Win32::Foundation:: UNICODE_STRING, name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, ignorecase : super::super::super::Win32::Foundation:: BOOLEAN, upcasetable : ::windows_sys::core::PCWSTR) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlIsNonEmptyDirectoryReparsePointAllowed(reparsetag : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlIsNtstatusExpected(exception : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlIsPagingFile(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlIsSystemPagingFile(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlIssueDeviceIoControl(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, ioctl : u32, flags : u8, inputbuffer : *const ::core::ffi::c_void, inputbufferlength : u32, outputbuffer : *const ::core::ffi::c_void, outputbufferlength : u32, iosbinformation : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlKernelFsControlFile(fileobject : *const super::super::Foundation:: FILE_OBJECT, fscontrolcode : u32, inputbuffer : *const ::core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut ::core::ffi::c_void, outputbufferlength : u32, retoutputbuffersize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlLogCcFlushError(filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, flusherror : super::super::super::Win32::Foundation:: NTSTATUS, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlLookupBaseMcbEntry(mcb : *const BASE_MCB, vbn : i64, lbn : *mut i64, sectorcountfromlbn : *mut i64, startinglbn : *mut i64, sectorcountfromstartinglbn : *mut i64, index : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlLookupLargeMcbEntry(mcb : *const LARGE_MCB, vbn : i64, lbn : *mut i64, sectorcountfromlbn : *mut i64, startinglbn : *mut i64, sectorcountfromstartinglbn : *mut i64, index : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlLookupLastBaseMcbEntry(mcb : *const BASE_MCB, vbn : *mut i64, lbn : *mut i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlLookupLastBaseMcbEntryAndIndex(opaquemcb : *const BASE_MCB, largevbn : *mut i64, largelbn : *mut i64, index : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlLookupLastLargeMcbEntry(mcb : *const LARGE_MCB, vbn : *mut i64, lbn : *mut i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlLookupLastLargeMcbEntryAndIndex(opaquemcb : *const LARGE_MCB, largevbn : *mut i64, largelbn : *mut i64, index : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlLookupLastMcbEntry(mcb : *const MCB, vbn : *mut u32, lbn : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlLookupMcbEntry(mcb : *const MCB, vbn : u32, lbn : *mut u32, sectorcount : *mut u32, index : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlLookupPerFileContext(perfilecontextpointer : *const *const ::core::ffi::c_void, ownerid : *const ::core::ffi::c_void, instanceid : *const ::core::ffi::c_void) -> *mut FSRTL_PER_FILE_CONTEXT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlLookupPerFileObjectContext(fileobject : *const super::super::Foundation:: FILE_OBJECT, ownerid : *const ::core::ffi::c_void, instanceid : *const ::core::ffi::c_void) -> *mut FSRTL_PER_FILEOBJECT_CONTEXT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlLookupPerStreamContextInternal(streamcontext : *const FSRTL_ADVANCED_FCB_HEADER, ownerid : *const ::core::ffi::c_void, instanceid : *const ::core::ffi::c_void) -> *mut FSRTL_PER_STREAM_CONTEXT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlMdlReadCompleteDev(fileobject : *const super::super::Foundation:: FILE_OBJECT, mdlchain : *const super::super::super::Win32::Graphics::DirectDraw:: MDL, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlMdlReadDev(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, lockkey : u32, mdlchain : *mut *mut super::super::super::Win32::Graphics::DirectDraw:: MDL, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlMdlReadEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, lockkey : u32, mdlchain : *mut *mut super::super::super::Win32::Graphics::DirectDraw:: MDL, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlMdlWriteCompleteDev(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, mdlchain : *const super::super::super::Win32::Graphics::DirectDraw:: MDL, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlMupGetProviderIdFromName(pprovidername : *const super::super::super::Win32::Foundation:: UNICODE_STRING, pproviderid : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlMupGetProviderInfoFromFileObject(pfileobject : *const super::super::Foundation:: FILE_OBJECT, level : u32, pbuffer : *mut ::core::ffi::c_void, pbuffersize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlNormalizeNtstatus(exception : super::super::super::Win32::Foundation:: NTSTATUS, genericexception : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlNotifyCleanup(notifysync : *const _REAL_NOTIFY_SYNC, notifylist : *const super::super::super::Win32::System::Kernel:: LIST_ENTRY, fscontext : *const ::core::ffi::c_void) -> ());
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlNotifyCleanupAll(notifysync : *const _REAL_NOTIFY_SYNC, notifylist : *const super::super::super::Win32::System::Kernel:: LIST_ENTRY) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlNotifyFilterChangeDirectory(notifysync : *const _REAL_NOTIFY_SYNC, notifylist : *const super::super::super::Win32::System::Kernel:: LIST_ENTRY, fscontext : *const ::core::ffi::c_void, fulldirectoryname : *const super::super::super::Win32::System::Kernel:: STRING, watchtree : super::super::super::Win32::Foundation:: BOOLEAN, ignorebuffer : super::super::super::Win32::Foundation:: BOOLEAN, completionfilter : u32, notifyirp : *const super::super::Foundation:: IRP, traversecallback : PCHECK_FOR_TRAVERSE_ACCESS, subjectcontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, filtercallback : PFILTER_REPORT_CHANGE) -> ());
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlNotifyFilterReportChange(notifysync : *const _REAL_NOTIFY_SYNC, notifylist : *const super::super::super::Win32::System::Kernel:: LIST_ENTRY, fulltargetname : *const super::super::super::Win32::System::Kernel:: STRING, targetnameoffset : u16, streamname : *const super::super::super::Win32::System::Kernel:: STRING, normalizedparentname : *const super::super::super::Win32::System::Kernel:: STRING, filtermatch : u32, action : u32, targetcontext : *const ::core::ffi::c_void, filtercontext : *const ::core::ffi::c_void) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlNotifyFullChangeDirectory(notifysync : *const _REAL_NOTIFY_SYNC, notifylist : *const super::super::super::Win32::System::Kernel:: LIST_ENTRY, fscontext : *const ::core::ffi::c_void, fulldirectoryname : *mut super::super::super::Win32::System::Kernel:: STRING, watchtree : super::super::super::Win32::Foundation:: BOOLEAN, ignorebuffer : super::super::super::Win32::Foundation:: BOOLEAN, completionfilter : u32, notifyirp : *const super::super::Foundation:: IRP, traversecallback : PCHECK_FOR_TRAVERSE_ACCESS, subjectcontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT) -> ());
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlNotifyFullReportChange(notifysync : *const _REAL_NOTIFY_SYNC, notifylist : *const super::super::super::Win32::System::Kernel:: LIST_ENTRY, fulltargetname : *const super::super::super::Win32::System::Kernel:: STRING, targetnameoffset : u16, streamname : *const super::super::super::Win32::System::Kernel:: STRING, normalizedparentname : *const super::super::super::Win32::System::Kernel:: STRING, filtermatch : u32, action : u32, targetcontext : *const ::core::ffi::c_void) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlNotifyInitializeSync(notifysync : *mut *mut _REAL_NOTIFY_SYNC) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlNotifyUninitializeSync(notifysync : *mut *mut _REAL_NOTIFY_SYNC) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlNotifyVolumeEvent(fileobject : *const super::super::Foundation:: FILE_OBJECT, eventcode : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlNotifyVolumeEventEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, eventcode : u32, event : *const super::super::Foundation:: TARGET_DEVICE_CUSTOM_NOTIFICATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlNumberOfRunsInBaseMcb(mcb : *const BASE_MCB) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlNumberOfRunsInLargeMcb(mcb : *const LARGE_MCB) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlNumberOfRunsInMcb(mcb : *const MCB) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlOplockBreakH(oplock : *const *const ::core::ffi::c_void, irp : *const super::super::Foundation:: IRP, flags : u32, context : *const ::core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine : POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlOplockBreakH2(oplock : *const *const ::core::ffi::c_void, irp : *const super::super::Foundation:: IRP, flags : u32, context : *const ::core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine : POPLOCK_FS_PREPOST_IRP, grantedaccess : *const u32, shareaccess : *const u16) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlOplockBreakToNone(oplock : *mut *mut ::core::ffi::c_void, irpsp : *const super::super::Foundation:: IO_STACK_LOCATION, irp : *const super::super::Foundation:: IRP, context : *const ::core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine : POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlOplockBreakToNoneEx(oplock : *mut *mut ::core::ffi::c_void, irp : *const super::super::Foundation:: IRP, flags : u32, context : *const ::core::ffi::c_void, completionroutine : POPLOCK_WAIT_COMPLETE_ROUTINE, postirproutine : POPLOCK_FS_PREPOST_IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlOplockFsctrl(oplock : *const *const ::core::ffi::c_void, irp : *const super::super::Foundation:: IRP, opencount : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlOplockFsctrlEx(oplock : *const *const ::core::ffi::c_void, irp : *const super::super::Foundation:: IRP, opencount : u32, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn FsRtlOplockGetAnyBreakOwnerProcess(oplock : *const *const ::core::ffi::c_void) -> *mut super::super::Foundation:: KPROCESS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlOplockIsFastIoPossible(oplock : *const *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlOplockIsSharedRequest(irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlOplockKeysEqual(fo1 : *const super::super::Foundation:: FILE_OBJECT, fo2 : *const super::super::Foundation:: FILE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlPostPagingFileStackOverflow(context : *const ::core::ffi::c_void, event : *const super::super::Foundation:: KEVENT, stackoverflowroutine : PFSRTL_STACK_OVERFLOW_ROUTINE) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlPostStackOverflow(context : *const ::core::ffi::c_void, event : *const super::super::Foundation:: KEVENT, stackoverflowroutine : PFSRTL_STACK_OVERFLOW_ROUTINE) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlPrepareMdlWriteDev(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, lockkey : u32, mdlchain : *mut *mut super::super::super::Win32::Graphics::DirectDraw:: MDL, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlPrepareMdlWriteEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, lockkey : u32, mdlchain : *mut *mut super::super::super::Win32::Graphics::DirectDraw:: MDL, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlPrepareToReuseEcp(ecpcontext : *const ::core::ffi::c_void) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlPrivateLock(filelock : *const FILE_LOCK, fileobject : *const super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : *const i64, processid : *const super::super::Foundation:: KPROCESS, key : u32, failimmediately : super::super::super::Win32::Foundation:: BOOLEAN, exclusivelock : super::super::super::Win32::Foundation:: BOOLEAN, iosb : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, irp : *const super::super::Foundation:: IRP, context : *const ::core::ffi::c_void, alreadysynchronized : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlProcessFileLock(filelock : *const FILE_LOCK, irp : *const super::super::Foundation:: IRP, context : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlQueryCachedVdl(fileobject : *const super::super::Foundation:: FILE_OBJECT, vdl : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlQueryInformationFile(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileinformation : *mut ::core::ffi::c_void, length : u32, fileinformationclass : super::super::super::Win32::System::WindowsProgramming:: FILE_INFORMATION_CLASS, retfileinformationsize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlQueryKernelEaFile(fileobject : *const super::super::Foundation:: FILE_OBJECT, returnedeadata : *mut ::core::ffi::c_void, length : u32, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, ealist : *const ::core::ffi::c_void, ealistlength : u32, eaindex : *const u32, restartscan : super::super::super::Win32::Foundation:: BOOLEAN, lengthreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlQueryMaximumVirtualDiskNestingLevel() -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlRegisterFileSystemFilterCallbacks(filterdriverobject : *const super::super::Foundation:: DRIVER_OBJECT, callbacks : *const FS_FILTER_CALLBACKS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlRegisterUncProvider(muphandle : *mut super::super::super::Win32::Foundation:: HANDLE, redirectordevicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, mailslotssupported : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlRegisterUncProviderEx(muphandle : *mut super::super::super::Win32::Foundation:: HANDLE, redirdevname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlRegisterUncProviderEx2(redirdevname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, registration : *const FSRTL_UNC_PROVIDER_REGISTRATION, muphandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlReleaseFile(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlRemoveBaseMcbEntry(mcb : *mut BASE_MCB, vbn : i64, sectorcount : i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlRemoveDotsFromPath(originalstring : ::windows_sys::core::PWSTR, pathlength : u16, newlength : *mut u16) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn FsRtlRemoveExtraCreateParameter(ecplist : *mut super::super::Foundation:: ECP_LIST, ecptype : *const ::windows_sys::core::GUID, ecpcontext : *mut *mut ::core::ffi::c_void, ecpcontextsize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlRemoveLargeMcbEntry(mcb : *mut LARGE_MCB, vbn : i64, sectorcount : i64) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlRemoveMcbEntry(mcb : *mut MCB, vbn : u32, sectorcount : u32) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlRemovePerFileContext(perfilecontextpointer : *const *const ::core::ffi::c_void, ownerid : *const ::core::ffi::c_void, instanceid : *const ::core::ffi::c_void) -> *mut FSRTL_PER_FILE_CONTEXT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlRemovePerFileObjectContext(fileobject : *const super::super::Foundation:: FILE_OBJECT, ownerid : *const ::core::ffi::c_void, instanceid : *const ::core::ffi::c_void) -> *mut FSRTL_PER_FILEOBJECT_CONTEXT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlRemovePerStreamContext(streamcontext : *const FSRTL_ADVANCED_FCB_HEADER, ownerid : *const ::core::ffi::c_void, instanceid : *const ::core::ffi::c_void) -> *mut FSRTL_PER_STREAM_CONTEXT);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlResetBaseMcb(mcb : *mut BASE_MCB) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlResetLargeMcb(mcb : *mut LARGE_MCB, selfsynchronized : super::super::super::Win32::Foundation:: BOOLEAN) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlSetDriverBacking(driverobj : *const super::super::Foundation:: DRIVER_OBJECT, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlSetEcpListIntoIrp(irp : *mut super::super::Foundation:: IRP, ecplist : *const super::super::Foundation:: ECP_LIST) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlSetKernelEaFile(fileobject : *const super::super::Foundation:: FILE_OBJECT, eabuffer : *const ::core::ffi::c_void, length : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlSplitBaseMcb(mcb : *mut BASE_MCB, vbn : i64, amount : i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlSplitLargeMcb(mcb : *mut LARGE_MCB, vbn : i64, amount : i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlTeardownPerFileContexts(perfilecontextpointer : *const *const ::core::ffi::c_void) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlTeardownPerStreamContexts(advancedheader : *const FSRTL_ADVANCED_FCB_HEADER) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlTruncateBaseMcb(mcb : *mut BASE_MCB, vbn : i64) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlTruncateLargeMcb(mcb : *mut LARGE_MCB, vbn : i64) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlTruncateMcb(mcb : *mut MCB, vbn : u32) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlUninitializeBaseMcb(mcb : *const BASE_MCB) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlUninitializeFileLock(filelock : *mut FILE_LOCK) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlUninitializeLargeMcb(mcb : *mut LARGE_MCB) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn FsRtlUninitializeMcb(mcb : *mut MCB) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlUninitializeOplock(oplock : *mut *mut ::core::ffi::c_void) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn FsRtlUpdateDiskCounters(bytesread : u64, byteswritten : u64) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlUpperOplockFsctrl(oplock : *const *const ::core::ffi::c_void, irp : *const super::super::Foundation:: IRP, opencount : u32, loweroplockstate : u32, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn FsRtlValidateReparsePointBuffer(bufferlength : u32, reparsebuffer : *const REPARSE_DATA_BUFFER) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FsRtlVolumeDeviceToCorrelationId(volumedeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, guid : *mut ::windows_sys::core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn GetSecurityUserInfo(logonid : *const super::super::super::Win32::Foundation:: LUID, flags : u32, userinformation : *mut *mut SECURITY_USER_DATA) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn ImpersonateSecurityContext(phcontext : *const SecHandle) -> ::windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ImportSecurityContextW(pszpackage : *const super::super::super::Win32::Foundation:: UNICODE_STRING, ppackedcontext : *const SecBuffer, token : *const ::core::ffi::c_void, phcontext : *mut SecHandle) -> ::windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn InitSecurityInterfaceW() -> *mut SecurityFunctionTableW);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn InitializeSecurityContextW(phcredential : *const SecHandle, phcontext : *const SecHandle, ptargetname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, fcontextreq : u32, reserved1 : u32, targetdatarep : u32, pinput : *const SecBufferDesc, reserved2 : u32, phnewcontext : *mut SecHandle, poutput : *mut SecBufferDesc, pfcontextattr : *mut u32, ptsexpiry : *mut i64) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn IoAcquireVpbSpinLock(irql : *mut u8) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn IoApplyPriorityInfoThread(inputpriorityinfo : *const IO_PRIORITY_INFO, outputpriorityinfo : *mut IO_PRIORITY_INFO, thread : *const super::super::Foundation:: KTHREAD) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn IoCheckDesiredAccess(desiredaccess : *mut u32, grantedaccess : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn IoCheckEaBufferValidity(eabuffer : *const FILE_FULL_EA_INFORMATION, ealength : u32, erroroffset : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn IoCheckFunctionAccess(grantedaccess : u32, majorfunction : u8, minorfunction : u8, iocontrolcode : u32, arg1 : *const ::core::ffi::c_void, arg2 : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoCheckQuerySetFileInformation(fileinformationclass : super::super::super::Win32::System::WindowsProgramming:: FILE_INFORMATION_CLASS, length : u32, setoperation : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn IoCheckQuerySetVolumeInformation(fsinformationclass : FS_INFORMATION_CLASS, length : u32, setoperation : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn IoCheckQuotaBufferValidity(quotabuffer : *const FILE_QUOTA_INFORMATION, quotalength : u32, erroroffset : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoClearFsTrackOffsetState(irp : *mut super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoCreateStreamFileObject(fileobject : *const super::super::Foundation:: FILE_OBJECT, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> *mut super::super::Foundation:: FILE_OBJECT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoCreateStreamFileObjectEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, filehandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> *mut super::super::Foundation:: FILE_OBJECT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoCreateStreamFileObjectEx2(createoptions : *const IO_CREATE_STREAM_FILE_OPTIONS, fileobject : *const super::super::Foundation:: FILE_OBJECT, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, streamfileobject : *mut *mut super::super::Foundation:: FILE_OBJECT, filehandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoCreateStreamFileObjectLite(fileobject : *const super::super::Foundation:: FILE_OBJECT, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> *mut super::super::Foundation:: FILE_OBJECT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoEnumerateDeviceObjectList(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, deviceobjectlist : *mut *mut super::super::Foundation:: DEVICE_OBJECT, deviceobjectlistsize : u32, actualnumberdeviceobjects : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoEnumerateRegisteredFiltersList(driverobjectlist : *mut *mut super::super::Foundation:: DRIVER_OBJECT, driverobjectlistsize : u32, actualnumberdriverobjects : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoFastQueryNetworkAttributes(objectattributes : *const super::super::super::Win32::System::WindowsProgramming:: OBJECT_ATTRIBUTES, desiredaccess : u32, openoptions : u32, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, buffer : *mut FILE_NETWORK_OPEN_INFORMATION) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoGetAttachedDevice(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> *mut super::super::Foundation:: DEVICE_OBJECT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoGetBaseFileSystemDeviceObject(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> *mut super::super::Foundation:: DEVICE_OBJECT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoGetDeviceAttachmentBaseRef(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> *mut super::super::Foundation:: DEVICE_OBJECT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoGetDeviceToVerify(thread : *const super::super::Foundation:: KTHREAD) -> *mut super::super::Foundation:: DEVICE_OBJECT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoGetDiskDeviceObject(filesystemdeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, diskdeviceobject : *mut *mut super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoGetFsTrackOffsetState(irp : *const super::super::Foundation:: IRP, retfstrackoffsetblob : *mut *mut IO_IRP_EXT_TRACK_OFFSET_HEADER, rettrackedoffset : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoGetLowerDeviceObject(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> *mut super::super::Foundation:: DEVICE_OBJECT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoGetOplockKeyContext(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> *mut OPLOCK_KEY_ECP_CONTEXT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoGetOplockKeyContextEx(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> *mut OPLOCK_KEY_CONTEXT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoGetRequestorProcess(irp : *const super::super::Foundation:: IRP) -> *mut super::super::Foundation:: KPROCESS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoGetRequestorProcessId(irp : *const super::super::Foundation:: IRP) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoGetRequestorSessionId(irp : *const super::super::Foundation:: IRP, psessionid : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoIrpHasFsTrackOffsetExtensionType(irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoIsOperationSynchronous(irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn IoIsSystemThread(thread : *const super::super::Foundation:: KTHREAD) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoIsValidNameGraftingBuffer(irp : *const super::super::Foundation:: IRP, reparsebuffer : *const REPARSE_DATA_BUFFER) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoPageRead(fileobject : *const super::super::Foundation:: FILE_OBJECT, memorydescriptorlist : *const super::super::super::Win32::Graphics::DirectDraw:: MDL, startingoffset : *const i64, event : *const super::super::Foundation:: KEVENT, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoQueryFileDosDeviceName(fileobject : *const super::super::Foundation:: FILE_OBJECT, objectnameinformation : *mut *mut super::super::Foundation:: OBJECT_NAME_INFORMATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoQueryFileInformation(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileinformationclass : super::super::super::Win32::System::WindowsProgramming:: FILE_INFORMATION_CLASS, length : u32, fileinformation : *mut ::core::ffi::c_void, returnedlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoQueryVolumeInformation(fileobject : *const super::super::Foundation:: FILE_OBJECT, fsinformationclass : FS_INFORMATION_CLASS, length : u32, fsinformation : *mut ::core::ffi::c_void, returnedlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoQueueThreadIrp(irp : *const super::super::Foundation:: IRP) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoRegisterFileSystem(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoRegisterFsRegistrationChange(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, drivernotificationroutine : PDRIVER_FS_NOTIFICATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoRegisterFsRegistrationChangeMountAware(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, drivernotificationroutine : PDRIVER_FS_NOTIFICATION, synchronizewithmounts : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn IoReleaseVpbSpinLock(irql : u8) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoReplaceFileObjectName(fileobject : *const super::super::Foundation:: FILE_OBJECT, newfilename : ::windows_sys::core::PCWSTR, filenamelength : u16) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoRequestDeviceRemovalForReset(physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoRetrievePriorityInfo(irp : *const super::super::Foundation:: IRP, fileobject : *const super::super::Foundation:: FILE_OBJECT, thread : *const super::super::Foundation:: KTHREAD, priorityinfo : *mut IO_PRIORITY_INFO) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoSetDeviceToVerify(thread : *const super::super::Foundation:: KTHREAD, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoSetFsTrackOffsetState(irp : *mut super::super::Foundation:: IRP, fstrackoffsetblob : *const IO_IRP_EXT_TRACK_OFFSET_HEADER, trackedoffset : i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoSetInformation(fileobject : *const super::super::Foundation:: FILE_OBJECT, fileinformationclass : super::super::super::Win32::System::WindowsProgramming:: FILE_INFORMATION_CLASS, length : u32, fileinformation : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoSynchronousPageWrite(fileobject : *const super::super::Foundation:: FILE_OBJECT, memorydescriptorlist : *const super::super::super::Win32::Graphics::DirectDraw:: MDL, startingoffset : *const i64, event : *const super::super::Foundation:: KEVENT, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn IoThreadToProcess(thread : *const super::super::Foundation:: KTHREAD) -> *mut super::super::Foundation:: KPROCESS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoUnregisterFileSystem(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoUnregisterFsRegistrationChange(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, drivernotificationroutine : PDRIVER_FS_NOTIFICATION) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn IoVerifyVolume(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, allowrawmount : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn KeAcquireQueuedSpinLock(number : super::super::Foundation:: KSPIN_LOCK_QUEUE_NUMBER) -> u8);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn KeAcquireSpinLockRaiseToSynch(spinlock : *mut usize) -> u8);
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn KeAttachProcess(process : *mut super::super::Foundation:: KPROCESS) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn KeDetachProcess() -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn KeInitializeMutant(mutant : *mut super::super::Foundation:: KMUTANT, initialowner : super::super::super::Win32::Foundation:: BOOLEAN) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn KeInitializeQueue(queue : *mut super::super::Foundation:: KQUEUE, count : u32) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn KeInsertHeadQueue(queue : *mut super::super::Foundation:: KQUEUE, entry : *mut super::super::super::Win32::System::Kernel:: LIST_ENTRY) -> i32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn KeInsertQueue(queue : *mut super::super::Foundation:: KQUEUE, entry : *mut super::super::super::Win32::System::Kernel:: LIST_ENTRY) -> i32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn KeReadStateMutant(mutant : *const super::super::Foundation:: KMUTANT) -> i32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn KeReadStateQueue(queue : *const super::super::Foundation:: KQUEUE) -> i32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn KeReleaseMutant(mutant : *mut super::super::Foundation:: KMUTANT, increment : i32, abandoned : super::super::super::Win32::Foundation:: BOOLEAN, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> i32);
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn KeReleaseQueuedSpinLock(number : super::super::Foundation:: KSPIN_LOCK_QUEUE_NUMBER, oldirql : u8) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn KeRemoveQueue(queue : *mut super::super::Foundation:: KQUEUE, waitmode : i8, timeout : *const i64) -> *mut super::super::super::Win32::System::Kernel:: LIST_ENTRY);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn KeRemoveQueueEx(queue : *mut super::super::Foundation:: KQUEUE, waitmode : i8, alertable : super::super::super::Win32::Foundation:: BOOLEAN, timeout : *const i64, entryarray : *mut *mut super::super::super::Win32::System::Kernel:: LIST_ENTRY, count : u32) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn KeRundownQueue(queue : *mut super::super::Foundation:: KQUEUE) -> *mut super::super::super::Win32::System::Kernel:: LIST_ENTRY);
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn KeSetIdealProcessorThread(thread : *mut super::super::Foundation:: KTHREAD, processor : u8) -> u8);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn KeSetKernelStackSwapEnable(enable : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn KeStackAttachProcess(process : *mut super::super::Foundation:: KPROCESS, apcstate : *mut KAPC_STATE) -> ());
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn KeTryToAcquireQueuedSpinLock(number : super::super::Foundation:: KSPIN_LOCK_QUEUE_NUMBER, oldirql : *mut u8) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn KeUnstackDetachProcess(apcstate : *const KAPC_STATE) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn LsaFreeReturnBuffer(buffer : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Kernel"))]
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`*"] fn LsaLogonUser(lsahandle : super::super::super::Win32::Foundation:: HANDLE, originname : *const super::super::super::Win32::System::Kernel:: STRING, logontype : SECURITY_LOGON_TYPE, authenticationpackage : u32, authenticationinformation : *const ::core::ffi::c_void, authenticationinformationlength : u32, localgroups : *const TOKEN_GROUPS, sourcecontext : *const TOKEN_SOURCE, profilebuffer : *mut *mut ::core::ffi::c_void, profilebufferlength : *mut u32, logonid : *mut super::super::super::Win32::Foundation:: LUID, token : *mut super::super::super::Win32::Foundation:: HANDLE, quotas : *mut super::super::super::Win32::Security:: QUOTA_LIMITS, substatus : *mut i32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn LsaRegisterLogonProcess(logonprocessname : *const super::super::super::Win32::System::Kernel:: STRING, lsahandle : *mut super::super::super::Win32::Foundation:: HANDLE, securitymode : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn MakeSignature(phcontext : *const SecHandle, fqop : u32, pmessage : *const SecBufferDesc, messageseqno : u32) -> ::windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn MapSecurityError(secstatus : ::windows_sys::core::HRESULT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn MmCanFileBeTruncated(sectionpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, newfilesize : *const i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn MmDoesFileHaveUserWritableReferences(sectionpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn MmFlushImageSection(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, flushtype : MMFLUSH_TYPE) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn MmForceSectionClosed(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, delayclose : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn MmForceSectionClosedEx(sectionobjectpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, forcecloseflags : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn MmGetMaximumFileSectionSize() -> u64);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn MmIsFileSectionActive(fssectionpointer : *const super::super::Foundation:: SECTION_OBJECT_POINTERS, flags : u32, sectionisactive : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn MmIsRecursiveIoFault() -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Graphics_DirectDraw")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Graphics_DirectDraw\"`*"] fn MmMdlPagesAreZero(mdl : *const super::super::super::Win32::Graphics::DirectDraw:: MDL) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"] fn MmPrefetchPages(numberoflists : u32, readlists : *const *const READ_LIST) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn MmSetAddressRangeModified(address : *const ::core::ffi::c_void, length : usize) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn NtAccessCheckAndAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const ::core::ffi::c_void, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, desiredaccess : u32, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, grantedaccess : *mut u32, accessstatus : *mut i32, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn NtAccessCheckByTypeAndAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const ::core::ffi::c_void, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, principalselfsid : super::super::super::Win32::Foundation:: PSID, desiredaccess : u32, audittype : AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *const OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, grantedaccess : *mut u32, accessstatus : *mut i32, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn NtAccessCheckByTypeResultListAndAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const ::core::ffi::c_void, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, principalselfsid : super::super::super::Win32::Foundation:: PSID, desiredaccess : u32, audittype : AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *const OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, grantedaccess : *mut u32, accessstatus : *mut i32, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn NtAccessCheckByTypeResultListAndAuditAlarmByHandle(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const ::core::ffi::c_void, clienttoken : super::super::super::Win32::Foundation:: HANDLE, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, principalselfsid : super::super::super::Win32::Foundation:: PSID, desiredaccess : u32, audittype : AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *const OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, grantedaccess : *mut u32, accessstatus : *mut i32, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtAdjustGroupsToken(tokenhandle : super::super::super::Win32::Foundation:: HANDLE, resettodefault : super::super::super::Win32::Foundation:: BOOLEAN, newstate : *const TOKEN_GROUPS, bufferlength : u32, previousstate : *mut TOKEN_GROUPS, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn NtAdjustPrivilegesToken(tokenhandle : super::super::super::Win32::Foundation:: HANDLE, disableallprivileges : super::super::super::Win32::Foundation:: BOOLEAN, newstate : *const TOKEN_PRIVILEGES, bufferlength : u32, previousstate : *mut TOKEN_PRIVILEGES, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtAllocateVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut ::core::ffi::c_void, zerobits : usize, regionsize : *mut usize, allocationtype : u32, protect : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtClose(handle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtCloseObjectAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const ::core::ffi::c_void, generateonclose : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtCreateFile(filehandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : super::super::super::Win32::Storage::FileSystem:: FILE_ACCESS_RIGHTS, objectattributes : *const super::super::super::Win32::System::WindowsProgramming:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, allocationsize : *const i64, fileattributes : super::super::super::Win32::Storage::FileSystem:: FILE_FLAGS_AND_ATTRIBUTES, shareaccess : super::super::super::Win32::Storage::FileSystem:: FILE_SHARE_MODE, createdisposition : NTCREATEFILE_CREATE_DISPOSITION, createoptions : NTCREATEFILE_CREATE_OPTIONS, eabuffer : *const ::core::ffi::c_void, ealength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtCreateSection(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::super::Win32::System::WindowsProgramming:: OBJECT_ATTRIBUTES, maximumsize : *const i64, sectionpageprotection : u32, allocationattributes : u32, filehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Memory\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtCreateSectionEx(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::super::Win32::System::WindowsProgramming:: OBJECT_ATTRIBUTES, maximumsize : *const i64, sectionpageprotection : u32, allocationattributes : u32, filehandle : super::super::super::Win32::Foundation:: HANDLE, extendedparameters : *mut super::super::super::Win32::System::Memory:: MEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtDeleteObjectAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const ::core::ffi::c_void, generateonclose : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtDeviceIoControlFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::Foundation:: PIO_APC_ROUTINE, apccontext : *const ::core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, iocontrolcode : u32, inputbuffer : *const ::core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut ::core::ffi::c_void, outputbufferlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtDuplicateToken(existingtokenhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::super::Win32::System::WindowsProgramming:: OBJECT_ATTRIBUTES, effectiveonly : super::super::super::Win32::Foundation:: BOOLEAN, tokentype : TOKEN_TYPE, newtokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn NtFilterToken(existingtokenhandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, sidstodisable : *const TOKEN_GROUPS, privilegestodelete : *const TOKEN_PRIVILEGES, restrictedsids : *const TOKEN_GROUPS, newtokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtFlushBuffersFileEx(filehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, parameters : *const ::core::ffi::c_void, parameterssize : u32, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtFreeVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut ::core::ffi::c_void, regionsize : *mut usize, freetype : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtFsControlFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::Foundation:: PIO_APC_ROUTINE, apccontext : *const ::core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, fscontrolcode : u32, inputbuffer : *const ::core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut ::core::ffi::c_void, outputbufferlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtImpersonateAnonymousToken(threadhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtLockFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::Foundation:: PIO_APC_ROUTINE, apccontext : *const ::core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, byteoffset : *const i64, length : *const i64, key : u32, failimmediately : super::super::super::Win32::Foundation:: BOOLEAN, exclusivelock : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtOpenFile(filehandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::super::Win32::System::WindowsProgramming:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, shareaccess : u32, openoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn NtOpenObjectAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const ::core::ffi::c_void, objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, clienttoken : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, grantedaccess : u32, privileges : *const super::super::super::Win32::Security:: PRIVILEGE_SET, objectcreation : super::super::super::Win32::Foundation:: BOOLEAN, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtOpenProcessToken(processhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, tokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtOpenProcessTokenEx(processhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, handleattributes : u32, tokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtOpenThreadToken(threadhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, openasself : super::super::super::Win32::Foundation:: BOOLEAN, tokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtOpenThreadTokenEx(threadhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, openasself : super::super::super::Win32::Foundation:: BOOLEAN, handleattributes : u32, tokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn NtPrivilegeCheck(clienttoken : super::super::super::Win32::Foundation:: HANDLE, requiredprivileges : *mut super::super::super::Win32::Security:: PRIVILEGE_SET, result : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn NtPrivilegeObjectAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, handleid : *const ::core::ffi::c_void, clienttoken : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, privileges : *const super::super::super::Win32::Security:: PRIVILEGE_SET, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn NtPrivilegedServiceAuditAlarm(subsystemname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, servicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, clienttoken : super::super::super::Win32::Foundation:: HANDLE, privileges : *const super::super::super::Win32::Security:: PRIVILEGE_SET, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtQueryDirectoryFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::Foundation:: PIO_APC_ROUTINE, apccontext : *const ::core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, fileinformation : *mut ::core::ffi::c_void, length : u32, fileinformationclass : super::super::super::Win32::System::WindowsProgramming:: FILE_INFORMATION_CLASS, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, restartscan : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtQueryDirectoryFileEx(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::Foundation:: PIO_APC_ROUTINE, apccontext : *const ::core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, fileinformation : *mut ::core::ffi::c_void, length : u32, fileinformationclass : super::super::super::Win32::System::WindowsProgramming:: FILE_INFORMATION_CLASS, queryflags : u32, filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtQueryInformationByName(objectattributes : *const super::super::super::Win32::System::WindowsProgramming:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, fileinformation : *mut ::core::ffi::c_void, length : u32, fileinformationclass : super::super::super::Win32::System::WindowsProgramming:: FILE_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtQueryInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, fileinformation : *mut ::core::ffi::c_void, length : u32, fileinformationclass : super::super::super::Win32::System::WindowsProgramming:: FILE_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtQueryInformationToken(tokenhandle : super::super::super::Win32::Foundation:: HANDLE, tokeninformationclass : TOKEN_INFORMATION_CLASS, tokeninformation : *mut ::core::ffi::c_void, tokeninformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtQueryObject(handle : super::super::super::Win32::Foundation:: HANDLE, objectinformationclass : OBJECT_INFORMATION_CLASS, objectinformation : *mut ::core::ffi::c_void, objectinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtQueryQuotaInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, buffer : *mut ::core::ffi::c_void, length : u32, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, sidlist : *const ::core::ffi::c_void, sidlistlength : u32, startsid : super::super::super::Win32::Foundation:: PSID, restartscan : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn NtQuerySecurityObject(handle : super::super::super::Win32::Foundation:: HANDLE, securityinformation : u32, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, length : u32, lengthneeded : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtQueryVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *const ::core::ffi::c_void, memoryinformationclass : MEMORY_INFORMATION_CLASS, memoryinformation : *mut ::core::ffi::c_void, memoryinformationlength : usize, returnlength : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtQueryVolumeInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, fsinformation : *mut ::core::ffi::c_void, length : u32, fsinformationclass : FS_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtReadFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::Foundation:: PIO_APC_ROUTINE, apccontext : *const ::core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, buffer : *mut ::core::ffi::c_void, length : u32, byteoffset : *const i64, key : *const u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtSetInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, fileinformation : *const ::core::ffi::c_void, length : u32, fileinformationclass : super::super::super::Win32::System::WindowsProgramming:: FILE_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"] fn NtSetInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : super::super::super::Win32::System::Threading:: THREADINFOCLASS, threadinformation : *mut ::core::ffi::c_void, threadinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtSetInformationToken(tokenhandle : super::super::super::Win32::Foundation:: HANDLE, tokeninformationclass : TOKEN_INFORMATION_CLASS, tokeninformation : *const ::core::ffi::c_void, tokeninformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn NtSetInformationVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, vminformationclass : VIRTUAL_MEMORY_INFORMATION_CLASS, numberofentries : usize, virtualaddresses : *const MEMORY_RANGE_ENTRY, vminformation : *const ::core::ffi::c_void, vminformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtSetQuotaInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, buffer : *const ::core::ffi::c_void, length : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn NtSetSecurityObject(handle : super::super::super::Win32::Foundation:: HANDLE, securityinformation : u32, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtSetVolumeInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, fsinformation : *const ::core::ffi::c_void, length : u32, fsinformationclass : FS_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtUnlockFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, byteoffset : *const i64, length : *const i64, key : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn NtWriteFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::Foundation:: PIO_APC_ROUTINE, apccontext : *const ::core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, buffer : *const ::core::ffi::c_void, length : u32, byteoffset : *const i64, key : *const u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn ObInsertObject(object : *const ::core::ffi::c_void, passedaccessstate : *mut super::super::Foundation:: ACCESS_STATE, desiredaccess : u32, objectpointerbias : u32, newobject : *mut *mut ::core::ffi::c_void, handle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ObIsKernelHandle(handle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: BOOLEAN);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn ObMakeTemporaryObject(object : *const ::core::ffi::c_void) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn ObOpenObjectByPointer(object : *const ::core::ffi::c_void, handleattributes : u32, passedaccessstate : *const super::super::Foundation:: ACCESS_STATE, desiredaccess : u32, objecttype : *const super::super::Foundation:: OBJECT_TYPE, accessmode : i8, handle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn ObOpenObjectByPointerWithTag(object : *const ::core::ffi::c_void, handleattributes : u32, passedaccessstate : *const super::super::Foundation:: ACCESS_STATE, desiredaccess : u32, objecttype : *const super::super::Foundation:: OBJECT_TYPE, accessmode : i8, tag : u32, handle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn ObQueryNameString(object : *const ::core::ffi::c_void, objectnameinfo : *mut super::super::Foundation:: OBJECT_NAME_INFORMATION, length : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ObQueryObjectAuditingByHandle(handle : super::super::super::Win32::Foundation:: HANDLE, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn PfxFindPrefix(prefixtable : *const PREFIX_TABLE, fullname : *const super::super::super::Win32::System::Kernel:: STRING) -> *mut PREFIX_TABLE_ENTRY);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn PfxInitialize(prefixtable : *mut PREFIX_TABLE) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn PfxInsertPrefix(prefixtable : *const PREFIX_TABLE, prefix : *const super::super::super::Win32::System::Kernel:: STRING, prefixtableentry : *mut PREFIX_TABLE_ENTRY) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn PfxRemovePrefix(prefixtable : *const PREFIX_TABLE, prefixtableentry : *const PREFIX_TABLE_ENTRY) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn PoQueueShutdownWorkItem(workitem : *mut super::super::Foundation:: WORK_QUEUE_ITEM) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn PsAssignImpersonationToken(thread : *const super::super::Foundation:: KTHREAD, token : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn PsChargePoolQuota(process : *const super::super::Foundation:: KPROCESS, pooltype : super::super::Foundation:: POOL_TYPE, amount : usize) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn PsChargeProcessPoolQuota(process : *const super::super::Foundation:: KPROCESS, pooltype : super::super::Foundation:: POOL_TYPE, amount : usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn PsDereferenceImpersonationToken(impersonationtoken : *const ::core::ffi::c_void) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn PsDereferencePrimaryToken(primarytoken : *const ::core::ffi::c_void) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn PsDisableImpersonation(thread : *mut super::super::Foundation:: KTHREAD, impersonationstate : *mut super::super::super::Win32::Security:: SE_IMPERSONATION_STATE) -> super::super::super::Win32::Foundation:: BOOLEAN);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn PsGetProcessExitTime() -> i64);
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn PsGetThreadProcess(thread : *const super::super::Foundation:: KTHREAD) -> *mut super::super::Foundation:: KPROCESS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn PsImpersonateClient(thread : *mut super::super::Foundation:: KTHREAD, token : *const ::core::ffi::c_void, copyonopen : super::super::super::Win32::Foundation:: BOOLEAN, effectiveonly : super::super::super::Win32::Foundation:: BOOLEAN, impersonationlevel : super::super::super::Win32::Security:: SECURITY_IMPERSONATION_LEVEL) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn PsIsDiskCountersEnabled() -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn PsIsSystemThread(thread : *const super::super::Foundation:: KTHREAD) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn PsIsThreadTerminating(thread : *const super::super::Foundation:: KTHREAD) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn PsLookupProcessByProcessId(processid : super::super::super::Win32::Foundation:: HANDLE, process : *mut *mut super::super::Foundation:: KPROCESS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn PsLookupThreadByThreadId(threadid : super::super::super::Win32::Foundation:: HANDLE, thread : *mut *mut super::super::Foundation:: KTHREAD) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn PsReferenceImpersonationToken(thread : *mut super::super::Foundation:: KTHREAD, copyonopen : *mut super::super::super::Win32::Foundation:: BOOLEAN, effectiveonly : *mut super::super::super::Win32::Foundation:: BOOLEAN, impersonationlevel : *mut super::super::super::Win32::Security:: SECURITY_IMPERSONATION_LEVEL) -> *mut ::core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn PsReferencePrimaryToken(process : *mut super::super::Foundation:: KPROCESS) -> *mut ::core::ffi::c_void);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn PsRestoreImpersonation(thread : *mut super::super::Foundation:: KTHREAD, impersonationstate : *const super::super::super::Win32::Security:: SE_IMPERSONATION_STATE) -> ());
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn PsReturnPoolQuota(process : *const super::super::Foundation:: KPROCESS, pooltype : super::super::Foundation:: POOL_TYPE, amount : usize) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn PsRevertToSelf() -> ());
#[cfg(feature = "Wdk_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"] fn PsUpdateDiskCounters(process : *mut super::super::Foundation:: KPROCESS, bytesread : u64, byteswritten : u64, readoperationcount : u32, writeoperationcount : u32, flushoperationcount : u32) -> ());
::windows_targets::link!("sspicli.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn QueryContextAttributesExW(phcontext : *const SecHandle, ulattribute : u32, pbuffer : *mut ::core::ffi::c_void, cbbuffer : u32) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn QueryContextAttributesW(phcontext : *const SecHandle, ulattribute : u32, pbuffer : *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("sspicli.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn QueryCredentialsAttributesExW(phcredential : *const SecHandle, ulattribute : u32, pbuffer : *mut ::core::ffi::c_void, cbbuffer : u32) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn QueryCredentialsAttributesW(phcredential : *const SecHandle, ulattribute : u32, pbuffer : *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn QuerySecurityContextToken(phcontext : *const SecHandle, token : *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn QuerySecurityPackageInfoW(ppackagename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, pppackageinfo : *mut *mut SecPkgInfoW) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RevertSecurityContext(phcontext : *const SecHandle) -> ::windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlAbsoluteToSelfRelativeSD(absolutesecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, selfrelativesecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, bufferlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlAddAccessAllowedAce(acl : *mut super::super::super::Win32::Security:: ACL, acerevision : u32, accessmask : u32, sid : super::super::super::Win32::Foundation:: PSID) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlAddAccessAllowedAceEx(acl : *mut super::super::super::Win32::Security:: ACL, acerevision : u32, aceflags : u32, accessmask : u32, sid : super::super::super::Win32::Foundation:: PSID) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlAddAce(acl : *mut super::super::super::Win32::Security:: ACL, acerevision : u32, startingaceindex : u32, acelist : *const ::core::ffi::c_void, acelistlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlAllocateAndInitializeSid(identifierauthority : *const SID_IDENTIFIER_AUTHORITY, subauthoritycount : u8, subauthority0 : u32, subauthority1 : u32, subauthority2 : u32, subauthority3 : u32, subauthority4 : u32, subauthority5 : u32, subauthority6 : u32, subauthority7 : u32, sid : *mut super::super::super::Win32::Foundation:: PSID) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlAllocateAndInitializeSidEx(identifierauthority : *const SID_IDENTIFIER_AUTHORITY, subauthoritycount : u8, subauthorities : *const u32, sid : *mut super::super::super::Win32::Foundation:: PSID) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlAllocateHeap(heaphandle : *const ::core::ffi::c_void, flags : u32, size : usize) -> *mut ::core::ffi::c_void);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn RtlAppendStringToString(destination : *mut super::super::super::Win32::System::Kernel:: STRING, source : *const super::super::super::Win32::System::Kernel:: STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("kernel32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlCaptureStackBackTrace(framestoskip : u32, framestocapture : u32, backtrace : *mut *mut ::core::ffi::c_void, backtracehash : *mut u32) -> u16);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlCompareAltitudes(altitude1 : *const super::super::super::Win32::Foundation:: UNICODE_STRING, altitude2 : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> i32);
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlCompareMemoryUlong(source : *const ::core::ffi::c_void, length : usize, pattern : u32) -> usize);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlCompressBuffer(compressionformatandengine : u16, uncompressedbuffer : *const u8, uncompressedbuffersize : u32, compressedbuffer : *mut u8, compressedbuffersize : u32, uncompressedchunksize : u32, finalcompressedsize : *mut u32, workspace : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlCompressChunks(uncompressedbuffer : *const u8, uncompressedbuffersize : u32, compressedbuffer : *mut u8, compressedbuffersize : u32, compresseddatainfo : *mut COMPRESSED_DATA_INFO, compresseddatainfolength : u32, workspace : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlConvertSidToUnicodeString(unicodestring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, sid : super::super::super::Win32::Foundation:: PSID, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlCopyLuid(destinationluid : *mut super::super::super::Win32::Foundation:: LUID, sourceluid : *const super::super::super::Win32::Foundation:: LUID) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlCopySid(destinationsidlength : u32, destinationsid : super::super::super::Win32::Foundation:: PSID, sourcesid : super::super::super::Win32::Foundation:: PSID) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlCreateAcl(acl : *mut super::super::super::Win32::Security:: ACL, acllength : u32, aclrevision : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlCreateHeap(flags : u32, heapbase : *const ::core::ffi::c_void, reservesize : usize, commitsize : usize, lock : *const ::core::ffi::c_void, parameters : *const RTL_HEAP_PARAMETERS) -> *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlCreateServiceSid(servicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, servicesid : super::super::super::Win32::Foundation:: PSID, servicesidlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlCreateSystemVolumeInformationFolder(volumerootpath : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlCreateUnicodeString(destinationstring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, sourcestring : ::windows_sys::core::PCWSTR) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlCreateVirtualAccountSid(name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, basesubauthority : u32, sid : super::super::super::Win32::Foundation:: PSID, sidlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlCustomCPToUnicodeN(customcp : *const CPTABLEINFO, unicodestring : ::windows_sys::core::PWSTR, maxbytesinunicodestring : u32, bytesinunicodestring : *mut u32, customcpstring : ::windows_sys::core::PCSTR, bytesincustomcpstring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlDecompressBuffer(compressionformat : u16, uncompressedbuffer : *mut u8, uncompressedbuffersize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, finaluncompressedsize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlDecompressBufferEx(compressionformat : u16, uncompressedbuffer : *mut u8, uncompressedbuffersize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, finaluncompressedsize : *mut u32, workspace : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlDecompressBufferEx2(compressionformat : u16, uncompressedbuffer : *mut u8, uncompressedbuffersize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, uncompressedchunksize : u32, finaluncompressedsize : *mut u32, workspace : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlDecompressChunks(uncompressedbuffer : *mut u8, uncompressedbuffersize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, compressedtail : *const u8, compressedtailsize : u32, compresseddatainfo : *const COMPRESSED_DATA_INFO) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlDecompressFragment(compressionformat : u16, uncompressedfragment : *mut u8, uncompressedfragmentsize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, fragmentoffset : u32, finaluncompressedsize : *mut u32, workspace : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlDecompressFragmentEx(compressionformat : u16, uncompressedfragment : *mut u8, uncompressedfragmentsize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, fragmentoffset : u32, uncompressedchunksize : u32, finaluncompressedsize : *mut u32, workspace : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlDeleteAce(acl : *mut super::super::super::Win32::Security:: ACL, aceindex : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlDescribeChunk(compressionformat : u16, compressedbuffer : *mut *mut u8, endofcompressedbufferplus1 : *const u8, chunkbuffer : *mut *mut u8, chunksize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlDestroyHeap(heaphandle : *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlDowncaseUnicodeString(destinationstring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, sourcestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlDuplicateUnicodeString(flags : u32, stringin : *const super::super::super::Win32::Foundation:: UNICODE_STRING, stringout : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlEqualPrefixSid(sid1 : super::super::super::Win32::Foundation:: PSID, sid2 : super::super::super::Win32::Foundation:: PSID) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlEqualSid(sid1 : super::super::super::Win32::Foundation:: PSID, sid2 : super::super::super::Win32::Foundation:: PSID) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn RtlFindUnicodePrefix(prefixtable : *const UNICODE_PREFIX_TABLE, fullname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, caseinsensitiveindex : u32) -> *mut UNICODE_PREFIX_TABLE_ENTRY);
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlFreeHeap(heaphandle : *const ::core::ffi::c_void, flags : u32, baseaddress : *const ::core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`*"] fn RtlFreeOemString(oemstring : *mut super::super::super::Win32::System::Kernel:: STRING) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlFreeSid(sid : super::super::super::Win32::Foundation:: PSID) -> *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlGenerate8dot3Name(name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, allowextendedcharacters : super::super::super::Win32::Foundation:: BOOLEAN, context : *mut GENERATE_NAME_CONTEXT, name8dot3 : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlGetAce(acl : *const super::super::super::Win32::Security:: ACL, aceindex : u32, ace : *mut *mut ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlGetCompressionWorkSpaceSize(compressionformatandengine : u16, compressbufferworkspacesize : *mut u32, compressfragmentworkspacesize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlGetDaclSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, daclpresent : *mut super::super::super::Win32::Foundation:: BOOLEAN, dacl : *mut *mut super::super::super::Win32::Security:: ACL, dacldefaulted : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlGetGroupSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, group : *mut super::super::super::Win32::Foundation:: PSID, groupdefaulted : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlGetOwnerSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, owner : *mut super::super::super::Win32::Foundation:: PSID, ownerdefaulted : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlGetSaclSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, saclpresent : *mut super::super::super::Win32::Foundation:: BOOLEAN, sacl : *mut *mut super::super::super::Win32::Security:: ACL, sacldefaulted : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlIdentifierAuthoritySid(sid : super::super::super::Win32::Foundation:: PSID) -> *mut SID_IDENTIFIER_AUTHORITY);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlIdnToAscii(flags : u32, sourcestring : ::windows_sys::core::PCWSTR, sourcestringlength : i32, destinationstring : ::windows_sys::core::PWSTR, destinationstringlength : *mut i32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlIdnToNameprepUnicode(flags : u32, sourcestring : ::windows_sys::core::PCWSTR, sourcestringlength : i32, destinationstring : ::windows_sys::core::PWSTR, destinationstringlength : *mut i32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlIdnToUnicode(flags : u32, sourcestring : ::windows_sys::core::PCWSTR, sourcestringlength : i32, destinationstring : ::windows_sys::core::PWSTR, destinationstringlength : *mut i32) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlInitCodePageTable(tablebase : *const u16, codepagetable : *mut CPTABLEINFO) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlInitUnicodeStringEx(destinationstring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, sourcestring : ::windows_sys::core::PCWSTR) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlInitializeSid(sid : super::super::super::Win32::Foundation:: PSID, identifierauthority : *const SID_IDENTIFIER_AUTHORITY, subauthoritycount : u8) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "cdecl" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlInitializeSidEx(sid : super::super::super::Win32::Foundation:: PSID, identifierauthority : *const SID_IDENTIFIER_AUTHORITY, subauthoritycount : u8, ...) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn RtlInitializeUnicodePrefix(prefixtable : *mut UNICODE_PREFIX_TABLE) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn RtlInsertUnicodePrefix(prefixtable : *const UNICODE_PREFIX_TABLE, prefix : *const super::super::super::Win32::Foundation:: UNICODE_STRING, prefixtableentry : *mut UNICODE_PREFIX_TABLE_ENTRY) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlIsCloudFilesPlaceholder(fileattributes : u32, reparsetag : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn RtlIsNameLegalDOS8Dot3(name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, oemname : *mut super::super::super::Win32::System::Kernel:: STRING, namecontainsspaces : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlIsNonEmptyDirectoryReparsePointAllowed(reparsetag : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlIsNormalizedString(normform : u32, sourcestring : ::windows_sys::core::PCWSTR, sourcestringlength : i32, normalized : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlIsPartialPlaceholder(fileattributes : u32, reparsetag : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlIsPartialPlaceholderFileHandle(filehandle : super::super::super::Win32::Foundation:: HANDLE, ispartialplaceholder : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn RtlIsPartialPlaceholderFileInfo(infobuffer : *const ::core::ffi::c_void, infoclass : super::super::super::Win32::System::WindowsProgramming:: FILE_INFORMATION_CLASS, ispartialplaceholder : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlIsSandboxedToken(context : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, previousmode : i8) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlIsValidOemCharacter(char : ::windows_sys::core::PWSTR) -> super::super::super::Win32::Foundation:: BOOLEAN);
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlLengthRequiredSid(subauthoritycount : u32) -> u32);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlLengthSid(sid : super::super::super::Win32::Foundation:: PSID) -> u32);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlMultiByteToUnicodeN(unicodestring : ::windows_sys::core::PWSTR, maxbytesinunicodestring : u32, bytesinunicodestring : *mut u32, multibytestring : ::windows_sys::core::PCSTR, bytesinmultibytestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlMultiByteToUnicodeSize(bytesinunicodestring : *mut u32, multibytestring : ::windows_sys::core::PCSTR, bytesinmultibytestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn RtlNextUnicodePrefix(prefixtable : *const UNICODE_PREFIX_TABLE, restart : super::super::super::Win32::Foundation:: BOOLEAN) -> *mut UNICODE_PREFIX_TABLE_ENTRY);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlNormalizeString(normform : u32, sourcestring : ::windows_sys::core::PCWSTR, sourcestringlength : i32, destinationstring : ::windows_sys::core::PWSTR, destinationstringlength : *mut i32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlNtStatusToDosError(status : super::super::super::Win32::Foundation:: NTSTATUS) -> u32);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlNtStatusToDosErrorNoTeb(status : super::super::super::Win32::Foundation:: NTSTATUS) -> u32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn RtlOemStringToCountedUnicodeString(destinationstring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, sourcestring : *const super::super::super::Win32::System::Kernel:: STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn RtlOemStringToUnicodeString(destinationstring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, sourcestring : *const super::super::super::Win32::System::Kernel:: STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlOemToUnicodeN(unicodestring : ::windows_sys::core::PWSTR, maxbytesinunicodestring : u32, bytesinunicodestring : *mut u32, oemstring : ::windows_sys::core::PCSTR, bytesinoemstring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn RtlPrefixString(string1 : *const super::super::super::Win32::System::Kernel:: STRING, string2 : *const super::super::super::Win32::System::Kernel:: STRING, caseinsensitive : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlQueryPackageIdentity(tokenobject : *const ::core::ffi::c_void, packagefullname : ::windows_sys::core::PWSTR, packagesize : *mut usize, appid : ::windows_sys::core::PWSTR, appidsize : *mut usize, packaged : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlQueryPackageIdentityEx(tokenobject : *const ::core::ffi::c_void, packagefullname : ::windows_sys::core::PWSTR, packagesize : *mut usize, appid : ::windows_sys::core::PWSTR, appidsize : *mut usize, dynamicid : *mut ::windows_sys::core::GUID, flags : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlQueryProcessPlaceholderCompatibilityMode() -> u8);
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlQueryThreadPlaceholderCompatibilityMode() -> u8);
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlRandom(seed : *mut u32) -> u32);
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlRandomEx(seed : *mut u32) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn RtlRemoveUnicodePrefix(prefixtable : *const UNICODE_PREFIX_TABLE, prefixtableentry : *const UNICODE_PREFIX_TABLE_ENTRY) -> ());
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlReplaceSidInSd(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, oldsid : super::super::super::Win32::Foundation:: PSID, newsid : super::super::super::Win32::Foundation:: PSID, numchanges : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlReserveChunk(compressionformat : u16, compressedbuffer : *mut *mut u8, endofcompressedbufferplus1 : *const u8, chunkbuffer : *mut *mut u8, chunksize : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlSecondsSince1970ToTime(elapsedseconds : u32, time : *mut i64) -> ());
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlSecondsSince1980ToTime(elapsedseconds : u32, time : *mut i64) -> ());
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlSelfRelativeToAbsoluteSD(selfrelativesecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, absolutesecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, absolutesecuritydescriptorsize : *mut u32, dacl : *mut super::super::super::Win32::Security:: ACL, daclsize : *mut u32, sacl : *mut super::super::super::Win32::Security:: ACL, saclsize : *mut u32, owner : super::super::super::Win32::Foundation:: PSID, ownersize : *mut u32, primarygroup : super::super::super::Win32::Foundation:: PSID, primarygroupsize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlSetGroupSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, group : super::super::super::Win32::Foundation:: PSID, groupdefaulted : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn RtlSetOwnerSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, owner : super::super::super::Win32::Foundation:: PSID, ownerdefaulted : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlSetProcessPlaceholderCompatibilityMode(mode : u8) -> u8);
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn RtlSetThreadPlaceholderCompatibilityMode(mode : u8) -> u8);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlSubAuthorityCountSid(sid : super::super::super::Win32::Foundation:: PSID) -> *mut u8);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlSubAuthoritySid(sid : super::super::super::Win32::Foundation:: PSID, subauthority : u32) -> *mut u32);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlTimeToSecondsSince1970(time : *const i64, elapsedseconds : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlTimeToSecondsSince1980(time : *const i64, elapsedseconds : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn RtlUnicodeStringToCountedOemString(destinationstring : *mut super::super::super::Win32::System::Kernel:: STRING, sourcestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn RtlUnicodeStringToOemString(destinationstring : *mut super::super::super::Win32::System::Kernel:: STRING, sourcestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlUnicodeToCustomCPN(customcp : *const CPTABLEINFO, customcpstring : ::windows_sys::core::PSTR, maxbytesincustomcpstring : u32, bytesincustomcpstring : *mut u32, unicodestring : ::windows_sys::core::PCWSTR, bytesinunicodestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlUnicodeToMultiByteN(multibytestring : ::windows_sys::core::PSTR, maxbytesinmultibytestring : u32, bytesinmultibytestring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlUnicodeToMultiByteSize(bytesinmultibytestring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlUnicodeToOemN(oemstring : ::windows_sys::core::PSTR, maxbytesinoemstring : u32, bytesinoemstring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn RtlUpcaseUnicodeStringToCountedOemString(destinationstring : *mut super::super::super::Win32::System::Kernel:: STRING, sourcestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn RtlUpcaseUnicodeStringToOemString(destinationstring : *mut super::super::super::Win32::System::Kernel:: STRING, sourcestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlUpcaseUnicodeToCustomCPN(customcp : *const CPTABLEINFO, customcpstring : ::windows_sys::core::PSTR, maxbytesincustomcpstring : u32, bytesincustomcpstring : *mut u32, unicodestring : ::windows_sys::core::PCWSTR, bytesinunicodestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlUpcaseUnicodeToMultiByteN(multibytestring : ::windows_sys::core::PSTR, maxbytesinmultibytestring : u32, bytesinmultibytestring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlUpcaseUnicodeToOemN(oemstring : ::windows_sys::core::PSTR, maxbytesinoemstring : u32, bytesinoemstring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlValidSid(sid : super::super::super::Win32::Foundation:: PSID) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlValidateUnicodeString(flags : u32, string : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`*"] fn RtlxOemStringToUnicodeSize(oemstring : *const super::super::super::Win32::System::Kernel:: STRING) -> u32);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn RtlxUnicodeStringToOemSize(unicodestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> u32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeAccessCheckFromState(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, primarytokeninformation : *const TOKEN_ACCESS_INFORMATION, clienttokeninformation : *const TOKEN_ACCESS_INFORMATION, desiredaccess : u32, previouslygrantedaccess : u32, privileges : *mut *mut super::super::super::Win32::Security:: PRIVILEGE_SET, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, accessmode : i8, grantedaccess : *mut u32, accessstatus : *mut i32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeAccessCheckFromStateEx(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, primarytoken : *const ::core::ffi::c_void, clienttoken : *const ::core::ffi::c_void, desiredaccess : u32, previouslygrantedaccess : u32, privileges : *mut *mut super::super::super::Win32::Security:: PRIVILEGE_SET, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, accessmode : i8, grantedaccess : *mut u32, accessstatus : *mut i32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeAdjustAccessStateForAccessConstraints(objecttype : *const ::core::ffi::c_void, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, accessstate : *mut super::super::Foundation:: ACCESS_STATE) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeAdjustAccessStateForTrustLabel(objecttype : *const ::core::ffi::c_void, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, accessstate : *mut super::super::Foundation:: ACCESS_STATE) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeAdjustObjectSecurity(objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, originaldescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, proposeddescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, adjusteddescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, applyadjusteddescriptor : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeAppendPrivileges(accessstate : *mut super::super::Foundation:: ACCESS_STATE, privileges : *const super::super::super::Win32::Security:: PRIVILEGE_SET) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeAuditFipsCryptoSelftests(bsuccess : super::super::super::Win32::Foundation:: BOOLEAN, selftestcode : u32) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeAuditHardLinkCreation(filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, linkname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, bsuccess : super::super::super::Win32::Foundation:: BOOLEAN) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeAuditHardLinkCreationWithTransaction(filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, linkname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, bsuccess : super::super::super::Win32::Foundation:: BOOLEAN, transactionid : *const ::windows_sys::core::GUID) -> ());
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SeAuditTransactionStateChange(transactionid : *const ::windows_sys::core::GUID, resourcemanagerid : *const ::windows_sys::core::GUID, newtransactionstate : u32) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeAuditingAnyFileEventsWithContext(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeAuditingAnyFileEventsWithContextEx(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, stagingenabled : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeAuditingFileEvents(accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeAuditingFileEventsWithContext(accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeAuditingFileEventsWithContextEx(accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, stagingenabled : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeAuditingFileOrGlobalEvents(accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeAuditingHardLinkEvents(accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeAuditingHardLinkEventsWithContext(accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Security\"`*"] fn SeCaptureSubjectContextEx(thread : *const super::super::Foundation:: KTHREAD, process : *const super::super::Foundation:: KPROCESS, subjectcontext : *mut super::super::Foundation:: SECURITY_SUBJECT_CONTEXT) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeCheckForCriticalAceRemoval(currentdescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, newdescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, aceremoved : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeCreateClientSecurity(clientthread : *const super::super::Foundation:: KTHREAD, clientsecurityqos : *const super::super::super::Win32::Security:: SECURITY_QUALITY_OF_SERVICE, remotesession : super::super::super::Win32::Foundation:: BOOLEAN, clientcontext : *mut SECURITY_CLIENT_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeCreateClientSecurityFromSubjectContext(subjectcontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, clientsecurityqos : *const super::super::super::Win32::Security:: SECURITY_QUALITY_OF_SERVICE, serverisremote : super::super::super::Win32::Foundation:: BOOLEAN, clientcontext : *mut SECURITY_CLIENT_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeDeleteClientSecurity(clientcontext : *mut SECURITY_CLIENT_CONTEXT) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeDeleteObjectAuditAlarm(object : *const ::core::ffi::c_void, handle : super::super::super::Win32::Foundation:: HANDLE) -> ());
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeDeleteObjectAuditAlarmWithTransaction(object : *const ::core::ffi::c_void, handle : super::super::super::Win32::Foundation:: HANDLE, transactionid : *const ::windows_sys::core::GUID) -> ());
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeExamineSacl(sacl : *const super::super::super::Win32::Security:: ACL, resourcesacl : *const super::super::super::Win32::Security:: ACL, token : *const ::core::ffi::c_void, desiredaccess : u32, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, generateaudit : *mut super::super::super::Win32::Foundation:: BOOLEAN, generatealarm : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> ());
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeFilterToken(existingtoken : *const ::core::ffi::c_void, flags : u32, sidstodisable : *const TOKEN_GROUPS, privilegestodelete : *const TOKEN_PRIVILEGES, restrictedsids : *const TOKEN_GROUPS, filteredtoken : *mut *mut ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeFreePrivileges(privileges : *const super::super::super::Win32::Security:: PRIVILEGE_SET) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeImpersonateClient(clientcontext : *const SECURITY_CLIENT_CONTEXT, serverthread : *const super::super::Foundation:: KTHREAD) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeImpersonateClientEx(clientcontext : *const SECURITY_CLIENT_CONTEXT, serverthread : *const super::super::Foundation:: KTHREAD) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn SeLocateProcessImageName(process : *mut super::super::Foundation:: KPROCESS, pimagefilename : *mut *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeMarkLogonSessionForTerminationNotification(logonid : *const super::super::super::Win32::Foundation:: LUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn SeMarkLogonSessionForTerminationNotificationEx(logonid : *const super::super::super::Win32::Foundation:: LUID, pserversilo : *const super::super::Foundation:: EJOB) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeOpenObjectAuditAlarm(objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, object : *const ::core::ffi::c_void, absoluteobjectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, accessstate : *const super::super::Foundation:: ACCESS_STATE, objectcreated : super::super::super::Win32::Foundation:: BOOLEAN, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, accessmode : i8, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeOpenObjectAuditAlarmWithTransaction(objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, object : *const ::core::ffi::c_void, absoluteobjectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, accessstate : *const super::super::Foundation:: ACCESS_STATE, objectcreated : super::super::super::Win32::Foundation:: BOOLEAN, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, accessmode : i8, transactionid : *const ::windows_sys::core::GUID, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeOpenObjectForDeleteAuditAlarm(objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, object : *const ::core::ffi::c_void, absoluteobjectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, accessstate : *const super::super::Foundation:: ACCESS_STATE, objectcreated : super::super::super::Win32::Foundation:: BOOLEAN, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, accessmode : i8, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeOpenObjectForDeleteAuditAlarmWithTransaction(objecttypename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, object : *const ::core::ffi::c_void, absoluteobjectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, accessstate : *const super::super::Foundation:: ACCESS_STATE, objectcreated : super::super::super::Win32::Foundation:: BOOLEAN, accessgranted : super::super::super::Win32::Foundation:: BOOLEAN, accessmode : i8, transactionid : *const ::windows_sys::core::GUID, generateonclose : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SePrivilegeCheck(requiredprivileges : *mut super::super::super::Win32::Security:: PRIVILEGE_SET, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, accessmode : i8) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeQueryAuthenticationIdToken(token : *const ::core::ffi::c_void, authenticationid : *mut super::super::super::Win32::Foundation:: LUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeQueryInformationToken(token : *const ::core::ffi::c_void, tokeninformationclass : TOKEN_INFORMATION_CLASS, tokeninformation : *mut *mut ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeQuerySecurityDescriptorInfo(securityinformation : *const u32, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, length : *mut u32, objectssecuritydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"] fn SeQueryServerSiloToken(token : *const ::core::ffi::c_void, pserversilo : *mut *mut super::super::Foundation:: EJOB) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeQuerySessionIdToken(token : *const ::core::ffi::c_void, sessionid : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeQuerySessionIdTokenEx(token : *const ::core::ffi::c_void, sessionid : *mut u32, isservicesession : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeRegisterLogonSessionTerminatedRoutine(callbackroutine : PSE_LOGON_SESSION_TERMINATED_ROUTINE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeRegisterLogonSessionTerminatedRoutineEx(callbackroutine : PSE_LOGON_SESSION_TERMINATED_ROUTINE_EX, context : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`*"] fn SeReportSecurityEventWithSubCategory(flags : u32, sourcename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, usersid : super::super::super::Win32::Foundation:: PSID, auditparameters : *const super::super::super::Win32::Security::Authentication::Identity:: SE_ADT_PARAMETER_ARRAY, auditsubcategoryid : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeSetAccessStateGenericMapping(accessstate : *mut super::super::Foundation:: ACCESS_STATE, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING) -> ());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeSetSecurityDescriptorInfo(object : *const ::core::ffi::c_void, securityinformation : *const u32, modificationdescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, objectssecuritydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, pooltype : super::super::Foundation:: POOL_TYPE, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeSetSecurityDescriptorInfoEx(object : *const ::core::ffi::c_void, securityinformation : *const u32, modificationdescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, objectssecuritydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, autoinheritflags : u32, pooltype : super::super::Foundation:: POOL_TYPE, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeShouldCheckForAccessRightsFromParent(objecttype : *const ::core::ffi::c_void, childdescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, accessstate : *const super::super::Foundation:: ACCESS_STATE) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SeTokenFromAccessInformation(accessinformation : *const TOKEN_ACCESS_INFORMATION, token : *mut ::core::ffi::c_void, length : u32, requiredlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeTokenIsAdmin(token : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeTokenIsRestricted(token : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeTokenIsWriteRestricted(token : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SeTokenType(token : *const ::core::ffi::c_void) -> TOKEN_TYPE);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeUnregisterLogonSessionTerminatedRoutine(callbackroutine : PSE_LOGON_SESSION_TERMINATED_ROUTINE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntoskrnl.exe" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SeUnregisterLogonSessionTerminatedRoutineEx(callbackroutine : PSE_LOGON_SESSION_TERMINATED_ROUTINE_EX, context : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SecLookupAccountName(name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, sidsize : *mut u32, sid : super::super::super::Win32::Foundation:: PSID, nameuse : *mut SID_NAME_USE, domainsize : *mut u32, referenceddomain : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SecLookupAccountSid(sid : super::super::super::Win32::Foundation:: PSID, namesize : *mut u32, namebuffer : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, domainsize : *mut u32, domainbuffer : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, nameuse : *mut SID_NAME_USE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn SecLookupWellKnownSid(sidtype : super::super::super::Win32::Security:: WELL_KNOWN_SID_TYPE, sid : super::super::super::Win32::Foundation:: PSID, sidbuffersize : u32, sidsize : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SecMakeSPN(serviceclass : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, servicename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, instancename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, instanceport : u16, referrer : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, spn : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, length : *mut u32, allocate : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SecMakeSPNEx(serviceclass : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, servicename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, instancename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, instanceport : u16, referrer : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, targetinfo : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, spn : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, length : *mut u32, allocate : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SecMakeSPNEx2(serviceclass : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, servicename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, instancename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, instanceport : u16, referrer : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, intargetinfo : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, spn : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, totalsize : *mut u32, allocate : super::super::super::Win32::Foundation:: BOOLEAN, istargetinfomarshaled : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SetContextAttributesW(phcontext : *const SecHandle, ulattribute : u32, pbuffer : *const ::core::ffi::c_void, cbbuffer : u32) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SetCredentialsAttributesW(phcredential : *const SecHandle, ulattribute : u32, pbuffer : *const ::core::ffi::c_void, cbbuffer : u32) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiAcceptSecurityContextAsync(asynccontext : *mut SspiAsyncContext, phcredential : *const SecHandle, phcontext : *const SecHandle, pinput : *const SecBufferDesc, fcontextreq : u32, targetdatarep : u32, phnewcontext : *const SecHandle, poutput : *const SecBufferDesc, pfcontextattr : *const u32, ptsexpiry : *const i64) -> ::windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SspiAcquireCredentialsHandleAsyncW(asynccontext : *mut SspiAsyncContext, pszprincipal : *const super::super::super::Win32::Foundation:: UNICODE_STRING, pszpackage : *const super::super::super::Win32::Foundation:: UNICODE_STRING, fcredentialuse : u32, pvlogonid : *const ::core::ffi::c_void, pauthdata : *const ::core::ffi::c_void, pgetkeyfn : SEC_GET_KEY_FN, pvgetkeyargument : *const ::core::ffi::c_void, phcredential : *const SecHandle, ptsexpiry : *const i64) -> ::windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SspiCompareAuthIdentities(authidentity1 : *const ::core::ffi::c_void, authidentity2 : *const ::core::ffi::c_void, samesupplieduser : *mut super::super::super::Win32::Foundation:: BOOLEAN, samesuppliedidentity : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiCopyAuthIdentity(authdata : *const ::core::ffi::c_void, authdatacopy : *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiCreateAsyncContext() -> *mut SspiAsyncContext);
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiDeleteSecurityContextAsync(asynccontext : *mut SspiAsyncContext, phcontext : *const SecHandle) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiEncodeAuthIdentityAsStrings(pauthidentity : *const ::core::ffi::c_void, ppszusername : *mut ::windows_sys::core::PCWSTR, ppszdomainname : *mut ::windows_sys::core::PCWSTR, ppszpackedcredentialsstring : *mut ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiEncodeStringsAsAuthIdentity(pszusername : ::windows_sys::core::PCWSTR, pszdomainname : ::windows_sys::core::PCWSTR, pszpackedcredentialsstring : ::windows_sys::core::PCWSTR, ppauthidentity : *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiExcludePackage(authidentity : *const ::core::ffi::c_void, pszpackagename : ::windows_sys::core::PCWSTR, ppnewauthidentity : *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiFreeAsyncContext(handle : *const SspiAsyncContext) -> ());
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiFreeAuthIdentity(authdata : *const ::core::ffi::c_void) -> ());
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiFreeCredentialsHandleAsync(asynccontext : *mut SspiAsyncContext, phcredential : *const SecHandle) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiGetAsyncCallStatus(handle : *const SspiAsyncContext) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiGetTargetHostName(psztargetname : ::windows_sys::core::PCWSTR, pszhostname : *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SspiInitializeSecurityContextAsyncW(asynccontext : *mut SspiAsyncContext, phcredential : *const SecHandle, phcontext : *const SecHandle, psztargetname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, fcontextreq : u32, reserved1 : u32, targetdatarep : u32, pinput : *const SecBufferDesc, reserved2 : u32, phnewcontext : *const SecHandle, poutput : *const SecBufferDesc, pfcontextattr : *const u32, ptsexpiry : *const i64) -> ::windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("credui.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SspiIsPromptingNeeded(errororntstatus : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiLocalFree(databuffer : *const ::core::ffi::c_void) -> ());
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiMarshalAuthIdentity(authidentity : *const ::core::ffi::c_void, authidentitylength : *mut u32, authidentitybytearray : *mut *mut i8) -> ::windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn SspiReinitAsyncContext(handle : *mut SspiAsyncContext) -> super::super::super::Win32::Foundation:: NTSTATUS);
::windows_targets::link!("ksecdd.sys" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiSetAsyncNotifyCallback(context : *const SspiAsyncContext, callback : SspiAsyncNotifyCallback, callbackdata : *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiUnmarshalAuthIdentity(authidentitylength : u32, authidentitybytearray : ::windows_sys::core::PCSTR, ppauthidentity : *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiValidateAuthIdentity(authdata : *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn SspiZeroAuthIdentity(authdata : *const ::core::ffi::c_void) -> ());
::windows_targets::link!("secur32.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"] fn VerifySignature(phcontext : *const SecHandle, pmessage : *const SecBufferDesc, messageseqno : u32, pfqop : *mut u32) -> ::windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ZwAllocateVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut ::core::ffi::c_void, zerobits : usize, regionsize : *mut usize, allocationtype : u32, protect : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Memory\"`*"] fn ZwAllocateVirtualMemoryEx(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut ::core::ffi::c_void, regionsize : *mut usize, allocationtype : u32, pageprotection : u32, extendedparameters : *mut super::super::super::Win32::System::Memory:: MEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwCreateEvent(eventhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::super::Win32::System::WindowsProgramming:: OBJECT_ATTRIBUTES, eventtype : super::super::super::Win32::System::Kernel:: EVENT_TYPE, initialstate : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwDeleteFile(objectattributes : *const super::super::super::Win32::System::WindowsProgramming:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ZwDuplicateObject(sourceprocesshandle : super::super::super::Win32::Foundation:: HANDLE, sourcehandle : super::super::super::Win32::Foundation:: HANDLE, targetprocesshandle : super::super::super::Win32::Foundation:: HANDLE, targethandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, handleattributes : u32, options : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwDuplicateToken(existingtokenhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::super::Win32::System::WindowsProgramming:: OBJECT_ATTRIBUTES, effectiveonly : super::super::super::Win32::Foundation:: BOOLEAN, tokentype : TOKEN_TYPE, newtokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwFlushBuffersFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwFlushBuffersFileEx(filehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, parameters : *const ::core::ffi::c_void, parameterssize : u32, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwFlushVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut ::core::ffi::c_void, regionsize : *mut usize, iostatus : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ZwFreeVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut ::core::ffi::c_void, regionsize : *mut usize, freetype : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwFsControlFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::Foundation:: PIO_APC_ROUTINE, apccontext : *const ::core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, fscontrolcode : u32, inputbuffer : *const ::core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut ::core::ffi::c_void, outputbufferlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwLockFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::Foundation:: PIO_APC_ROUTINE, apccontext : *const ::core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, byteoffset : *const i64, length : *const i64, key : u32, failimmediately : super::super::super::Win32::Foundation:: BOOLEAN, exclusivelock : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwNotifyChangeKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::Foundation:: PIO_APC_ROUTINE, apccontext : *const ::core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, completionfilter : u32, watchtree : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *mut ::core::ffi::c_void, buffersize : u32, asynchronous : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwOpenDirectoryObject(directoryhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::super::Win32::System::WindowsProgramming:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ZwOpenProcessTokenEx(processhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, handleattributes : u32, tokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ZwOpenThreadTokenEx(threadhandle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, openasself : super::super::super::Win32::Foundation:: BOOLEAN, handleattributes : u32, tokenhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwQueryDirectoryFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::Foundation:: PIO_APC_ROUTINE, apccontext : *const ::core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, fileinformation : *mut ::core::ffi::c_void, length : u32, fileinformationclass : super::super::super::Win32::System::WindowsProgramming:: FILE_INFORMATION_CLASS, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, restartscan : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwQueryDirectoryFileEx(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::Foundation:: PIO_APC_ROUTINE, apccontext : *const ::core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, fileinformation : *mut ::core::ffi::c_void, length : u32, fileinformationclass : super::super::super::Win32::System::WindowsProgramming:: FILE_INFORMATION_CLASS, queryflags : u32, filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwQueryEaFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, buffer : *mut ::core::ffi::c_void, length : u32, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, ealist : *const ::core::ffi::c_void, ealistlength : u32, eaindex : *const u32, restartscan : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwQueryFullAttributesFile(objectattributes : *const super::super::super::Win32::System::WindowsProgramming:: OBJECT_ATTRIBUTES, fileinformation : *mut FILE_NETWORK_OPEN_INFORMATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ZwQueryInformationToken(tokenhandle : super::super::super::Win32::Foundation:: HANDLE, tokeninformationclass : TOKEN_INFORMATION_CLASS, tokeninformation : *mut ::core::ffi::c_void, tokeninformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ZwQueryObject(handle : super::super::super::Win32::Foundation:: HANDLE, objectinformationclass : OBJECT_INFORMATION_CLASS, objectinformation : *mut ::core::ffi::c_void, objectinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwQueryQuotaInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, buffer : *mut ::core::ffi::c_void, length : u32, returnsingleentry : super::super::super::Win32::Foundation:: BOOLEAN, sidlist : *const ::core::ffi::c_void, sidlistlength : u32, startsid : super::super::super::Win32::Foundation:: PSID, restartscan : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn ZwQuerySecurityObject(handle : super::super::super::Win32::Foundation:: HANDLE, securityinformation : u32, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, length : u32, lengthneeded : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ZwQueryVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *const ::core::ffi::c_void, memoryinformationclass : MEMORY_INFORMATION_CLASS, memoryinformation : *mut ::core::ffi::c_void, memoryinformationlength : usize, returnlength : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwQueryVolumeInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, fsinformation : *mut ::core::ffi::c_void, length : u32, fsinformationclass : FS_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwSetEaFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, buffer : *const ::core::ffi::c_void, length : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ZwSetEvent(eventhandle : super::super::super::Win32::Foundation:: HANDLE, previousstate : *mut i32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ZwSetInformationToken(tokenhandle : super::super::super::Win32::Foundation:: HANDLE, tokeninformationclass : TOKEN_INFORMATION_CLASS, tokeninformation : *const ::core::ffi::c_void, tokeninformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ZwSetInformationVirtualMemory(processhandle : super::super::super::Win32::Foundation:: HANDLE, vminformationclass : VIRTUAL_MEMORY_INFORMATION_CLASS, numberofentries : usize, virtualaddresses : *const MEMORY_RANGE_ENTRY, vminformation : *const ::core::ffi::c_void, vminformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwSetQuotaInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, buffer : *const ::core::ffi::c_void, length : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn ZwSetSecurityObject(handle : super::super::super::Win32::Foundation:: HANDLE, securityinformation : u32, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwSetVolumeInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, fsinformation : *const ::core::ffi::c_void, length : u32, fsinformationclass : FS_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn ZwUnlockFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::WindowsProgramming:: IO_STATUS_BLOCK, byteoffset : *const i64, length : *const i64, key : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"] fn ZwWaitForSingleObject(handle : super::super::super::Win32::Foundation:: HANDLE, alertable : super::super::super::Win32::Foundation:: BOOLEAN, timeout : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_ALLOWED_ACE_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_ALLOWED_CALLBACK_ACE_TYPE: u32 = 9u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_ALLOWED_CALLBACK_OBJECT_ACE_TYPE: u32 = 11u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_ALLOWED_COMPOUND_ACE_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_ALLOWED_OBJECT_ACE_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_DENIED_ACE_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_DENIED_CALLBACK_ACE_TYPE: u32 = 10u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_DENIED_CALLBACK_OBJECT_ACE_TYPE: u32 = 12u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_DENIED_OBJECT_ACE_TYPE: u32 = 6u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_DS_OBJECT_TYPE_NAME_A: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("Directory Service Object");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_DS_OBJECT_TYPE_NAME_W: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Directory Service Object");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_DS_SOURCE_A: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("DS");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_DS_SOURCE_W: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("DS");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_MAX_LEVEL: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_MAX_MS_ACE_TYPE: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_MAX_MS_OBJECT_ACE_TYPE: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_MAX_MS_V2_ACE_TYPE: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_MAX_MS_V3_ACE_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_MAX_MS_V4_ACE_TYPE: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_MAX_MS_V5_ACE_TYPE: u32 = 21u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_MIN_MS_ACE_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_MIN_MS_OBJECT_ACE_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_OBJECT_GUID: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_PROPERTY_GUID: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_PROPERTY_SET_GUID: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_REASON_DATA_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_REASON_EXDATA_MASK: u32 = 2130706432u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_REASON_STAGING_MASK: u32 = 2147483648u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ACCESS_REASON_TYPE_MASK: u32 = 16711680u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_ALLOCATE_MEMORY: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_ALLOW_CONTEXT_REPLAY: u32 = 4194304u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_ALLOW_MISSING_BINDINGS: u32 = 268435456u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_ALLOW_NON_USER_LOGONS: u32 = 2097152u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_ALLOW_NULL_SESSION: u32 = 1048576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_CALL_LEVEL: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_CONFIDENTIALITY: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_CONNECTION: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_DATAGRAM: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_DELEGATE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_EXTENDED_ERROR: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_FRAGMENT_SUPPLIED: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_FRAGMENT_TO_FIT: u32 = 8388608u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_IDENTIFY: u32 = 524288u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_INTEGRITY: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_LICENSING: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_MESSAGES: u64 = 4294967296u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_MUTUAL_AUTH: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_NO_TOKEN: u32 = 16777216u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_PROXY_BINDINGS: u32 = 67108864u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_REPLAY_DETECT: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_SEQUENCE_DETECT: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_SESSION_TICKET: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_STREAM: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_USE_DCE_STYLE: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_REQ_USE_SESSION_KEY: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_ALLOCATED_MEMORY: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_ALLOW_CONTEXT_REPLAY: u32 = 4194304u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_ALLOW_NON_USER_LOGONS: u32 = 2097152u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_CALL_LEVEL: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_CONFIDENTIALITY: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_CONNECTION: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_DATAGRAM: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_DELEGATE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_EXTENDED_ERROR: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_FRAGMENT_ONLY: u32 = 8388608u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_IDENTIFY: u32 = 524288u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_INTEGRITY: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_LICENSING: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_MESSAGES: u64 = 4294967296u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_MUTUAL_AUTH: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_NO_ADDITIONAL_TOKEN: u32 = 33554432u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_NO_TOKEN: u32 = 16777216u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_NULL_SESSION: u32 = 1048576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_REPLAY_DETECT: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_SEQUENCE_DETECT: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_SESSION_TICKET: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_STREAM: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_THIRD_LEG_FAILED: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_USED_DCE_STYLE: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ASC_RET_USE_SESSION_KEY: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_BEST_EFFORT: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_EOF_SPECIFIED: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_FILE_ATTRIBUTES_SPECIFIED: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_GEN_FLAGS_SPECIFIED: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_MARK_USN_SOURCE_INFO: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_OPERATION_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_OP_FLAGS_SPECIFIED: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_REPARSE_POINT_SPECIFIED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_SPARSE_SPECIFIED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_SUPPRESS_DIR_CHANGE_NOTIFY: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_SUPPRESS_FILE_ATTRIBUTE_INHERITANCE: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_SUPPRESS_PARENT_TIMESTAMPS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_TIMESTAMPS_SPECIFIED: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_VDL_SPECIFIED: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_FLAG_WRITE_USN_CLOSE_RECORD: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_IN_OP_FLAG_CASE_SENSITIVE_FLAGS_SPECIFIED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_FLAG_EOF_SET: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_FLAG_FILE_ATTRIBUTES_RETURNED: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_FLAG_FILE_ATTRIBUTES_SET: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_FLAG_FILE_ATTRIBUTE_INHERITANCE_SUPPRESSED: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_FLAG_OPERATION_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_FLAG_OP_FLAGS_HONORED: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_FLAG_REPARSE_POINT_SET: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_FLAG_SPARSE_SET: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_FLAG_TIMESTAMPS_RETURNED: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_FLAG_TIMESTAMPS_SET: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_FLAG_USN_CLOSE_RECORD_WRITTEN: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_FLAG_USN_RETURNED: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_FLAG_USN_SOURCE_INFO_MARKED: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_FLAG_VDL_SET: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ATOMIC_CREATE_ECP_OUT_OP_FLAG_CASE_SENSITIVE_FLAGS_SET: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AUDIT_ALLOW_NO_PRIVILEGE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CACHE_MANAGER_CALLBACKS_EX_V1: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CACHE_USE_DIRECT_ACCESS_MAPPING: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CACHE_VALID_FLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CC_ACQUIRE_DONT_WAIT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CC_ACQUIRE_SUPPORTS_ASYNC_LAZYWRITE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CC_AGGRESSIVE_UNMAP_BEHIND: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CC_DISABLE_DIRTY_PAGE_TRACKING: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CC_DISABLE_READ_AHEAD: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CC_DISABLE_UNMAP_BEHIND: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CC_DISABLE_WRITE_BEHIND: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CC_ENABLE_CPU_CACHE: u32 = 268435456u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CC_ENABLE_DISK_IO_ACCOUNTING: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CC_FLUSH_AND_PURGE_GATHER_DIRTY_BITS: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CC_FLUSH_AND_PURGE_NO_PURGE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CC_FLUSH_AND_PURGE_WRITEABLE_VIEWS_NOTSEEN: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CHECKSUM_TYPE_CRC32: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CHECKSUM_TYPE_CRC64: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CHECKSUM_TYPE_ECC: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CHECKSUM_TYPE_FIRST_UNUSED_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CHECKSUM_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CHECKSUM_TYPE_UNCHANGED: i32 = -1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_CUSTOM_FLAGS: u32 = 4294901760u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_MANDATORY: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_NON_INHERITABLE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: u32 = 6u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_FQBN: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INT64: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_SID: u32 = 5u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_STRING: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_UINT64: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COMPRESSION_ENGINE_HIBER: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COMPRESSION_ENGINE_MASK: u32 = 65280u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COMPRESSION_ENGINE_MAX: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COMPRESSION_ENGINE_MAXIMUM: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COMPRESSION_ENGINE_STANDARD: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COMPRESSION_FORMAT_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COMPRESSION_FORMAT_LZNT1: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COMPRESSION_FORMAT_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COMPRESSION_FORMAT_MAX: u32 = 5u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COMPRESSION_FORMAT_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COMPRESSION_FORMAT_XP10: u32 = 5u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COMPRESSION_FORMAT_XPRESS: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COMPRESSION_FORMAT_XPRESS_HUFF: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CONTAINER_INHERIT_ACE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_BIND_DO_NOT_MAP_NAME: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_BIND_EXCEPTION_ROOT: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_BIND_ROOT: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_BIND_TARGET_ROOT: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_LAYER_ROOT: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_SCRATCH_ROOT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_UNION_LAYER_ROOT: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_EXCEPTION_ROOT: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_ROOT: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_TARGET_ROOT: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CONTAINER_ROOT_INFO_VALID_FLAGS: u32 = 1023u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CONTAINER_VOLUME_STATE_HOSTING_CONTAINER: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COPYFILE_SIS_FLAGS: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COPYFILE_SIS_LINK: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const COPYFILE_SIS_REPLACE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_LAYER: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_REGISTERED_LAYER: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_REMOTE_LAYER: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_SCRATCH: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_USER_MODE: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CRITICAL_ACE_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CSV_INVALID_DEVICE_NUMBER: u32 = 4294967295u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CSV_MGMTLOCK_CHECK_VOLUME_REDIRECTED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CSV_QUERY_MDS_PATH_FLAG_CSV_DIRECT_IO_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CSV_QUERY_MDS_PATH_FLAG_SMB_BYPASS_CSV_ENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CSV_QUERY_MDS_PATH_FLAG_STORAGE_ON_THIS_NODE_IS_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CSV_QUERY_MDS_PATH_V2_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DAX_ALLOC_ALIGNMENT_FLAG_FALLBACK_SPECIFIED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DAX_ALLOC_ALIGNMENT_FLAG_MANDATORY: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DD_MUP_DEVICE_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("\\Device\\Mup");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DEVICE_RESET_KEEP_STACK: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DEVICE_RESET_RESERVED_0: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DEVICE_RESET_RESERVED_1: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_ACCESS_CONTROL_ASSISTANCE_OPS: i32 = 579i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_ACCOUNT_OPS: i32 = 548i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_ADMINS: i32 = 544i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_AUTHORIZATIONACCESS: i32 = 560i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_BACKUP_OPS: i32 = 551i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_CACHEABLE_PRINCIPALS_GROUP: i32 = 571i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_CERTSVC_DCOM_ACCESS_GROUP: i32 = 574i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_CRYPTO_OPERATORS: i32 = 569i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_DCOM_USERS: i32 = 562i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_DEFAULT_ACCOUNT: i32 = 581i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_DEVICE_OWNERS: i32 = 583i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_EVENT_LOG_READERS_GROUP: i32 = 573i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_GUESTS: i32 = 546i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_HYPER_V_ADMINS: i32 = 578i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_INCOMING_FOREST_TRUST_BUILDERS: i32 = 557i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_IUSERS: i32 = 568i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_LOGGING_USERS: i32 = 559i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_MONITORING_USERS: i32 = 558i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_NETWORK_CONFIGURATION_OPS: i32 = 556i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_NON_CACHEABLE_PRINCIPALS_GROUP: i32 = 572i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_POWER_USERS: i32 = 547i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_PREW2KCOMPACCESS: i32 = 554i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_PRINT_OPS: i32 = 550i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_RAS_SERVERS: i32 = 553i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_RDS_ENDPOINT_SERVERS: i32 = 576i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_RDS_MANAGEMENT_SERVERS: i32 = 577i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_RDS_REMOTE_ACCESS_SERVERS: i32 = 575i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_REMOTE_DESKTOP_USERS: i32 = 555i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_REMOTE_MANAGEMENT_USERS: i32 = 580i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_REPLICATOR: i32 = 552i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_STORAGE_REPLICA_ADMINS: i32 = 582i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_SYSTEM_OPS: i32 = 549i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_TS_LICENSE_SERVERS: i32 = 561i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_ALIAS_RID_USERS: i32 = 545i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_ADMINS: i32 = 512i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_AUTHORIZATION_DATA_CONTAINS_CLAIMS: i32 = 497i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_AUTHORIZATION_DATA_IS_COMPOUNDED: i32 = 496i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_CDC_RESERVED: i32 = 524i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_CERT_ADMINS: i32 = 517i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_CLONEABLE_CONTROLLERS: i32 = 522i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_COMPUTERS: i32 = 515i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_CONTROLLERS: i32 = 516i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_ENTERPRISE_ADMINS: i32 = 519i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_ENTERPRISE_KEY_ADMINS: i32 = 527i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_ENTERPRISE_READONLY_DOMAIN_CONTROLLERS: i32 = 498i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_GUESTS: i32 = 514i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_KEY_ADMINS: i32 = 526i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_POLICY_ADMINS: i32 = 520i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_PROTECTED_USERS: i32 = 525i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_READONLY_CONTROLLERS: i32 = 521i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_SCHEMA_ADMINS: i32 = 518i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_GROUP_RID_USERS: i32 = 513i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_USER_RID_ADMIN: i32 = 500i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_USER_RID_DEFAULT_ACCOUNT: i32 = 503i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_USER_RID_GUEST: i32 = 501i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_USER_RID_KRBTGT: i32 = 502i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_USER_RID_MAX: i32 = 999i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DOMAIN_USER_RID_WDAG_ACCOUNT: i32 = 504i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_BOOT_CRITICAL: u32 = 536870912u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_BUFFERED_IO: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_BUS_ENUMERATED_DEVICE: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_DAX_VOLUME: u32 = 268435456u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_DEVICE_HAS_NAME: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_DEVICE_INITIALIZING: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_DEVICE_IRP_REQUIRES_EXTENSION: u32 = 134217728u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_DEVICE_TO_BE_RESET: u32 = 67108864u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_DIRECT_IO: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_DISALLOW_EXECUTE: u32 = 8388608u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_EXCLUSIVE: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_FORCE_NEITHER_IO: u32 = 524288u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_LONG_TERM_REQUESTS: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_LOW_PRIORITY_FILESYSTEM: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_MAP_IO_BUFFER: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_NEVER_LAST_DEVICE: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_NOT_PURGE_DIRTY_PAGES: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_NOT_RETRY_PURGE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_POWER_INRUSH: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_POWER_PAGABLE: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_SHUTDOWN_REGISTERED: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_SUPPORTS_PERSISTENT_ACLS: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_SUPPORTS_TRANSACTIONS: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_SYSTEM_BOOT_PARTITION: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_SYSTEM_CRITICAL_PARTITION: u32 = 4194304u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_SYSTEM_SYSTEM_PARTITION: u32 = 2097152u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_VERIFY_VOLUME: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DO_VOLUME_DEVICE_OBJECT: u32 = 1048576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DUPLICATE_EXTENTS_DATA_EX_ASYNC: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DUPLICATE_EXTENTS_DATA_EX_SOURCE_ATOMIC: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const EA_NAME_NETWORK_OPEN_ECP_INTEGRITY: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-INTEGRITY");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const EA_NAME_NETWORK_OPEN_ECP_INTEGRITY_U: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-INTEGRITY");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const EA_NAME_NETWORK_OPEN_ECP_PRIVACY: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-PRIVACY");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const EA_NAME_NETWORK_OPEN_ECP_PRIVACY_U: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-PRIVACY");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ECP_OPEN_PARAMETERS_FLAG_FAIL_ON_CASE_SENSITIVE_DIR: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ECP_OPEN_PARAMETERS_FLAG_IGNORE_DIR_CASE_SENSITIVITY: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ECP_OPEN_PARAMETERS_FLAG_OPEN_FOR_DELETE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ECP_OPEN_PARAMETERS_FLAG_OPEN_FOR_READ: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ECP_OPEN_PARAMETERS_FLAG_OPEN_FOR_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ECP_TYPE_CLFS_CREATE_CONTAINER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8650c9fe_0cec_8bf6_bd1e_835956541090);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ECP_TYPE_IO_STOP_ON_SYMLINK_FILTER_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x940e5d56_1646_4d3c_87b6_577ec36a1466);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ECP_TYPE_OPEN_REPARSE_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x323eb6a8_affd_4d95_8230_863bce09d37a);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const EFS_TRACKED_OFFSET_HEADER_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ENCRYPTED_DATA_INFO_SPARSE_FILE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ENCRYPTION_FORMAT_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const EVENT_INCREMENT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FAILED_ACCESS_ACE_FLAG: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILESYSTEM_STATISTICS_TYPE_EXFAT: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILESYSTEM_STATISTICS_TYPE_FAT: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILESYSTEM_STATISTICS_TYPE_NTFS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILESYSTEM_STATISTICS_TYPE_REFS: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ACTION_ADDED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ACTION_ADDED_STREAM: u32 = 6u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ACTION_ID_NOT_TUNNELLED: u32 = 10u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ACTION_MODIFIED: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ACTION_MODIFIED_STREAM: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ACTION_REMOVED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ACTION_REMOVED_BY_DELETE: u32 = 9u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ACTION_REMOVED_STREAM: u32 = 7u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ACTION_RENAMED_NEW_NAME: u32 = 5u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ACTION_RENAMED_OLD_NAME: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ACTION_TUNNELLED_ID_COLLISION: u32 = 11u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ANY_ACCESS: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_CASE_PRESERVED_NAMES: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_CASE_SENSITIVE_SEARCH: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_CLEANUP_FILE_DELETED: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_CLEANUP_FILE_REMAINS: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_CLEANUP_LINK_DELETED: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_CLEANUP_POSIX_STYLE_DELETE: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_CLEANUP_STREAM_DELETED: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_CLEANUP_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_CLEANUP_WRONG_DEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_CLEAR_ENCRYPTION: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_CS_FLAG_CASE_SENSITIVE_DIR: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DAX_VOLUME: u32 = 536870912u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_8042_PORT: u32 = 39u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_ACPI: u32 = 50u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_BATTERY: u32 = 41u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_BEEP: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_BIOMETRIC: u32 = 68u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_BLUETOOTH: u32 = 65u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_BUS_EXTENDER: u32 = 42u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_CD_ROM: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_CD_ROM_FILE_SYSTEM: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_CHANGER: u32 = 48u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_CONSOLE: u32 = 80u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_CONTROLLER: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_CRYPT_PROVIDER: u32 = 63u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_DATALINK: u32 = 5u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_DEVAPI: u32 = 71u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_DFS: u32 = 6u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_DFS_FILE_SYSTEM: u32 = 53u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_DFS_VOLUME: u32 = 54u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_DISK: u32 = 7u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_DISK_FILE_SYSTEM: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_DVD: u32 = 51u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_EHSTOR: u32 = 70u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_EVENT_COLLECTOR: u32 = 95u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_FILE_SYSTEM: u32 = 9u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_FIPS: u32 = 58u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_FULLSCREEN_VIDEO: u32 = 52u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_GPIO: u32 = 72u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_HOLOGRAPHIC: u32 = 91u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_INFINIBAND: u32 = 59u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_INPORT_PORT: u32 = 10u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_KEYBOARD: u32 = 11u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_KS: u32 = 47u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_KSEC: u32 = 57u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_MAILSLOT: u32 = 12u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_MASS_STORAGE: u32 = 45u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_MIDI_IN: u32 = 13u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_MIDI_OUT: u32 = 14u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_MODEM: u32 = 43u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_MOUSE: u32 = 15u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_MT_COMPOSITE: u32 = 66u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_MT_TRANSPORT: u32 = 67u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_MULTI_UNC_PROVIDER: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_NAMED_PIPE: u32 = 17u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_NETWORK: u32 = 18u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_NETWORK_BROWSER: u32 = 19u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_NETWORK_FILE_SYSTEM: u32 = 20u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_NETWORK_REDIRECTOR: u32 = 40u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_NFP: u32 = 81u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_NULL: u32 = 21u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_NVDIMM: u32 = 90u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_PARALLEL_PORT: u32 = 22u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_PERSISTENT_MEMORY: u32 = 89u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_PHYSICAL_NETCARD: u32 = 23u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_PMI: u32 = 69u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_POINT_OF_SERVICE: u32 = 84u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_PRINTER: u32 = 24u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_PRM: u32 = 94u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_SCANNER: u32 = 25u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_SCREEN: u32 = 28u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_SDFXHCI: u32 = 92u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_SERENUM: u32 = 55u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_SERIAL_MOUSE_PORT: u32 = 26u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_SERIAL_PORT: u32 = 27u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_SMARTCARD: u32 = 49u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_SMB: u32 = 46u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_SOUND: u32 = 29u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_SOUNDWIRE: u32 = 97u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_STORAGE_REPLICATION: u32 = 85u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_STREAMS: u32 = 30u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_SYSENV: u32 = 82u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_TAPE: u32 = 31u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_TAPE_FILE_SYSTEM: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_TERMSRV: u32 = 56u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_TRANSPORT: u32 = 33u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_TRUST_ENV: u32 = 86u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_UCM: u32 = 87u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_UCMTCPCI: u32 = 88u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_UCMUCSI: u32 = 93u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_UNKNOWN: u32 = 34u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_USB4: u32 = 96u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_USBEX: u32 = 73u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_VDM: u32 = 44u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_VIDEO: u32 = 35u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_VIRTUAL_BLOCK: u32 = 83u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_VIRTUAL_DISK: u32 = 36u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_VMBUS: u32 = 62u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_WAVE_IN: u32 = 37u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_WAVE_OUT: u32 = 38u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_WPD: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_EA_TYPE_ASCII: u32 = 65533u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_EA_TYPE_ASN1: u32 = 65501u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_EA_TYPE_BINARY: u32 = 65534u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_EA_TYPE_BITMAP: u32 = 65531u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_EA_TYPE_EA: u32 = 65518u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_EA_TYPE_FAMILY_IDS: u32 = 65281u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_EA_TYPE_ICON: u32 = 65529u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_EA_TYPE_METAFILE: u32 = 65530u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_EA_TYPE_MVMT: u32 = 65503u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_EA_TYPE_MVST: u32 = 65502u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_FILE_COMPRESSION: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_VISIBLE_OUTSIDE_TX: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_VISIBLE_TO_TX: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_WRITELOCKED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_NON_RESIDENT: u64 = 137438953472u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_NOT_FOUND: u64 = 4096u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_TOO_SMALL: u64 = 68719476736u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_CLUSTERS_ALREADY_IN_USE: u64 = 32768u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_DENY_DEFRAG: u64 = 274877906944u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_IS_BASE_RECORD: u64 = 524288u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_BASE_RECORD: u64 = 8u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_EXIST: u64 = 4u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_IN_USE: u64 = 1u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_ORPHAN: u64 = 262144u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_REUSED: u64 = 2u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INDEX_ENTRY_MISMATCH: u64 = 1099511627776u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_ARRAY_LENGTH_COUNT: u64 = 1048576u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_LCN: u64 = 4294967296u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_ORPHAN_RECOVERY_NAME: u64 = 2199023255552u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_PARENT: u64 = 8388608u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_RUN_LENGTH: u64 = 131072u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_VCN: u64 = 8589934592u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_LCN_NOT_EXIST: u64 = 65536u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_MULTIPLE_FILE_NAME_ATTRIBUTES: u64 = 4398046511104u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_NAME_CONFLICT: u64 = 17179869184u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_NOTHING_WRONG: u64 = 2048u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_NOT_IMPLEMENTED: u64 = 32u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ORPHAN: u64 = 34359738368u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ORPHAN_GENERATED: u64 = 512u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_OUT_OF_GENERIC_NAMES: u64 = 1073741824u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_OUT_OF_RESOURCE: u64 = 2147483648u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_BASE_RECORD: u64 = 134217728u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_EXIST: u64 = 67108864u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_INDEX: u64 = 268435456u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_IN_USE: u64 = 16777216u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_REUSED: u64 = 33554432u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_POTENTIAL_CROSSLINK: u64 = 8192u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PREVIOUS_PARENT_STILL_VALID: u64 = 549755813888u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_RECURSIVELY_CORRUPTED: u64 = 256u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_REPAIRED: u64 = 1024u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_REPAIR_DISABLED: u64 = 128u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_SID_MISMATCH: u64 = 4194304u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_SID_VALID: u64 = 2097152u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_STALE_INFORMATION: u64 = 16384u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_SYSTEM_FILE: u64 = 16u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_UNABLE_TO_REPAIR: u64 = 64u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_VALID_INDEX_ENTRY: u64 = 536870912u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_LAYOUT_NAME_ENTRY_DOS: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_LAYOUT_NAME_ENTRY_PRIMARY: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_LINK_FORCE_RESIZE_SOURCE_SR: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_LINK_FORCE_RESIZE_SR: u32 = 384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_LINK_FORCE_RESIZE_TARGET_SR: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_LINK_IGNORE_READONLY_ATTRIBUTE: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_LINK_NO_DECREASE_AVAILABLE_SPACE: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_LINK_NO_INCREASE_AVAILABLE_SPACE: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_LINK_POSIX_SEMANTICS: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_LINK_PRESERVE_AVAILABLE_SPACE: u32 = 48u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_LINK_REPLACE_IF_EXISTS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_LINK_SUPPRESS_STORAGE_RESERVE_INHERITANCE: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NAMED_STREAMS: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NAME_FLAGS_UNSPECIFIED: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NAME_FLAG_BOTH: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NAME_FLAG_DOS: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NAME_FLAG_HARDLINK: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NAME_FLAG_NTFS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NEED_EA: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_ATTRIBUTES: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_CREATION: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_DIR_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_EA: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_FILE_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_LAST_ACCESS: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_LAST_WRITE: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_NAME: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_SECURITY: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_STREAM_NAME: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_STREAM_SIZE: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_STREAM_WRITE: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_VALID_MASK: u32 = 4095u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_OPBATCH_BREAK_UNDERWAY: u32 = 9u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_OPLOCK_BROKEN_TO_LEVEL_2: u32 = 7u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_OPLOCK_BROKEN_TO_NONE: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PERSISTENT_ACLS: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_ACCEPT_REMOTE_CLIENTS: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_BYTE_STREAM_MODE: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_BYTE_STREAM_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_CLIENT_END: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_CLOSING_STATE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_COMPLETE_OPERATION: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_COMPUTER_NAME_LENGTH: u32 = 15u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_CONNECTED_STATE: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_DISCONNECTED_STATE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_FULL_DUPLEX: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_INBOUND: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_LISTENING_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_MESSAGE_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_MESSAGE_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_OUTBOUND: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_QUEUE_OPERATION: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_READ_DATA: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_REJECT_REMOTE_CLIENTS: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_SERVER_END: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_SYMLINK_FLAG_GLOBAL: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_SYMLINK_FLAG_RELATIVE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_TYPE_VALID_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PIPE_WRITE_SPACE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PREFETCH_TYPE_FOR_CREATE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PREFETCH_TYPE_FOR_CREATE_EX: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PREFETCH_TYPE_FOR_DIRENUM: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PREFETCH_TYPE_FOR_DIRENUM_EX: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PREFETCH_TYPE_MAX: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_COMPRESSION_LZX: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_COMPRESSION_MAXIMUM: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_COMPRESSION_XPRESS16K: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_COMPRESSION_XPRESS4K: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_COMPRESSION_XPRESS8K: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_FLAG_COMPRESS_ON_WRITE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_SINGLE_FILE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_READ_ACCESS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_READ_ONLY_VOLUME: u32 = 524288u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_REGION_USAGE_HUGE_PAGE_ALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_REGION_USAGE_LARGE_PAGE_ALIGNMENT: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_REGION_USAGE_OTHER_PAGE_ALIGNMENT: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_REGION_USAGE_QUERY_ALIGNMENT: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_REGION_USAGE_VALID_CACHED_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_REGION_USAGE_VALID_NONCACHED_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_RENAME_FORCE_RESIZE_SOURCE_SR: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_RENAME_FORCE_RESIZE_SR: u32 = 384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_RENAME_FORCE_RESIZE_TARGET_SR: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_RENAME_IGNORE_READONLY_ATTRIBUTE: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_RENAME_NO_DECREASE_AVAILABLE_SPACE: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_RENAME_NO_INCREASE_AVAILABLE_SPACE: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_RENAME_POSIX_SEMANTICS: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_RENAME_PRESERVE_AVAILABLE_SPACE: u32 = 48u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_RENAME_REPLACE_IF_EXISTS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_RENAME_SUPPRESS_PIN_STATE_INHERITANCE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_RENAME_SUPPRESS_STORAGE_RESERVE_INHERITANCE: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_RETURNS_CLEANUP_RESULT_INFO: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SEQUENTIAL_WRITE_ONCE: u32 = 1048576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SET_ENCRYPTION: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SPECIAL_ACCESS: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_STORAGE_TIER_DESCRIPTION_LENGTH: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_STORAGE_TIER_FLAG_NO_SEEK_PENALTY: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_STORAGE_TIER_FLAG_PARITY: u32 = 8388608u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_STORAGE_TIER_FLAG_READ_CACHE: u32 = 4194304u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_STORAGE_TIER_FLAG_SMR: u32 = 16777216u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_STORAGE_TIER_FLAG_WRITE_BACK_CACHE: u32 = 2097152u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_STORAGE_TIER_NAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_BLOCK_REFCOUNTING: u32 = 134217728u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_BYPASS_IO: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_CASE_SENSITIVE_DIRS: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_ENCRYPTION: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_EXTENDED_ATTRIBUTES: u32 = 8388608u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_GHOSTING: u32 = 1073741824u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_HARD_LINKS: u32 = 4194304u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_INTEGRITY_STREAMS: u32 = 67108864u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_OBJECT_IDS: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_OPEN_BY_FILE_ID: u32 = 16777216u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_POSIX_UNLINK_RENAME: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_REMOTE_STORAGE: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_REPARSE_POINTS: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_SPARSE_FILES: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_SPARSE_VDL: u32 = 268435456u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_STREAM_SNAPSHOTS: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_TRANSACTIONS: u32 = 2097152u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPPORTS_USN_JOURNAL: u32 = 33554432u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_TYPE_NOTIFICATION_FLAG_USAGE_BEGIN: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_TYPE_NOTIFICATION_FLAG_USAGE_END: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_TYPE_NOTIFICATION_GUID_CRASHDUMP_FILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d453eb7_d2a6_4dbd_a2e3_fbd0ed9109a9);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_TYPE_NOTIFICATION_GUID_HIBERNATION_FILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb7624d64_b9a3_4cf8_8011_5b86c940e7b7);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0d0a64a1_38fc_4db8_9fe7_3f4352cd7c5c);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_UNICODE_ON_DISK: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_VC_CONTENT_INDEX_DISABLED: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_VC_LOG_QUOTA_LIMIT: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_VC_LOG_QUOTA_THRESHOLD: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_VC_LOG_VOLUME_LIMIT: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_VC_LOG_VOLUME_THRESHOLD: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_VC_QUOTAS_INCOMPLETE: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_VC_QUOTAS_REBUILDING: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_VC_QUOTA_ENFORCE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_VC_QUOTA_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_VC_QUOTA_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_VC_QUOTA_TRACK: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_VC_VALID_MASK: u32 = 1023u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_VOLUME_IS_COMPRESSED: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_VOLUME_QUOTAS: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_WRITE_ACCESS: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_ZERO_DATA_INFORMATION_FLAG_PRESERVE_CACHED_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FLAGS_DELAY_REASONS_BITMAP_SCANNED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FLAGS_DELAY_REASONS_LOG_FILE_FULL: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FLAGS_END_OF_FILE_INFO_EX_EXTEND_PAGING: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FLAGS_END_OF_FILE_INFO_EX_NO_EXTRA_PAGING_EXTEND: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FLAGS_END_OF_FILE_INFO_EX_TIME_CONSTRAINED: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FLAG_USN_TRACK_MODIFIED_RANGES_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FLUSH_FLAGS_FILE_DATA_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FLUSH_FLAGS_FILE_DATA_SYNC_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FLUSH_FLAGS_NO_SYNC: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FOREST_USER_RID_MAX: i32 = 499i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_ADD_OVERLAY: u32 = 623408u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_ADVANCE_FILE_ID: u32 = 590532u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_ALLOW_EXTENDED_DASD_IO: u32 = 589955u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CLEAN_VOLUME_METADATA: u32 = 590716u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CORRUPTION_HANDLING: u32 = 590432u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CREATE_LCN_WEAK_REFERENCE: u32 = 590944u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CREATE_OR_GET_OBJECT_ID: u32 = 590016u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CREATE_USN_JOURNAL: u32 = 590055u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CSC_INTERNAL: u32 = 590255u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CSV_CONTROL: u32 = 590548u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CSV_GET_VOLUME_NAME_FOR_VOLUME_MOUNT_POINT: u32 = 590420u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CSV_GET_VOLUME_PATH_NAME: u32 = 590416u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CSV_GET_VOLUME_PATH_NAMES_FOR_VOLUME_NAME: u32 = 590424u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CSV_H_BREAKING_SYNC_TUNNEL_REQUEST: u32 = 590564u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CSV_INTERNAL: u32 = 590444u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CSV_MGMT_LOCK: u32 = 590524u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CSV_QUERY_DOWN_LEVEL_FILE_SYSTEM_CHARACTERISTICS: u32 = 590528u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CSV_QUERY_VETO_FILE_DIRECT_IO: u32 = 590540u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CSV_SYNC_TUNNEL_REQUEST: u32 = 590536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_CSV_TUNNEL_REQUEST: u32 = 590404u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_DELETE_CORRUPTED_REFS_CONTAINER: u32 = 590836u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_DELETE_EXTERNAL_BACKING: u32 = 590612u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_DELETE_LCN_WEAK_REFERENCE: u32 = 590948u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_DELETE_LCN_WEAK_REFERENCES: u32 = 590956u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_DELETE_OBJECT_ID: u32 = 589984u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_DELETE_REPARSE_POINT: u32 = 589996u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_DELETE_USN_JOURNAL: u32 = 590072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_DFSR_SET_GHOST_HANDLE_STATE: u32 = 590264u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_DISABLE_LOCAL_BUFFERING: u32 = 590520u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_DISMOUNT_VOLUME: u32 = 589856u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_DUPLICATE_CLUSTER: u32 = 590940u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_DUPLICATE_EXTENTS_TO_FILE: u32 = 623428u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_DUPLICATE_EXTENTS_TO_FILE_EX: u32 = 623592u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_ENABLE_PER_IO_FLAGS: u32 = 590892u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_ENABLE_UPGRADE: u32 = 622800u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_ENCRYPTION_FSCTL_IO: u32 = 590043u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_ENCRYPTION_KEY_CONTROL: u32 = 590852u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_ENUM_EXTERNAL_BACKING: u32 = 590616u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_ENUM_OVERLAY: u32 = 590623u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_ENUM_USN_DATA: u32 = 590003u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_EXTEND_VOLUME: u32 = 590064u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_FILESYSTEM_GET_STATISTICS: u32 = 589920u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_FILESYSTEM_GET_STATISTICS_EX: u32 = 590732u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_FILE_LEVEL_TRIM: u32 = 623112u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_FILE_PREFETCH: u32 = 590112u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_FILE_TYPE_NOTIFICATION: u32 = 590340u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_FIND_FILES_BY_SID: u32 = 589967u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_BOOT_AREA_INFO: u32 = 590384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_COMPRESSION: u32 = 589884u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_EXTERNAL_BACKING: u32 = 590608u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_FILTER_FILE_IDENTIFIER: u32 = 590788u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_INTEGRITY_INFORMATION: u32 = 590460u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_NTFS_FILE_RECORD: u32 = 589928u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_NTFS_VOLUME_DATA: u32 = 589924u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_OBJECT_ID: u32 = 589980u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_REFS_VOLUME_DATA: u32 = 590552u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_REPAIR: u32 = 590236u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_REPARSE_POINT: u32 = 589992u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_RETRIEVAL_POINTERS: u32 = 589939u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_RETRIEVAL_POINTERS_AND_REFCOUNT: u32 = 590803u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_RETRIEVAL_POINTER_BASE: u32 = 590388u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_RETRIEVAL_POINTER_COUNT: u32 = 590891u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_VOLUME_BITMAP: u32 = 589935u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GET_WOF_VERSION: u32 = 590696u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_GHOST_FILE_EXTENTS: u32 = 623532u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_HCS_ASYNC_TUNNEL_REQUEST: u32 = 590704u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_HCS_SYNC_NO_WRITE_TUNNEL_REQUEST: u32 = 590776u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_HCS_SYNC_TUNNEL_REQUEST: u32 = 590700u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_INITIATE_FILE_METADATA_OPTIMIZATION: u32 = 590684u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_INITIATE_REPAIR: u32 = 590248u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_INTEGRITY_FLAG_CHECKSUM_ENFORCEMENT_OFF: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_INVALIDATE_VOLUMES: u32 = 589908u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_IS_CSV_FILE: u32 = 590408u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_IS_FILE_ON_CSV_VOLUME: u32 = 590428u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_IS_PATHNAME_VALID: u32 = 589868u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_IS_VOLUME_DIRTY: u32 = 589944u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_IS_VOLUME_MOUNTED: u32 = 589864u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_IS_VOLUME_OWNED_BYCSVFS: u32 = 590456u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_LMR_GET_LINK_TRACKING_INFORMATION: u32 = 1310952u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_LMR_QUERY_INFO: u32 = 590968u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_LMR_SET_LINK_TRACKING_INFORMATION: u32 = 1310956u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_LOCK_VOLUME: u32 = 589848u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_LOOKUP_STREAM_FROM_CLUSTER: u32 = 590332u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_MAILSLOT_PEEK: u32 = 802819u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_MAKE_MEDIA_COMPATIBLE: u32 = 622896u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_MANAGE_BYPASS_IO: u32 = 590920u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_MARK_AS_SYSTEM_HIVE: u32 = 589903u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_MARK_HANDLE: u32 = 590076u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_MARK_VOLUME_DIRTY: u32 = 589872u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_MOVE_FILE: u32 = 589940u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_NOTIFY_DATA_CHANGE: u32 = 590844u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_NOTIFY_STORAGE_SPACE_ALLOCATION: u32 = 590748u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_OFFLOAD_READ: u32 = 606820u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_OFFLOAD_WRITE: u32 = 623208u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_OPBATCH_ACK_CLOSE_PENDING: u32 = 589840u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_OPLOCK_BREAK_ACKNOWLEDGE: u32 = 589836u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_OPLOCK_BREAK_ACK_NO_2: u32 = 589904u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_OPLOCK_BREAK_NOTIFY: u32 = 589844u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_ASSIGN_EVENT: u32 = 1114112u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_CREATE_SYMLINK: u32 = 1114188u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_DELETE_SYMLINK: u32 = 1114192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_DISABLE_IMPERSONATE: u32 = 1114180u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_DISCONNECT: u32 = 1114116u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_FLUSH: u32 = 1146944u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_GET_CONNECTION_ATTRIBUTE: u32 = 1114160u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_GET_HANDLE_ATTRIBUTE: u32 = 1114168u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_GET_PIPE_ATTRIBUTE: u32 = 1114152u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_IMPERSONATE: u32 = 1114140u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_INTERNAL_READ: u32 = 1138676u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_INTERNAL_READ_OVFLOW: u32 = 1138688u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_INTERNAL_TRANSCEIVE: u32 = 1171455u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_INTERNAL_WRITE: u32 = 1155064u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_LISTEN: u32 = 1114120u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_PEEK: u32 = 1130508u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_QUERY_CLIENT_PROCESS: u32 = 1114148u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_QUERY_CLIENT_PROCESS_V2: u32 = 1114196u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_QUERY_EVENT: u32 = 1114128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_SET_CLIENT_PROCESS: u32 = 1114144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_SET_CONNECTION_ATTRIBUTE: u32 = 1114164u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_SET_HANDLE_ATTRIBUTE: u32 = 1114172u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_SET_PIPE_ATTRIBUTE: u32 = 1114156u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_SILO_ARRIVAL: u32 = 1146952u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_TRANSCEIVE: u32 = 1163287u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_PIPE_WAIT: u32 = 1114136u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_ALLOCATED_RANGES: u32 = 606415u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_ASYNC_DUPLICATE_EXTENTS_STATUS: u32 = 590896u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_BAD_RANGES: u32 = 590828u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_DEPENDENT_VOLUME: u32 = 590320u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_DIRECT_ACCESS_EXTENTS: u32 = 590747u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_DIRECT_IMAGE_ORIGINAL_BASE: u32 = 590756u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_EXTENT_READ_CACHE_INFO: u32 = 590711u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_FAT_BPB: u32 = 589912u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_FILE_LAYOUT: u32 = 590455u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_FILE_METADATA_OPTIMIZATION: u32 = 590688u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_FILE_REGIONS: u32 = 590468u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_FILE_SYSTEM_RECOGNITION: u32 = 590412u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_GHOSTED_FILE_EXTENTS: u32 = 590768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_LCN_WEAK_REFERENCE: u32 = 590952u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_ON_DISK_VOLUME_INFO: u32 = 590140u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_PAGEFILE_ENCRYPTION: u32 = 590312u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_PERSISTENT_VOLUME_STATE: u32 = 590396u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_REFS_SMR_VOLUME_INFO: u32 = 590812u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_REFS_VOLUME_COUNTER_INFO: u32 = 590715u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_REGION_INFO: u32 = 590576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_RETRIEVAL_POINTERS: u32 = 589883u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_SHARED_VIRTUAL_DISK_SUPPORT: u32 = 590592u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_SPARING_INFO: u32 = 590136u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_STORAGE_CLASSES: u32 = 590572u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_USN_JOURNAL: u32 = 590068u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_VOLUME_CONTAINER_STATE: u32 = 590736u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_QUERY_VOLUME_NUMA_INFO: u32 = 590804u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_READ_FILE_USN_DATA: u32 = 590059u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_READ_FROM_PLEX: u32 = 606494u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_READ_RAW_ENCRYPTED: u32 = 590051u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_READ_UNPRIVILEGED_USN_JOURNAL: u32 = 590763u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_READ_USN_JOURNAL: u32 = 590011u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REARRANGE_FILE: u32 = 640032u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_RECALL_FILE: u32 = 590103u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REFS_DEALLOCATE_RANGES: u32 = 590808u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REFS_DEALLOCATE_RANGES_EX: u32 = 590924u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REFS_QUERY_VOLUME_COMPRESSION_INFO: u32 = 590936u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REFS_QUERY_VOLUME_DEDUP_INFO: u32 = 590964u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REFS_SET_VOLUME_COMPRESSION_INFO: u32 = 590932u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REFS_SET_VOLUME_DEDUP_INFO: u32 = 590960u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REFS_STREAM_SNAPSHOT_MANAGEMENT: u32 = 590912u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REMOVE_OVERLAY: u32 = 623412u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REPAIR_COPIES: u32 = 639668u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REQUEST_BATCH_OPLOCK: u32 = 589832u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REQUEST_FILTER_OPLOCK: u32 = 589916u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REQUEST_OPLOCK: u32 = 590400u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REQUEST_OPLOCK_LEVEL_1: u32 = 589824u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_REQUEST_OPLOCK_LEVEL_2: u32 = 589828u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_RESET_VOLUME_ALLOCATION_HINTS: u32 = 590316u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_RKF_INTERNAL: u32 = 590511u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SCRUB_DATA: u32 = 590512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SCRUB_UNDISCOVERABLE_ID: u32 = 590840u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SD_GLOBAL_CHANGE: u32 = 590324u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SECURITY_ID_CHECK: u32 = 606391u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_BOOTLOADER_ACCESSED: u32 = 589903u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_CACHED_RUNS_STATE: u32 = 590928u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_COMPRESSION: u32 = 639040u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_DAX_ALLOC_ALIGNMENT_HINT: u32 = 590832u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_DEFECT_MANAGEMENT: u32 = 622900u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_ENCRYPTION: u32 = 590039u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_EXTERNAL_BACKING: u32 = 590604u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_INTEGRITY_INFORMATION: u32 = 639616u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_INTEGRITY_INFORMATION_EX: u32 = 590720u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_LAYER_ROOT: u32 = 590740u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_OBJECT_ID: u32 = 589976u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_OBJECT_ID_EXTENDED: u32 = 590012u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_PERSISTENT_VOLUME_STATE: u32 = 590392u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_PURGE_FAILURE_MODE: u32 = 590448u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_REFS_FILE_STRICTLY_SEQUENTIAL: u32 = 590820u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_REFS_SMR_VOLUME_GC_PARAMETERS: u32 = 590816u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_REPAIR: u32 = 590232u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_REPARSE_POINT: u32 = 589988u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_REPARSE_POINT_EX: u32 = 590860u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_SHORT_NAME_BEHAVIOR: u32 = 590260u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_SPARSE: u32 = 590020u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_VOLUME_COMPRESSION_STATE: u32 = 590144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_ZERO_DATA: u32 = 622792u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SET_ZERO_ON_DEALLOCATION: u32 = 590228u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SHRINK_VOLUME: u32 = 590256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SHUFFLE_FILE: u32 = 639808u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SIS_COPYFILE: u32 = 590080u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SIS_LINK_FILES: u32 = 639236u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SMB_SHARE_FLUSH_AND_PURGE: u32 = 590908u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SPARSE_OVERALLOCATE: u32 = 590668u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SSDI_STORAGE_REQUEST: u32 = 590752u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_START_VIRTUALIZATION_INSTANCE: u32 = 590784u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_START_VIRTUALIZATION_INSTANCE_EX: u32 = 590848u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_STORAGE_QOS_CONTROL: u32 = 590672u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_STREAMS_ASSOCIATE_ID: u32 = 590792u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_STREAMS_QUERY_ID: u32 = 590796u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_STREAMS_QUERY_PARAMETERS: u32 = 590788u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SUSPEND_OVERLAY: u32 = 590724u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SVHDX_ASYNC_TUNNEL_REQUEST: u32 = 590692u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SVHDX_SET_INITIATOR_INFORMATION: u32 = 590600u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_SVHDX_SYNC_TUNNEL_REQUEST: u32 = 590596u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_CREATE_MINIVERSION: u32 = 622972u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_CREATE_SECONDARY_RM: u32 = 622952u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_GET_METADATA_INFO: u32 = 606572u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_GET_TRANSACTED_VERSION: u32 = 606576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_LIST_TRANSACTIONS: u32 = 606692u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_LIST_TRANSACTION_LOCKED_FILES: u32 = 606688u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_MODIFY_RM: u32 = 622916u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_QUERY_RM_INFORMATION: u32 = 606536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_READ_BACKUP_INFORMATION: u32 = 606560u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_READ_BACKUP_INFORMATION2: u32 = 590328u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_ROLLFORWARD_REDO: u32 = 622928u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_ROLLFORWARD_UNDO: u32 = 622932u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_SAVEPOINT_INFORMATION: u32 = 622968u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_SHUTDOWN_RM: u32 = 622940u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_START_RM: u32 = 622936u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_TRANSACTION_ACTIVE: u32 = 606604u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_WRITE_BACKUP_INFORMATION: u32 = 622948u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_TXFS_WRITE_BACKUP_INFORMATION2: u32 = 590336u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_UNLOCK_VOLUME: u32 = 589852u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_UNMAP_SPACE: u32 = 590772u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_UPDATE_OVERLAY: u32 = 623416u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_USN_TRACK_MODIFIED_RANGES: u32 = 590580u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_VIRTUAL_STORAGE_PASSTHROUGH: u32 = 590884u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_VIRTUAL_STORAGE_QUERY_PROPERTY: u32 = 590728u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_VIRTUAL_STORAGE_SET_BEHAVIOR: u32 = 590856u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_WAIT_FOR_REPAIR: u32 = 590240u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_WRITE_RAW_ENCRYPTED: u32 = 590047u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_WRITE_USN_CLOSE_RECORD: u32 = 590063u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSCTL_WRITE_USN_REASON: u32 = 590544u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_ADD_TC_CASE_SENSITIVE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_ADD_TC_KEY_BY_SHORT_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_ALLOCATE_ECPLIST_FLAG_CHARGE_QUOTA: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_ALLOCATE_ECP_FLAG_CHARGE_QUOTA: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_ALLOCATE_ECP_FLAG_NONPAGED_POOL: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_AUXILIARY_FLAG_DEALLOCATE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_CC_FLUSH_ERROR_FLAG_NO_HARD_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_CC_FLUSH_ERROR_FLAG_NO_LOG_ENTRY: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_DRIVER_BACKING_FLAG_USE_PAGE_FILE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_ECP_LOOKASIDE_FLAG_NONPAGED_POOL: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FAT_LEGAL: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FCB_HEADER_V0: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FCB_HEADER_V1: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FCB_HEADER_V2: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FCB_HEADER_V3: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FCB_HEADER_V4: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FIND_TC_CASE_SENSITIVE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FLAG2_BYPASSIO_STREAM_PAUSED: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FLAG2_DO_MODIFIED_WRITE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FLAG2_IS_PAGING_FILE: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FLAG2_PURGE_WHEN_MAPPED: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FLAG2_SUPPORTS_FILTER_CONTEXTS: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FLAG2_WRITABLE_USER_MAPPED_FILE: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FLAG_ACQUIRE_MAIN_RSRC_EX: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FLAG_ACQUIRE_MAIN_RSRC_SH: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FLAG_ADVANCED_HEADER: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FLAG_EOF_ADVANCE_ACTIVE: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FLAG_FILE_LENGTH_CHANGED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FLAG_FILE_MODIFIED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FLAG_LIMIT_MODIFIED_PAGES: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_FLAG_USER_MAPPED_FILE: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_HPFS_LEGAL: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_NTFS_LEGAL: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_OLE_LEGAL: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_UNC_HARDENING_CAPABILITIES_INTEGRITY: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_UNC_HARDENING_CAPABILITIES_MUTUAL_AUTH: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_UNC_HARDENING_CAPABILITIES_PRIVACY: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_UNC_PROVIDER_FLAGS_CONTAINER_AWARE: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_UNC_PROVIDER_FLAGS_CSC_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_UNC_PROVIDER_FLAGS_DOMAIN_SVC_AWARE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_UNC_PROVIDER_FLAGS_MAILSLOTS_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_UNC_REGISTRATION_CURRENT_VERSION: u32 = 513u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_UNC_REGISTRATION_VERSION_0200: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_UNC_REGISTRATION_VERSION_0201: u32 = 513u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VIRTDISK_FULLY_ALLOCATED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VIRTDISK_NO_DRIVE_LETTER: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VOLUME_BACKGROUND_FORMAT: u32 = 14u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VOLUME_CHANGE_SIZE: u32 = 13u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VOLUME_DISMOUNT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VOLUME_DISMOUNT_FAILED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VOLUME_FORCED_CLOSED: u32 = 10u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VOLUME_INFO_MAKE_COMPAT: u32 = 11u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VOLUME_LOCK: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VOLUME_LOCK_FAILED: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VOLUME_MOUNT: u32 = 6u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VOLUME_NEEDS_CHKDSK: u32 = 7u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VOLUME_PREPARING_EJECT: u32 = 12u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VOLUME_UNLOCK: u32 = 5u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VOLUME_WEARING_OUT: u32 = 9u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_VOLUME_WORM_NEAR_FULL: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSRTL_WILD_CHARACTER: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_FILTER_SECTION_SYNC_IMAGE_EXTENTS_ARE_NOT_RVA: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_FILTER_SECTION_SYNC_IN_FLAG_DONT_UPDATE_LAST_ACCESS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_FILTER_SECTION_SYNC_IN_FLAG_DONT_UPDATE_LAST_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_FILTER_SECTION_SYNC_SUPPORTS_ASYNC_PARALLEL_IO: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_FILTER_SECTION_SYNC_SUPPORTS_DIRECT_MAP_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_FILTER_SECTION_SYNC_SUPPORTS_DIRECT_MAP_IMAGE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GCR_ALLOW_LM: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GCR_ALLOW_NO_TARGET: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GCR_ALLOW_NTLM: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GCR_MACHINE_CREDENTIAL: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GCR_NTLM3_PARMS: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GCR_TARGET_INFO: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GCR_USE_OEM_SET: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GCR_USE_OWF_PASSWORD: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GCR_VSM_PROTECTED_PASSWORD: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GENERATE_CLIENT_CHALLENGE: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GET_VOLUME_BITMAP_FLAG_MASK_METADATA: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_ATOMIC_CREATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4720bd83_52ac_4104_a130_d1ec6a8cc8e5);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_CLOUDFILES_ATTRIBUTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2932ff52_8378_4fc1_8edb_6bdc8f602709);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_CREATE_REDIRECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x188d6bd6_a126_4fa8_bdf2_1ccdf896f3e0);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_CSV_DOWN_LEVEL_OPEN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4248be44_647f_488f_8be5_a08aaf70f028);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_CSV_QUERY_FILE_REVISION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x44aec90b_de65_4d46_8fbf_763f9d970b1d);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_CSV_QUERY_FILE_REVISION_FILE_ID_128: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7a3a4aa1_aa74_4bc6_b070_ab56a38c1fed);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_CSV_SET_HANDLE_PROPERTIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7a9fdd94_7b58_42bb_9740_3cb86983a615);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_DUAL_OPLOCK_KEY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x41621a14_b08b_4df1_b676_a05ffdf01bea);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_IO_DEVICE_HINT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf315b732_ac6b_4d4d_be0c_b3126490e1a3);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_NETWORK_APP_INSTANCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6aa6bc45_a7ef_4af7_9008_fa462e144d74);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_NETWORK_APP_INSTANCE_VERSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb7d082b9_563b_4f07_a07b_524a8116a010);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_NETWORK_OPEN_CONTEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc584edbf_00df_4d28_b884_35baca8911e8);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_NFS_OPEN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf326d30c_e5f8_4fe7_ab74_f5a3196d92db);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_OPEN_PARAMETERS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcd0a93c3_3bb7_463d_accb_969d3435a5a5);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_OPLOCK_KEY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x48850596_3050_4be7_9863_fec350ce8d7f);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_PREFETCH_OPEN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe1777b21_847e_4837_aa45_64161d280655);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_QUERY_ON_CREATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1aca62e9_abb4_4ff2_bb5c_1c79025e417f);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_RKF_BYPASS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x02378cc6_f73c_489c_8282_564d1a99131b);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GUID_ECP_SRV_OPEN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbebfaebc_aabf_489d_9d2c_e9e361102853);
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CLASS_0: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CLASS_1: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CLASS_2: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CLASS_3: u32 = 12288u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CLASS_4: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CLASS_5: u32 = 20480u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CLASS_6: u32 = 24576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CLASS_7: u32 = 28672u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CLASS_8: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CLASS_MASK: u32 = 61440u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CREATE_ALIGN_16: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CREATE_ENABLE_EXECUTE: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CREATE_ENABLE_TRACING: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CREATE_HARDENED: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_CREATE_SEGMENT_HEAP: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_DISABLE_COALESCE_ON_FREE: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_FREE_CHECKING_ENABLED: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_GENERATE_EXCEPTIONS: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_GLOBAL_TAG: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_GROWABLE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_MAXIMUM_TAG: u32 = 4095u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_NO_SERIALIZE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_PSEUDO_TAG_FLAG: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_REALLOC_IN_PLACE_ONLY: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_SETTABLE_USER_FLAG1: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_SETTABLE_USER_FLAG2: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_SETTABLE_USER_FLAG3: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_SETTABLE_USER_FLAGS: u32 = 3584u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_SETTABLE_USER_VALUE: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_TAG_SHIFT: u32 = 18u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_TAIL_CHECKING_ENABLED: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HEAP_ZERO_MEMORY: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const INHERITED_ACE: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const INHERIT_ONLY_ACE: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const INVALID_PROCESSOR_INDEX: u32 = 4294967295u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IOCTL_LMR_ARE_FILE_OBJECTS_ON_SAME_SERVER: u32 = 1310960u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IOCTL_REDIR_QUERY_PATH: u32 = 1311119u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IOCTL_REDIR_QUERY_PATH_EX: u32 = 1311123u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IOCTL_VOLSNAP_FLUSH_AND_HOLD_WRITES: u32 = 5488640u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_CD_ROM_INCREMENT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_CREATE_STREAM_FILE_LITE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_CREATE_STREAM_FILE_RAISE_ON_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_DISK_INCREMENT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_FILE_OBJECT_NON_PAGED_POOL_CHARGE: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_FILE_OBJECT_PAGED_POOL_CHARGE: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_IGNORE_READONLY_ATTRIBUTE: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_MAILSLOT_INCREMENT: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_MM_PAGING_FILE: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_NAMED_PIPE_INCREMENT: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_NETWORK_INCREMENT: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_NO_INCREMENT: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_OPEN_PAGING_FILE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_OPEN_TARGET_DIRECTORY: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_0: i32 = 96i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_1: i32 = 97i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_2: i32 = 98i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_3: i32 = 99i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_4: i32 = 100i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_5: i32 = 101i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_6: i32 = 102i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_7: i32 = 103i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_8: i32 = 104i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_9: i32 = 105i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_A: i32 = 106i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_B: i32 = 107i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_C: i32 = 108i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_D: i32 = 109i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_E: i32 = 110i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACRONIS_HSM_F: i32 = 111i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ACTIVISION_HSM: i32 = 71i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ADA_HSM: i32 = 38i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ADOBE_HSM: i32 = 69i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_AF_UNIX: i32 = -2147483613i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ALERTBOOT: i32 = 536870988i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ALTIRIS_HSM: i32 = 25i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_APPEXECLINK: i32 = -2147483621i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_APPXSTRM: i32 = -1073741804i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ARCO_BACKUP: i32 = 59i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ARKIVIO: i32 = 12i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_AURISTOR_FS: i32 = 73i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_AUTN_HSM: i32 = 39i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_BRIDGEHEAD_HSM: i32 = 22i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_C2CSYSTEMS_HSM: i32 = 49i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CARINGO_HSM: i32 = 52i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CARROLL_HSM: i32 = 60i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CITRIX_PM: i32 = 54i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD: i32 = -1879048166i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_1: i32 = -1879044070i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_2: i32 = -1879039974i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_3: i32 = -1879035878i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_4: i32 = -1879031782i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_5: i32 = -1879027686i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_6: i32 = -1879023590i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_7: i32 = -1879019494i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_8: i32 = -1879015398i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_9: i32 = -1879011302i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_A: i32 = -1879007206i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_B: i32 = -1879003110i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_C: i32 = -1878999014i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_D: i32 = -1878994918i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_E: i32 = -1878990822i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_F: i32 = -1878986726i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CLOUD_MASK: i32 = 61440i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_COMMVAULT: i32 = 14i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_COMMVAULT_HSM: i32 = 29i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_COMTRADE_HSM: i32 = 61i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CSV: i32 = -2147483639i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_CTERA_HSM: i32 = 78i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_DATAFIRST_HSM: i32 = 48i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_DATAGLOBAL_HSM: i32 = 46i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_DATALESS_CIM: i32 = -1610612696i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_DATASTOR_SIS: i32 = 30i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_DEDUP: i32 = -2147483629i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_DFM: i32 = -2147483626i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_DFS: i32 = -2147483638i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_DFSR: i32 = -2147483630i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_DOR_HSM: i32 = 82i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_DOUBLE_TAKE_HSM: i32 = 34i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_DOUBLE_TAKE_SIS: i32 = 41i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_DRIVE_EXTENDER: i32 = -2147483643i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_DROPBOX_HSM: i32 = 68i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_EASEFILTER_HSM: i32 = 87i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_EASEVAULT_HSM: i32 = 62i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_EDSI_HSM: i32 = 31i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ELTAN_HSM: i32 = 43i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_EMC_HSM: i32 = 57i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ENIGMA_HSM: i32 = 17i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_FILE_PLACEHOLDER: i32 = -2147483627i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_FILTER_MANAGER: i32 = -2147483637i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_GLOBAL360_HSM: i32 = 24i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_GLOBAL_REPARSE: i32 = -1610612711i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_GOOGLE_HSM: i32 = 65i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_GRAU_DATASTORAGE_HSM: i32 = 28i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_HDS_HCP_HSM: i32 = 72i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_HDS_HSM: i32 = 63i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_HERMES_HSM: i32 = 26i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_HP_BACKUP: i32 = 67i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_HP_DATA_PROTECT: i32 = 70i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_HP_HSM: i32 = 32i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_HSAG_HSM: i32 = 37i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_HSM: i32 = -1073741820i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_HSM2: i32 = -2147483642i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_HUBSTOR_HSM: i32 = 85i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_IFSTEST_CONGRUENT: i32 = 9i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_IIS_CACHE: i32 = -1610612720i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_IMANAGE_HSM: i32 = 536870998i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_INTERCOPE_HSM: i32 = 19i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ITSTATION: i32 = 74i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_KOM_NETWORKS_HSM: i32 = 20i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_LX_BLK: i32 = -2147483610i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_LX_CHR: i32 = -2147483611i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_LX_FIFO: i32 = -2147483612i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_LX_SYMLINK: i32 = -1610612707i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_MAGINATICS_RDR: i32 = 64i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_MAXISCALE_HSM: i32 = 536870965i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_MEMORY_TECH_HSM: i32 = 21i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_MIMOSA_HSM: i32 = 36i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_MOONWALK_HSM: i32 = 10i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_MOUNT_POINT: i32 = -1610612733i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_MTALOS: i32 = 77i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_NEUSHIELD: i32 = 81i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_NEXSAN_HSM: i32 = 40i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_NFS: i32 = -2147483628i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_NIPPON_HSM: i32 = 79i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_NVIDIA_UNIONFS: i32 = 536870996i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ONEDRIVE: i32 = -2147483615i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_OPENAFS_DFS: i32 = 55i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_OSR_SAMPLE: i32 = 536870935i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_OVERTONE: i32 = 15i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_POINTSOFT_HSM: i32 = 27i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_PROJFS: i32 = -1879048164i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_PROJFS_TOMBSTONE: i32 = -1610612702i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_QI_TECH_HSM: i32 = 536870959i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_QUADDRA_HSM: i32 = 66i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_QUEST_HSM: i32 = 45i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_REDSTOR_HSM: i32 = 80i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_RESERVED_ONE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_RESERVED_RANGE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_RESERVED_TWO: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_RESERVED_ZERO: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_RIVERBED_HSM: i32 = 51i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_SER_HSM: i32 = 33i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_SHX_BACKUP: i32 = 83i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_SIS: i32 = -2147483641i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_SOLUTIONSOFT: i32 = 536870925i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_SONY_HSM: i32 = 42i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_SPHARSOFT: i32 = 75i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_STORAGE_SYNC: i32 = -2147483618i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_SYMANTEC_HSM: i32 = 18i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_SYMANTEC_HSM2: i32 = 16i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_SYMLINK: i32 = -1610612724i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_TSINGHUA_UNIVERSITY_RESEARCH: i32 = 11i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_UNHANDLED: i32 = -2147483616i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_UTIXO_HSM: i32 = 44i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_VALID_VALUES: u32 = 4026597375u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_VMWARE_PM: i32 = 58i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_WATERFORD: i32 = 50i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_WCI: i32 = -2147483624i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_WCI_1: i32 = -1879044072i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_WCI_LINK: i32 = -1610612697i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_WCI_LINK_1: i32 = -1610608601i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_WCI_TOMBSTONE: i32 = -1610612705i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_WIM: i32 = -2147483640i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_WISDATA_HSM: i32 = 35i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_WOF: i32 = -2147483625i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_REPARSE_TAG_ZLTI_HSM: i32 = 56i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const IO_STOP_ON_SYMLINK: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_ALLOCATE_MEMORY: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_CALL_LEVEL: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_CONFIDENTIALITY: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_CONFIDENTIALITY_ONLY: u32 = 1073741824u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_CONNECTION: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_DATAGRAM: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_DEFERRED_CRED_VALIDATION: u64 = 8589934592u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_DELEGATE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_EXTENDED_ERROR: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_FORWARD_CREDENTIALS: u32 = 4194304u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_FRAGMENT_SUPPLIED: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_FRAGMENT_TO_FIT: u32 = 2097152u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_IDENTIFY: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_INTEGRITY: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_MANUAL_CRED_VALIDATION: u32 = 524288u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_MESSAGES: u64 = 4294967296u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_MUTUAL_AUTH: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_NO_INTEGRITY: u32 = 8388608u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_NO_POST_HANDSHAKE_AUTH: u64 = 17179869184u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_NULL_SESSION: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_PROMPT_FOR_CREDS: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_REPLAY_DETECT: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_RESERVED1: u32 = 1048576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_SEQUENCE_DETECT: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_STREAM: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_UNVERIFIED_TARGET_NAME: u32 = 536870912u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_USE_DCE_STYLE: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_USE_HTTP_STYLE: u32 = 16777216u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_USE_SESSION_KEY: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_REQ_USE_SUPPLIED_CREDS: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_ALLOCATED_MEMORY: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_CALL_LEVEL: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_CONFIDENTIALITY: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_CONFIDENTIALITY_ONLY: u32 = 1073741824u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_CONNECTION: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_DATAGRAM: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_DEFERRED_CRED_VALIDATION: u64 = 8589934592u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_DELEGATE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_EXTENDED_ERROR: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_FORWARD_CREDENTIALS: u32 = 4194304u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_FRAGMENT_ONLY: u32 = 2097152u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_IDENTIFY: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_INTEGRITY: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_INTERMEDIATE_RETURN: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_MANUAL_CRED_VALIDATION: u32 = 524288u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_MESSAGES: u64 = 4294967296u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_MUTUAL_AUTH: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_NO_ADDITIONAL_TOKEN: u32 = 33554432u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_NO_POST_HANDSHAKE_AUTH: u64 = 17179869184u64;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_NULL_SESSION: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_REAUTHENTICATION: u32 = 134217728u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_REPLAY_DETECT: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_RESERVED1: u32 = 1048576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_SEQUENCE_DETECT: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_STREAM: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_USED_COLLECTED_CREDS: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_USED_DCE_STYLE: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_USED_HTTP_STYLE: u32 = 16777216u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_USED_SUPPLIED_CREDS: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISC_RET_USE_SESSION_KEY: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISSP_LEVEL: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ISSP_MODE: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const KDC_PROXY_SETTINGS_FLAGS_FORCEPROXY: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const KDC_PROXY_SETTINGS_V1: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_CACHED_ACCOUNT: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_EXTRA_SIDS: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_GRACE_LOGON: u32 = 16777216u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_GUEST: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_LM_V2: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_MANAGED_SERVICE: u32 = 524288u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_NOENCRYPTION: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_NO_ELEVATION: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_NO_OPTIMIZED: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_NTLMV2_ENABLED: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_NTLM_V2: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_NT_V2: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_OPTIMIZED: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_PKINIT: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_PROFILE_PATH_RETURNED: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_RESOURCE_GROUPS: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_SERVER_TRUST_ACCOUNT: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_SUBAUTH_SESSION_KEY: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_USED_LM_PASSWORD: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOGON_WINLOGON: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_DATA: u32 = 16777216u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_INDEX: u32 = 33554432u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_SYSTEM: u32 = 50331648u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_DENY_DEFRAG_SET: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_FS_SYSTEM_FILE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_PAGE_FILE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_TXF_SYSTEM_FILE: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LX_FILE_CASE_SENSITIVE_DIR: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LX_FILE_METADATA_DEVICE_ID_EA_NAME: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("$LXDEV");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LX_FILE_METADATA_GID_EA_NAME: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("$LXGID");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LX_FILE_METADATA_HAS_DEVICE_ID: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LX_FILE_METADATA_HAS_GID: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LX_FILE_METADATA_HAS_MODE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LX_FILE_METADATA_HAS_UID: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LX_FILE_METADATA_MODE_EA_NAME: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("$LXMOD");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LX_FILE_METADATA_UID_EA_NAME: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("$LXUID");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MAP_DISABLE_PAGEFAULT_CLUSTERING: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MAP_HIGH_PRIORITY: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MAP_NO_READ: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MAP_WAIT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_CLOUD_SYNC: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_DISABLE_FILE_METADATA_OPTIMIZATION: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_ENABLE_CPU_CACHE: u32 = 268435456u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_ENABLE_USN_SOURCE_ON_PAGING_IO: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_FILTER_METADATA: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_NOT_READ_COPY: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_NOT_REALTIME: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_NOT_TXF_SYSTEM_LOG: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_PROTECT_CLUSTERS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_READ_COPY: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_REALTIME: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_RETURN_PURGE_FAILURE: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_SKIP_COHERENCY_SYNC_DISALLOW_WRITES: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_SUPPRESS_VOLUME_OPEN_FLUSH: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MARK_HANDLE_TXF_SYSTEM_LOG: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MAXIMUM_ENCRYPTION_VALUE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MAXIMUM_LEADBYTES: u32 = 12u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MAX_UNICODE_STACK_BUFFER_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MCB_FLAG_RAISE_ON_ALLOCATION_FAILURE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const METHOD_BUFFERED: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const METHOD_DIRECT_FROM_HARDWARE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const METHOD_DIRECT_TO_HARDWARE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const METHOD_IN_DIRECT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const METHOD_NEITHER: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const METHOD_OUT_DIRECT: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MM_FORCE_CLOSED_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MM_FORCE_CLOSED_IMAGE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MM_FORCE_CLOSED_LATER_OK: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MM_IS_FILE_SECTION_ACTIVE_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MM_IS_FILE_SECTION_ACTIVE_IMAGE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MM_IS_FILE_SECTION_ACTIVE_USER: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_ALLOW_FORCE_GUEST: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_ALLOW_MSVCHAPV2: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_ALLOW_SERVER_TRUST_ACCOUNT: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_ALLOW_WORKSTATION_TRUST_ACCOUNT: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_AV_FLAG_FORCE_GUEST: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_AV_FLAG_MIC_HANDSHAKE_MESSAGES: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_AV_FLAG_UNVERIFIED_TARGET: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CHALLENGE_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CHECK_LOGONHOURS_FOR_S4U: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CLEARTEXT_PASSWORD_ALLOWED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CLEARTEXT_PASSWORD_SUPPLIED: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CREDENTIAL_KEY_LENGTH: u32 = 20u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CRED_CREDKEY_PRESENT: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CRED_LM_PRESENT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CRED_NT_PRESENT: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CRED_REMOVED: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CRED_SHA_PRESENT: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CRED_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CRED_VERSION_ARSO: u32 = 4294901763u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CRED_VERSION_INVALID: u32 = 4294967295u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CRED_VERSION_IUM: u32 = 4294901761u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CRED_VERSION_REMOTE: u32 = 4294901762u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CRED_VERSION_RESERVED_1: u32 = 4294967294u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CRED_VERSION_V2: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_CRED_VERSION_V3: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_DISABLE_PERSONAL_FALLBACK: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_DONT_TRY_GUEST_ACCOUNT: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_INTERNET_DOMAIN: u32 = 524288u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_LANMAN_SESSION_KEY_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_MAX_AVL_SIZE: u32 = 64000u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_MAX_NTLM3_LIFE: u32 = 1800u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_MNS_LOGON: u32 = 16777216u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_NTLM3_OWF_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_NTLM3_RESPONSE_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_OWF_PASSWORD_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_PACKAGE_NAME: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("MICROSOFT_AUTHENTICATION_PACKAGE_V1_0");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_PACKAGE_NAMEW: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("MICROSOFT_AUTHENTICATION_PACKAGE_V1_0");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_RETURN_PASSWORD_EXPIRY: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_RETURN_PROFILE_PATH: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_RETURN_USER_PARAMETERS: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_S4U2SELF: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_S4U_LOGON_FLAG_CHECK_LOGONHOURS: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_SHA_PASSWORD_LENGTH: u32 = 20u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL: u32 = 4278190080u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL_EX: u32 = 1048576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL_IIS: u32 = 132u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL_RAS: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL_SHIFT: u32 = 24u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_FLAGS: u32 = 4278190080u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_KEY: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("SYSTEM\\CurrentControlSet\\Control\\Lsa\\MSV1_0");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_VALUE: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("Auth");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_TRY_GUEST_ACCOUNT_ONLY: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_TRY_SPECIFIED_DOMAIN_ONLY: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_UPDATE_LOGON_STATISTICS: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_USER_SESSION_KEY_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_USE_CLIENT_CHALLENGE: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MSV1_0_USE_DOMAIN_FOR_ROUTING_ONLY: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NETWORK_APP_INSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NETWORK_OPEN_ECP_IN_FLAG_DISABLE_HANDLE_COLLAPSING: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NETWORK_OPEN_ECP_IN_FLAG_DISABLE_HANDLE_DURABILITY: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NETWORK_OPEN_ECP_IN_FLAG_DISABLE_OPLOCKS: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NETWORK_OPEN_ECP_IN_FLAG_FORCE_BUFFERED_SYNCHRONOUS_IO_HACK: u32 = 2147483648u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NETWORK_OPEN_ECP_IN_FLAG_FORCE_MAX_EOF_HACK: u32 = 1073741824u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NETWORK_OPEN_ECP_IN_FLAG_REQ_MUTUAL_AUTH: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NETWORK_OPEN_ECP_OUT_FLAG_RET_MUTUAL_AUTH: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NO_8DOT3_NAME_PRESENT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NO_LONG_NAMES: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NO_PROPAGATE_INHERIT_ACE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OBJECT_INHERIT_ACE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OFFLOAD_READ_FLAG_ALL_ZERO_BEYOND_CURRENT_RANGE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_OVERRIDE_CREATE_OPTION: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_REPARSE_ALWAYS: u32 = 126u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_REPARSE_IF_CHILD_EXISTS: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_REPARSE_IF_CHILD_NOT_EXISTS: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_REPARSE_IF_DIRECTORY_FINAL_COMPONENT: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_REPARSE_IF_DIRECTORY_FINAL_COMPONENT_ALWAYS: u32 = 72u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_REPARSE_IF_FINAL_COMPONENT: u32 = 40u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_REPARSE_IF_FINAL_COMPONENT_ALWAYS: u32 = 104u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_FINAL_COMPONENT: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_FINAL_COMPONENT_ALWAYS: u32 = 96u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_NON_FINAL_COMPONENT: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_NON_FINAL_COMPONENT_ALWAYS: u32 = 80u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_FINAL_COMPONENT: u32 = 22u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_RETURN_REPARSE_DATA_BUFFER: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_TAG_ENCOUNTERED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPEN_REPARSE_POINT_VERSION_EX: u32 = 2147483648u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_FLAG_BACK_OUT_ATOMIC_OPLOCK: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_FLAG_BREAKING_FOR_SHARING_VIOLATION: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_FLAG_CLOSING_DELETE_ON_CLOSE: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_FLAG_COMPLETE_IF_OPLOCKED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_FLAG_IGNORE_OPLOCK_KEYS: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_FLAG_OPLOCK_KEY_CHECK_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_FLAG_PARENT_OBJECT: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_FLAG_REMOVING_FILE_OR_LINK: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_FSCTRL_FLAG_ALL_KEYS_MATCH: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_LEVEL_CACHE_HANDLE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_LEVEL_CACHE_READ: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_LEVEL_CACHE_WRITE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_UPPER_FLAG_CHECK_NO_BREAK: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_UPPER_FLAG_NOTIFY_REFRESH_READ: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PERSISTENT_VOLUME_STATE_BACKED_BY_WIM: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PERSISTENT_VOLUME_STATE_CHKDSK_RAN_ONCE: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PERSISTENT_VOLUME_STATE_CONTAINS_BACKING_WIM: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PERSISTENT_VOLUME_STATE_DAX_FORMATTED: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PERSISTENT_VOLUME_STATE_GLOBAL_METADATA_NO_SEEK_PENALTY: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PERSISTENT_VOLUME_STATE_LOCAL_METADATA_NO_SEEK_PENALTY: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PERSISTENT_VOLUME_STATE_MODIFIED_BY_CHKDSK: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PERSISTENT_VOLUME_STATE_NO_HEAT_GATHERING: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PERSISTENT_VOLUME_STATE_NO_WRITE_AUTO_TIERING: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PERSISTENT_VOLUME_STATE_REALLOCATE_ALL_DATA_WRITES: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PERSISTENT_VOLUME_STATE_SHORT_NAME_CREATION_DISABLED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PERSISTENT_VOLUME_STATE_TXF_DISABLED: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PERSISTENT_VOLUME_STATE_VOLUME_SCRUB_DISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PIN_CALLER_TRACKS_DIRTY_DATA: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PIN_EXCLUSIVE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PIN_HIGH_PRIORITY: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PIN_IF_BCB: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PIN_NO_READ: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PIN_VERIFY_REQUIRED: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PIN_WAIT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const POLICY_AUDIT_SUBCATEGORY_COUNT: u32 = 59u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PO_CB_AC_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PO_CB_BUTTON_COLLISION: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PO_CB_LID_SWITCH_STATE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PO_CB_PROCESSOR_POWER_POLICY: u32 = 5u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PO_CB_SYSTEM_POWER_POLICY: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PO_CB_SYSTEM_STATE_LOCK: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PROJFS_PROTOCOL_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PSMP_MAXIMUM_SYSAPP_CLAIM_VALUES: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PSMP_MINIMUM_SYSAPP_CLAIM_VALUES: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const PURGE_WITH_ACTIVE_VIEWS: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_DEPENDENT_VOLUME_REQUEST_FLAG_GUEST_VOLUMES: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_DEPENDENT_VOLUME_REQUEST_FLAG_HOST_VOLUMES: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_DIRECT_ACCESS_DATA_EXTENTS: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_DIRECT_ACCESS_IMAGE_EXTENTS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_EXTENTS: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_EXTRA_INFO: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_FILES_WITH_DSC_ATTRIBUTE: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_FULL_PATH_IN_NAMES: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_NAMES: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_ONLY_FILES_WITH_SPECIFIC_ATTRIBUTES: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAMS: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAMS_WITH_NO_CLUSTERS_ALLOCATED: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_DATA_ATTRIBUTE: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_DSC_ATTRIBUTE: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_EA_ATTRIBUTE: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_EFS_ATTRIBUTE: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_REPARSE_ATTRIBUTE: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_TXF_ATTRIBUTE: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_REPARSE_DATA_INVALID: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_REPARSE_TAG_INVALID: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_RESTART: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_SINGLE_INSTANCED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_STORAGE_CLASSES_FLAGS_MEASURE_READ: u32 = 1073741824u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_STORAGE_CLASSES_FLAGS_MEASURE_WRITE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_STORAGE_CLASSES_FLAGS_NO_DEFRAG_VOLUME: u32 = 536870912u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QoCFileEaInformation: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QoCFileLxInformation: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QoCFileStatInformation: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_SMR_VOLUME_GC_PARAMETERS_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_SMR_VOLUME_INFO_OUTPUT_VERSION_V0: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_SMR_VOLUME_INFO_OUTPUT_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REMOTE_PROTOCOL_FLAG_INTEGRITY: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REMOTE_PROTOCOL_FLAG_LOOPBACK: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REMOTE_PROTOCOL_FLAG_MUTUAL_AUTH: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REMOTE_PROTOCOL_FLAG_OFFLINE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REMOTE_PROTOCOL_FLAG_PERSISTENT_HANDLE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REMOTE_PROTOCOL_FLAG_PRIVACY: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REMOVED_8DOT3_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REPARSE_DATA_EX_FLAG_GIVEN_TAG_OR_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REQUEST_OPLOCK_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REQUEST_OPLOCK_INPUT_FLAG_ACK: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REQUEST_OPLOCK_INPUT_FLAG_COMPLETE_ACK_ON_CLOSE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REQUEST_OPLOCK_INPUT_FLAG_REQUEST: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REQUEST_OPLOCK_OUTPUT_FLAG_ACK_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REQUEST_OPLOCK_OUTPUT_FLAG_MODES_PROVIDED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RETURN_NON_NT_USER_SESSION_KEY: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RETURN_PRIMARY_LOGON_DOMAINNAME: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RETURN_PRIMARY_USERNAME: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RETURN_RESERVED_PARAMETER: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SERVERCAP_DFS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SERVERCAP_DIRECTORY_LEASING: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SERVERCAP_ENCRYPTION_AWARE: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SERVERCAP_LARGEMTU: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SERVERCAP_LEASING: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SERVERCAP_MULTICHANNEL: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SERVERCAP_PERSISTENT_HANDLES: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SHARECAP_ACCESS_BASED_DIRECTORY_ENUM: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SHARECAP_ASYMMETRIC_SCALEOUT: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SHARECAP_CLUSTER: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SHARECAP_CONTINUOUS_AVAILABILITY: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SHARECAP_DFS: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SHARECAP_ENCRYPTED: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SHARECAP_IDENTITY_REMOTING: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SHARECAP_SCALEOUT: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SHARECAP_TIMEWARP: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SHAREFLAG_COMPRESS_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SHAREFLAG_ENCRYPT_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SHARETYPE_DISK: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SHARETYPE_PIPE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RPI_SMB2_SHARETYPE_PRINT: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RTL_DUPLICATE_UNICODE_STRING_ALLOCATE_NULL_STRING: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RTL_DUPLICATE_UNICODE_STRING_NULL_TERMINATE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RTL_HEAP_MEMORY_LIMIT_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RTL_SYSTEM_VOLUME_INFORMATION_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("System Volume Information");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SCRUB_DATA_INPUT_FLAG_IGNORE_REDUNDANCY: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SCRUB_DATA_INPUT_FLAG_OPLOCK_NOT_ACQUIRED: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SCRUB_DATA_INPUT_FLAG_RESUME: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SCRUB_DATA_INPUT_FLAG_SCRUB_BY_OBJECT_ID: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SCRUB_DATA_INPUT_FLAG_SKIP_DATA: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SCRUB_DATA_INPUT_FLAG_SKIP_IN_SYNC: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SCRUB_DATA_INPUT_FLAG_SKIP_NON_INTEGRITY_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SCRUB_DATA_OUTPUT_FLAG_INCOMPLETE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SCRUB_DATA_OUTPUT_FLAG_NON_USER_DATA_RANGE: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SCRUB_DATA_OUTPUT_FLAG_PARITY_EXTENT_DATA_RETURNED: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SCRUB_DATA_OUTPUT_FLAG_RESUME_CONTEXT_LENGTH_SPECIFIED: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SD_GLOBAL_CHANGE_TYPE_ENUM_SDS: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SD_GLOBAL_CHANGE_TYPE_MACHINE_SID: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SD_GLOBAL_CHANGE_TYPE_QUERY_STATS: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_ALERT: u32 = 17u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_APPLICATION_PROTOCOLS: u32 = 18u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_ATTRMASK: u32 = 4026531840u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_CERTIFICATE_REQUEST_CONTEXT: u32 = 29u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_CHANGE_PASS_RESPONSE: u32 = 15u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_CHANNEL_BINDINGS: u32 = 14u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_DTLS_MTU: u32 = 24u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_EMPTY: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_EXTRA: u32 = 5u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_FLAGS: u32 = 27u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_MECHLIST: u32 = 11u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_MECHLIST_SIGNATURE: u32 = 12u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_MISSING: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_NEGOTIATION_INFO: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_PADDING: u32 = 9u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_PKG_PARAMS: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_PRESHARED_KEY: u32 = 22u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_PRESHARED_KEY_IDENTITY: u32 = 23u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_READONLY: u32 = 2147483648u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_READONLY_WITH_CHECKSUM: u32 = 268435456u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_RESERVED: u32 = 1610612736u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_SEND_GENERIC_TLS_EXTENSION: u32 = 25u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_SRTP_MASTER_KEY_IDENTIFIER: u32 = 20u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_SRTP_PROTECTION_PROFILES: u32 = 19u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_STREAM: u32 = 10u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_STREAM_HEADER: u32 = 7u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_STREAM_TRAILER: u32 = 6u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_SUBSCRIBE_GENERIC_TLS_EXTENSION: u32 = 26u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_TARGET: u32 = 13u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_TARGET_HOST: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_TOKEN: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_TOKEN_BINDING: u32 = 21u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_TRAFFIC_SECRETS: u32 = 28u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECBUFFER_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_ACCESS_TOKEN: u32 = 18u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_APPLICATION_PROTOCOL: u32 = 35u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_AUTHENTICATION_ID: u32 = 20u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_AUTHORITY: u32 = 6u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_CLIENT_SPECIFIED_TARGET: u32 = 27u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_CONTEXT_DELETED: u32 = 33u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_CREDENTIAL_NAME: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_DATAGRAM_SIZES: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_DCE_INFO: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_DTLS_MTU: u32 = 34u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_ENDPOINT_BINDINGS: u32 = 26u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_FLAGS: u32 = 14u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_IS_LOOPBACK: u32 = 37u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_KEY_INFO: u32 = 5u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_LAST_CLIENT_TOKEN_STATUS: u32 = 30u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_LIFESPAN: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_LOGOFF_TIME: u32 = 21u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_NAMES: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_NATIVE_NAMES: u32 = 13u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_NEGOTIATED_TLS_EXTENSIONS: u32 = 36u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_NEGOTIATION_INFO: u32 = 12u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_NEGO_INFO_FLAG_NO_KERBEROS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_NEGO_INFO_FLAG_NO_NTLM: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_NEGO_KEYS: u32 = 22u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_NEGO_PKG_INFO: u32 = 31u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_NEGO_STATUS: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_PACKAGE_INFO: u32 = 10u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_PASSWORD_EXPIRY: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_PROMPTING_NEEDED: u32 = 24u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_PROTO_INFO: u32 = 7u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_SESSION_KEY: u32 = 9u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_SIZES: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_STREAM_SIZES: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_SUBJECT_SECURITY_ATTRIBUTES: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_TARGET: u32 = 19u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_TARGET_INFORMATION: u32 = 17u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_UNIQUE_BINDINGS: u32 = 25u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_USER_FLAGS: u32 = 11u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ATTR_USE_VALIDATED: u32 = 15u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CALLFLAGS_APPCONTAINER: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CALLFLAGS_APPCONTAINER_AUTHCAPABLE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CALLFLAGS_APPCONTAINER_UPNCAPABLE: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CALLFLAGS_FORCE_SUPPLIED: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CONTEXT_EXPORT_DELETE_OLD: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CONTEXT_EXPORT_RESET_NEW: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CONTEXT_EXPORT_TO_KERNEL: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CRED_ATTR_CERT: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CRED_ATTR_KDC_PROXY_SETTINGS: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CRED_ATTR_NAMES: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CRED_ATTR_PAC_BYPASS: u32 = 5u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CRED_ATTR_SSI_PROVIDER: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CRED_AUTOLOGON_RESTRICTED: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CRED_BOTH: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CRED_DEFAULT: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CRED_INBOUND: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CRED_OUTBOUND: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CRED_PROCESS_POLICY_ONLY: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_CRED_RESERVED: u32 = 4026531840u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_ACCEPT_WIN32_NAME: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_APPCONTAINER_CHECKS: u32 = 8388608u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_APPCONTAINER_PASSTHROUGH: u32 = 4194304u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_APPLY_LOOPBACK: u32 = 33554432u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_ASCII_BUFFERS: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_CLIENT_ONLY: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_CONNECTION: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_CREDENTIAL_ISOLATION_ENABLED: u32 = 16777216u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_DATAGRAM: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_DELEGATION: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_EXTENDED_ERROR: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_FRAGMENT: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_GSS_COMPATIBLE: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_IMPERSONATION: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_INTEGRITY: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_LOGON: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_MULTI_REQUIRED: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_MUTUAL_AUTH: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_NEGOTIABLE: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_NEGOTIABLE2: u32 = 2097152u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_NEGO_EXTENDER: u32 = 1048576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_PRIVACY: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_READONLY_WITH_CHECKSUM: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_RESTRICTED_TOKENS: u32 = 524288u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_STREAM: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_FLAG_TOKEN_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECPKG_ID_NONE: u32 = 65535u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECQOP_WRAP_NO_ENCRYPT: u32 = 2147483649u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECQOP_WRAP_OOB_DATA: u32 = 1073741824u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_ANONYMOUS_LOGON_RID: i32 = 7i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_APPPOOL_ID_BASE_RID: i32 = 82i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_APPPOOL_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_APP_PACKAGE_BASE_RID: i32 = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_APP_PACKAGE_RID_COUNT: i32 = 8i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_AUTHENTICATED_USER_RID: i32 = 11i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_AUTHENTICATION_AUTHORITY_ASSERTED_RID: i32 = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_AUTHENTICATION_AUTHORITY_RID_COUNT: i32 = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_AUTHENTICATION_FRESH_KEY_AUTH_RID: i32 = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_AUTHENTICATION_KEY_PROPERTY_ATTESTATION_RID: i32 = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_AUTHENTICATION_KEY_PROPERTY_MFA_RID: i32 = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_AUTHENTICATION_KEY_TRUST_RID: i32 = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_AUTHENTICATION_SERVICE_ASSERTED_RID: i32 = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_BATCH_RID: i32 = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_BUILTIN_APP_PACKAGE_RID_COUNT: i32 = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_BUILTIN_CAPABILITY_RID_COUNT: i32 = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_BUILTIN_DOMAIN_RID: i32 = 32i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_BUILTIN_PACKAGE_ANY_PACKAGE: i32 = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_BUILTIN_PACKAGE_ANY_RESTRICTED_PACKAGE: i32 = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_APPOINTMENTS: i32 = 11i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_APP_RID: i32 = 1024i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_APP_SILO_RID: i32 = 65536i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_BASE_RID: i32 = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_CONTACTS: i32 = 12i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_DOCUMENTS_LIBRARY: i32 = 7i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_ENTERPRISE_AUTHENTICATION: i32 = 8i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_INTERNET_CLIENT: i32 = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_INTERNET_CLIENT_SERVER: i32 = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_INTERNET_EXPLORER: i32 = 4096i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_MUSIC_LIBRARY: i32 = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_PICTURES_LIBRARY: i32 = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER: i32 = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_REMOVABLE_STORAGE: i32 = 10i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_RID_COUNT: i32 = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_SHARED_USER_CERTIFICATES: i32 = 9i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CAPABILITY_VIDEOS_LIBRARY: i32 = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CCG_ID_BASE_RID: i32 = 95i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CHILD_PACKAGE_RID_COUNT: i32 = 12i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CLOUD_INFRASTRUCTURE_SERVICES_ID_BASE_RID: i32 = 85i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CLOUD_INFRASTRUCTURE_SERVICES_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_COM_ID_BASE_RID: i32 = 89i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CREATOR_GROUP_RID: i32 = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CREATOR_GROUP_SERVER_RID: i32 = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CREATOR_OWNER_RID: i32 = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CREATOR_OWNER_RIGHTS_RID: i32 = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CREATOR_OWNER_SERVER_RID: i32 = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CRED_TYPE_BASE_RID: i32 = 65i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CRED_TYPE_RID_COUNT: i32 = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_CRED_TYPE_THIS_ORG_CERT_RID: i32 = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_DASHOST_ID_BASE_RID: i32 = 92i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_DASHOST_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_DIALUP_RID: i32 = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_ENTERPRISE_CONTROLLERS_RID: i32 = 9i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_ENTERPRISE_READONLY_CONTROLLERS_RID: i32 = 22i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_INSTALLER_CAPABILITY_RID_COUNT: u32 = 10u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_INSTALLER_GROUP_CAPABILITY_BASE: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_INSTALLER_GROUP_CAPABILITY_RID_COUNT: u32 = 9u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_INTERACTIVE_RID: i32 = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_IUSER_RID: i32 = 17i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_LOCAL_ACCOUNT_AND_ADMIN_RID: i32 = 114i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_LOCAL_ACCOUNT_RID: i32 = 113i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_LOCAL_LOGON_RID: i32 = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_LOCAL_RID: i32 = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_LOCAL_SERVICE_RID: i32 = 19i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_LOCAL_SYSTEM_RID: i32 = 18i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_LOGON_IDS_RID: i32 = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_LOGON_IDS_RID_COUNT: i32 = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_MANDATORY_HIGH_RID: i32 = 12288i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_MANDATORY_LOW_RID: i32 = 4096i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_MANDATORY_MAXIMUM_USER_RID: i32 = 16384i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_MANDATORY_MEDIUM_PLUS_RID: u32 = 8448u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_MANDATORY_MEDIUM_RID: i32 = 8192i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_MANDATORY_PROTECTED_PROCESS_RID: i32 = 20480i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_MANDATORY_SYSTEM_RID: i32 = 16384i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_MANDATORY_UNTRUSTED_RID: i32 = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_MAX_ALWAYS_FILTERED: i32 = 999i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_MAX_BASE_RID: i32 = 111i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_MIN_BASE_RID: i32 = 80i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_MIN_NEVER_FILTERED: i32 = 1000i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_NATIVE_DREP: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_NETWORK_DREP: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_NETWORK_RID: i32 = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_NETWORK_SERVICE_RID: i32 = 20i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_NFS_ID_BASE_RID: i32 = 88i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_NT_NON_UNIQUE: i32 = 21i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_NT_NON_UNIQUE_SUB_AUTH_COUNT: i32 = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_NULL_RID: i32 = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_OTHER_ORGANIZATION_RID: i32 = 1000i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PACKAGE_BASE_RID: i32 = 64i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PACKAGE_DIGEST_RID: i32 = 21i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PACKAGE_NTLM_RID: i32 = 10i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PACKAGE_RID_COUNT: i32 = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PACKAGE_SCHANNEL_RID: i32 = 14i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PARENT_PACKAGE_RID_COUNT: i32 = 8i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PRINCIPAL_SELF_RID: i32 = 10i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PROCESS_PROTECTION_LEVEL_ANTIMALWARE_RID: i32 = 1536i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PROCESS_PROTECTION_LEVEL_APP_RID: i32 = 2048i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PROCESS_PROTECTION_LEVEL_AUTHENTICODE_RID: i32 = 1024i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PROCESS_PROTECTION_LEVEL_NONE_RID: i32 = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PROCESS_PROTECTION_LEVEL_WINDOWS_RID: i32 = 4096i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PROCESS_PROTECTION_LEVEL_WINTCB_RID: i32 = 8192i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PROCESS_PROTECTION_TYPE_FULL_RID: i32 = 1024i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PROCESS_PROTECTION_TYPE_LITE_RID: i32 = 512i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PROCESS_PROTECTION_TYPE_NONE_RID: i32 = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PROCESS_TRUST_AUTHORITY_RID_COUNT: i32 = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_PROXY_RID: i32 = 8i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_RDV_GFX_BASE_RID: i32 = 91i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_REMOTE_LOGON_RID: i32 = 14i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_RESERVED_ID_BASE_RID: i32 = 81i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_RESTRICTED_CODE_RID: i32 = 12i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_SERVER_LOGON_RID: i32 = 9i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_SERVICE_ID_BASE_RID: i32 = 80i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_SERVICE_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_SERVICE_RID: i32 = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_4: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_5: u32 = 5u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_TASK_ID_BASE_RID: i32 = 87i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_TERMINAL_SERVER_RID: i32 = 13i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_THIS_ORGANIZATION_RID: i32 = 15i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_TRUSTED_INSTALLER_RID1: u32 = 956008885u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_TRUSTED_INSTALLER_RID2: u32 = 3418522649u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_TRUSTED_INSTALLER_RID3: u32 = 1831038044u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_TRUSTED_INSTALLER_RID4: u32 = 1853292631u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_TRUSTED_INSTALLER_RID5: u32 = 2271478464u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_UMFD_BASE_RID: i32 = 96i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_USERMANAGER_ID_BASE_RID: i32 = 93i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_USERMANAGER_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_USERMODEDRIVERHOST_ID_BASE_RID: i32 = 84i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_USERMODEDRIVERHOST_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_VIRTUALACCOUNT_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_VIRTUALSERVER_ID_BASE_RID: i32 = 83i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_VIRTUALSERVER_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_WINDOWSMOBILE_ID_BASE_RID: i32 = 112i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_WINDOW_MANAGER_BASE_RID: i32 = 90i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_WINRM_ID_BASE_RID: i32 = 94i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_WINRM_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_WMIHOST_ID_BASE_RID: i32 = 86i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_WMIHOST_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_WORLD_RID: i32 = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SECURITY_WRITE_RESTRICTED_CODE_RID: i32 = 33i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_ANSI: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_MARSHALLED: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_UNICODE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_VERSION: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_VERSION_2: u32 = 513u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEF_AI_USE_EXTRA_PARAMS: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEF_AVOID_OWNER_CHECK: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEF_AVOID_OWNER_RESTRICTION: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEF_AVOID_PRIVILEGE_CHECK: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEF_DACL_AUTO_INHERIT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEF_DEFAULT_DESCRIPTOR_FOR_OBJECT: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEF_DEFAULT_GROUP_FROM_PARENT: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEF_DEFAULT_OWNER_FROM_PARENT: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEF_FORCE_USER_MODE: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEF_MACL_NO_EXECUTE_UP: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEF_MACL_NO_READ_UP: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEF_MACL_NO_WRITE_UP: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEF_NORMALIZE_OUTPUT_DESCRIPTOR: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEF_SACL_AUTO_INHERIT: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEGMENT_HEAP_FLG_USE_PAGE_HEAP: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEGMENT_HEAP_PARAMETERS_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEGMENT_HEAP_PARAMS_VALID_FLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SEMAPHORE_INCREMENT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SET_PURGE_FAILURE_MODE_DISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SET_PURGE_FAILURE_MODE_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SET_REPAIR_DISABLED_AND_BUGCHECK_ON_CORRUPT: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SET_REPAIR_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SET_REPAIR_VALID_MASK: u32 = 25u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SET_REPAIR_WARN_ABOUT_DATA_LOSS: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_ACCESS_CHECK_FLAG_NO_LEARNING_MODE_LOGGING: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_ACCESS_CHECK_VALID_FLAGS: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_ADT_OBJECT_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_ADT_PARAMETERS_SELF_RELATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_ADT_PARAMETERS_SEND_TO_LSA: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_ADT_PARAMETER_EXTENSIBLE_AUDIT: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_ADT_PARAMETER_GENERIC_AUDIT: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_ADT_PARAMETER_WRITE_SYNCHRONOUS: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_BACKUP_PRIVILEGES_CHECKED: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_DACL_AUTO_INHERITED: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_DACL_AUTO_INHERIT_REQ: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_DACL_DEFAULTED: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_DACL_PRESENT: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_DACL_PROTECTED: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_DACL_UNTRUSTED: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_GROUP_DEFAULTED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_MAX_AUDIT_PARAMETERS: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_MAX_GENERIC_AUDIT_PARAMETERS: u32 = 28u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_OWNER_DEFAULTED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_RM_CONTROL_VALID: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_SACL_AUTO_INHERITED: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_SACL_AUTO_INHERIT_REQ: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_SACL_DEFAULTED: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_SACL_PRESENT: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_SACL_PROTECTED: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_SECURITY_DESCRIPTOR_FLAG_NO_ACCESS_FILTER_ACE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_SECURITY_DESCRIPTOR_FLAG_NO_LABEL_ACE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_SECURITY_DESCRIPTOR_FLAG_NO_OWNER_ACE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_SECURITY_DESCRIPTOR_VALID_FLAGS: u32 = 7u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_SELF_RELATIVE: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SE_SERVER_SECURITY: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SHUFFLE_FILE_FLAG_SKIP_INITIALIZING_NEW_CLUSTERS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SID_HASH_SIZE: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SID_MAX_SUB_AUTHORITIES: u32 = 15u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SID_RECOMMENDED_SUB_AUTHORITIES: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SID_REVISION: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SMB_CCF_APP_INSTANCE_EA_NAME: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("ClusteredApplicationInstance");
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SPACES_TRACKED_OFFSET_HEADER_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SPECIAL_ENCRYPTED_OPEN: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SRV_OPEN_ECP_CONTEXT_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const STREAMS_ASSOCIATE_ID_CLEAR: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const STREAMS_ASSOCIATE_ID_SET: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const STREAMS_INVALID_ID: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const STREAM_CLEAR_ENCRYPTION: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const STREAM_EXTENT_ENTRY_ALL_EXTENTS: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const STREAM_EXTENT_ENTRY_AS_RETRIEVAL_POINTERS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const STREAM_LAYOUT_ENTRY_HAS_INFORMATION: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const STREAM_LAYOUT_ENTRY_IMMOVABLE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const STREAM_LAYOUT_ENTRY_NO_CLUSTERS_ALLOCATED: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const STREAM_LAYOUT_ENTRY_PINNED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const STREAM_LAYOUT_ENTRY_RESIDENT: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const STREAM_SET_ENCRYPTION: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SUCCESSFUL_ACCESS_ACE_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SUPPORTED_FS_FEATURES_BYPASS_IO: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SUPPORTED_FS_FEATURES_OFFLOAD_READ: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SUPPORTED_FS_FEATURES_OFFLOAD_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SUPPORTED_FS_FEATURES_QUERY_OPEN: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYMLINK_DIRECTORY: u32 = 2147483648u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYMLINK_FILE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYMLINK_FLAG_RELATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYMLINK_RESERVED_MASK: u32 = 4026531840u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_ACCESS_FILTER_ACE_TYPE: u32 = 21u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_ACCESS_FILTER_NOCONSTRAINT_MASK: u32 = 4294967295u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_ACCESS_FILTER_VALID_MASK: u32 = 16777215u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_ALARM_ACE_TYPE: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_ALARM_CALLBACK_ACE_TYPE: u32 = 14u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_ALARM_CALLBACK_OBJECT_ACE_TYPE: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_ALARM_OBJECT_ACE_TYPE: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_AUDIT_ACE_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_AUDIT_CALLBACK_ACE_TYPE: u32 = 13u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_AUDIT_CALLBACK_OBJECT_ACE_TYPE: u32 = 15u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_AUDIT_OBJECT_ACE_TYPE: u32 = 7u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_MANDATORY_LABEL_ACE_TYPE: u32 = 17u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_MANDATORY_LABEL_NO_EXECUTE_UP: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_MANDATORY_LABEL_NO_READ_UP: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_MANDATORY_LABEL_NO_WRITE_UP: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_PAGE_PRIORITY_BITS: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_PROCESS_TRUST_LABEL_ACE_TYPE: u32 = 20u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_PROCESS_TRUST_LABEL_VALID_MASK: u32 = 16777215u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_PROCESS_TRUST_NOCONSTRAINT_MASK: u32 = 4294967295u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_RESOURCE_ATTRIBUTE_ACE_TYPE: u32 = 18u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SYSTEM_SCOPED_POLICY_ID_ACE_TYPE: u32 = 19u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SZ_ALG_MAX_SIZE: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_ADJUST_DEFAULT: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_ADJUST_GROUPS: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_ADJUST_PRIVILEGES: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_ADJUST_SESSIONID: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_ASSIGN_PRIMARY: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_AUDIT_NO_CHILD_PROCESS: u32 = 2097152u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_AUDIT_REDIRECTION_TRUST: u32 = 8388608u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_DO_NOT_USE_GLOBAL_ATTRIBS_FOR_QUERY: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_DUPLICATE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_ENFORCE_REDIRECTION_TRUST: u32 = 4194304u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_HAS_BACKUP_PRIVILEGE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_HAS_IMPERSONATE_PRIVILEGE: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_HAS_OWN_CLAIM_ATTRIBUTES: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_HAS_RESTORE_PRIVILEGE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_HAS_TRAVERSE_PRIVILEGE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_IMPERSONATE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_IS_FILTERED: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_IS_RESTRICTED: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_LEARNING_MODE_LOGGING: u32 = 16777216u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_LOWBOX: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_MANDATORY_POLICY_NO_WRITE_UP: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_MANDATORY_POLICY_OFF: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_NOT_LOW: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_NO_CHILD_PROCESS: u32 = 524288u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_NO_CHILD_PROCESS_UNLESS_SECURE: u32 = 1048576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_PERMISSIVE_LEARNING_MODE: u32 = 50331648u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_PRIVATE_NAMESPACE: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_QUERY: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_QUERY_SOURCE: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_SANDBOX_INERT: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_SESSION_NOT_REFERENCED: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_SOURCE_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_UIACCESS: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_VIRTUALIZE_ALLOWED: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_VIRTUALIZE_ENABLED: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TOKEN_WRITE_RESTRICTED: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TRUST_PROTECTED_FILTER_ACE_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_CREATED: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_DELETED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_LOGGING_MODE_FULL: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_LOGGING_MODE_SIMPLE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_DO_NOT_RESET_RM_AT_NEXT_START: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_ENFORCE_MINIMUM_SIZE: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_GROW_LOG: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_LOGGING_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MAX: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MIN: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_PREFER_AVAILABILITY: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_PREFER_CONSISTENCY: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_PRESERVE_CHANGES: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_RENAME_RM: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_RESET_RM_AT_NEXT_START: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_FLAG_SHRINK_LOG: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_STATE_ACTIVE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_STATE_NOT_STARTED: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_STATE_SHUTTING_DOWN: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_RM_STATE_STARTING: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_ROLLFORWARD_REDO_FLAG_USE_LAST_REDO_LSN: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_ROLLFORWARD_REDO_FLAG_USE_LAST_VIRTUAL_CLOCK: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_SAVEPOINT_CLEAR: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_SAVEPOINT_CLEAR_ALL: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_SAVEPOINT_ROLLBACK: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_SAVEPOINT_SET: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_START_RM_FLAG_LOGGING_MODE: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_START_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MAX: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MIN: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_SIZE: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN: u32 = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_START_RM_FLAG_PREFER_AVAILABILITY: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_START_RM_FLAG_PREFER_CONSISTENCY: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_START_RM_FLAG_PRESERVE_CHANGES: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_START_RM_FLAG_RECOVER_BEST_EFFORT: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_TRANSACTED_VERSION_NONTRANSACTED: u32 = 4294967294u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_TRANSACTED_VERSION_UNCOMMITTED: u32 = 4294967295u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_TRANSACTION_STATE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_TRANSACTION_STATE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_TRANSACTION_STATE_NOTACTIVE: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TXFS_TRANSACTION_STATE_PREPARED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const UNDERSTANDS_LONG_NAMES: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const UNINITIALIZE_CACHE_MAPS: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USE_PRIMARY_PASSWORD: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_DELETE_FLAG_DELETE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_DELETE_FLAG_NOTIFY: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_DELETE_VALID_FLAGS: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_PAGE_SIZE: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_BASIC_INFO_CHANGE: u32 = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_CLOSE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_COMPRESSION_CHANGE: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_DATA_EXTEND: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_DATA_OVERWRITE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_DATA_TRUNCATION: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_DESIRED_STORAGE_CLASS_CHANGE: u32 = 16777216u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_EA_CHANGE: u32 = 1024u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_ENCRYPTION_CHANGE: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_FILE_CREATE: u32 = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_FILE_DELETE: u32 = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_HARD_LINK_CHANGE: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_INDEXABLE_CHANGE: u32 = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_INTEGRITY_CHANGE: u32 = 8388608u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_NAMED_DATA_EXTEND: u32 = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_NAMED_DATA_OVERWRITE: u32 = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_NAMED_DATA_TRUNCATION: u32 = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_OBJECT_ID_CHANGE: u32 = 524288u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_RENAME_NEW_NAME: u32 = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_RENAME_OLD_NAME: u32 = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_REPARSE_POINT_CHANGE: u32 = 1048576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_SECURITY_CHANGE: u32 = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_STREAM_CHANGE: u32 = 2097152u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_REASON_TRANSACTED_CHANGE: u32 = 4194304u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_SOURCE_AUXILIARY_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_SOURCE_CLIENT_REPLICATION_MANAGEMENT: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_SOURCE_DATA_MANAGEMENT: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const USN_SOURCE_REPLICATION_MANAGEMENT: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const VACB_MAPPING_GRANULARITY: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const VACB_OFFSET_SHIFT: u32 = 18u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const VALID_INHERIT_FLAGS: u32 = 31u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const VOLSNAPCONTROLTYPE: u32 = 83u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const VOLUME_IS_DIRTY: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const VOLUME_SESSION_OPEN: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const VOLUME_UPGRADE_SCHEDULED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_LAYER: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_REGISTERED_LAYER: u32 = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_REMOTE_LAYER: u32 = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_SCRATCH: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WIM_BOOT_NOT_OS_WIM: u32 = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WIM_BOOT_OS_WIM: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WIM_PROVIDER_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WIM_PROVIDER_EXTERNAL_FLAG_NOT_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WIM_PROVIDER_EXTERNAL_FLAG_SUSPENDED: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WIM_PROVIDER_HASH_SIZE: u32 = 20u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_CRED_MANAGER: u32 = 4294901760u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_10NET: u32 = 327680u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_3IN1: u32 = 2555904u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_9P: u32 = 4718592u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_9TILES: u32 = 589824u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_APPLETALK: u32 = 1245184u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_AS400: u32 = 720896u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_AURISTOR_FS: u32 = 4587520u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_AVID: u32 = 1703936u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_AVID1: u32 = 3801088u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_BMC: u32 = 1572864u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_BWNFS: u32 = 1048576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_CLEARCASE: u32 = 1441792u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_COGENT: u32 = 1114112u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_CSC: u32 = 2490368u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_DAV: u32 = 3014656u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_DCE: u32 = 1638400u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_DECORB: u32 = 2097152u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_DFS: u32 = 3866624u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_DISTINCT: u32 = 2293760u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_DOCUSHARE: u32 = 4521984u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_DOCUSPACE: u32 = 1769472u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_DRIVEONWEB: u32 = 4063232u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_EXIFS: u32 = 2949120u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_EXTENDNET: u32 = 2686976u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_FARALLON: u32 = 1179648u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_FJ_REDIR: u32 = 2228224u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_FOXBAT: u32 = 2818048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_FRONTIER: u32 = 1507328u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_FTP_NFS: u32 = 786432u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_GOOGLE: u32 = 4390912u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_HOB_NFS: u32 = 3276800u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_IBMAL: u32 = 3407872u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_INTERGRAPH: u32 = 1310720u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_KNOWARE: u32 = 3080192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_KWNP: u32 = 3932160u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_LANMAN: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_LANSTEP: u32 = 524288u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_LANTASTIC: u32 = 655360u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_LIFENET: u32 = 917504u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_LOCK: u32 = 3473408u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_LOCUS: u32 = 393216u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_MANGOSOFT: u32 = 1835008u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_MASFAX: u32 = 3211264u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_MFILES: u32 = 4259840u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_MSNET: u32 = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_MS_NFS: u32 = 4325376u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_NDFS: u32 = 4456448u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_NETWARE: u32 = 196608u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_OBJECT_DIRE: u32 = 3145728u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_OPENAFS: u32 = 3735552u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_PATHWORKS: u32 = 851968u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_POWERLAN: u32 = 983040u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_PROTSTOR: u32 = 2162688u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_QUINCY: u32 = 3670016u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_RDR2SAMPLE: u32 = 2424832u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_RIVERFRONT1: u32 = 1966080u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_RIVERFRONT2: u32 = 2031616u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_RSFX: u32 = 4194304u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_SECUREAGENT: u32 = 4653056u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_SERNET: u32 = 1900544u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_SHIVA: u32 = 3342336u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_SMB: u32 = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_SRT: u32 = 3604480u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_STAC: u32 = 2752512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_SUN_PC_NFS: u32 = 458752u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_SYMFONET: u32 = 1376256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_TERMSRV: u32 = 3538944u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_TWINS: u32 = 2359296u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_VINES: u32 = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_VMWARE: u32 = 4128768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_YAHOO: u32 = 2883584u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WNNC_NET_ZENWORKS: u32 = 3997696u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WOF_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WOF_PROVIDER_CLOUD: u32 = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WOF_PROVIDER_FILE: u32 = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const WOF_PROVIDER_WIM: u32 = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type ACCESS_REASON_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonNone: ACCESS_REASON_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonAllowedAce: ACCESS_REASON_TYPE = 65536i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonDeniedAce: ACCESS_REASON_TYPE = 131072i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonAllowedParentAce: ACCESS_REASON_TYPE = 196608i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonDeniedParentAce: ACCESS_REASON_TYPE = 262144i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonNotGrantedByCape: ACCESS_REASON_TYPE = 327680i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonNotGrantedByParentCape: ACCESS_REASON_TYPE = 393216i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonNotGrantedToAppContainer: ACCESS_REASON_TYPE = 458752i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonMissingPrivilege: ACCESS_REASON_TYPE = 1048576i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonFromPrivilege: ACCESS_REASON_TYPE = 2097152i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonIntegrityLevel: ACCESS_REASON_TYPE = 3145728i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonOwnership: ACCESS_REASON_TYPE = 4194304i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonNullDacl: ACCESS_REASON_TYPE = 5242880i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonEmptyDacl: ACCESS_REASON_TYPE = 6291456i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonNoSD: ACCESS_REASON_TYPE = 7340032i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonNoGrant: ACCESS_REASON_TYPE = 8388608i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonTrustLabel: ACCESS_REASON_TYPE = 9437184i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AccessReasonFilterAce: ACCESS_REASON_TYPE = 10485760i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type AUDIT_EVENT_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditEventObjectAccess: AUDIT_EVENT_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditEventDirectoryServiceAccess: AUDIT_EVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type CSVFS_DISK_CONNECTIVITY = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvFsDiskConnectivityNone: CSVFS_DISK_CONNECTIVITY = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvFsDiskConnectivityMdsNodeOnly: CSVFS_DISK_CONNECTIVITY = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvFsDiskConnectivitySubsetOfNodes: CSVFS_DISK_CONNECTIVITY = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvFsDiskConnectivityAllNodes: CSVFS_DISK_CONNECTIVITY = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type CSV_CONTROL_OP = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlStartRedirectFile: CSV_CONTROL_OP = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlStopRedirectFile: CSV_CONTROL_OP = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlQueryRedirectState: CSV_CONTROL_OP = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlQueryFileRevision: CSV_CONTROL_OP = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlQueryMdsPath: CSV_CONTROL_OP = 8i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlQueryFileRevisionFileId128: CSV_CONTROL_OP = 9i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlQueryVolumeRedirectState: CSV_CONTROL_OP = 10i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlEnableUSNRangeModificationTracking: CSV_CONTROL_OP = 13i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlMarkHandleLocalVolumeMount: CSV_CONTROL_OP = 14i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlUnmarkHandleLocalVolumeMount: CSV_CONTROL_OP = 15i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlGetCsvFsMdsPathV2: CSV_CONTROL_OP = 18i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlDisableCaching: CSV_CONTROL_OP = 19i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlEnableCaching: CSV_CONTROL_OP = 20i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlStartForceDFO: CSV_CONTROL_OP = 21i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlStopForceDFO: CSV_CONTROL_OP = 22i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlQueryMdsPathNoPause: CSV_CONTROL_OP = 23i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlSetVolumeId: CSV_CONTROL_OP = 24i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvControlQueryVolumeId: CSV_CONTROL_OP = 25i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type CSV_DOWN_LEVEL_FILE_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvDownLevelFileObject: CSV_DOWN_LEVEL_FILE_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CsvCsvFsInternalFileObject: CSV_DOWN_LEVEL_FILE_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type DUPLICATE_EXTENTS_STATE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileSnapStateInactive: DUPLICATE_EXTENTS_STATE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileSnapStateSource: DUPLICATE_EXTENTS_STATE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileSnapStateTarget: DUPLICATE_EXTENTS_STATE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FAST_IO_POSSIBLE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FastIoIsNotPossible: FAST_IO_POSSIBLE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FastIoIsPossible: FAST_IO_POSSIBLE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FastIoIsQuestionable: FAST_IO_POSSIBLE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FILE_DISPOSITION_INFORMATION_EX_FLAGS = u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DISPOSITION_DO_NOT_DELETE: FILE_DISPOSITION_INFORMATION_EX_FLAGS = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DISPOSITION_DELETE: FILE_DISPOSITION_INFORMATION_EX_FLAGS = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DISPOSITION_POSIX_SEMANTICS: FILE_DISPOSITION_INFORMATION_EX_FLAGS = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DISPOSITION_FORCE_IMAGE_SECTION_CHECK: FILE_DISPOSITION_INFORMATION_EX_FLAGS = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DISPOSITION_ON_CLOSE: FILE_DISPOSITION_INFORMATION_EX_FLAGS = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DISPOSITION_IGNORE_READONLY_ATTRIBUTE: FILE_DISPOSITION_INFORMATION_EX_FLAGS = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FILE_KNOWN_FOLDER_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const KnownFolderNone: FILE_KNOWN_FOLDER_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const KnownFolderDesktop: FILE_KNOWN_FOLDER_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const KnownFolderDocuments: FILE_KNOWN_FOLDER_TYPE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const KnownFolderDownloads: FILE_KNOWN_FOLDER_TYPE = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const KnownFolderMusic: FILE_KNOWN_FOLDER_TYPE = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const KnownFolderPictures: FILE_KNOWN_FOLDER_TYPE = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const KnownFolderVideos: FILE_KNOWN_FOLDER_TYPE = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const KnownFolderOther: FILE_KNOWN_FOLDER_TYPE = 7i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const KnownFolderMax: FILE_KNOWN_FOLDER_TYPE = 7i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FILE_STORAGE_TIER_CLASS = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileStorageTierClassUnspecified: FILE_STORAGE_TIER_CLASS = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileStorageTierClassCapacity: FILE_STORAGE_TIER_CLASS = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileStorageTierClassPerformance: FILE_STORAGE_TIER_CLASS = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileStorageTierClassMax: FILE_STORAGE_TIER_CLASS = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FILE_STORAGE_TIER_MEDIA_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileStorageTierMediaTypeUnspecified: FILE_STORAGE_TIER_MEDIA_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileStorageTierMediaTypeDisk: FILE_STORAGE_TIER_MEDIA_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileStorageTierMediaTypeSsd: FILE_STORAGE_TIER_MEDIA_TYPE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileStorageTierMediaTypeScm: FILE_STORAGE_TIER_MEDIA_TYPE = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileStorageTierMediaTypeMax: FILE_STORAGE_TIER_MEDIA_TYPE = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FSRTL_CHANGE_BACKING_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ChangeDataControlArea: FSRTL_CHANGE_BACKING_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ChangeImageControlArea: FSRTL_CHANGE_BACKING_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ChangeSharedCacheMap: FSRTL_CHANGE_BACKING_TYPE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FSRTL_COMPARISON_RESULT = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LessThan: FSRTL_COMPARISON_RESULT = -1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const EqualTo: FSRTL_COMPARISON_RESULT = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const GreaterThan: FSRTL_COMPARISON_RESULT = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FS_BPIO_INFLAGS = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSBPIO_INFL_None: FS_BPIO_INFLAGS = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSBPIO_INFL_SKIP_STORAGE_STACK_QUERY: FS_BPIO_INFLAGS = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FS_BPIO_OPERATIONS = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_BPIO_OP_ENABLE: FS_BPIO_OPERATIONS = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_BPIO_OP_DISABLE: FS_BPIO_OPERATIONS = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_BPIO_OP_QUERY: FS_BPIO_OPERATIONS = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_BPIO_OP_VOLUME_STACK_PAUSE: FS_BPIO_OPERATIONS = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_BPIO_OP_VOLUME_STACK_RESUME: FS_BPIO_OPERATIONS = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_BPIO_OP_STREAM_PAUSE: FS_BPIO_OPERATIONS = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_BPIO_OP_STREAM_RESUME: FS_BPIO_OPERATIONS = 7i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_BPIO_OP_GET_INFO: FS_BPIO_OPERATIONS = 8i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FS_BPIO_OP_MAX_OPERATION: FS_BPIO_OPERATIONS = 9i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FS_BPIO_OUTFLAGS = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSBPIO_OUTFL_None: FS_BPIO_OUTFLAGS = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSBPIO_OUTFL_VOLUME_STACK_BYPASS_PAUSED: FS_BPIO_OUTFLAGS = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSBPIO_OUTFL_STREAM_BYPASS_PAUSED: FS_BPIO_OUTFLAGS = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSBPIO_OUTFL_FILTER_ATTACH_BLOCKED: FS_BPIO_OUTFLAGS = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FSBPIO_OUTFL_COMPATIBLE_STORAGE_DRIVER: FS_BPIO_OUTFLAGS = 8i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FS_FILTER_SECTION_SYNC_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SyncTypeOther: FS_FILTER_SECTION_SYNC_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SyncTypeCreateSection: FS_FILTER_SECTION_SYNC_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FS_FILTER_STREAM_FO_NOTIFICATION_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NotifyTypeCreate: FS_FILTER_STREAM_FO_NOTIFICATION_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NotifyTypeRetired: FS_FILTER_STREAM_FO_NOTIFICATION_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FS_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsVolumeInformation: FS_INFORMATION_CLASS = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsLabelInformation: FS_INFORMATION_CLASS = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsSizeInformation: FS_INFORMATION_CLASS = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsDeviceInformation: FS_INFORMATION_CLASS = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsAttributeInformation: FS_INFORMATION_CLASS = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsControlInformation: FS_INFORMATION_CLASS = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsFullSizeInformation: FS_INFORMATION_CLASS = 7i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsObjectIdInformation: FS_INFORMATION_CLASS = 8i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsDriverPathInformation: FS_INFORMATION_CLASS = 9i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsVolumeFlagsInformation: FS_INFORMATION_CLASS = 10i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsSectorSizeInformation: FS_INFORMATION_CLASS = 11i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsDataCopyInformation: FS_INFORMATION_CLASS = 12i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsMetadataSizeInformation: FS_INFORMATION_CLASS = 13i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsFullSizeInformationEx: FS_INFORMATION_CLASS = 14i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FileFsMaximumInformation: FS_INFORMATION_CLASS = 15i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type HEAP_MEMORY_INFO_CLASS = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const HeapMemoryBasicInformation: HEAP_MEMORY_INFO_CLASS = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type LINK_TRACKING_INFORMATION_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NtfsLinkTrackingInformation: LINK_TRACKING_INFORMATION_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DfsLinkTrackingInformation: LINK_TRACKING_INFORMATION_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type LMR_QUERY_INFO_CLASS = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LMRQuerySessionInfo: LMR_QUERY_INFO_CLASS = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type MANDATORY_LEVEL = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MandatoryLevelUntrusted: MANDATORY_LEVEL = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MandatoryLevelLow: MANDATORY_LEVEL = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MandatoryLevelMedium: MANDATORY_LEVEL = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MandatoryLevelHigh: MANDATORY_LEVEL = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MandatoryLevelSystem: MANDATORY_LEVEL = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MandatoryLevelSecureProcess: MANDATORY_LEVEL = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MandatoryLevelCount: MANDATORY_LEVEL = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type MEMORY_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MemoryBasicInformation: MEMORY_INFORMATION_CLASS = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type MMFLUSH_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MmFlushForDelete: MMFLUSH_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MmFlushForWrite: MMFLUSH_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type MSV1_0_AVID = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsvAvEOL: MSV1_0_AVID = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsvAvNbComputerName: MSV1_0_AVID = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsvAvNbDomainName: MSV1_0_AVID = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsvAvDnsComputerName: MSV1_0_AVID = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsvAvDnsDomainName: MSV1_0_AVID = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsvAvDnsTreeName: MSV1_0_AVID = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsvAvFlags: MSV1_0_AVID = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsvAvTimestamp: MSV1_0_AVID = 7i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsvAvRestrictions: MSV1_0_AVID = 8i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsvAvTargetName: MSV1_0_AVID = 9i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsvAvChannelBindings: MSV1_0_AVID = 10i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type MSV1_0_CREDENTIAL_KEY_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const InvalidCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DeprecatedIUMCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const DomainUserCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LocalUserCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ExternallySuppliedCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type MSV1_0_LOGON_SUBMIT_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0InteractiveLogon: MSV1_0_LOGON_SUBMIT_TYPE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0Lm20Logon: MSV1_0_LOGON_SUBMIT_TYPE = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0NetworkLogon: MSV1_0_LOGON_SUBMIT_TYPE = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0SubAuthLogon: MSV1_0_LOGON_SUBMIT_TYPE = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0WorkstationUnlockLogon: MSV1_0_LOGON_SUBMIT_TYPE = 7i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0S4ULogon: MSV1_0_LOGON_SUBMIT_TYPE = 12i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0VirtualLogon: MSV1_0_LOGON_SUBMIT_TYPE = 82i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0NoElevationLogon: MSV1_0_LOGON_SUBMIT_TYPE = 83i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0LuidLogon: MSV1_0_LOGON_SUBMIT_TYPE = 84i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type MSV1_0_PROFILE_BUFFER_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0InteractiveProfile: MSV1_0_PROFILE_BUFFER_TYPE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0Lm20LogonProfile: MSV1_0_PROFILE_BUFFER_TYPE = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0SmartCardProfile: MSV1_0_PROFILE_BUFFER_TYPE = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type MSV1_0_PROTOCOL_MESSAGE_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0Lm20ChallengeRequest: MSV1_0_PROTOCOL_MESSAGE_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0Lm20GetChallengeResponse: MSV1_0_PROTOCOL_MESSAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0EnumerateUsers: MSV1_0_PROTOCOL_MESSAGE_TYPE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0GetUserInfo: MSV1_0_PROTOCOL_MESSAGE_TYPE = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0ReLogonUsers: MSV1_0_PROTOCOL_MESSAGE_TYPE = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0ChangePassword: MSV1_0_PROTOCOL_MESSAGE_TYPE = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0ChangeCachedPassword: MSV1_0_PROTOCOL_MESSAGE_TYPE = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0GenericPassthrough: MSV1_0_PROTOCOL_MESSAGE_TYPE = 7i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0CacheLogon: MSV1_0_PROTOCOL_MESSAGE_TYPE = 8i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0SubAuth: MSV1_0_PROTOCOL_MESSAGE_TYPE = 9i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0DeriveCredential: MSV1_0_PROTOCOL_MESSAGE_TYPE = 10i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0CacheLookup: MSV1_0_PROTOCOL_MESSAGE_TYPE = 11i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0SetProcessOption: MSV1_0_PROTOCOL_MESSAGE_TYPE = 12i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0ConfigLocalAliases: MSV1_0_PROTOCOL_MESSAGE_TYPE = 13i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0ClearCachedCredentials: MSV1_0_PROTOCOL_MESSAGE_TYPE = 14i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0LookupToken: MSV1_0_PROTOCOL_MESSAGE_TYPE = 15i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0ValidateAuth: MSV1_0_PROTOCOL_MESSAGE_TYPE = 16i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0CacheLookupEx: MSV1_0_PROTOCOL_MESSAGE_TYPE = 17i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0GetCredentialKey: MSV1_0_PROTOCOL_MESSAGE_TYPE = 18i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0SetThreadOption: MSV1_0_PROTOCOL_MESSAGE_TYPE = 19i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0DecryptDpapiMasterKey: MSV1_0_PROTOCOL_MESSAGE_TYPE = 20i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0GetStrongCredentialKey: MSV1_0_PROTOCOL_MESSAGE_TYPE = 21i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0TransferCred: MSV1_0_PROTOCOL_MESSAGE_TYPE = 22i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0ProvisionTbal: MSV1_0_PROTOCOL_MESSAGE_TYPE = 23i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MsV1_0DeleteTbalSecrets: MSV1_0_PROTOCOL_MESSAGE_TYPE = 24i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type NETWORK_OPEN_INTEGRITY_QUALIFIER = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NetworkOpenIntegrityAny: NETWORK_OPEN_INTEGRITY_QUALIFIER = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NetworkOpenIntegrityNone: NETWORK_OPEN_INTEGRITY_QUALIFIER = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NetworkOpenIntegritySigned: NETWORK_OPEN_INTEGRITY_QUALIFIER = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NetworkOpenIntegrityEncrypted: NETWORK_OPEN_INTEGRITY_QUALIFIER = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NetworkOpenIntegrityMaximum: NETWORK_OPEN_INTEGRITY_QUALIFIER = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type NETWORK_OPEN_LOCATION_QUALIFIER = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NetworkOpenLocationAny: NETWORK_OPEN_LOCATION_QUALIFIER = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NetworkOpenLocationRemote: NETWORK_OPEN_LOCATION_QUALIFIER = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NetworkOpenLocationLoopback: NETWORK_OPEN_LOCATION_QUALIFIER = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type NTCREATEFILE_CREATE_DISPOSITION = u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SUPERSEDE: NTCREATEFILE_CREATE_DISPOSITION = 0u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_CREATE: NTCREATEFILE_CREATE_DISPOSITION = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_OPEN: NTCREATEFILE_CREATE_DISPOSITION = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_OPEN_IF: NTCREATEFILE_CREATE_DISPOSITION = 3u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_OVERWRITE: NTCREATEFILE_CREATE_DISPOSITION = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_OVERWRITE_IF: NTCREATEFILE_CREATE_DISPOSITION = 5u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type NTCREATEFILE_CREATE_OPTIONS = u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DIRECTORY_FILE: NTCREATEFILE_CREATE_OPTIONS = 1u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NON_DIRECTORY_FILE: NTCREATEFILE_CREATE_OPTIONS = 64u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_WRITE_THROUGH: NTCREATEFILE_CREATE_OPTIONS = 2u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SEQUENTIAL_ONLY: NTCREATEFILE_CREATE_OPTIONS = 4u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_RANDOM_ACCESS: NTCREATEFILE_CREATE_OPTIONS = 2048u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NO_INTERMEDIATE_BUFFERING: NTCREATEFILE_CREATE_OPTIONS = 8u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SYNCHRONOUS_IO_ALERT: NTCREATEFILE_CREATE_OPTIONS = 16u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SYNCHRONOUS_IO_NONALERT: NTCREATEFILE_CREATE_OPTIONS = 32u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_CREATE_TREE_CONNECTION: NTCREATEFILE_CREATE_OPTIONS = 128u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NO_EA_KNOWLEDGE: NTCREATEFILE_CREATE_OPTIONS = 512u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_OPEN_REPARSE_POINT: NTCREATEFILE_CREATE_OPTIONS = 2097152u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DELETE_ON_CLOSE: NTCREATEFILE_CREATE_OPTIONS = 4096u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_OPEN_BY_FILE_ID: NTCREATEFILE_CREATE_OPTIONS = 8192u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_OPEN_FOR_BACKUP_INTENT: NTCREATEFILE_CREATE_OPTIONS = 16384u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_RESERVE_OPFILTER: NTCREATEFILE_CREATE_OPTIONS = 1048576u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_OPEN_REQUIRING_OPLOCK: NTCREATEFILE_CREATE_OPTIONS = 65536u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_COMPLETE_IF_OPLOCKED: NTCREATEFILE_CREATE_OPTIONS = 256u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_OPEN_FOR_FREE_SPACE_QUERY: NTCREATEFILE_CREATE_OPTIONS = 8388608u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_CONTAINS_EXTENDED_CREATE_INFORMATION: NTCREATEFILE_CREATE_OPTIONS = 268435456u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_NO_COMPRESSION: NTCREATEFILE_CREATE_OPTIONS = 32768u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_DISALLOW_EXCLUSIVE: NTCREATEFILE_CREATE_OPTIONS = 131072u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_SESSION_AWARE: NTCREATEFILE_CREATE_OPTIONS = 262144u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const FILE_OPEN_NO_RECALL: NTCREATEFILE_CREATE_OPTIONS = 4194304u32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type OBJECT_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ObjectBasicInformation: OBJECT_INFORMATION_CLASS = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ObjectTypeInformation: OBJECT_INFORMATION_CLASS = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type OPLOCK_NOTIFY_REASON = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_NOTIFY_BREAK_WAIT_INTERIM_TIMEOUT: OPLOCK_NOTIFY_REASON = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const OPLOCK_NOTIFY_BREAK_WAIT_TERMINATED: OPLOCK_NOTIFY_REASON = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type QUERY_FILE_LAYOUT_FILTER_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_NONE: QUERY_FILE_LAYOUT_FILTER_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_CLUSTERS: QUERY_FILE_LAYOUT_FILTER_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_FILEID: QUERY_FILE_LAYOUT_FILTER_TYPE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_STORAGE_RESERVE_ID: QUERY_FILE_LAYOUT_FILTER_TYPE = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const QUERY_FILE_LAYOUT_NUM_FILTER_TYPES: QUERY_FILE_LAYOUT_FILTER_TYPE = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type REFS_COMPRESSION_FORMATS = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_COMPRESSION_FORMAT_UNCOMPRESSED: REFS_COMPRESSION_FORMATS = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_COMPRESSION_FORMAT_LZ4: REFS_COMPRESSION_FORMATS = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_COMPRESSION_FORMAT_ZSTD: REFS_COMPRESSION_FORMATS = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_COMPRESSION_FORMAT_MAX: REFS_COMPRESSION_FORMATS = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type REFS_DEALLOCATE_RANGES_ALLOCATOR = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_DEALLOCATE_RANGES_ALLOCATOR_NONE: REFS_DEALLOCATE_RANGES_ALLOCATOR = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_DEALLOCATE_RANGES_ALLOCATOR_SAA: REFS_DEALLOCATE_RANGES_ALLOCATOR = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_DEALLOCATE_RANGES_ALLOCATOR_CAA: REFS_DEALLOCATE_RANGES_ALLOCATOR = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_DEALLOCATE_RANGES_ALLOCATOR_MAA: REFS_DEALLOCATE_RANGES_ALLOCATOR = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_COMPRESS_SYNC: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_MAX: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type REFS_SMR_VOLUME_GC_ACTION = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SmrGcActionStart: REFS_SMR_VOLUME_GC_ACTION = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SmrGcActionStartFullSpeed: REFS_SMR_VOLUME_GC_ACTION = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SmrGcActionPause: REFS_SMR_VOLUME_GC_ACTION = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SmrGcActionStop: REFS_SMR_VOLUME_GC_ACTION = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type REFS_SMR_VOLUME_GC_METHOD = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SmrGcMethodCompaction: REFS_SMR_VOLUME_GC_METHOD = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SmrGcMethodCompression: REFS_SMR_VOLUME_GC_METHOD = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SmrGcMethodRotation: REFS_SMR_VOLUME_GC_METHOD = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type REFS_SMR_VOLUME_GC_STATE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SmrGcStateInactive: REFS_SMR_VOLUME_GC_STATE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SmrGcStatePaused: REFS_SMR_VOLUME_GC_STATE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SmrGcStateActive: REFS_SMR_VOLUME_GC_STATE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SmrGcStateActiveFullSpeed: REFS_SMR_VOLUME_GC_STATE = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type REFS_STREAM_SNAPSHOT_OPERATION = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_SNAPSHOT_OPERATION_INVALID: REFS_STREAM_SNAPSHOT_OPERATION = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_SNAPSHOT_OPERATION_CREATE: REFS_STREAM_SNAPSHOT_OPERATION = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_SNAPSHOT_OPERATION_LIST: REFS_STREAM_SNAPSHOT_OPERATION = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_SNAPSHOT_OPERATION_QUERY_DELTAS: REFS_STREAM_SNAPSHOT_OPERATION = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_SNAPSHOT_OPERATION_REVERT: REFS_STREAM_SNAPSHOT_OPERATION = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_SNAPSHOT_OPERATION_SET_SHADOW_BTREE: REFS_STREAM_SNAPSHOT_OPERATION = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_SNAPSHOT_OPERATION_CLEAR_SHADOW_BTREE: REFS_STREAM_SNAPSHOT_OPERATION = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_SNAPSHOT_OPERATION_MAX: REFS_STREAM_SNAPSHOT_OPERATION = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type RTL_MEMORY_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MemoryTypePaged: RTL_MEMORY_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MemoryTypeNonPaged: RTL_MEMORY_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MemoryType64KPage: RTL_MEMORY_TYPE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MemoryTypeLargePage: RTL_MEMORY_TYPE = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MemoryTypeHugePage: RTL_MEMORY_TYPE = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MemoryTypeCustom: RTL_MEMORY_TYPE = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MemoryTypeMax: RTL_MEMORY_TYPE = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type SECPKG_CRED_CLASS = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SecPkgCredClass_None: SECPKG_CRED_CLASS = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SecPkgCredClass_Ephemeral: SECPKG_CRED_CLASS = 10i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SecPkgCredClass_PersistedGeneric: SECPKG_CRED_CLASS = 20i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SecPkgCredClass_PersistedSpecific: SECPKG_CRED_CLASS = 30i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SecPkgCredClass_Explicit: SECPKG_CRED_CLASS = 40i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type SECURITY_LOGON_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const UndefinedLogonType: SECURITY_LOGON_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const Interactive: SECURITY_LOGON_TYPE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const Network: SECURITY_LOGON_TYPE = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const Batch: SECURITY_LOGON_TYPE = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const Service: SECURITY_LOGON_TYPE = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const Proxy: SECURITY_LOGON_TYPE = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const Unlock: SECURITY_LOGON_TYPE = 7i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NetworkCleartext: SECURITY_LOGON_TYPE = 8i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const NewCredentials: SECURITY_LOGON_TYPE = 9i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const RemoteInteractive: SECURITY_LOGON_TYPE = 10i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CachedInteractive: SECURITY_LOGON_TYPE = 11i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CachedRemoteInteractive: SECURITY_LOGON_TYPE = 12i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const CachedUnlock: SECURITY_LOGON_TYPE = 13i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SecApplicationProtocolNegotiationExt_None: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SecApplicationProtocolNegotiationExt_NPN: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SecApplicationProtocolNegotiationExt_ALPN: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type SEC_TRAFFIC_SECRET_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SecTrafficSecret_None: SEC_TRAFFIC_SECRET_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SecTrafficSecret_Client: SEC_TRAFFIC_SECRET_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SecTrafficSecret_Server: SEC_TRAFFIC_SECRET_TYPE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type SE_AUDIT_OPERATION = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditPrivilegeObject: SE_AUDIT_OPERATION = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditPrivilegeService: SE_AUDIT_OPERATION = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditAccessCheck: SE_AUDIT_OPERATION = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditOpenObject: SE_AUDIT_OPERATION = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditOpenObjectWithTransaction: SE_AUDIT_OPERATION = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditCloseObject: SE_AUDIT_OPERATION = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditDeleteObject: SE_AUDIT_OPERATION = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditOpenObjectForDelete: SE_AUDIT_OPERATION = 7i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditOpenObjectForDeleteWithTransaction: SE_AUDIT_OPERATION = 8i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditCloseNonObject: SE_AUDIT_OPERATION = 9i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditOpenNonObject: SE_AUDIT_OPERATION = 10i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditObjectReference: SE_AUDIT_OPERATION = 11i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const AuditHandleCreation: SE_AUDIT_OPERATION = 12i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type SHRINK_VOLUME_REQUEST_TYPES = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ShrinkPrepare: SHRINK_VOLUME_REQUEST_TYPES = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ShrinkCommit: SHRINK_VOLUME_REQUEST_TYPES = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const ShrinkAbort: SHRINK_VOLUME_REQUEST_TYPES = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type SID_NAME_USE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SidTypeUser: SID_NAME_USE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SidTypeGroup: SID_NAME_USE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SidTypeDomain: SID_NAME_USE = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SidTypeAlias: SID_NAME_USE = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SidTypeWellKnownGroup: SID_NAME_USE = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SidTypeDeletedAccount: SID_NAME_USE = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SidTypeInvalid: SID_NAME_USE = 7i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SidTypeUnknown: SID_NAME_USE = 8i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SidTypeComputer: SID_NAME_USE = 9i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SidTypeLabel: SID_NAME_USE = 10i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SidTypeLogonSession: SID_NAME_USE = 11i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type SRV_INSTANCE_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SrvInstanceTypeUndefined: SRV_INSTANCE_TYPE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SrvInstanceTypePrimary: SRV_INSTANCE_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SrvInstanceTypeCsv: SRV_INSTANCE_TYPE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SrvInstanceTypeSBL: SRV_INSTANCE_TYPE = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SrvInstanceTypeSR: SRV_INSTANCE_TYPE = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SrvInstanceTypeVSMB: SRV_INSTANCE_TYPE = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type STORAGE_RESERVE_ID = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const StorageReserveIdNone: STORAGE_RESERVE_ID = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const StorageReserveIdHard: STORAGE_RESERVE_ID = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const StorageReserveIdSoft: STORAGE_RESERVE_ID = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const StorageReserveIdUpdateScratch: STORAGE_RESERVE_ID = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const StorageReserveIdMax: STORAGE_RESERVE_ID = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type SharedVirtualDiskHandleState = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SharedVirtualDiskHandleStateNone: SharedVirtualDiskHandleState = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SharedVirtualDiskHandleStateFileShared: SharedVirtualDiskHandleState = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SharedVirtualDiskHandleStateHandleShared: SharedVirtualDiskHandleState = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type SharedVirtualDiskSupportType = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SharedVirtualDisksUnsupported: SharedVirtualDiskSupportType = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SharedVirtualDisksSupported: SharedVirtualDiskSupportType = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SharedVirtualDiskSnapshotsSupported: SharedVirtualDiskSupportType = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const SharedVirtualDiskCDPSnapshotsSupported: SharedVirtualDiskSupportType = 7i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type TOKEN_ELEVATION_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenElevationTypeDefault: TOKEN_ELEVATION_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenElevationTypeFull: TOKEN_ELEVATION_TYPE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenElevationTypeLimited: TOKEN_ELEVATION_TYPE = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type TOKEN_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenUser: TOKEN_INFORMATION_CLASS = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenGroups: TOKEN_INFORMATION_CLASS = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenPrivileges: TOKEN_INFORMATION_CLASS = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenOwner: TOKEN_INFORMATION_CLASS = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenPrimaryGroup: TOKEN_INFORMATION_CLASS = 5i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenDefaultDacl: TOKEN_INFORMATION_CLASS = 6i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenSource: TOKEN_INFORMATION_CLASS = 7i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenType: TOKEN_INFORMATION_CLASS = 8i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenImpersonationLevel: TOKEN_INFORMATION_CLASS = 9i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenStatistics: TOKEN_INFORMATION_CLASS = 10i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenRestrictedSids: TOKEN_INFORMATION_CLASS = 11i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenSessionId: TOKEN_INFORMATION_CLASS = 12i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenGroupsAndPrivileges: TOKEN_INFORMATION_CLASS = 13i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenSessionReference: TOKEN_INFORMATION_CLASS = 14i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenSandBoxInert: TOKEN_INFORMATION_CLASS = 15i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenAuditPolicy: TOKEN_INFORMATION_CLASS = 16i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenOrigin: TOKEN_INFORMATION_CLASS = 17i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenElevationType: TOKEN_INFORMATION_CLASS = 18i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenLinkedToken: TOKEN_INFORMATION_CLASS = 19i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenElevation: TOKEN_INFORMATION_CLASS = 20i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenHasRestrictions: TOKEN_INFORMATION_CLASS = 21i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenAccessInformation: TOKEN_INFORMATION_CLASS = 22i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenVirtualizationAllowed: TOKEN_INFORMATION_CLASS = 23i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenVirtualizationEnabled: TOKEN_INFORMATION_CLASS = 24i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenIntegrityLevel: TOKEN_INFORMATION_CLASS = 25i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenUIAccess: TOKEN_INFORMATION_CLASS = 26i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenMandatoryPolicy: TOKEN_INFORMATION_CLASS = 27i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenLogonSid: TOKEN_INFORMATION_CLASS = 28i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenIsAppContainer: TOKEN_INFORMATION_CLASS = 29i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenCapabilities: TOKEN_INFORMATION_CLASS = 30i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenAppContainerSid: TOKEN_INFORMATION_CLASS = 31i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenAppContainerNumber: TOKEN_INFORMATION_CLASS = 32i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenUserClaimAttributes: TOKEN_INFORMATION_CLASS = 33i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenDeviceClaimAttributes: TOKEN_INFORMATION_CLASS = 34i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenRestrictedUserClaimAttributes: TOKEN_INFORMATION_CLASS = 35i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenRestrictedDeviceClaimAttributes: TOKEN_INFORMATION_CLASS = 36i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenDeviceGroups: TOKEN_INFORMATION_CLASS = 37i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenRestrictedDeviceGroups: TOKEN_INFORMATION_CLASS = 38i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenSecurityAttributes: TOKEN_INFORMATION_CLASS = 39i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenIsRestricted: TOKEN_INFORMATION_CLASS = 40i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenProcessTrustLevel: TOKEN_INFORMATION_CLASS = 41i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenPrivateNameSpace: TOKEN_INFORMATION_CLASS = 42i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenSingletonAttributes: TOKEN_INFORMATION_CLASS = 43i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenBnoIsolation: TOKEN_INFORMATION_CLASS = 44i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenChildProcessFlags: TOKEN_INFORMATION_CLASS = 45i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenIsLessPrivilegedAppContainer: TOKEN_INFORMATION_CLASS = 46i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenIsSandboxed: TOKEN_INFORMATION_CLASS = 47i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenIsAppSilo: TOKEN_INFORMATION_CLASS = 48i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const MaxTokenInfoClass: TOKEN_INFORMATION_CLASS = 49i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type TOKEN_TYPE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenPrimary: TOKEN_TYPE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const TokenImpersonation: TOKEN_TYPE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type VIRTUAL_MEMORY_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const VmPrefetchInformation: VIRTUAL_MEMORY_INFORMATION_CLASS = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type VIRTUAL_STORAGE_BEHAVIOR_CODE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const VirtualStorageBehaviorUndefined: VIRTUAL_STORAGE_BEHAVIOR_CODE = 0i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const VirtualStorageBehaviorCacheWriteThrough: VIRTUAL_STORAGE_BEHAVIOR_CODE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const VirtualStorageBehaviorCacheWriteBack: VIRTUAL_STORAGE_BEHAVIOR_CODE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const VirtualStorageBehaviorStopIoProcessing: VIRTUAL_STORAGE_BEHAVIOR_CODE = 3i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const VirtualStorageBehaviorRestartIoProcessing: VIRTUAL_STORAGE_BEHAVIOR_CODE = 4i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type _LCN_WEAK_REFERENCE_STATE = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LCN_WEAK_REFERENCE_VALID: _LCN_WEAK_REFERENCE_STATE = 1i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const LCN_CHECKSUM_VALID: _LCN_WEAK_REFERENCE_STATE = 2i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type _REFS_STREAM_EXTENT_PROPERTIES = i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_EXTENT_PROPERTY_VALID: _REFS_STREAM_EXTENT_PROPERTIES = 16i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_EXTENT_PROPERTY_STREAM_RESERVED: _REFS_STREAM_EXTENT_PROPERTIES = 32i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_EXTENT_PROPERTY_CRC32: _REFS_STREAM_EXTENT_PROPERTIES = 128i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_EXTENT_PROPERTY_CRC64: _REFS_STREAM_EXTENT_PROPERTIES = 256i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_EXTENT_PROPERTY_GHOSTED: _REFS_STREAM_EXTENT_PROPERTIES = 512i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_EXTENT_PROPERTY_READONLY: _REFS_STREAM_EXTENT_PROPERTIES = 1024i32;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub const REFS_STREAM_EXTENT_PROPERTY_SPARSE: _REFS_STREAM_EXTENT_PROPERTIES = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct ACCESS_ALLOWED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_ALLOWED_ACE {}
impl ::core::clone::Clone for ACCESS_ALLOWED_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct ACCESS_DENIED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_DENIED_ACE {}
impl ::core::clone::Clone for ACCESS_DENIED_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct ACCESS_REASONS {
    pub Data: [u32; 32],
}
impl ::core::marker::Copy for ACCESS_REASONS {}
impl ::core::clone::Clone for ACCESS_REASONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct ACE_HEADER {
    pub AceType: u8,
    pub AceFlags: u8,
    pub AceSize: u16,
}
impl ::core::marker::Copy for ACE_HEADER {}
impl ::core::clone::Clone for ACE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct ASYNC_DUPLICATE_EXTENTS_STATUS {
    pub Version: u32,
    pub State: DUPLICATE_EXTENTS_STATE,
    pub SourceFileOffset: u64,
    pub TargetFileOffset: u64,
    pub ByteCount: u64,
    pub BytesDuplicated: u64,
}
impl ::core::marker::Copy for ASYNC_DUPLICATE_EXTENTS_STATUS {}
impl ::core::clone::Clone for ASYNC_DUPLICATE_EXTENTS_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct ATOMIC_CREATE_ECP_CONTEXT {
    pub Size: u16,
    pub InFlags: u16,
    pub OutFlags: u16,
    pub ReparseBufferLength: u16,
    pub ReparseBuffer: *mut REPARSE_DATA_BUFFER,
    pub FileSize: i64,
    pub ValidDataLength: i64,
    pub FileTimestamps: *mut FILE_TIMESTAMPS,
    pub FileAttributes: u32,
    pub UsnSourceInfo: u32,
    pub Usn: i64,
    pub SuppressFileAttributeInheritanceMask: u32,
    pub InOpFlags: u32,
    pub OutOpFlags: u32,
    pub InGenFlags: u32,
    pub OutGenFlags: u32,
    pub CaseSensitiveFlagsMask: u32,
    pub InCaseSensitiveFlags: u32,
    pub OutCaseSensitiveFlags: u32,
}
impl ::core::marker::Copy for ATOMIC_CREATE_ECP_CONTEXT {}
impl ::core::clone::Clone for ATOMIC_CREATE_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct BASE_MCB {
    pub MaximumPairCount: u32,
    pub PairCount: u32,
    pub PoolType: u16,
    pub Flags: u16,
    pub Mapping: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for BASE_MCB {}
impl ::core::clone::Clone for BASE_MCB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct BOOT_AREA_INFO {
    pub BootSectorCount: u32,
    pub BootSectors: [BOOT_AREA_INFO_0; 2],
}
impl ::core::marker::Copy for BOOT_AREA_INFO {}
impl ::core::clone::Clone for BOOT_AREA_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct BOOT_AREA_INFO_0 {
    pub Offset: i64,
}
impl ::core::marker::Copy for BOOT_AREA_INFO_0 {}
impl ::core::clone::Clone for BOOT_AREA_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct BULK_SECURITY_TEST_DATA {
    pub DesiredAccess: u32,
    pub SecurityIds: [u32; 1],
}
impl ::core::marker::Copy for BULK_SECURITY_TEST_DATA {}
impl ::core::clone::Clone for BULK_SECURITY_TEST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CACHE_MANAGER_CALLBACKS {
    pub AcquireForLazyWrite: PACQUIRE_FOR_LAZY_WRITE,
    pub ReleaseFromLazyWrite: PRELEASE_FROM_LAZY_WRITE,
    pub AcquireForReadAhead: PACQUIRE_FOR_READ_AHEAD,
    pub ReleaseFromReadAhead: PRELEASE_FROM_READ_AHEAD,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CACHE_MANAGER_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CACHE_MANAGER_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CACHE_MANAGER_CALLBACKS_EX {
    pub Version: u16,
    pub Size: u16,
    pub Functions: CACHE_MANAGER_CALLBACK_FUNCTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CACHE_MANAGER_CALLBACKS_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CACHE_MANAGER_CALLBACKS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CACHE_MANAGER_CALLBACK_FUNCTIONS {
    pub AcquireForLazyWriteEx: PACQUIRE_FOR_LAZY_WRITE_EX,
    pub ReleaseFromLazyWrite: PRELEASE_FROM_LAZY_WRITE,
    pub AcquireForReadAhead: PACQUIRE_FOR_READ_AHEAD,
    pub ReleaseFromReadAhead: PRELEASE_FROM_READ_AHEAD,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CACHE_MANAGER_CALLBACK_FUNCTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CACHE_MANAGER_CALLBACK_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct CACHE_UNINITIALIZE_EVENT {
    pub Next: *mut CACHE_UNINITIALIZE_EVENT,
    pub Event: super::super::Foundation::KEVENT,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for CACHE_UNINITIALIZE_EVENT {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for CACHE_UNINITIALIZE_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub struct CC_ASYNC_READ_CONTEXT {
    pub CompletionRoutine: PASYNC_READ_COMPLETION_CALLBACK,
    pub Context: *mut ::core::ffi::c_void,
    pub Mdl: *mut super::super::super::Win32::Graphics::DirectDraw::MDL,
    pub RequestorMode: i8,
    pub NestingLevel: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::core::marker::Copy for CC_ASYNC_READ_CONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::core::clone::Clone for CC_ASYNC_READ_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CC_ERROR_CALLBACK_CONTEXT {
    pub NodeByteSize: i16,
    pub ErrorCode: super::super::super::Win32::Foundation::NTSTATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CC_ERROR_CALLBACK_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CC_ERROR_CALLBACK_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CC_FILE_SIZES {
    pub AllocationSize: i64,
    pub FileSize: i64,
    pub ValidDataLength: i64,
}
impl ::core::marker::Copy for CC_FILE_SIZES {}
impl ::core::clone::Clone for CC_FILE_SIZES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: u16,
    pub Reserved: u16,
    pub AttributeCount: u32,
    pub Attribute: CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    pub pAttributeV1: *mut CLAIM_SECURITY_ATTRIBUTE_V1,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: u64,
    pub Name: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: *mut ::core::ffi::c_void,
    pub ValueLength: u32,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    pub Name: u32,
    pub ValueType: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub ValueCount: u32,
    pub Values: CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    pub pInt64: [u32; 1],
    pub pUint64: [u32; 1],
    pub ppString: [u32; 1],
    pub pFqbn: [u32; 1],
    pub pOctetString: [u32; 1],
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CLAIM_SECURITY_ATTRIBUTE_V1 {
    pub Name: ::windows_sys::core::PWSTR,
    pub ValueType: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub ValueCount: u32,
    pub Values: CLAIM_SECURITY_ATTRIBUTE_V1_0,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_V1 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    pub pInt64: *mut i64,
    pub pUint64: *mut u64,
    pub ppString: *mut ::windows_sys::core::PWSTR,
    pub pFqbn: *mut CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pub pOctetString: *mut CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_V1_0 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CLUSTER_RANGE {
    pub StartingCluster: i64,
    pub ClusterCount: i64,
}
impl ::core::marker::Copy for CLUSTER_RANGE {}
impl ::core::clone::Clone for CLUSTER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct COMPRESSED_DATA_INFO {
    pub CompressionFormatAndEngine: u16,
    pub CompressionUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub Reserved: u8,
    pub NumberOfChunks: u16,
    pub CompressedChunkSizes: [u32; 1],
}
impl ::core::marker::Copy for COMPRESSED_DATA_INFO {}
impl ::core::clone::Clone for COMPRESSED_DATA_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CONTAINER_ROOT_INFO_INPUT {
    pub Flags: u32,
}
impl ::core::marker::Copy for CONTAINER_ROOT_INFO_INPUT {}
impl ::core::clone::Clone for CONTAINER_ROOT_INFO_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CONTAINER_ROOT_INFO_OUTPUT {
    pub ContainerRootIdLength: u16,
    pub ContainerRootId: [u8; 1],
}
impl ::core::marker::Copy for CONTAINER_ROOT_INFO_OUTPUT {}
impl ::core::clone::Clone for CONTAINER_ROOT_INFO_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CONTAINER_VOLUME_STATE {
    pub Flags: u32,
}
impl ::core::marker::Copy for CONTAINER_VOLUME_STATE {}
impl ::core::clone::Clone for CONTAINER_VOLUME_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub struct COPY_INFORMATION {
    pub SourceFileObject: *mut super::super::Foundation::FILE_OBJECT,
    pub SourceFileOffset: i64,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for COPY_INFORMATION {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for COPY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CPTABLEINFO {
    pub CodePage: u16,
    pub MaximumCharacterSize: u16,
    pub DefaultChar: u16,
    pub UniDefaultChar: u16,
    pub TransDefaultChar: u16,
    pub TransUniDefaultChar: u16,
    pub DBCSCodePage: u16,
    pub LeadByte: [u8; 12],
    pub MultiByteTable: *mut u16,
    pub WideCharTable: *mut ::core::ffi::c_void,
    pub DBCSRanges: *mut u16,
    pub DBCSOffsets: *mut u16,
}
impl ::core::marker::Copy for CPTABLEINFO {}
impl ::core::clone::Clone for CPTABLEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CREATE_REDIRECTION_ECP_CONTEXT {
    pub Size: u16,
    pub Flags: u16,
    pub FileId: FILE_ID_128,
    pub VolumeGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CREATE_REDIRECTION_ECP_CONTEXT {}
impl ::core::clone::Clone for CREATE_REDIRECTION_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CREATE_USN_JOURNAL_DATA {
    pub MaximumSize: u64,
    pub AllocationDelta: u64,
}
impl ::core::marker::Copy for CREATE_USN_JOURNAL_DATA {}
impl ::core::clone::Clone for CREATE_USN_JOURNAL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CSV_CONTROL_PARAM {
    pub Operation: CSV_CONTROL_OP,
    pub Unused: i64,
}
impl ::core::marker::Copy for CSV_CONTROL_PARAM {}
impl ::core::clone::Clone for CSV_CONTROL_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CSV_DOWN_LEVEL_OPEN_ECP_CONTEXT {
    pub Version: u32,
    pub IsResume: super::super::super::Win32::Foundation::BOOLEAN,
    pub FileType: CSV_DOWN_LEVEL_FILE_TYPE,
    pub SourceNodeId: u32,
    pub DestinationNodeId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CSV_DOWN_LEVEL_OPEN_ECP_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CSV_DOWN_LEVEL_OPEN_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CSV_IS_OWNED_BY_CSVFS {
    pub OwnedByCSVFS: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CSV_IS_OWNED_BY_CSVFS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CSV_IS_OWNED_BY_CSVFS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CSV_MGMT_LOCK {
    pub Flags: u32,
}
impl ::core::marker::Copy for CSV_MGMT_LOCK {}
impl ::core::clone::Clone for CSV_MGMT_LOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CSV_NAMESPACE_INFO {
    pub Version: u32,
    pub DeviceNumber: u32,
    pub StartingOffset: i64,
    pub SectorSize: u32,
}
impl ::core::marker::Copy for CSV_NAMESPACE_INFO {}
impl ::core::clone::Clone for CSV_NAMESPACE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CSV_QUERY_FILE_REVISION {
    pub FileId: i64,
    pub FileRevision: [i64; 3],
}
impl ::core::marker::Copy for CSV_QUERY_FILE_REVISION {}
impl ::core::clone::Clone for CSV_QUERY_FILE_REVISION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CSV_QUERY_FILE_REVISION_ECP_CONTEXT {
    pub FileId: i64,
    pub FileRevision: [i64; 3],
}
impl ::core::marker::Copy for CSV_QUERY_FILE_REVISION_ECP_CONTEXT {}
impl ::core::clone::Clone for CSV_QUERY_FILE_REVISION_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CSV_QUERY_FILE_REVISION_ECP_CONTEXT_FILE_ID_128 {
    pub FileId: FILE_ID_128,
    pub FileRevision: [i64; 3],
}
impl ::core::marker::Copy for CSV_QUERY_FILE_REVISION_ECP_CONTEXT_FILE_ID_128 {}
impl ::core::clone::Clone for CSV_QUERY_FILE_REVISION_ECP_CONTEXT_FILE_ID_128 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    pub FileId: FILE_ID_128,
    pub FileRevision: [i64; 3],
}
impl ::core::marker::Copy for CSV_QUERY_FILE_REVISION_FILE_ID_128 {}
impl ::core::clone::Clone for CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CSV_QUERY_MDS_PATH {
    pub MdsNodeId: u32,
    pub DsNodeId: u32,
    pub PathLength: u32,
    pub Path: [u16; 1],
}
impl ::core::marker::Copy for CSV_QUERY_MDS_PATH {}
impl ::core::clone::Clone for CSV_QUERY_MDS_PATH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CSV_QUERY_MDS_PATH_V2 {
    pub Version: i64,
    pub RequiredSize: u32,
    pub MdsNodeId: u32,
    pub DsNodeId: u32,
    pub Flags: u32,
    pub DiskConnectivity: CSVFS_DISK_CONNECTIVITY,
    pub VolumeId: ::windows_sys::core::GUID,
    pub IpAddressOffset: u32,
    pub IpAddressLength: u32,
    pub PathOffset: u32,
    pub PathLength: u32,
}
impl ::core::marker::Copy for CSV_QUERY_MDS_PATH_V2 {}
impl ::core::clone::Clone for CSV_QUERY_MDS_PATH_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CSV_QUERY_REDIRECT_STATE {
    pub MdsNodeId: u32,
    pub DsNodeId: u32,
    pub FileRedirected: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CSV_QUERY_REDIRECT_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CSV_QUERY_REDIRECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    pub VetoedFromAltitudeIntegral: u64,
    pub VetoedFromAltitudeDecimal: u64,
    pub Reason: [u16; 256],
}
impl ::core::marker::Copy for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {}
impl ::core::clone::Clone for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CSV_QUERY_VOLUME_ID {
    pub VolumeId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CSV_QUERY_VOLUME_ID {}
impl ::core::clone::Clone for CSV_QUERY_VOLUME_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CSV_QUERY_VOLUME_REDIRECT_STATE {
    pub MdsNodeId: u32,
    pub DsNodeId: u32,
    pub IsDiskConnected: super::super::super::Win32::Foundation::BOOLEAN,
    pub ClusterEnableDirectIo: super::super::super::Win32::Foundation::BOOLEAN,
    pub DiskConnectivity: CSVFS_DISK_CONNECTIVITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CSV_QUERY_VOLUME_REDIRECT_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CSV_QUERY_VOLUME_REDIRECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT {
    pub Size: usize,
    pub PauseTimeoutInSeconds: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for CSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT {}
impl ::core::clone::Clone for CSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct CSV_SET_VOLUME_ID {
    pub VolumeId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CSV_SET_VOLUME_ID {}
impl ::core::clone::Clone for CSV_SET_VOLUME_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DECRYPTION_STATUS_BUFFER {
    pub NoEncryptedStreams: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DECRYPTION_STATUS_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DECRYPTION_STATUS_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct DELETE_USN_JOURNAL_DATA {
    pub UsnJournalID: u64,
    pub DeleteFlags: u32,
}
impl ::core::marker::Copy for DELETE_USN_JOURNAL_DATA {}
impl ::core::clone::Clone for DELETE_USN_JOURNAL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DUAL_OPLOCK_KEY_ECP_CONTEXT {
    pub ParentOplockKey: ::windows_sys::core::GUID,
    pub TargetOplockKey: ::windows_sys::core::GUID,
    pub ParentOplockKeySet: super::super::super::Win32::Foundation::BOOLEAN,
    pub TargetOplockKeySet: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DUAL_OPLOCK_KEY_ECP_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DUAL_OPLOCK_KEY_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct DUPLICATE_CLUSTER_DATA {
    pub SourceLcn: i64,
    pub TargetFileOffset: i64,
    pub DuplicationLimit: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for DUPLICATE_CLUSTER_DATA {}
impl ::core::clone::Clone for DUPLICATE_CLUSTER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DUPLICATE_EXTENTS_DATA {
    pub FileHandle: super::super::super::Win32::Foundation::HANDLE,
    pub SourceFileOffset: i64,
    pub TargetFileOffset: i64,
    pub ByteCount: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DUPLICATE_EXTENTS_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DUPLICATE_EXTENTS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DUPLICATE_EXTENTS_DATA_EX {
    pub Size: usize,
    pub FileHandle: super::super::super::Win32::Foundation::HANDLE,
    pub SourceFileOffset: i64,
    pub TargetFileOffset: i64,
    pub ByteCount: i64,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DUPLICATE_EXTENTS_DATA_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DUPLICATE_EXTENTS_DATA_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ECP_HEADER(pub u8);
impl ::core::marker::Copy for ECP_HEADER {}
impl ::core::clone::Clone for ECP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct ECP_OPEN_PARAMETERS {
    pub Size: u16,
    pub Reserved: u16,
    pub Flags: u32,
}
impl ::core::marker::Copy for ECP_OPEN_PARAMETERS {}
impl ::core::clone::Clone for ECP_OPEN_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct ENCRYPTED_DATA_INFO {
    pub StartingFileOffset: u64,
    pub OutputBufferOffset: u32,
    pub BytesWithinFileSize: u32,
    pub BytesWithinValidDataLength: u32,
    pub CompressionFormat: u16,
    pub DataUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub EncryptionFormat: u8,
    pub NumberOfDataBlocks: u16,
    pub DataBlockSize: [u32; 1],
}
impl ::core::marker::Copy for ENCRYPTED_DATA_INFO {}
impl ::core::clone::Clone for ENCRYPTED_DATA_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct ENCRYPTION_BUFFER {
    pub EncryptionOperation: u32,
    pub Private: [u8; 1],
}
impl ::core::marker::Copy for ENCRYPTION_BUFFER {}
impl ::core::clone::Clone for ENCRYPTION_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct ENCRYPTION_KEY_CTRL_INPUT {
    pub HeaderSize: u32,
    pub StructureSize: u32,
    pub KeyOffset: u16,
    pub KeySize: u16,
    pub DplLock: u32,
    pub DplUserId: u64,
    pub DplCredentialId: u64,
}
impl ::core::marker::Copy for ENCRYPTION_KEY_CTRL_INPUT {}
impl ::core::clone::Clone for ENCRYPTION_KEY_CTRL_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct EOF_WAIT_BLOCK {
    pub EofWaitLinks: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub Event: super::super::Foundation::KEVENT,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for EOF_WAIT_BLOCK {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for EOF_WAIT_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct EXFAT_STATISTICS {
    pub CreateHits: u32,
    pub SuccessfulCreates: u32,
    pub FailedCreates: u32,
    pub NonCachedReads: u32,
    pub NonCachedReadBytes: u32,
    pub NonCachedWrites: u32,
    pub NonCachedWriteBytes: u32,
    pub NonCachedDiskReads: u32,
    pub NonCachedDiskWrites: u32,
}
impl ::core::marker::Copy for EXFAT_STATISTICS {}
impl ::core::clone::Clone for EXFAT_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct EXTENDED_ENCRYPTED_DATA_INFO {
    pub ExtendedCode: u32,
    pub Length: u32,
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for EXTENDED_ENCRYPTED_DATA_INFO {}
impl ::core::clone::Clone for EXTENDED_ENCRYPTED_DATA_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct EXTENT_READ_CACHE_INFO_BUFFER {
    pub AllocatedCache: i64,
    pub PopulatedCache: i64,
    pub InErrorCache: i64,
}
impl ::core::marker::Copy for EXTENT_READ_CACHE_INFO_BUFFER {}
impl ::core::clone::Clone for EXTENT_READ_CACHE_INFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FAT_STATISTICS {
    pub CreateHits: u32,
    pub SuccessfulCreates: u32,
    pub FailedCreates: u32,
    pub NonCachedReads: u32,
    pub NonCachedReadBytes: u32,
    pub NonCachedWrites: u32,
    pub NonCachedWriteBytes: u32,
    pub NonCachedDiskReads: u32,
    pub NonCachedDiskWrites: u32,
}
impl ::core::marker::Copy for FAT_STATISTICS {}
impl ::core::clone::Clone for FAT_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILESYSTEM_STATISTICS {
    pub FileSystemType: u16,
    pub Version: u16,
    pub SizeOfCompleteStructure: u32,
    pub UserFileReads: u32,
    pub UserFileReadBytes: u32,
    pub UserDiskReads: u32,
    pub UserFileWrites: u32,
    pub UserFileWriteBytes: u32,
    pub UserDiskWrites: u32,
    pub MetaDataReads: u32,
    pub MetaDataReadBytes: u32,
    pub MetaDataDiskReads: u32,
    pub MetaDataWrites: u32,
    pub MetaDataWriteBytes: u32,
    pub MetaDataDiskWrites: u32,
}
impl ::core::marker::Copy for FILESYSTEM_STATISTICS {}
impl ::core::clone::Clone for FILESYSTEM_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILESYSTEM_STATISTICS_EX {
    pub FileSystemType: u16,
    pub Version: u16,
    pub SizeOfCompleteStructure: u32,
    pub UserFileReads: u64,
    pub UserFileReadBytes: u64,
    pub UserDiskReads: u64,
    pub UserFileWrites: u64,
    pub UserFileWriteBytes: u64,
    pub UserDiskWrites: u64,
    pub MetaDataReads: u64,
    pub MetaDataReadBytes: u64,
    pub MetaDataDiskReads: u64,
    pub MetaDataWrites: u64,
    pub MetaDataWriteBytes: u64,
    pub MetaDataDiskWrites: u64,
}
impl ::core::marker::Copy for FILESYSTEM_STATISTICS_EX {}
impl ::core::clone::Clone for FILESYSTEM_STATISTICS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_ACCESS_INFORMATION {
    pub AccessFlags: u32,
}
impl ::core::marker::Copy for FILE_ACCESS_INFORMATION {}
impl ::core::clone::Clone for FILE_ACCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_ALIGNMENT_INFORMATION {
    pub AlignmentRequirement: u32,
}
impl ::core::marker::Copy for FILE_ALIGNMENT_INFORMATION {}
impl ::core::clone::Clone for FILE_ALIGNMENT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_ALLOCATED_RANGE_BUFFER {
    pub FileOffset: i64,
    pub Length: i64,
}
impl ::core::marker::Copy for FILE_ALLOCATED_RANGE_BUFFER {}
impl ::core::clone::Clone for FILE_ALLOCATED_RANGE_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_ALLOCATION_INFORMATION {
    pub AllocationSize: i64,
}
impl ::core::marker::Copy for FILE_ALLOCATION_INFORMATION {}
impl ::core::clone::Clone for FILE_ALLOCATION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_ALL_INFORMATION {
    pub BasicInformation: FILE_BASIC_INFORMATION,
    pub StandardInformation: FILE_STANDARD_INFORMATION,
    pub InternalInformation: FILE_INTERNAL_INFORMATION,
    pub EaInformation: FILE_EA_INFORMATION,
    pub AccessInformation: FILE_ACCESS_INFORMATION,
    pub PositionInformation: FILE_POSITION_INFORMATION,
    pub ModeInformation: FILE_MODE_INFORMATION,
    pub AlignmentInformation: FILE_ALIGNMENT_INFORMATION,
    pub NameInformation: FILE_NAME_INFORMATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_ALL_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_ALL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_BASIC_INFORMATION {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
impl ::core::marker::Copy for FILE_BASIC_INFORMATION {}
impl ::core::clone::Clone for FILE_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: i8,
    pub ShortName: [u16; 12],
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_BOTH_DIR_INFORMATION {}
impl ::core::clone::Clone for FILE_BOTH_DIR_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_CASE_SENSITIVE_INFORMATION {
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_CASE_SENSITIVE_INFORMATION {}
impl ::core::clone::Clone for FILE_CASE_SENSITIVE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_COMPLETION_INFORMATION {
    pub Port: super::super::super::Win32::Foundation::HANDLE,
    pub Key: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_COMPLETION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_COMPLETION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_COMPRESSION_INFORMATION {
    pub CompressedFileSize: i64,
    pub CompressionFormat: u16,
    pub CompressionUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub Reserved: [u8; 3],
}
impl ::core::marker::Copy for FILE_COMPRESSION_INFORMATION {}
impl ::core::clone::Clone for FILE_COMPRESSION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_DESIRED_STORAGE_CLASS_INFORMATION {
    pub Class: FILE_STORAGE_TIER_CLASS,
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_DESIRED_STORAGE_CLASS_INFORMATION {}
impl ::core::clone::Clone for FILE_DESIRED_STORAGE_CLASS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_DIRECTORY_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_DIRECTORY_INFORMATION {}
impl ::core::clone::Clone for FILE_DIRECTORY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_DISPOSITION_INFORMATION {
    pub DeleteFile: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_DISPOSITION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_DISPOSITION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_DISPOSITION_INFORMATION_EX {
    pub Flags: FILE_DISPOSITION_INFORMATION_EX_FLAGS,
}
impl ::core::marker::Copy for FILE_DISPOSITION_INFORMATION_EX {}
impl ::core::clone::Clone for FILE_DISPOSITION_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_EA_INFORMATION {
    pub EaSize: u32,
}
impl ::core::marker::Copy for FILE_EA_INFORMATION {}
impl ::core::clone::Clone for FILE_EA_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_END_OF_FILE_INFORMATION_EX {
    pub EndOfFile: i64,
    pub PagingFileSizeInMM: i64,
    pub PagingFileMaxSize: i64,
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_END_OF_FILE_INFORMATION_EX {}
impl ::core::clone::Clone for FILE_END_OF_FILE_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_FS_ATTRIBUTE_INFORMATION {
    pub FileSystemAttributes: u32,
    pub MaximumComponentNameLength: i32,
    pub FileSystemNameLength: u32,
    pub FileSystemName: [u16; 1],
}
impl ::core::marker::Copy for FILE_FS_ATTRIBUTE_INFORMATION {}
impl ::core::clone::Clone for FILE_FS_ATTRIBUTE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_FS_CONTROL_INFORMATION {
    pub FreeSpaceStartFiltering: i64,
    pub FreeSpaceThreshold: i64,
    pub FreeSpaceStopFiltering: i64,
    pub DefaultQuotaThreshold: i64,
    pub DefaultQuotaLimit: i64,
    pub FileSystemControlFlags: u32,
}
impl ::core::marker::Copy for FILE_FS_CONTROL_INFORMATION {}
impl ::core::clone::Clone for FILE_FS_CONTROL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_FS_DATA_COPY_INFORMATION {
    pub NumberOfCopies: u32,
}
impl ::core::marker::Copy for FILE_FS_DATA_COPY_INFORMATION {}
impl ::core::clone::Clone for FILE_FS_DATA_COPY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_FS_DRIVER_PATH_INFORMATION {
    pub DriverInPath: super::super::super::Win32::Foundation::BOOLEAN,
    pub DriverNameLength: u32,
    pub DriverName: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_FS_DRIVER_PATH_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_FS_DRIVER_PATH_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_FS_PERSISTENT_VOLUME_INFORMATION {
    pub VolumeFlags: u32,
    pub FlagMask: u32,
    pub Version: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for FILE_FS_PERSISTENT_VOLUME_INFORMATION {}
impl ::core::clone::Clone for FILE_FS_PERSISTENT_VOLUME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_FS_SECTOR_SIZE_INFORMATION {
    pub LogicalBytesPerSector: u32,
    pub PhysicalBytesPerSectorForAtomicity: u32,
    pub PhysicalBytesPerSectorForPerformance: u32,
    pub FileSystemEffectivePhysicalBytesPerSectorForAtomicity: u32,
    pub Flags: u32,
    pub ByteOffsetForSectorAlignment: u32,
    pub ByteOffsetForPartitionAlignment: u32,
}
impl ::core::marker::Copy for FILE_FS_SECTOR_SIZE_INFORMATION {}
impl ::core::clone::Clone for FILE_FS_SECTOR_SIZE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_FS_VOLUME_FLAGS_INFORMATION {
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_FS_VOLUME_FLAGS_INFORMATION {}
impl ::core::clone::Clone for FILE_FS_VOLUME_FLAGS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_FULL_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_FULL_DIR_INFORMATION {}
impl ::core::clone::Clone for FILE_FULL_DIR_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_FULL_EA_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u8,
    pub EaNameLength: u8,
    pub EaValueLength: u16,
    pub EaName: [u8; 1],
}
impl ::core::marker::Copy for FILE_FULL_EA_INFORMATION {}
impl ::core::clone::Clone for FILE_FULL_EA_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_GET_EA_INFORMATION {
    pub NextEntryOffset: u32,
    pub EaNameLength: u8,
    pub EaName: [u8; 1],
}
impl ::core::marker::Copy for FILE_GET_EA_INFORMATION {}
impl ::core::clone::Clone for FILE_GET_EA_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct FILE_GET_QUOTA_INFORMATION {
    pub NextEntryOffset: u32,
    pub SidLength: u32,
    pub Sid: super::super::super::Win32::Security::SID,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for FILE_GET_QUOTA_INFORMATION {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for FILE_GET_QUOTA_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_ID_128 {
    pub Identifier: [u8; 16],
}
impl ::core::marker::Copy for FILE_ID_128 {}
impl ::core::clone::Clone for FILE_ID_128 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_ID_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: i8,
    pub ShortName: [u16; 12],
    pub FileId: i64,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_ID_BOTH_DIR_INFORMATION {}
impl ::core::clone::Clone for FILE_ID_BOTH_DIR_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_ID_EXTD_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: FILE_ID_128,
    pub ShortNameLength: i8,
    pub ShortName: [u16; 12],
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_ID_EXTD_BOTH_DIR_INFORMATION {}
impl ::core::clone::Clone for FILE_ID_EXTD_BOTH_DIR_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_ID_EXTD_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: FILE_ID_128,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_ID_EXTD_DIR_INFORMATION {}
impl ::core::clone::Clone for FILE_ID_EXTD_DIR_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_ID_FULL_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileId: i64,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_ID_FULL_DIR_INFORMATION {}
impl ::core::clone::Clone for FILE_ID_FULL_DIR_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_ID_GLOBAL_TX_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub FileId: i64,
    pub LockingTransactionId: ::windows_sys::core::GUID,
    pub TxInfoFlags: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_ID_GLOBAL_TX_DIR_INFORMATION {}
impl ::core::clone::Clone for FILE_ID_GLOBAL_TX_DIR_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_ID_INFORMATION {
    pub VolumeSerialNumber: u64,
    pub FileId: FILE_ID_128,
}
impl ::core::marker::Copy for FILE_ID_INFORMATION {}
impl ::core::clone::Clone for FILE_ID_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(feature = "Win32_System_WindowsProgramming")]
pub struct FILE_INFORMATION_DEFINITION {
    pub Class: super::super::super::Win32::System::WindowsProgramming::FILE_INFORMATION_CLASS,
    pub NextEntryOffset: u32,
    pub FileNameLengthOffset: u32,
    pub FileNameOffset: u32,
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
impl ::core::marker::Copy for FILE_INFORMATION_DEFINITION {}
#[cfg(feature = "Win32_System_WindowsProgramming")]
impl ::core::clone::Clone for FILE_INFORMATION_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_INITIATE_REPAIR_OUTPUT_BUFFER {
    pub Hint1: u64,
    pub Hint2: u64,
    pub Clsn: u64,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_INITIATE_REPAIR_OUTPUT_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_INITIATE_REPAIR_OUTPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_INTERNAL_INFORMATION {
    pub IndexNumber: i64,
}
impl ::core::marker::Copy for FILE_INTERNAL_INFORMATION {}
impl ::core::clone::Clone for FILE_INTERNAL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_KNOWN_FOLDER_INFORMATION {
    pub Type: FILE_KNOWN_FOLDER_TYPE,
}
impl ::core::marker::Copy for FILE_KNOWN_FOLDER_INFORMATION {}
impl ::core::clone::Clone for FILE_KNOWN_FOLDER_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_LAYOUT_ENTRY {
    pub Version: u32,
    pub NextFileOffset: u32,
    pub Flags: u32,
    pub FileAttributes: u32,
    pub FileReferenceNumber: u64,
    pub FirstNameOffset: u32,
    pub FirstStreamOffset: u32,
    pub ExtraInfoOffset: u32,
    pub ExtraInfoLength: u32,
}
impl ::core::marker::Copy for FILE_LAYOUT_ENTRY {}
impl ::core::clone::Clone for FILE_LAYOUT_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_LAYOUT_INFO_ENTRY {
    pub BasicInformation: FILE_LAYOUT_INFO_ENTRY_0,
    pub OwnerId: u32,
    pub SecurityId: u32,
    pub Usn: i64,
    pub StorageReserveId: STORAGE_RESERVE_ID,
}
impl ::core::marker::Copy for FILE_LAYOUT_INFO_ENTRY {}
impl ::core::clone::Clone for FILE_LAYOUT_INFO_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_LAYOUT_INFO_ENTRY_0 {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
impl ::core::marker::Copy for FILE_LAYOUT_INFO_ENTRY_0 {}
impl ::core::clone::Clone for FILE_LAYOUT_INFO_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_LAYOUT_NAME_ENTRY {
    pub NextNameOffset: u32,
    pub Flags: u32,
    pub ParentFileReferenceNumber: u64,
    pub FileNameLength: u32,
    pub Reserved: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_LAYOUT_NAME_ENTRY {}
impl ::core::clone::Clone for FILE_LAYOUT_NAME_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_LEVEL_TRIM {
    pub Key: u32,
    pub NumRanges: u32,
    pub Ranges: [FILE_LEVEL_TRIM_RANGE; 1],
}
impl ::core::marker::Copy for FILE_LEVEL_TRIM {}
impl ::core::clone::Clone for FILE_LEVEL_TRIM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_LEVEL_TRIM_OUTPUT {
    pub NumRangesProcessed: u32,
}
impl ::core::marker::Copy for FILE_LEVEL_TRIM_OUTPUT {}
impl ::core::clone::Clone for FILE_LEVEL_TRIM_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_LEVEL_TRIM_RANGE {
    pub Offset: u64,
    pub Length: u64,
}
impl ::core::marker::Copy for FILE_LEVEL_TRIM_RANGE {}
impl ::core::clone::Clone for FILE_LEVEL_TRIM_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_LINKS_FULL_ID_INFORMATION {
    pub BytesNeeded: u32,
    pub EntriesReturned: u32,
    pub Entry: FILE_LINK_ENTRY_FULL_ID_INFORMATION,
}
impl ::core::marker::Copy for FILE_LINKS_FULL_ID_INFORMATION {}
impl ::core::clone::Clone for FILE_LINKS_FULL_ID_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_LINKS_INFORMATION {
    pub BytesNeeded: u32,
    pub EntriesReturned: u32,
    pub Entry: FILE_LINK_ENTRY_INFORMATION,
}
impl ::core::marker::Copy for FILE_LINKS_INFORMATION {}
impl ::core::clone::Clone for FILE_LINKS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_LINK_ENTRY_FULL_ID_INFORMATION {
    pub NextEntryOffset: u32,
    pub ParentFileId: FILE_ID_128,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_LINK_ENTRY_FULL_ID_INFORMATION {}
impl ::core::clone::Clone for FILE_LINK_ENTRY_FULL_ID_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_LINK_ENTRY_INFORMATION {
    pub NextEntryOffset: u32,
    pub ParentFileId: i64,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_LINK_ENTRY_INFORMATION {}
impl ::core::clone::Clone for FILE_LINK_ENTRY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_LINK_INFORMATION {
    pub Anonymous: FILE_LINK_INFORMATION_0,
    pub RootDirectory: super::super::super::Win32::Foundation::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_LINK_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_LINK_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union FILE_LINK_INFORMATION_0 {
    pub ReplaceIfExists: super::super::super::Win32::Foundation::BOOLEAN,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_LINK_INFORMATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_LINK_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub struct FILE_LOCK {
    pub CompleteLockIrpRoutine: PCOMPLETE_LOCK_IRP_ROUTINE,
    pub UnlockRoutine: PUNLOCK_ROUTINE,
    pub FastIoIsQuestionable: super::super::super::Win32::Foundation::BOOLEAN,
    pub SpareC: [super::super::super::Win32::Foundation::BOOLEAN; 3],
    pub LockInformation: *mut ::core::ffi::c_void,
    pub LastReturnedLockInfo: FILE_LOCK_INFO,
    pub LastReturnedLock: *mut ::core::ffi::c_void,
    pub LockRequestsInProgress: i32,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for FILE_LOCK {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for FILE_LOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub struct FILE_LOCK_INFO {
    pub StartingByte: i64,
    pub Length: i64,
    pub ExclusiveLock: super::super::super::Win32::Foundation::BOOLEAN,
    pub Key: u32,
    pub FileObject: *mut super::super::Foundation::FILE_OBJECT,
    pub ProcessId: *mut ::core::ffi::c_void,
    pub EndingByte: i64,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for FILE_LOCK_INFO {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for FILE_LOCK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_MAILSLOT_QUERY_INFORMATION {
    pub MaximumMessageSize: u32,
    pub MailslotQuota: u32,
    pub NextMessageSize: u32,
    pub MessagesAvailable: u32,
    pub ReadTimeout: i64,
}
impl ::core::marker::Copy for FILE_MAILSLOT_QUERY_INFORMATION {}
impl ::core::clone::Clone for FILE_MAILSLOT_QUERY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_MAILSLOT_SET_INFORMATION {
    pub ReadTimeout: *mut i64,
}
impl ::core::marker::Copy for FILE_MAILSLOT_SET_INFORMATION {}
impl ::core::clone::Clone for FILE_MAILSLOT_SET_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_MAKE_COMPATIBLE_BUFFER {
    pub CloseDisc: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_MAKE_COMPATIBLE_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_MAKE_COMPATIBLE_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_MODE_INFORMATION {
    pub Mode: u32,
}
impl ::core::marker::Copy for FILE_MODE_INFORMATION {}
impl ::core::clone::Clone for FILE_MODE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_MOVE_CLUSTER_INFORMATION {
    pub ClusterCount: u32,
    pub RootDirectory: super::super::super::Win32::Foundation::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_MOVE_CLUSTER_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_MOVE_CLUSTER_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_NAMES_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NAMES_INFORMATION {}
impl ::core::clone::Clone for FILE_NAMES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_NAME_INFORMATION {
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NAME_INFORMATION {}
impl ::core::clone::Clone for FILE_NAME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_NETWORK_OPEN_INFORMATION {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
}
impl ::core::marker::Copy for FILE_NETWORK_OPEN_INFORMATION {}
impl ::core::clone::Clone for FILE_NETWORK_OPEN_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_NETWORK_PHYSICAL_NAME_INFORMATION {
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NETWORK_PHYSICAL_NAME_INFORMATION {}
impl ::core::clone::Clone for FILE_NETWORK_PHYSICAL_NAME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_NOTIFY_EXTENDED_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: u32,
    pub CreationTime: i64,
    pub LastModificationTime: i64,
    pub LastChangeTime: i64,
    pub LastAccessTime: i64,
    pub AllocatedLength: i64,
    pub FileSize: i64,
    pub FileAttributes: u32,
    pub Anonymous: FILE_NOTIFY_EXTENDED_INFORMATION_0,
    pub FileId: i64,
    pub ParentFileId: i64,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NOTIFY_EXTENDED_INFORMATION {}
impl ::core::clone::Clone for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union FILE_NOTIFY_EXTENDED_INFORMATION_0 {
    pub ReparsePointTag: u32,
    pub EaSize: u32,
}
impl ::core::marker::Copy for FILE_NOTIFY_EXTENDED_INFORMATION_0 {}
impl ::core::clone::Clone for FILE_NOTIFY_EXTENDED_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_NOTIFY_FULL_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: u32,
    pub CreationTime: i64,
    pub LastModificationTime: i64,
    pub LastChangeTime: i64,
    pub LastAccessTime: i64,
    pub AllocatedLength: i64,
    pub FileSize: i64,
    pub FileAttributes: u32,
    pub Anonymous: FILE_NOTIFY_FULL_INFORMATION_0,
    pub FileId: i64,
    pub ParentFileId: i64,
    pub FileNameLength: u16,
    pub FileNameFlags: u8,
    pub Reserved: u8,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NOTIFY_FULL_INFORMATION {}
impl ::core::clone::Clone for FILE_NOTIFY_FULL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union FILE_NOTIFY_FULL_INFORMATION_0 {
    pub ReparsePointTag: u32,
    pub EaSize: u32,
}
impl ::core::marker::Copy for FILE_NOTIFY_FULL_INFORMATION_0 {}
impl ::core::clone::Clone for FILE_NOTIFY_FULL_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_NOTIFY_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NOTIFY_INFORMATION {}
impl ::core::clone::Clone for FILE_NOTIFY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_OBJECTID_BUFFER {
    pub ObjectId: [u8; 16],
    pub Anonymous: FILE_OBJECTID_BUFFER_0,
}
impl ::core::marker::Copy for FILE_OBJECTID_BUFFER {}
impl ::core::clone::Clone for FILE_OBJECTID_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union FILE_OBJECTID_BUFFER_0 {
    pub Anonymous: FILE_OBJECTID_BUFFER_0_0,
    pub ExtendedInfo: [u8; 48],
}
impl ::core::marker::Copy for FILE_OBJECTID_BUFFER_0 {}
impl ::core::clone::Clone for FILE_OBJECTID_BUFFER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_OBJECTID_BUFFER_0_0 {
    pub BirthVolumeId: [u8; 16],
    pub BirthObjectId: [u8; 16],
    pub DomainId: [u8; 16],
}
impl ::core::marker::Copy for FILE_OBJECTID_BUFFER_0_0 {}
impl ::core::clone::Clone for FILE_OBJECTID_BUFFER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_OBJECTID_INFORMATION {
    pub FileReference: i64,
    pub ObjectId: [u8; 16],
    pub Anonymous: FILE_OBJECTID_INFORMATION_0,
}
impl ::core::marker::Copy for FILE_OBJECTID_INFORMATION {}
impl ::core::clone::Clone for FILE_OBJECTID_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union FILE_OBJECTID_INFORMATION_0 {
    pub Anonymous: FILE_OBJECTID_INFORMATION_0_0,
    pub ExtendedInfo: [u8; 48],
}
impl ::core::marker::Copy for FILE_OBJECTID_INFORMATION_0 {}
impl ::core::clone::Clone for FILE_OBJECTID_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_OBJECTID_INFORMATION_0_0 {
    pub BirthVolumeId: [u8; 16],
    pub BirthObjectId: [u8; 16],
    pub DomainId: [u8; 16],
}
impl ::core::marker::Copy for FILE_OBJECTID_INFORMATION_0_0 {}
impl ::core::clone::Clone for FILE_OBJECTID_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_PIPE_ASSIGN_EVENT_BUFFER {
    pub EventHandle: super::super::super::Win32::Foundation::HANDLE,
    pub KeyValue: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_PIPE_ASSIGN_EVENT_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_PIPE_ASSIGN_EVENT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER {
    pub ClientSession: *mut ::core::ffi::c_void,
    pub ClientProcess: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for FILE_PIPE_CLIENT_PROCESS_BUFFER {}
impl ::core::clone::Clone for FILE_PIPE_CLIENT_PROCESS_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER_EX {
    pub ClientSession: *mut ::core::ffi::c_void,
    pub ClientProcess: *mut ::core::ffi::c_void,
    pub ClientComputerNameLength: u16,
    pub ClientComputerBuffer: [u16; 16],
}
impl ::core::marker::Copy for FILE_PIPE_CLIENT_PROCESS_BUFFER_EX {}
impl ::core::clone::Clone for FILE_PIPE_CLIENT_PROCESS_BUFFER_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER_V2 {
    pub ClientSession: u64,
    pub ClientProcess: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for FILE_PIPE_CLIENT_PROCESS_BUFFER_V2 {}
impl ::core::clone::Clone for FILE_PIPE_CLIENT_PROCESS_BUFFER_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_PIPE_CREATE_SYMLINK_INPUT {
    pub NameOffset: u16,
    pub NameLength: u16,
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_PIPE_CREATE_SYMLINK_INPUT {}
impl ::core::clone::Clone for FILE_PIPE_CREATE_SYMLINK_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_PIPE_DELETE_SYMLINK_INPUT {
    pub NameOffset: u16,
    pub NameLength: u16,
}
impl ::core::marker::Copy for FILE_PIPE_DELETE_SYMLINK_INPUT {}
impl ::core::clone::Clone for FILE_PIPE_DELETE_SYMLINK_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_PIPE_EVENT_BUFFER {
    pub NamedPipeState: u32,
    pub EntryType: u32,
    pub ByteCount: u32,
    pub KeyValue: u32,
    pub NumberRequests: u32,
}
impl ::core::marker::Copy for FILE_PIPE_EVENT_BUFFER {}
impl ::core::clone::Clone for FILE_PIPE_EVENT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_PIPE_INFORMATION {
    pub ReadMode: u32,
    pub CompletionMode: u32,
}
impl ::core::marker::Copy for FILE_PIPE_INFORMATION {}
impl ::core::clone::Clone for FILE_PIPE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_PIPE_LOCAL_INFORMATION {
    pub NamedPipeType: u32,
    pub NamedPipeConfiguration: u32,
    pub MaximumInstances: u32,
    pub CurrentInstances: u32,
    pub InboundQuota: u32,
    pub ReadDataAvailable: u32,
    pub OutboundQuota: u32,
    pub WriteQuotaAvailable: u32,
    pub NamedPipeState: u32,
    pub NamedPipeEnd: u32,
}
impl ::core::marker::Copy for FILE_PIPE_LOCAL_INFORMATION {}
impl ::core::clone::Clone for FILE_PIPE_LOCAL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_PIPE_PEEK_BUFFER {
    pub NamedPipeState: u32,
    pub ReadDataAvailable: u32,
    pub NumberOfMessages: u32,
    pub MessageLength: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for FILE_PIPE_PEEK_BUFFER {}
impl ::core::clone::Clone for FILE_PIPE_PEEK_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_PIPE_REMOTE_INFORMATION {
    pub CollectDataTime: i64,
    pub MaximumCollectionCount: u32,
}
impl ::core::marker::Copy for FILE_PIPE_REMOTE_INFORMATION {}
impl ::core::clone::Clone for FILE_PIPE_REMOTE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_PIPE_SILO_ARRIVAL_INPUT {
    pub JobHandle: super::super::super::Win32::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_PIPE_SILO_ARRIVAL_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_PIPE_SILO_ARRIVAL_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_PIPE_WAIT_FOR_BUFFER {
    pub Timeout: i64,
    pub NameLength: u32,
    pub TimeoutSpecified: super::super::super::Win32::Foundation::BOOLEAN,
    pub Name: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_PIPE_WAIT_FOR_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_PIPE_WAIT_FOR_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_POSITION_INFORMATION {
    pub CurrentByteOffset: i64,
}
impl ::core::marker::Copy for FILE_POSITION_INFORMATION {}
impl ::core::clone::Clone for FILE_POSITION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_PREFETCH {
    pub Type: u32,
    pub Count: u32,
    pub Prefetch: [u64; 1],
}
impl ::core::marker::Copy for FILE_PREFETCH {}
impl ::core::clone::Clone for FILE_PREFETCH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_PREFETCH_EX {
    pub Type: u32,
    pub Count: u32,
    pub Context: *mut ::core::ffi::c_void,
    pub Prefetch: [u64; 1],
}
impl ::core::marker::Copy for FILE_PREFETCH_EX {}
impl ::core::clone::Clone for FILE_PREFETCH_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_PROVIDER_EXTERNAL_INFO_V0 {
    pub Version: u32,
    pub Algorithm: u32,
}
impl ::core::marker::Copy for FILE_PROVIDER_EXTERNAL_INFO_V0 {}
impl ::core::clone::Clone for FILE_PROVIDER_EXTERNAL_INFO_V0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_PROVIDER_EXTERNAL_INFO_V1 {
    pub Version: u32,
    pub Algorithm: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_PROVIDER_EXTERNAL_INFO_V1 {}
impl ::core::clone::Clone for FILE_PROVIDER_EXTERNAL_INFO_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    pub DirectoryCount: i64,
    pub FileCount: i64,
    pub FsFormatMajVersion: u16,
    pub FsFormatMinVersion: u16,
    pub FsFormatName: [u16; 12],
    pub FormatTime: i64,
    pub LastUpdateTime: i64,
    pub CopyrightInfo: [u16; 34],
    pub AbstractInfo: [u16; 34],
    pub FormattingImplementationInfo: [u16; 34],
    pub LastModifyingImplementationInfo: [u16; 34],
}
impl ::core::marker::Copy for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {}
impl ::core::clone::Clone for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_QUERY_SPARING_BUFFER {
    pub SparingUnitBytes: u32,
    pub SoftwareSparing: super::super::super::Win32::Foundation::BOOLEAN,
    pub TotalSpareBlocks: u32,
    pub FreeSpareBlocks: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_QUERY_SPARING_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_QUERY_SPARING_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct FILE_QUOTA_INFORMATION {
    pub NextEntryOffset: u32,
    pub SidLength: u32,
    pub ChangeTime: i64,
    pub QuotaUsed: i64,
    pub QuotaThreshold: i64,
    pub QuotaLimit: i64,
    pub Sid: super::super::super::Win32::Security::SID,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for FILE_QUOTA_INFORMATION {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for FILE_QUOTA_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_REFERENCE_RANGE {
    pub StartingFileReferenceNumber: u64,
    pub EndingFileReferenceNumber: u64,
}
impl ::core::marker::Copy for FILE_REFERENCE_RANGE {}
impl ::core::clone::Clone for FILE_REFERENCE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_REGION_INFO {
    pub FileOffset: i64,
    pub Length: i64,
    pub Usage: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for FILE_REGION_INFO {}
impl ::core::clone::Clone for FILE_REGION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_REGION_INPUT {
    pub FileOffset: i64,
    pub Length: i64,
    pub DesiredUsage: u32,
}
impl ::core::marker::Copy for FILE_REGION_INPUT {}
impl ::core::clone::Clone for FILE_REGION_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_REGION_OUTPUT {
    pub Flags: u32,
    pub TotalRegionEntryCount: u32,
    pub RegionEntryCount: u32,
    pub Reserved: u32,
    pub Region: [FILE_REGION_INFO; 1],
}
impl ::core::marker::Copy for FILE_REGION_OUTPUT {}
impl ::core::clone::Clone for FILE_REGION_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION {
    pub StructureVersion: u16,
    pub StructureSize: u16,
    pub Protocol: u32,
    pub ProtocolMajorVersion: u16,
    pub ProtocolMinorVersion: u16,
    pub ProtocolRevision: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub GenericReserved: FILE_REMOTE_PROTOCOL_INFORMATION_0,
    pub ProtocolSpecific: FILE_REMOTE_PROTOCOL_INFORMATION_1,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFORMATION {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_0 {
    pub Reserved: [u32; 8],
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFORMATION_0 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union FILE_REMOTE_PROTOCOL_INFORMATION_1 {
    pub Smb2: FILE_REMOTE_PROTOCOL_INFORMATION_1_0,
    pub Reserved: [u32; 16],
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFORMATION_1 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFORMATION_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_1_0 {
    pub Server: FILE_REMOTE_PROTOCOL_INFORMATION_1_0_0,
    pub Share: FILE_REMOTE_PROTOCOL_INFORMATION_1_0_1,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFORMATION_1_0 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFORMATION_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_1_0_0 {
    pub Capabilities: u32,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFORMATION_1_0_0 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFORMATION_1_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_1_0_1 {
    pub Capabilities: u32,
    pub ShareFlags: u32,
    pub ShareType: u8,
    pub Reserved0: [u8; 3],
    pub Reserved1: u32,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFORMATION_1_0_1 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFORMATION_1_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_RENAME_INFORMATION {
    pub Anonymous: FILE_RENAME_INFORMATION_0,
    pub RootDirectory: super::super::super::Win32::Foundation::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_RENAME_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_RENAME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union FILE_RENAME_INFORMATION_0 {
    pub ReplaceIfExists: super::super::super::Win32::Foundation::BOOLEAN,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_RENAME_INFORMATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_RENAME_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_REPARSE_POINT_INFORMATION {
    pub FileReference: i64,
    pub Tag: u32,
}
impl ::core::marker::Copy for FILE_REPARSE_POINT_INFORMATION {}
impl ::core::clone::Clone for FILE_REPARSE_POINT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_SET_DEFECT_MGMT_BUFFER {
    pub Disable: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_SET_DEFECT_MGMT_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_SET_DEFECT_MGMT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_SET_SPARSE_BUFFER {
    pub SetSparse: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_SET_SPARSE_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_SET_SPARSE_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_STANDARD_INFORMATION {
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub NumberOfLinks: u32,
    pub DeletePending: super::super::super::Win32::Foundation::BOOLEAN,
    pub Directory: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_STANDARD_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_STANDARD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_STANDARD_LINK_INFORMATION {
    pub NumberOfAccessibleLinks: u32,
    pub TotalNumberOfLinks: u32,
    pub DeletePending: super::super::super::Win32::Foundation::BOOLEAN,
    pub Directory: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_STANDARD_LINK_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_STANDARD_LINK_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_STAT_INFORMATION {
    pub FileId: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
    pub ReparseTag: u32,
    pub NumberOfLinks: u32,
    pub EffectiveAccess: u32,
}
impl ::core::marker::Copy for FILE_STAT_INFORMATION {}
impl ::core::clone::Clone for FILE_STAT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_STAT_LX_INFORMATION {
    pub FileId: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
    pub ReparseTag: u32,
    pub NumberOfLinks: u32,
    pub EffectiveAccess: u32,
    pub LxFlags: u32,
    pub LxUid: u32,
    pub LxGid: u32,
    pub LxMode: u32,
    pub LxDeviceIdMajor: u32,
    pub LxDeviceIdMinor: u32,
}
impl ::core::marker::Copy for FILE_STAT_LX_INFORMATION {}
impl ::core::clone::Clone for FILE_STAT_LX_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_STORAGE_RESERVE_ID_INFORMATION {
    pub StorageReserveId: STORAGE_RESERVE_ID,
}
impl ::core::marker::Copy for FILE_STORAGE_RESERVE_ID_INFORMATION {}
impl ::core::clone::Clone for FILE_STORAGE_RESERVE_ID_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_STORAGE_TIER {
    pub Id: ::windows_sys::core::GUID,
    pub Name: [u16; 256],
    pub Description: [u16; 256],
    pub Flags: u64,
    pub ProvisionedCapacity: u64,
    pub MediaType: FILE_STORAGE_TIER_MEDIA_TYPE,
    pub Class: FILE_STORAGE_TIER_CLASS,
}
impl ::core::marker::Copy for FILE_STORAGE_TIER {}
impl ::core::clone::Clone for FILE_STORAGE_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_STORAGE_TIER_REGION {
    pub TierId: ::windows_sys::core::GUID,
    pub Offset: u64,
    pub Length: u64,
}
impl ::core::marker::Copy for FILE_STORAGE_TIER_REGION {}
impl ::core::clone::Clone for FILE_STORAGE_TIER_REGION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_STREAM_INFORMATION {
    pub NextEntryOffset: u32,
    pub StreamNameLength: u32,
    pub StreamSize: i64,
    pub StreamAllocationSize: i64,
    pub StreamName: [u16; 1],
}
impl ::core::marker::Copy for FILE_STREAM_INFORMATION {}
impl ::core::clone::Clone for FILE_STREAM_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_SYSTEM_RECOGNITION_INFORMATION {
    pub FileSystem: [u8; 9],
}
impl ::core::marker::Copy for FILE_SYSTEM_RECOGNITION_INFORMATION {}
impl ::core::clone::Clone for FILE_SYSTEM_RECOGNITION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_TIMESTAMPS {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
}
impl ::core::marker::Copy for FILE_TIMESTAMPS {}
impl ::core::clone::Clone for FILE_TIMESTAMPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_TRACKING_INFORMATION {
    pub DestinationFile: super::super::super::Win32::Foundation::HANDLE,
    pub ObjectInformationLength: u32,
    pub ObjectInformation: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_TRACKING_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_TRACKING_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_TYPE_NOTIFICATION_INPUT {
    pub Flags: u32,
    pub NumFileTypeIDs: u32,
    pub FileTypeID: [::windows_sys::core::GUID; 1],
}
impl ::core::marker::Copy for FILE_TYPE_NOTIFICATION_INPUT {}
impl ::core::clone::Clone for FILE_TYPE_NOTIFICATION_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_VOLUME_NAME_INFORMATION {
    pub DeviceNameLength: u32,
    pub DeviceName: [u16; 1],
}
impl ::core::marker::Copy for FILE_VOLUME_NAME_INFORMATION {}
impl ::core::clone::Clone for FILE_VOLUME_NAME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_ZERO_DATA_INFORMATION {
    pub FileOffset: i64,
    pub BeyondFinalZero: i64,
}
impl ::core::marker::Copy for FILE_ZERO_DATA_INFORMATION {}
impl ::core::clone::Clone for FILE_ZERO_DATA_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FILE_ZERO_DATA_INFORMATION_EX {
    pub FileOffset: i64,
    pub BeyondFinalZero: i64,
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_ZERO_DATA_INFORMATION_EX {}
impl ::core::clone::Clone for FILE_ZERO_DATA_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct FIND_BY_SID_DATA {
    pub Restart: u32,
    pub Sid: super::super::super::Win32::Security::SID,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for FIND_BY_SID_DATA {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for FIND_BY_SID_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FIND_BY_SID_OUTPUT {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FIND_BY_SID_OUTPUT {}
impl ::core::clone::Clone for FIND_BY_SID_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {
    pub ChecksumAlgorithm: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub ChecksumChunkSizeInBytes: u32,
    pub ClusterSizeInBytes: u32,
}
impl ::core::marker::Copy for FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {}
impl ::core::clone::Clone for FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER {
    pub FileOffset: i64,
    pub ByteCount: i64,
    pub RecallOwnerGuid: ::windows_sys::core::GUID,
    pub RecallMetadataBufferSize: u32,
    pub RecallMetadataBuffer: [u8; 1],
}
impl ::core::marker::Copy for FSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER {}
impl ::core::clone::Clone for FSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_OFFLOAD_READ_INPUT {
    pub Size: u32,
    pub Flags: u32,
    pub TokenTimeToLive: u32,
    pub Reserved: u32,
    pub FileOffset: u64,
    pub CopyLength: u64,
}
impl ::core::marker::Copy for FSCTL_OFFLOAD_READ_INPUT {}
impl ::core::clone::Clone for FSCTL_OFFLOAD_READ_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_OFFLOAD_READ_OUTPUT {
    pub Size: u32,
    pub Flags: u32,
    pub TransferLength: u64,
    pub Token: [u8; 512],
}
impl ::core::marker::Copy for FSCTL_OFFLOAD_READ_OUTPUT {}
impl ::core::clone::Clone for FSCTL_OFFLOAD_READ_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_OFFLOAD_WRITE_INPUT {
    pub Size: u32,
    pub Flags: u32,
    pub FileOffset: u64,
    pub CopyLength: u64,
    pub TransferOffset: u64,
    pub Token: [u8; 512],
}
impl ::core::marker::Copy for FSCTL_OFFLOAD_WRITE_INPUT {}
impl ::core::clone::Clone for FSCTL_OFFLOAD_WRITE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_OFFLOAD_WRITE_OUTPUT {
    pub Size: u32,
    pub Flags: u32,
    pub LengthWritten: u64,
}
impl ::core::marker::Copy for FSCTL_OFFLOAD_WRITE_OUTPUT {}
impl ::core::clone::Clone for FSCTL_OFFLOAD_WRITE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_QUERY_FAT_BPB_BUFFER {
    pub First0x24BytesOfBootSector: [u8; 36],
}
impl ::core::marker::Copy for FSCTL_QUERY_FAT_BPB_BUFFER {}
impl ::core::clone::Clone for FSCTL_QUERY_FAT_BPB_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_QUERY_GHOSTED_FILE_EXTENTS_INPUT_RANGE {
    pub FileOffset: i64,
    pub ByteCount: i64,
}
impl ::core::marker::Copy for FSCTL_QUERY_GHOSTED_FILE_EXTENTS_INPUT_RANGE {}
impl ::core::clone::Clone for FSCTL_QUERY_GHOSTED_FILE_EXTENTS_INPUT_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_QUERY_GHOSTED_FILE_EXTENTS_OUTPUT {
    pub ExtentCount: u32,
    pub TotalExtentCount: u32,
    pub Extents: [u8; 1],
}
impl ::core::marker::Copy for FSCTL_QUERY_GHOSTED_FILE_EXTENTS_OUTPUT {}
impl ::core::clone::Clone for FSCTL_QUERY_GHOSTED_FILE_EXTENTS_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_QUERY_REGION_INFO_INPUT {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub NumberOfTierIds: u32,
    pub TierIds: [::windows_sys::core::GUID; 1],
}
impl ::core::marker::Copy for FSCTL_QUERY_REGION_INFO_INPUT {}
impl ::core::clone::Clone for FSCTL_QUERY_REGION_INFO_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_QUERY_REGION_INFO_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Reserved: u32,
    pub Alignment: u64,
    pub TotalNumberOfRegions: u32,
    pub NumberOfRegionsReturned: u32,
    pub Regions: [FILE_STORAGE_TIER_REGION; 1],
}
impl ::core::marker::Copy for FSCTL_QUERY_REGION_INFO_OUTPUT {}
impl ::core::clone::Clone for FSCTL_QUERY_REGION_INFO_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub TotalNumberOfTiers: u32,
    pub NumberOfTiersReturned: u32,
    pub Tiers: [FILE_STORAGE_TIER; 1],
}
impl ::core::marker::Copy for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {}
impl ::core::clone::Clone for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_QUERY_VOLUME_NUMA_INFO_OUTPUT {
    pub NumaNode: u32,
}
impl ::core::marker::Copy for FSCTL_QUERY_VOLUME_NUMA_INFO_OUTPUT {}
impl ::core::clone::Clone for FSCTL_QUERY_VOLUME_NUMA_INFO_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {
    pub ChecksumAlgorithm: u16,
    pub Reserved: u16,
    pub Flags: u32,
}
impl ::core::marker::Copy for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {}
impl ::core::clone::Clone for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    pub EnableIntegrity: u8,
    pub KeepIntegrityStateUnchanged: u8,
    pub Reserved: u16,
    pub Flags: u32,
    pub Version: u8,
    pub Reserved2: [u8; 7],
}
impl ::core::marker::Copy for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {}
impl ::core::clone::Clone for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_UNMAP_SPACE_INPUT_BUFFER {
    pub BytesToUnmap: i64,
}
impl ::core::marker::Copy for FSCTL_UNMAP_SPACE_INPUT_BUFFER {}
impl ::core::clone::Clone for FSCTL_UNMAP_SPACE_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSCTL_UNMAP_SPACE_OUTPUT {
    pub BytesUnmapped: i64,
}
impl ::core::marker::Copy for FSCTL_UNMAP_SPACE_OUTPUT {}
impl ::core::clone::Clone for FSCTL_UNMAP_SPACE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct FSRTL_ADVANCED_FCB_HEADER {
    pub Base: FSRTL_COMMON_FCB_HEADER,
    pub FastMutex: *mut super::super::Foundation::FAST_MUTEX,
    pub FilterContexts: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub PushLock: usize,
    pub FileContextSupportPointer: *mut *mut ::core::ffi::c_void,
    pub Anonymous: FSRTL_ADVANCED_FCB_HEADER_0,
    pub AePushLock: *mut ::core::ffi::c_void,
    pub BypassIoOpenCount: u32,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for FSRTL_ADVANCED_FCB_HEADER {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for FSRTL_ADVANCED_FCB_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub union FSRTL_ADVANCED_FCB_HEADER_0 {
    pub Oplock: *mut ::core::ffi::c_void,
    pub ReservedForRemote: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for FSRTL_ADVANCED_FCB_HEADER_0 {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for FSRTL_ADVANCED_FCB_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Graphics_DirectDraw\"`*"]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub struct FSRTL_AUXILIARY_BUFFER {
    pub Buffer: *mut ::core::ffi::c_void,
    pub Length: u32,
    pub Flags: u32,
    pub Mdl: *mut super::super::super::Win32::Graphics::DirectDraw::MDL,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::core::marker::Copy for FSRTL_AUXILIARY_BUFFER {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::core::clone::Clone for FSRTL_AUXILIARY_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
pub struct FSRTL_COMMON_FCB_HEADER {
    pub NodeTypeCode: i16,
    pub NodeByteSize: i16,
    pub Flags: u8,
    pub IsFastIoPossible: u8,
    pub Flags2: u8,
    pub _bitfield: u8,
    pub Resource: *mut super::super::Foundation::ERESOURCE,
    pub PagingIoResource: *mut super::super::Foundation::ERESOURCE,
    pub AllocationSize: i64,
    pub FileSize: i64,
    pub ValidDataLength: i64,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for FSRTL_COMMON_FCB_HEADER {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for FSRTL_COMMON_FCB_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSRTL_MUP_PROVIDER_INFO_LEVEL_1 {
    pub ProviderId: u32,
}
impl ::core::marker::Copy for FSRTL_MUP_PROVIDER_INFO_LEVEL_1 {}
impl ::core::clone::Clone for FSRTL_MUP_PROVIDER_INFO_LEVEL_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FSRTL_MUP_PROVIDER_INFO_LEVEL_2 {
    pub ProviderId: u32,
    pub ProviderName: super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FSRTL_MUP_PROVIDER_INFO_LEVEL_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FSRTL_MUP_PROVIDER_INFO_LEVEL_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
pub struct FSRTL_PER_FILEOBJECT_CONTEXT {
    pub Links: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub OwnerId: *mut ::core::ffi::c_void,
    pub InstanceId: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for FSRTL_PER_FILEOBJECT_CONTEXT {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for FSRTL_PER_FILEOBJECT_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
pub struct FSRTL_PER_FILE_CONTEXT {
    pub Links: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub OwnerId: *mut ::core::ffi::c_void,
    pub InstanceId: *mut ::core::ffi::c_void,
    pub FreeCallback: super::super::Foundation::PFREE_FUNCTION,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for FSRTL_PER_FILE_CONTEXT {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for FSRTL_PER_FILE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
pub struct FSRTL_PER_STREAM_CONTEXT {
    pub Links: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub OwnerId: *mut ::core::ffi::c_void,
    pub InstanceId: *mut ::core::ffi::c_void,
    pub FreeCallback: super::super::Foundation::PFREE_FUNCTION,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for FSRTL_PER_STREAM_CONTEXT {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for FSRTL_PER_STREAM_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSRTL_UNC_PROVIDER_REGISTRATION {
    pub Size: u16,
    pub Version: u16,
    pub Anonymous1: FSRTL_UNC_PROVIDER_REGISTRATION_0,
    pub Anonymous2: FSRTL_UNC_PROVIDER_REGISTRATION_1,
}
impl ::core::marker::Copy for FSRTL_UNC_PROVIDER_REGISTRATION {}
impl ::core::clone::Clone for FSRTL_UNC_PROVIDER_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union FSRTL_UNC_PROVIDER_REGISTRATION_0 {
    pub ProviderFlags: u32,
    pub Anonymous: FSRTL_UNC_PROVIDER_REGISTRATION_0_0,
}
impl ::core::marker::Copy for FSRTL_UNC_PROVIDER_REGISTRATION_0 {}
impl ::core::clone::Clone for FSRTL_UNC_PROVIDER_REGISTRATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSRTL_UNC_PROVIDER_REGISTRATION_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for FSRTL_UNC_PROVIDER_REGISTRATION_0_0 {}
impl ::core::clone::Clone for FSRTL_UNC_PROVIDER_REGISTRATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union FSRTL_UNC_PROVIDER_REGISTRATION_1 {
    pub HardeningCapabilities: u32,
    pub Anonymous: FSRTL_UNC_PROVIDER_REGISTRATION_1_0,
}
impl ::core::marker::Copy for FSRTL_UNC_PROVIDER_REGISTRATION_1 {}
impl ::core::clone::Clone for FSRTL_UNC_PROVIDER_REGISTRATION_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FSRTL_UNC_PROVIDER_REGISTRATION_1_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for FSRTL_UNC_PROVIDER_REGISTRATION_1_0 {}
impl ::core::clone::Clone for FSRTL_UNC_PROVIDER_REGISTRATION_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FS_BPIO_INFO {
    pub ActiveBypassIoCount: u32,
    pub StorageDriverNameLen: u16,
    pub StorageDriverName: [u16; 32],
}
impl ::core::marker::Copy for FS_BPIO_INFO {}
impl ::core::clone::Clone for FS_BPIO_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FS_BPIO_INPUT {
    pub Operation: FS_BPIO_OPERATIONS,
    pub InFlags: FS_BPIO_INFLAGS,
    pub Reserved1: u64,
    pub Reserved2: u64,
}
impl ::core::marker::Copy for FS_BPIO_INPUT {}
impl ::core::clone::Clone for FS_BPIO_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FS_BPIO_OUTPUT {
    pub Operation: FS_BPIO_OPERATIONS,
    pub OutFlags: FS_BPIO_OUTFLAGS,
    pub Reserved1: u64,
    pub Reserved2: u64,
    pub Anonymous: FS_BPIO_OUTPUT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FS_BPIO_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FS_BPIO_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union FS_BPIO_OUTPUT_0 {
    pub Enable: FS_BPIO_RESULTS,
    pub Query: FS_BPIO_RESULTS,
    pub VolumeStackResume: FS_BPIO_RESULTS,
    pub StreamResume: FS_BPIO_RESULTS,
    pub GetInfo: FS_BPIO_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FS_BPIO_OUTPUT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FS_BPIO_OUTPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FS_BPIO_RESULTS {
    pub OpStatus: super::super::super::Win32::Foundation::NTSTATUS,
    pub FailingDriverNameLen: u16,
    pub FailingDriverName: [u16; 32],
    pub FailureReasonLen: u16,
    pub FailureReason: [u16; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FS_BPIO_RESULTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FS_BPIO_RESULTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub struct FS_FILTER_CALLBACKS {
    pub SizeOfFsFilterCallbacks: u32,
    pub Reserved: u32,
    pub PreAcquireForSectionSynchronization: PFS_FILTER_CALLBACK,
    pub PostAcquireForSectionSynchronization: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreReleaseForSectionSynchronization: PFS_FILTER_CALLBACK,
    pub PostReleaseForSectionSynchronization: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreAcquireForCcFlush: PFS_FILTER_CALLBACK,
    pub PostAcquireForCcFlush: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreReleaseForCcFlush: PFS_FILTER_CALLBACK,
    pub PostReleaseForCcFlush: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreAcquireForModifiedPageWriter: PFS_FILTER_CALLBACK,
    pub PostAcquireForModifiedPageWriter: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreReleaseForModifiedPageWriter: PFS_FILTER_CALLBACK,
    pub PostReleaseForModifiedPageWriter: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreQueryOpen: PFS_FILTER_CALLBACK,
    pub PostQueryOpen: PFS_FILTER_COMPLETION_CALLBACK,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for FS_FILTER_CALLBACKS {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for FS_FILTER_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub struct FS_FILTER_CALLBACK_DATA {
    pub SizeOfFsFilterCallbackData: u32,
    pub Operation: u8,
    pub Reserved: u8,
    pub DeviceObject: *mut super::super::Foundation::DEVICE_OBJECT,
    pub FileObject: *mut super::super::Foundation::FILE_OBJECT,
    pub Parameters: FS_FILTER_PARAMETERS,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for FS_FILTER_CALLBACK_DATA {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for FS_FILTER_CALLBACK_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub union FS_FILTER_PARAMETERS {
    pub AcquireForModifiedPageWriter: FS_FILTER_PARAMETERS_0,
    pub ReleaseForModifiedPageWriter: FS_FILTER_PARAMETERS_4,
    pub AcquireForSectionSynchronization: FS_FILTER_PARAMETERS_1,
    pub QueryOpen: FS_FILTER_PARAMETERS_3,
    pub Others: FS_FILTER_PARAMETERS_2,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for FS_FILTER_PARAMETERS {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for FS_FILTER_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub struct FS_FILTER_PARAMETERS_0 {
    pub EndingOffset: *mut i64,
    pub ResourceToRelease: *mut *mut super::super::Foundation::ERESOURCE,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for FS_FILTER_PARAMETERS_0 {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for FS_FILTER_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub struct FS_FILTER_PARAMETERS_1 {
    pub SyncType: FS_FILTER_SECTION_SYNC_TYPE,
    pub PageProtection: u32,
    pub OutputInformation: *mut FS_FILTER_SECTION_SYNC_OUTPUT,
    pub Flags: u32,
    pub AllocationAttributes: u32,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for FS_FILTER_PARAMETERS_1 {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for FS_FILTER_PARAMETERS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub struct FS_FILTER_PARAMETERS_2 {
    pub Argument1: *mut ::core::ffi::c_void,
    pub Argument2: *mut ::core::ffi::c_void,
    pub Argument3: *mut ::core::ffi::c_void,
    pub Argument4: *mut ::core::ffi::c_void,
    pub Argument5: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for FS_FILTER_PARAMETERS_2 {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for FS_FILTER_PARAMETERS_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub struct FS_FILTER_PARAMETERS_3 {
    pub Irp: *mut super::super::Foundation::IRP,
    pub FileInformation: *mut ::core::ffi::c_void,
    pub Length: *mut u32,
    pub FileInformationClass: super::super::super::Win32::System::WindowsProgramming::FILE_INFORMATION_CLASS,
    pub CompletionStatus: super::super::super::Win32::Foundation::NTSTATUS,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for FS_FILTER_PARAMETERS_3 {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for FS_FILTER_PARAMETERS_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub struct FS_FILTER_PARAMETERS_4 {
    pub ResourceToRelease: *mut super::super::Foundation::ERESOURCE,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for FS_FILTER_PARAMETERS_4 {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for FS_FILTER_PARAMETERS_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct FS_FILTER_SECTION_SYNC_OUTPUT {
    pub StructureSize: u32,
    pub SizeReturned: u32,
    pub Flags: u32,
    pub DesiredReadAlignment: u32,
}
impl ::core::marker::Copy for FS_FILTER_SECTION_SYNC_OUTPUT {}
impl ::core::clone::Clone for FS_FILTER_SECTION_SYNC_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GENERATE_NAME_CONTEXT {
    pub Checksum: u16,
    pub ChecksumInserted: super::super::super::Win32::Foundation::BOOLEAN,
    pub NameLength: u8,
    pub NameBuffer: [u16; 8],
    pub ExtensionLength: u32,
    pub ExtensionBuffer: [u16; 4],
    pub LastIndexValue: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GENERATE_NAME_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GENERATE_NAME_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct GET_FILTER_FILE_IDENTIFIER_INPUT {
    pub AltitudeLength: u16,
    pub Altitude: [u16; 1],
}
impl ::core::marker::Copy for GET_FILTER_FILE_IDENTIFIER_INPUT {}
impl ::core::clone::Clone for GET_FILTER_FILE_IDENTIFIER_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    pub FilterFileIdentifierLength: u16,
    pub FilterFileIdentifier: [u8; 1],
}
impl ::core::marker::Copy for GET_FILTER_FILE_IDENTIFIER_OUTPUT {}
impl ::core::clone::Clone for GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct GHOSTED_FILE_EXTENT {
    pub FileOffset: i64,
    pub ByteCount: i64,
    pub RecallOwnerGuid: ::windows_sys::core::GUID,
    pub NextEntryOffset: u32,
    pub RecallMetadataBufferSize: u32,
    pub RecallMetadataBuffer: [u8; 1],
}
impl ::core::marker::Copy for GHOSTED_FILE_EXTENT {}
impl ::core::clone::Clone for GHOSTED_FILE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub struct IO_CREATE_STREAM_FILE_OPTIONS {
    pub Size: u16,
    pub Flags: u16,
    pub TargetDeviceObject: *mut super::super::Foundation::DEVICE_OBJECT,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for IO_CREATE_STREAM_FILE_OPTIONS {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for IO_CREATE_STREAM_FILE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub struct IO_DEVICE_HINT_ECP_CONTEXT {
    pub TargetDevice: *mut super::super::Foundation::DEVICE_OBJECT,
    pub RemainingName: super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for IO_DEVICE_HINT_ECP_CONTEXT {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for IO_DEVICE_HINT_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct IO_IRP_EXT_TRACK_OFFSET_HEADER {
    pub Validation: u16,
    pub Flags: u16,
    pub TrackedOffsetCallback: PIO_IRP_EXT_PROCESS_TRACKED_OFFSET_CALLBACK,
}
impl ::core::marker::Copy for IO_IRP_EXT_TRACK_OFFSET_HEADER {}
impl ::core::clone::Clone for IO_IRP_EXT_TRACK_OFFSET_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`*"]
#[cfg(feature = "Wdk_Foundation")]
pub struct IO_PRIORITY_INFO {
    pub Size: u32,
    pub ThreadPriority: u32,
    pub PagePriority: u32,
    pub IoPriority: super::super::Foundation::IO_PRIORITY_HINT,
}
#[cfg(feature = "Wdk_Foundation")]
impl ::core::marker::Copy for IO_PRIORITY_INFO {}
#[cfg(feature = "Wdk_Foundation")]
impl ::core::clone::Clone for IO_PRIORITY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct IO_STOP_ON_SYMLINK_FILTER_ECP_v0 {
    pub Out: IO_STOP_ON_SYMLINK_FILTER_ECP_v0_0,
}
impl ::core::marker::Copy for IO_STOP_ON_SYMLINK_FILTER_ECP_v0 {}
impl ::core::clone::Clone for IO_STOP_ON_SYMLINK_FILTER_ECP_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct IO_STOP_ON_SYMLINK_FILTER_ECP_v0_0 {
    pub ReparseCount: u32,
    pub RemainingPathLength: u32,
}
impl ::core::marker::Copy for IO_STOP_ON_SYMLINK_FILTER_ECP_v0_0 {}
impl ::core::clone::Clone for IO_STOP_ON_SYMLINK_FILTER_ECP_v0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct KAPC_STATE {
    pub ApcListHead: [super::super::super::Win32::System::Kernel::LIST_ENTRY; 2],
    pub Process: *mut super::super::Foundation::KPROCESS,
    pub Anonymous1: KAPC_STATE_0,
    pub KernelApcPending: super::super::super::Win32::Foundation::BOOLEAN,
    pub Anonymous2: KAPC_STATE_1,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for KAPC_STATE {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for KAPC_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub union KAPC_STATE_0 {
    pub InProgressFlags: u8,
    pub Anonymous: KAPC_STATE_0_0,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for KAPC_STATE_0 {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for KAPC_STATE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct KAPC_STATE_0_0 {
    pub _bitfield: u8,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for KAPC_STATE_0_0 {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for KAPC_STATE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub union KAPC_STATE_1 {
    pub UserApcPendingAll: super::super::super::Win32::Foundation::BOOLEAN,
    pub Anonymous: KAPC_STATE_1_0,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for KAPC_STATE_1 {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for KAPC_STATE_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct KAPC_STATE_1_0 {
    pub _bitfield: u8,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for KAPC_STATE_1_0 {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for KAPC_STATE_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct LARGE_MCB {
    pub GuardedMutex: *mut super::super::Foundation::FAST_MUTEX,
    pub BaseMcb: BASE_MCB,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for LARGE_MCB {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for LARGE_MCB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct LCN_WEAK_REFERENCE_BUFFER {
    pub Lcn: i64,
    pub LengthInClusters: i64,
    pub ReferenceCount: u32,
    pub State: u16,
}
impl ::core::marker::Copy for LCN_WEAK_REFERENCE_BUFFER {}
impl ::core::clone::Clone for LCN_WEAK_REFERENCE_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct LCN_WEAK_REFERENCE_CREATE_INPUT_BUFFER {
    pub Offset: i64,
    pub Length: i64,
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for LCN_WEAK_REFERENCE_CREATE_INPUT_BUFFER {}
impl ::core::clone::Clone for LCN_WEAK_REFERENCE_CREATE_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct LINK_TRACKING_INFORMATION {
    pub Type: LINK_TRACKING_INFORMATION_TYPE,
    pub VolumeId: [u8; 16],
}
impl ::core::marker::Copy for LINK_TRACKING_INFORMATION {}
impl ::core::clone::Clone for LINK_TRACKING_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct LMR_QUERY_INFO_PARAM {
    pub Operation: LMR_QUERY_INFO_CLASS,
}
impl ::core::marker::Copy for LMR_QUERY_INFO_PARAM {}
impl ::core::clone::Clone for LMR_QUERY_INFO_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct LMR_QUERY_SESSION_INFO {
    pub SessionId: u64,
}
impl ::core::marker::Copy for LMR_QUERY_SESSION_INFO {}
impl ::core::clone::Clone for LMR_QUERY_SESSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    pub OffsetToNext: u32,
    pub Flags: u32,
    pub Reserved: i64,
    pub Cluster: i64,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {}
impl ::core::clone::Clone for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    pub Flags: u32,
    pub NumberOfClusters: u32,
    pub Cluster: [i64; 1],
}
impl ::core::marker::Copy for LOOKUP_STREAM_FROM_CLUSTER_INPUT {}
impl ::core::clone::Clone for LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {
    pub Offset: u32,
    pub NumberOfMatches: u32,
    pub BufferSizeRequired: u32,
}
impl ::core::marker::Copy for LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {}
impl ::core::clone::Clone for LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MARK_HANDLE_INFO {
    pub Anonymous: MARK_HANDLE_INFO_0,
    pub VolumeHandle: super::super::super::Win32::Foundation::HANDLE,
    pub HandleInfo: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MARK_HANDLE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MARK_HANDLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union MARK_HANDLE_INFO_0 {
    pub UsnSourceInfo: u32,
    pub CopyNumber: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MARK_HANDLE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MARK_HANDLE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct MCB {
    pub DummyFieldThatSizesThisStructureCorrectly: LARGE_MCB,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for MCB {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for MCB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub AllocationBase: *mut ::core::ffi::c_void,
    pub AllocationProtect: u32,
    pub RegionSize: usize,
    pub State: u32,
    pub Protect: u32,
    pub Type: u32,
}
impl ::core::marker::Copy for MEMORY_BASIC_INFORMATION {}
impl ::core::clone::Clone for MEMORY_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MEMORY_RANGE_ENTRY {
    pub VirtualAddress: *mut ::core::ffi::c_void,
    pub NumberOfBytes: usize,
}
impl ::core::marker::Copy for MEMORY_RANGE_ENTRY {}
impl ::core::clone::Clone for MEMORY_RANGE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MFT_ENUM_DATA {
    pub StartFileReferenceNumber: u64,
    pub LowUsn: i64,
    pub HighUsn: i64,
    pub MinMajorVersion: u16,
    pub MaxMajorVersion: u16,
}
impl ::core::marker::Copy for MFT_ENUM_DATA {}
impl ::core::clone::Clone for MFT_ENUM_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MFT_ENUM_DATA_V0 {
    pub StartFileReferenceNumber: u64,
    pub LowUsn: i64,
    pub HighUsn: i64,
}
impl ::core::marker::Copy for MFT_ENUM_DATA_V0 {}
impl ::core::clone::Clone for MFT_ENUM_DATA_V0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union MM_PREFETCH_FLAGS {
    pub Flags: MM_PREFETCH_FLAGS_0,
    pub AllFlags: u32,
}
impl ::core::marker::Copy for MM_PREFETCH_FLAGS {}
impl ::core::clone::Clone for MM_PREFETCH_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MM_PREFETCH_FLAGS_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for MM_PREFETCH_FLAGS_0 {}
impl ::core::clone::Clone for MM_PREFETCH_FLAGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MOVE_FILE_DATA {
    pub FileHandle: super::super::super::Win32::Foundation::HANDLE,
    pub StartingVcn: i64,
    pub StartingLcn: i64,
    pub ClusterCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MOVE_FILE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MOVE_FILE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MOVE_FILE_RECORD_DATA {
    pub FileHandle: super::super::super::Win32::Foundation::HANDLE,
    pub SourceFileRecord: i64,
    pub TargetFileRecord: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MOVE_FILE_RECORD_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MOVE_FILE_RECORD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MSV1_0_AV_PAIR {
    pub AvId: u16,
    pub AvLen: u16,
}
impl ::core::marker::Copy for MSV1_0_AV_PAIR {}
impl ::core::clone::Clone for MSV1_0_AV_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MSV1_0_CREDENTIAL_KEY {
    pub Data: [u8; 20],
}
impl ::core::marker::Copy for MSV1_0_CREDENTIAL_KEY {}
impl ::core::clone::Clone for MSV1_0_CREDENTIAL_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MSV1_0_ENUMUSERS_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
}
impl ::core::marker::Copy for MSV1_0_ENUMUSERS_REQUEST {}
impl ::core::clone::Clone for MSV1_0_ENUMUSERS_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_ENUMUSERS_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub NumberOfLoggedOnUsers: u32,
    pub LogonIds: *mut super::super::super::Win32::Foundation::LUID,
    pub EnumHandles: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_ENUMUSERS_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_ENUMUSERS_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_GETCHALLENRESP_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub ParameterControl: u32,
    pub LogonId: super::super::super::Win32::Foundation::LUID,
    pub Password: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
    pub UserName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub LogonDomainName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ServerName: super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_GETCHALLENRESP_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_GETCHALLENRESP_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_GETCHALLENRESP_REQUEST_V1 {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub ParameterControl: u32,
    pub LogonId: super::super::super::Win32::Foundation::LUID,
    pub Password: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_GETCHALLENRESP_REQUEST_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_GETCHALLENRESP_REQUEST_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct MSV1_0_GETCHALLENRESP_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub CaseSensitiveChallengeResponse: super::super::super::Win32::System::Kernel::STRING,
    pub CaseInsensitiveChallengeResponse: super::super::super::Win32::System::Kernel::STRING,
    pub UserName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub LogonDomainName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub UserSessionKey: [u8; 16],
    pub LanmanSessionKey: [u8; 8],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for MSV1_0_GETCHALLENRESP_RESPONSE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for MSV1_0_GETCHALLENRESP_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_GETUSERINFO_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Win32::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_GETUSERINFO_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_GETUSERINFO_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_GETUSERINFO_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub UserSid: super::super::super::Win32::Foundation::PSID,
    pub UserName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub LogonDomainName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub LogonServer: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub LogonType: SECURITY_LOGON_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_GETUSERINFO_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_GETUSERINFO_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_INTERACTIVE_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub UserName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Password: super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_INTERACTIVE_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_INTERACTIVE_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_INTERACTIVE_PROFILE {
    pub MessageType: MSV1_0_PROFILE_BUFFER_TYPE,
    pub LogonCount: u16,
    pub BadPasswordCount: u16,
    pub LogonTime: i64,
    pub LogoffTime: i64,
    pub KickOffTime: i64,
    pub PasswordLastSet: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
    pub LogonScript: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub HomeDirectory: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub FullName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ProfilePath: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub HomeDirectoryDrive: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub LogonServer: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub UserFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_INTERACTIVE_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_INTERACTIVE_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {
    pub Version: u32,
    pub EncryptedCredsSize: u32,
    pub EncryptedCreds: [u8; 1],
}
impl ::core::marker::Copy for MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {}
impl ::core::clone::Clone for MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MSV1_0_LM20_CHALLENGE_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
}
impl ::core::marker::Copy for MSV1_0_LM20_CHALLENGE_REQUEST {}
impl ::core::clone::Clone for MSV1_0_LM20_CHALLENGE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MSV1_0_LM20_CHALLENGE_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub ChallengeToClient: [u8; 8],
}
impl ::core::marker::Copy for MSV1_0_LM20_CHALLENGE_RESPONSE {}
impl ::core::clone::Clone for MSV1_0_LM20_CHALLENGE_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct MSV1_0_LM20_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub UserName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Workstation: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
    pub CaseSensitiveChallengeResponse: super::super::super::Win32::System::Kernel::STRING,
    pub CaseInsensitiveChallengeResponse: super::super::super::Win32::System::Kernel::STRING,
    pub ParameterControl: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for MSV1_0_LM20_LOGON {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for MSV1_0_LM20_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_LM20_LOGON_PROFILE {
    pub MessageType: MSV1_0_PROFILE_BUFFER_TYPE,
    pub KickOffTime: i64,
    pub LogoffTime: i64,
    pub UserFlags: u32,
    pub UserSessionKey: [u8; 16],
    pub LogonDomainName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub LanmanSessionKey: [u8; 8],
    pub LogonServer: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub UserParameters: super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_LM20_LOGON_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_LM20_LOGON_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MSV1_0_NTLM3_RESPONSE {
    pub Response: [u8; 16],
    pub RespType: u8,
    pub HiRespType: u8,
    pub Flags: u16,
    pub MsgWord: u32,
    pub TimeStamp: u64,
    pub ChallengeFromClient: [u8; 8],
    pub AvPairsOff: u32,
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for MSV1_0_NTLM3_RESPONSE {}
impl ::core::clone::Clone for MSV1_0_NTLM3_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL {
    pub Version: u32,
    pub Flags: u32,
    pub CredentialKey: MSV1_0_CREDENTIAL_KEY,
    pub CredentialKeyType: MSV1_0_CREDENTIAL_KEY_TYPE,
    pub EncryptedCredsSize: u32,
    pub EncryptedCreds: [u8; 1],
}
impl ::core::marker::Copy for MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL {}
impl ::core::clone::Clone for MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_S4U_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub UserPrincipalName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub DomainName: super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_S4U_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_S4U_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct MSV1_0_SUBAUTH_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub UserName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Workstation: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
    pub AuthenticationInfo1: super::super::super::Win32::System::Kernel::STRING,
    pub AuthenticationInfo2: super::super::super::Win32::System::Kernel::STRING,
    pub ParameterControl: u32,
    pub SubAuthPackageId: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for MSV1_0_SUBAUTH_LOGON {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for MSV1_0_SUBAUTH_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    pub Version: u32,
    pub Flags: u32,
    pub LmPassword: [u8; 16],
    pub NtPassword: [u8; 16],
}
impl ::core::marker::Copy for MSV1_0_SUPPLEMENTAL_CREDENTIAL {}
impl ::core::clone::Clone for MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {
    pub Version: u32,
    pub Flags: u32,
    pub NtPassword: [u8; 16],
    pub CredentialKey: MSV1_0_CREDENTIAL_KEY,
}
impl ::core::marker::Copy for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {}
impl ::core::clone::Clone for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {
    pub Version: u32,
    pub Flags: u32,
    pub CredentialKeyType: MSV1_0_CREDENTIAL_KEY_TYPE,
    pub NtPassword: [u8; 16],
    pub CredentialKey: MSV1_0_CREDENTIAL_KEY,
    pub ShaPassword: [u8; 20],
}
impl ::core::marker::Copy for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {}
impl ::core::clone::Clone for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NETWORK_APP_INSTANCE_EA {
    pub AppInstanceID: ::windows_sys::core::GUID,
    pub CsvFlags: u32,
}
impl ::core::marker::Copy for NETWORK_APP_INSTANCE_EA {}
impl ::core::clone::Clone for NETWORK_APP_INSTANCE_EA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NETWORK_APP_INSTANCE_ECP_CONTEXT {
    pub Size: u16,
    pub Reserved: u16,
    pub AppInstanceID: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for NETWORK_APP_INSTANCE_ECP_CONTEXT {}
impl ::core::clone::Clone for NETWORK_APP_INSTANCE_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NETWORK_APP_INSTANCE_VERSION_ECP_CONTEXT {
    pub Size: u16,
    pub Reserved: u16,
    pub VersionHigh: u64,
    pub VersionLow: u64,
}
impl ::core::marker::Copy for NETWORK_APP_INSTANCE_VERSION_ECP_CONTEXT {}
impl ::core::clone::Clone for NETWORK_APP_INSTANCE_VERSION_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NETWORK_OPEN_ECP_CONTEXT {
    pub Size: u16,
    pub Reserved: u16,
    pub Anonymous: NETWORK_OPEN_ECP_CONTEXT_0,
}
impl ::core::marker::Copy for NETWORK_OPEN_ECP_CONTEXT {}
impl ::core::clone::Clone for NETWORK_OPEN_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NETWORK_OPEN_ECP_CONTEXT_0 {
    pub r#in: NETWORK_OPEN_ECP_CONTEXT_0_0,
    pub out: NETWORK_OPEN_ECP_CONTEXT_0_1,
}
impl ::core::marker::Copy for NETWORK_OPEN_ECP_CONTEXT_0 {}
impl ::core::clone::Clone for NETWORK_OPEN_ECP_CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NETWORK_OPEN_ECP_CONTEXT_0_0 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
    pub Flags: u32,
}
impl ::core::marker::Copy for NETWORK_OPEN_ECP_CONTEXT_0_0 {}
impl ::core::clone::Clone for NETWORK_OPEN_ECP_CONTEXT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NETWORK_OPEN_ECP_CONTEXT_0_1 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
    pub Flags: u32,
}
impl ::core::marker::Copy for NETWORK_OPEN_ECP_CONTEXT_0_1 {}
impl ::core::clone::Clone for NETWORK_OPEN_ECP_CONTEXT_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0 {
    pub Size: u16,
    pub Reserved: u16,
    pub Anonymous: NETWORK_OPEN_ECP_CONTEXT_V0_0,
}
impl ::core::marker::Copy for NETWORK_OPEN_ECP_CONTEXT_V0 {}
impl ::core::clone::Clone for NETWORK_OPEN_ECP_CONTEXT_V0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0_0 {
    pub r#in: NETWORK_OPEN_ECP_CONTEXT_V0_0_0,
    pub out: NETWORK_OPEN_ECP_CONTEXT_V0_0_1,
}
impl ::core::marker::Copy for NETWORK_OPEN_ECP_CONTEXT_V0_0 {}
impl ::core::clone::Clone for NETWORK_OPEN_ECP_CONTEXT_V0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0_0_0 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
}
impl ::core::marker::Copy for NETWORK_OPEN_ECP_CONTEXT_V0_0_0 {}
impl ::core::clone::Clone for NETWORK_OPEN_ECP_CONTEXT_V0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0_0_1 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
}
impl ::core::marker::Copy for NETWORK_OPEN_ECP_CONTEXT_V0_0_1 {}
impl ::core::clone::Clone for NETWORK_OPEN_ECP_CONTEXT_V0_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NFS_OPEN_ECP_CONTEXT {
    pub ExportAlias: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ClientSocketAddress: *mut SOCKADDR_STORAGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NFS_OPEN_ECP_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NFS_OPEN_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NLSTABLEINFO {
    pub OemTableInfo: CPTABLEINFO,
    pub AnsiTableInfo: CPTABLEINFO,
    pub UpperCaseTable: *mut u16,
    pub LowerCaseTable: *mut u16,
}
impl ::core::marker::Copy for NLSTABLEINFO {}
impl ::core::clone::Clone for NLSTABLEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_EXTENDED_VOLUME_DATA {
    pub ByteCount: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BytesPerPhysicalSector: u32,
    pub LfsMajorVersion: u16,
    pub LfsMinorVersion: u16,
    pub MaxDeviceTrimExtentCount: u32,
    pub MaxDeviceTrimByteCount: u32,
    pub MaxVolumeTrimExtentCount: u32,
    pub MaxVolumeTrimByteCount: u32,
}
impl ::core::marker::Copy for NTFS_EXTENDED_VOLUME_DATA {}
impl ::core::clone::Clone for NTFS_EXTENDED_VOLUME_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_FILE_RECORD_INPUT_BUFFER {
    pub FileReferenceNumber: i64,
}
impl ::core::marker::Copy for NTFS_FILE_RECORD_INPUT_BUFFER {}
impl ::core::clone::Clone for NTFS_FILE_RECORD_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_FILE_RECORD_OUTPUT_BUFFER {
    pub FileReferenceNumber: i64,
    pub FileRecordLength: u32,
    pub FileRecordBuffer: [u8; 1],
}
impl ::core::marker::Copy for NTFS_FILE_RECORD_OUTPUT_BUFFER {}
impl ::core::clone::Clone for NTFS_FILE_RECORD_OUTPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_STATISTICS {
    pub LogFileFullExceptions: u32,
    pub OtherExceptions: u32,
    pub MftReads: u32,
    pub MftReadBytes: u32,
    pub MftWrites: u32,
    pub MftWriteBytes: u32,
    pub MftWritesUserLevel: NTFS_STATISTICS_4,
    pub MftWritesFlushForLogFileFull: u16,
    pub MftWritesLazyWriter: u16,
    pub MftWritesUserRequest: u16,
    pub Mft2Writes: u32,
    pub Mft2WriteBytes: u32,
    pub Mft2WritesUserLevel: NTFS_STATISTICS_2,
    pub Mft2WritesFlushForLogFileFull: u16,
    pub Mft2WritesLazyWriter: u16,
    pub Mft2WritesUserRequest: u16,
    pub RootIndexReads: u32,
    pub RootIndexReadBytes: u32,
    pub RootIndexWrites: u32,
    pub RootIndexWriteBytes: u32,
    pub BitmapReads: u32,
    pub BitmapReadBytes: u32,
    pub BitmapWrites: u32,
    pub BitmapWriteBytes: u32,
    pub BitmapWritesFlushForLogFileFull: u16,
    pub BitmapWritesLazyWriter: u16,
    pub BitmapWritesUserRequest: u16,
    pub BitmapWritesUserLevel: NTFS_STATISTICS_1,
    pub MftBitmapReads: u32,
    pub MftBitmapReadBytes: u32,
    pub MftBitmapWrites: u32,
    pub MftBitmapWriteBytes: u32,
    pub MftBitmapWritesFlushForLogFileFull: u16,
    pub MftBitmapWritesLazyWriter: u16,
    pub MftBitmapWritesUserRequest: u16,
    pub MftBitmapWritesUserLevel: NTFS_STATISTICS_3,
    pub UserIndexReads: u32,
    pub UserIndexReadBytes: u32,
    pub UserIndexWrites: u32,
    pub UserIndexWriteBytes: u32,
    pub LogFileReads: u32,
    pub LogFileReadBytes: u32,
    pub LogFileWrites: u32,
    pub LogFileWriteBytes: u32,
    pub Allocate: NTFS_STATISTICS_0,
    pub DiskResourcesExhausted: u32,
}
impl ::core::marker::Copy for NTFS_STATISTICS {}
impl ::core::clone::Clone for NTFS_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_STATISTICS_0 {
    pub Calls: u32,
    pub Clusters: u32,
    pub Hints: u32,
    pub RunsReturned: u32,
    pub HintsHonored: u32,
    pub HintsClusters: u32,
    pub Cache: u32,
    pub CacheClusters: u32,
    pub CacheMiss: u32,
    pub CacheMissClusters: u32,
}
impl ::core::marker::Copy for NTFS_STATISTICS_0 {}
impl ::core::clone::Clone for NTFS_STATISTICS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_STATISTICS_1 {
    pub Write: u16,
    pub Create: u16,
    pub SetInfo: u16,
}
impl ::core::marker::Copy for NTFS_STATISTICS_1 {}
impl ::core::clone::Clone for NTFS_STATISTICS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_STATISTICS_2 {
    pub Write: u16,
    pub Create: u16,
    pub SetInfo: u16,
    pub Flush: u16,
}
impl ::core::marker::Copy for NTFS_STATISTICS_2 {}
impl ::core::clone::Clone for NTFS_STATISTICS_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_STATISTICS_3 {
    pub Write: u16,
    pub Create: u16,
    pub SetInfo: u16,
    pub Flush: u16,
}
impl ::core::marker::Copy for NTFS_STATISTICS_3 {}
impl ::core::clone::Clone for NTFS_STATISTICS_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_STATISTICS_4 {
    pub Write: u16,
    pub Create: u16,
    pub SetInfo: u16,
    pub Flush: u16,
}
impl ::core::marker::Copy for NTFS_STATISTICS_4 {}
impl ::core::clone::Clone for NTFS_STATISTICS_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_STATISTICS_EX {
    pub LogFileFullExceptions: u32,
    pub OtherExceptions: u32,
    pub MftReads: u64,
    pub MftReadBytes: u64,
    pub MftWrites: u64,
    pub MftWriteBytes: u64,
    pub MftWritesUserLevel: NTFS_STATISTICS_EX_4,
    pub MftWritesFlushForLogFileFull: u32,
    pub MftWritesLazyWriter: u32,
    pub MftWritesUserRequest: u32,
    pub Mft2Writes: u64,
    pub Mft2WriteBytes: u64,
    pub Mft2WritesUserLevel: NTFS_STATISTICS_EX_2,
    pub Mft2WritesFlushForLogFileFull: u32,
    pub Mft2WritesLazyWriter: u32,
    pub Mft2WritesUserRequest: u32,
    pub RootIndexReads: u64,
    pub RootIndexReadBytes: u64,
    pub RootIndexWrites: u64,
    pub RootIndexWriteBytes: u64,
    pub BitmapReads: u64,
    pub BitmapReadBytes: u64,
    pub BitmapWrites: u64,
    pub BitmapWriteBytes: u64,
    pub BitmapWritesFlushForLogFileFull: u32,
    pub BitmapWritesLazyWriter: u32,
    pub BitmapWritesUserRequest: u32,
    pub BitmapWritesUserLevel: NTFS_STATISTICS_EX_1,
    pub MftBitmapReads: u64,
    pub MftBitmapReadBytes: u64,
    pub MftBitmapWrites: u64,
    pub MftBitmapWriteBytes: u64,
    pub MftBitmapWritesFlushForLogFileFull: u32,
    pub MftBitmapWritesLazyWriter: u32,
    pub MftBitmapWritesUserRequest: u32,
    pub MftBitmapWritesUserLevel: NTFS_STATISTICS_EX_3,
    pub UserIndexReads: u64,
    pub UserIndexReadBytes: u64,
    pub UserIndexWrites: u64,
    pub UserIndexWriteBytes: u64,
    pub LogFileReads: u64,
    pub LogFileReadBytes: u64,
    pub LogFileWrites: u64,
    pub LogFileWriteBytes: u64,
    pub Allocate: NTFS_STATISTICS_EX_0,
    pub DiskResourcesExhausted: u32,
    pub VolumeTrimCount: u64,
    pub VolumeTrimTime: u64,
    pub VolumeTrimByteCount: u64,
    pub FileLevelTrimCount: u64,
    pub FileLevelTrimTime: u64,
    pub FileLevelTrimByteCount: u64,
    pub VolumeTrimSkippedCount: u64,
    pub VolumeTrimSkippedByteCount: u64,
    pub NtfsFillStatInfoFromMftRecordCalledCount: u64,
    pub NtfsFillStatInfoFromMftRecordBailedBecauseOfAttributeListCount: u64,
    pub NtfsFillStatInfoFromMftRecordBailedBecauseOfNonResReparsePointCount: u64,
}
impl ::core::marker::Copy for NTFS_STATISTICS_EX {}
impl ::core::clone::Clone for NTFS_STATISTICS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_STATISTICS_EX_0 {
    pub Calls: u32,
    pub RunsReturned: u32,
    pub Hints: u32,
    pub HintsHonored: u32,
    pub Cache: u32,
    pub CacheMiss: u32,
    pub Clusters: u64,
    pub HintsClusters: u64,
    pub CacheClusters: u64,
    pub CacheMissClusters: u64,
}
impl ::core::marker::Copy for NTFS_STATISTICS_EX_0 {}
impl ::core::clone::Clone for NTFS_STATISTICS_EX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_STATISTICS_EX_1 {
    pub Write: u32,
    pub Create: u32,
    pub SetInfo: u32,
    pub Flush: u32,
}
impl ::core::marker::Copy for NTFS_STATISTICS_EX_1 {}
impl ::core::clone::Clone for NTFS_STATISTICS_EX_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_STATISTICS_EX_2 {
    pub Write: u32,
    pub Create: u32,
    pub SetInfo: u32,
    pub Flush: u32,
}
impl ::core::marker::Copy for NTFS_STATISTICS_EX_2 {}
impl ::core::clone::Clone for NTFS_STATISTICS_EX_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_STATISTICS_EX_3 {
    pub Write: u32,
    pub Create: u32,
    pub SetInfo: u32,
    pub Flush: u32,
}
impl ::core::marker::Copy for NTFS_STATISTICS_EX_3 {}
impl ::core::clone::Clone for NTFS_STATISTICS_EX_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_STATISTICS_EX_4 {
    pub Write: u32,
    pub Create: u32,
    pub SetInfo: u32,
    pub Flush: u32,
}
impl ::core::marker::Copy for NTFS_STATISTICS_EX_4 {}
impl ::core::clone::Clone for NTFS_STATISTICS_EX_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct NTFS_VOLUME_DATA_BUFFER {
    pub VolumeSerialNumber: i64,
    pub NumberSectors: i64,
    pub TotalClusters: i64,
    pub FreeClusters: i64,
    pub TotalReserved: i64,
    pub BytesPerSector: u32,
    pub BytesPerCluster: u32,
    pub BytesPerFileRecordSegment: u32,
    pub ClustersPerFileRecordSegment: u32,
    pub MftValidDataLength: i64,
    pub MftStartLcn: i64,
    pub Mft2StartLcn: i64,
    pub MftZoneStart: i64,
    pub MftZoneEnd: i64,
}
impl ::core::marker::Copy for NTFS_VOLUME_DATA_BUFFER {}
impl ::core::clone::Clone for NTFS_VOLUME_DATA_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct OBJECT_TYPE_LIST {
    pub Level: u16,
    pub Sbz: u16,
    pub ObjectType: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for OBJECT_TYPE_LIST {}
impl ::core::clone::Clone for OBJECT_TYPE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
pub struct OPEN_REPARSE_LIST {
    pub OpenReparseList: super::super::super::Win32::System::Kernel::LIST_ENTRY,
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for OPEN_REPARSE_LIST {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for OPEN_REPARSE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
pub struct OPEN_REPARSE_LIST_ENTRY {
    pub OpenReparseListEntry: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub ReparseTag: u32,
    pub Flags: u32,
    pub ReparseGuid: ::windows_sys::core::GUID,
    pub Size: u16,
    pub RemainingLength: u16,
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for OPEN_REPARSE_LIST_ENTRY {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for OPEN_REPARSE_LIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct OPLOCK_KEY_CONTEXT {
    pub Version: u16,
    pub Flags: u16,
    pub ParentOplockKey: ::windows_sys::core::GUID,
    pub TargetOplockKey: ::windows_sys::core::GUID,
    pub Reserved: u32,
}
impl ::core::marker::Copy for OPLOCK_KEY_CONTEXT {}
impl ::core::clone::Clone for OPLOCK_KEY_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct OPLOCK_KEY_ECP_CONTEXT {
    pub OplockKey: ::windows_sys::core::GUID,
    pub Reserved: u32,
}
impl ::core::marker::Copy for OPLOCK_KEY_ECP_CONTEXT {}
impl ::core::clone::Clone for OPLOCK_KEY_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub struct OPLOCK_NOTIFY_PARAMS {
    pub NotifyReason: OPLOCK_NOTIFY_REASON,
    pub NotifyContext: *mut ::core::ffi::c_void,
    pub Irp: *mut super::super::Foundation::IRP,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for OPLOCK_NOTIFY_PARAMS {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for OPLOCK_NOTIFY_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct PATHNAME_BUFFER {
    pub PathNameLength: u32,
    pub Name: [u16; 1],
}
impl ::core::marker::Copy for PATHNAME_BUFFER {}
impl ::core::clone::Clone for PATHNAME_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct PHYSICAL_EXTENTS_DESCRIPTOR {
    pub NumberOfRuns: u32,
    pub NumberOfValidRuns: u32,
    pub Run: [PHYSICAL_MEMORY_RUN; 1],
}
impl ::core::marker::Copy for PHYSICAL_EXTENTS_DESCRIPTOR {}
impl ::core::clone::Clone for PHYSICAL_EXTENTS_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct PHYSICAL_MEMORY_DESCRIPTOR {
    pub NumberOfRuns: u32,
    pub NumberOfPages: u32,
    pub Run: [PHYSICAL_MEMORY_RUN; 1],
}
impl ::core::marker::Copy for PHYSICAL_MEMORY_DESCRIPTOR {}
impl ::core::clone::Clone for PHYSICAL_MEMORY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct PHYSICAL_MEMORY_RUN {
    pub BasePage: u32,
    pub PageCount: u32,
}
impl ::core::marker::Copy for PHYSICAL_MEMORY_RUN {}
impl ::core::clone::Clone for PHYSICAL_MEMORY_RUN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct PLEX_READ_DATA_REQUEST {
    pub ByteOffset: i64,
    pub ByteLength: u32,
    pub PlexNumber: u32,
}
impl ::core::marker::Copy for PLEX_READ_DATA_REQUEST {}
impl ::core::clone::Clone for PLEX_READ_DATA_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct PREFETCH_OPEN_ECP_CONTEXT {
    pub Context: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for PREFETCH_OPEN_ECP_CONTEXT {}
impl ::core::clone::Clone for PREFETCH_OPEN_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
pub struct PREFIX_TABLE {
    pub NodeTypeCode: i16,
    pub NameLength: i16,
    pub NextPrefixTree: *mut PREFIX_TABLE_ENTRY,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for PREFIX_TABLE {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for PREFIX_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
pub struct PREFIX_TABLE_ENTRY {
    pub NodeTypeCode: i16,
    pub NameLength: i16,
    pub NextPrefixTree: *mut PREFIX_TABLE_ENTRY,
    pub Links: super::super::Foundation::RTL_SPLAY_LINKS,
    pub Prefix: *mut super::super::super::Win32::System::Kernel::STRING,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for PREFIX_TABLE_ENTRY {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for PREFIX_TABLE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct PUBLIC_BCB {
    pub NodeTypeCode: i16,
    pub NodeByteSize: i16,
    pub MappedLength: u32,
    pub MappedFileOffset: i64,
}
impl ::core::marker::Copy for PUBLIC_BCB {}
impl ::core::clone::Clone for PUBLIC_BCB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct PUBLIC_OBJECT_BASIC_INFORMATION {
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
    pub Reserved: [u32; 10],
}
impl ::core::marker::Copy for PUBLIC_OBJECT_BASIC_INFORMATION {}
impl ::core::clone::Clone for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PUBLIC_OBJECT_TYPE_INFORMATION {
    pub TypeName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Reserved: [u32; 22],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PUBLIC_OBJECT_TYPE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct QUERY_BAD_RANGES_INPUT {
    pub Flags: u32,
    pub NumRanges: u32,
    pub Ranges: [QUERY_BAD_RANGES_INPUT_RANGE; 1],
}
impl ::core::marker::Copy for QUERY_BAD_RANGES_INPUT {}
impl ::core::clone::Clone for QUERY_BAD_RANGES_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct QUERY_BAD_RANGES_INPUT_RANGE {
    pub StartOffset: u64,
    pub LengthInBytes: u64,
}
impl ::core::marker::Copy for QUERY_BAD_RANGES_INPUT_RANGE {}
impl ::core::clone::Clone for QUERY_BAD_RANGES_INPUT_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct QUERY_BAD_RANGES_OUTPUT {
    pub Flags: u32,
    pub NumBadRanges: u32,
    pub NextOffsetToLookUp: u64,
    pub BadRanges: [QUERY_BAD_RANGES_OUTPUT_RANGE; 1],
}
impl ::core::marker::Copy for QUERY_BAD_RANGES_OUTPUT {}
impl ::core::clone::Clone for QUERY_BAD_RANGES_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct QUERY_BAD_RANGES_OUTPUT_RANGE {
    pub Flags: u32,
    pub Reserved: u32,
    pub StartOffset: u64,
    pub LengthInBytes: u64,
}
impl ::core::marker::Copy for QUERY_BAD_RANGES_OUTPUT_RANGE {}
impl ::core::clone::Clone for QUERY_BAD_RANGES_OUTPUT_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct QUERY_DIRECT_ACCESS_EXTENTS {
    pub FileOffset: i64,
    pub Length: i64,
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for QUERY_DIRECT_ACCESS_EXTENTS {}
impl ::core::clone::Clone for QUERY_DIRECT_ACCESS_EXTENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct QUERY_FILE_LAYOUT_INPUT {
    pub Anonymous: QUERY_FILE_LAYOUT_INPUT_0,
    pub Flags: u32,
    pub FilterType: QUERY_FILE_LAYOUT_FILTER_TYPE,
    pub Reserved: u32,
    pub Filter: QUERY_FILE_LAYOUT_INPUT_1,
}
impl ::core::marker::Copy for QUERY_FILE_LAYOUT_INPUT {}
impl ::core::clone::Clone for QUERY_FILE_LAYOUT_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union QUERY_FILE_LAYOUT_INPUT_0 {
    pub FilterEntryCount: u32,
    pub NumberOfPairs: u32,
}
impl ::core::marker::Copy for QUERY_FILE_LAYOUT_INPUT_0 {}
impl ::core::clone::Clone for QUERY_FILE_LAYOUT_INPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union QUERY_FILE_LAYOUT_INPUT_1 {
    pub ClusterRanges: [CLUSTER_RANGE; 1],
    pub FileReferenceRanges: [FILE_REFERENCE_RANGE; 1],
    pub StorageReserveIds: [STORAGE_RESERVE_ID; 1],
}
impl ::core::marker::Copy for QUERY_FILE_LAYOUT_INPUT_1 {}
impl ::core::clone::Clone for QUERY_FILE_LAYOUT_INPUT_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct QUERY_FILE_LAYOUT_OUTPUT {
    pub FileEntryCount: u32,
    pub FirstFileOffset: u32,
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for QUERY_FILE_LAYOUT_OUTPUT {}
impl ::core::clone::Clone for QUERY_FILE_LAYOUT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct QUERY_ON_CREATE_EA_INFORMATION {
    pub EaBufferSize: u32,
    pub EaBuffer: *mut FILE_FULL_EA_INFORMATION,
}
impl ::core::marker::Copy for QUERY_ON_CREATE_EA_INFORMATION {}
impl ::core::clone::Clone for QUERY_ON_CREATE_EA_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct QUERY_ON_CREATE_ECP_CONTEXT {
    pub RequestedClasses: u32,
    pub ClassesProcessed: u32,
    pub ClassesWithErrors: u32,
    pub ClassesWithNoData: u32,
    pub StatInformation: QUERY_ON_CREATE_FILE_STAT_INFORMATION,
    pub LxInformation: QUERY_ON_CREATE_FILE_LX_INFORMATION,
    pub EaInformation: QUERY_ON_CREATE_EA_INFORMATION,
}
impl ::core::marker::Copy for QUERY_ON_CREATE_ECP_CONTEXT {}
impl ::core::clone::Clone for QUERY_ON_CREATE_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct QUERY_ON_CREATE_FILE_LX_INFORMATION {
    pub EffectiveAccess: u32,
    pub LxFlags: u32,
    pub LxUid: u32,
    pub LxGid: u32,
    pub LxMode: u32,
    pub LxDeviceIdMajor: u32,
    pub LxDeviceIdMinor: u32,
}
impl ::core::marker::Copy for QUERY_ON_CREATE_FILE_LX_INFORMATION {}
impl ::core::clone::Clone for QUERY_ON_CREATE_FILE_LX_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct QUERY_ON_CREATE_FILE_STAT_INFORMATION {
    pub FileId: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
    pub ReparseTag: u32,
    pub NumberOfLinks: u32,
}
impl ::core::marker::Copy for QUERY_ON_CREATE_FILE_STAT_INFORMATION {}
impl ::core::clone::Clone for QUERY_ON_CREATE_FILE_STAT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct QUERY_PATH_REQUEST {
    pub PathNameLength: u32,
    pub SecurityContext: *mut super::super::Foundation::IO_SECURITY_CONTEXT,
    pub FilePathName: [u16; 1],
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for QUERY_PATH_REQUEST {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for QUERY_PATH_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct QUERY_PATH_REQUEST_EX {
    pub pSecurityContext: *mut super::super::Foundation::IO_SECURITY_CONTEXT,
    pub EaLength: u32,
    pub pEaBuffer: *mut ::core::ffi::c_void,
    pub PathName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub DomainServiceName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub EcpList: *mut super::super::Foundation::ECP_LIST,
    pub Silo: *mut super::super::Foundation::EJOB,
    pub Reserved: usize,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for QUERY_PATH_REQUEST_EX {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for QUERY_PATH_REQUEST_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct QUERY_PATH_RESPONSE {
    pub LengthAccepted: u32,
}
impl ::core::marker::Copy for QUERY_PATH_RESPONSE {}
impl ::core::clone::Clone for QUERY_PATH_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct READ_AHEAD_PARAMETERS {
    pub NodeByteSize: i16,
    pub Granularity: u32,
    pub PipelinedRequestSize: u32,
    pub ReadAheadGrowthPercentage: u32,
}
impl ::core::marker::Copy for READ_AHEAD_PARAMETERS {}
impl ::core::clone::Clone for READ_AHEAD_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct READ_FILE_USN_DATA {
    pub MinMajorVersion: u16,
    pub MaxMajorVersion: u16,
}
impl ::core::marker::Copy for READ_FILE_USN_DATA {}
impl ::core::clone::Clone for READ_FILE_USN_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub struct READ_LIST {
    pub FileObject: *mut super::super::Foundation::FILE_OBJECT,
    pub NumberOfEntries: u32,
    pub IsImage: u32,
    pub List: [super::super::super::Win32::Storage::FileSystem::FILE_SEGMENT_ELEMENT; 1],
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for READ_LIST {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for READ_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct READ_USN_JOURNAL_DATA {
    pub StartUsn: i64,
    pub ReasonMask: u32,
    pub ReturnOnlyOnClose: u32,
    pub Timeout: u64,
    pub BytesToWaitFor: u64,
    pub UsnJournalID: u64,
    pub MinMajorVersion: u16,
    pub MaxMajorVersion: u16,
}
impl ::core::marker::Copy for READ_USN_JOURNAL_DATA {}
impl ::core::clone::Clone for READ_USN_JOURNAL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct READ_USN_JOURNAL_DATA_V0 {
    pub StartUsn: i64,
    pub ReasonMask: u32,
    pub ReturnOnlyOnClose: u32,
    pub Timeout: u64,
    pub BytesToWaitFor: u64,
    pub UsnJournalID: u64,
}
impl ::core::marker::Copy for READ_USN_JOURNAL_DATA_V0 {}
impl ::core::clone::Clone for READ_USN_JOURNAL_DATA_V0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REARRANGE_FILE_DATA {
    pub SourceStartingOffset: u64,
    pub TargetOffset: u64,
    pub SourceFileHandle: super::super::super::Win32::Foundation::HANDLE,
    pub Length: u32,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REARRANGE_FILE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REARRANGE_FILE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REFS_DEALLOCATE_RANGES_INPUT_BUFFER {
    pub RangeCount: u32,
    pub Ranges: [REFS_DEALLOCATE_RANGES_RANGE; 1],
}
impl ::core::marker::Copy for REFS_DEALLOCATE_RANGES_INPUT_BUFFER {}
impl ::core::clone::Clone for REFS_DEALLOCATE_RANGES_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REFS_DEALLOCATE_RANGES_INPUT_BUFFER_EX {
    pub RangeCount: u32,
    pub Allocator: REFS_DEALLOCATE_RANGES_ALLOCATOR,
    pub StreamReserveUpdateCount: i64,
    pub OffsetToRanges: u32,
    pub OffsetToLeakCounts: u32,
    pub Reserved: [u64; 2],
}
impl ::core::marker::Copy for REFS_DEALLOCATE_RANGES_INPUT_BUFFER_EX {}
impl ::core::clone::Clone for REFS_DEALLOCATE_RANGES_INPUT_BUFFER_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REFS_DEALLOCATE_RANGES_RANGE {
    pub StartOfRange: u64,
    pub CountOfRange: u64,
}
impl ::core::marker::Copy for REFS_DEALLOCATE_RANGES_RANGE {}
impl ::core::clone::Clone for REFS_DEALLOCATE_RANGES_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER {
    pub DefaultCompressionFormat: REFS_COMPRESSION_FORMATS,
    pub DefaultCompressionLevel: i16,
    pub DefaultCompressionChunkSizeBytes: u32,
    pub VolumeClusterSizeBytes: u32,
    pub TotalVolumeClusters: u64,
    pub TotalAllocatedClusters: u64,
    pub TotalCompressibleClustersAllocated: u64,
    pub TotalCompressibleClustersInUse: u64,
    pub TotalCompressedClusters: u64,
    pub Reserved: [u64; 6],
}
impl ::core::marker::Copy for REFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER {}
impl ::core::clone::Clone for REFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REFS_QUERY_VOLUME_DEDUP_INFO_OUTPUT_BUFFER {
    pub Enabled: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REFS_QUERY_VOLUME_DEDUP_INFO_OUTPUT_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REFS_QUERY_VOLUME_DEDUP_INFO_OUTPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REFS_REMOVE_HARDLINK_BACKPOINTER {
    pub ParentDirectory: u64,
    pub Reserved: u64,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for REFS_REMOVE_HARDLINK_BACKPOINTER {}
impl ::core::clone::Clone for REFS_REMOVE_HARDLINK_BACKPOINTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REFS_SET_VOLUME_COMPRESSION_INFO_INPUT_BUFFER {
    pub CompressionFormat: REFS_COMPRESSION_FORMATS,
    pub CompressionLevel: i16,
    pub CompressionChunkSizeBytes: u32,
    pub Flags: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS,
    pub Reserved: [u64; 8],
}
impl ::core::marker::Copy for REFS_SET_VOLUME_COMPRESSION_INFO_INPUT_BUFFER {}
impl ::core::clone::Clone for REFS_SET_VOLUME_COMPRESSION_INFO_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REFS_SET_VOLUME_DEDUP_INFO_INPUT_BUFFER {
    pub Enable: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REFS_SET_VOLUME_DEDUP_INFO_INPUT_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REFS_SET_VOLUME_DEDUP_INFO_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REFS_SMR_VOLUME_GC_PARAMETERS {
    pub Version: u32,
    pub Flags: u32,
    pub Action: REFS_SMR_VOLUME_GC_ACTION,
    pub Method: REFS_SMR_VOLUME_GC_METHOD,
    pub IoGranularity: u32,
    pub CompressionFormat: u32,
    pub Unused: [u64; 8],
}
impl ::core::marker::Copy for REFS_SMR_VOLUME_GC_PARAMETERS {}
impl ::core::clone::Clone for REFS_SMR_VOLUME_GC_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REFS_SMR_VOLUME_INFO_OUTPUT {
    pub Version: u32,
    pub Flags: u32,
    pub SizeOfRandomlyWritableTier: i64,
    pub FreeSpaceInRandomlyWritableTier: i64,
    pub SizeofSMRTier: i64,
    pub FreeSpaceInSMRTier: i64,
    pub UsableFreeSpaceInSMRTier: i64,
    pub VolumeGcState: REFS_SMR_VOLUME_GC_STATE,
    pub VolumeGcLastStatus: super::super::super::Win32::Foundation::NTSTATUS,
    pub CurrentGcBandFillPercentage: u32,
    pub Unused: [u64; 6],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REFS_SMR_VOLUME_INFO_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REFS_SMR_VOLUME_INFO_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REFS_STREAM_EXTENT {
    pub Vcn: i64,
    pub Lcn: i64,
    pub Length: i64,
    pub Properties: u16,
}
impl ::core::marker::Copy for REFS_STREAM_EXTENT {}
impl ::core::clone::Clone for REFS_STREAM_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER {
    pub EntryCount: u32,
    pub BufferSizeRequiredForQuery: u32,
    pub Reserved: [u32; 2],
    pub Entries: [REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER_ENTRY; 1],
}
impl ::core::marker::Copy for REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER {}
impl ::core::clone::Clone for REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER_ENTRY {
    pub NextEntryOffset: u32,
    pub SnapshotNameLength: u16,
    pub SnapshotCreationTime: u64,
    pub StreamSize: u64,
    pub StreamAllocationSize: u64,
    pub Reserved: [u64; 2],
    pub SnapshotName: [u16; 1],
}
impl ::core::marker::Copy for REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER_ENTRY {}
impl ::core::clone::Clone for REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REFS_STREAM_SNAPSHOT_MANAGEMENT_INPUT_BUFFER {
    pub Operation: REFS_STREAM_SNAPSHOT_OPERATION,
    pub SnapshotNameLength: u16,
    pub OperationInputBufferLength: u16,
    pub Reserved: [u64; 2],
    pub NameAndInputBuffer: [u16; 1],
}
impl ::core::marker::Copy for REFS_STREAM_SNAPSHOT_MANAGEMENT_INPUT_BUFFER {}
impl ::core::clone::Clone for REFS_STREAM_SNAPSHOT_MANAGEMENT_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REFS_STREAM_SNAPSHOT_QUERY_DELTAS_INPUT_BUFFER {
    pub StartingVcn: i64,
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for REFS_STREAM_SNAPSHOT_QUERY_DELTAS_INPUT_BUFFER {}
impl ::core::clone::Clone for REFS_STREAM_SNAPSHOT_QUERY_DELTAS_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REFS_STREAM_SNAPSHOT_QUERY_DELTAS_OUTPUT_BUFFER {
    pub ExtentCount: u32,
    pub Reserved: [u32; 2],
    pub Extents: [REFS_STREAM_EXTENT; 1],
}
impl ::core::marker::Copy for REFS_STREAM_SNAPSHOT_QUERY_DELTAS_OUTPUT_BUFFER {}
impl ::core::clone::Clone for REFS_STREAM_SNAPSHOT_QUERY_DELTAS_OUTPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REFS_VOLUME_COUNTER_INFO_INPUT_BUFFER {
    pub ResetCounters: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REFS_VOLUME_COUNTER_INFO_INPUT_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REFS_VOLUME_COUNTER_INFO_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REFS_VOLUME_DATA_BUFFER {
    pub ByteCount: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub BytesPerPhysicalSector: u32,
    pub VolumeSerialNumber: i64,
    pub NumberSectors: i64,
    pub TotalClusters: i64,
    pub FreeClusters: i64,
    pub TotalReserved: i64,
    pub BytesPerSector: u32,
    pub BytesPerCluster: u32,
    pub MaximumSizeOfResidentFile: i64,
    pub FastTierDataFillRatio: u16,
    pub SlowTierDataFillRatio: u16,
    pub DestagesFastTierToSlowTierRate: u32,
    pub Reserved: [i64; 9],
}
impl ::core::marker::Copy for REFS_VOLUME_DATA_BUFFER {}
impl ::core::clone::Clone for REFS_VOLUME_DATA_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REMOTE_LINK_TRACKING_INFORMATION {
    pub TargetFileObject: *mut ::core::ffi::c_void,
    pub TargetLinkTrackingInformationLength: u32,
    pub TargetLinkTrackingInformationBuffer: [u8; 1],
}
impl ::core::marker::Copy for REMOTE_LINK_TRACKING_INFORMATION {}
impl ::core::clone::Clone for REMOTE_LINK_TRACKING_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REPAIR_COPIES_INPUT {
    pub Size: u32,
    pub Flags: u32,
    pub FileOffset: i64,
    pub Length: u32,
    pub SourceCopy: u32,
    pub NumberOfRepairCopies: u32,
    pub RepairCopies: [u32; 1],
}
impl ::core::marker::Copy for REPAIR_COPIES_INPUT {}
impl ::core::clone::Clone for REPAIR_COPIES_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REPAIR_COPIES_OUTPUT {
    pub Size: u32,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
    pub ResumeFileOffset: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REPAIR_COPIES_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REPAIR_COPIES_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REPARSE_DATA_BUFFER {
    pub ReparseTag: u32,
    pub ReparseDataLength: u16,
    pub Reserved: u16,
    pub Anonymous: REPARSE_DATA_BUFFER_0,
}
impl ::core::marker::Copy for REPARSE_DATA_BUFFER {}
impl ::core::clone::Clone for REPARSE_DATA_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union REPARSE_DATA_BUFFER_0 {
    pub SymbolicLinkReparseBuffer: REPARSE_DATA_BUFFER_0_2,
    pub MountPointReparseBuffer: REPARSE_DATA_BUFFER_0_1,
    pub GenericReparseBuffer: REPARSE_DATA_BUFFER_0_0,
}
impl ::core::marker::Copy for REPARSE_DATA_BUFFER_0 {}
impl ::core::clone::Clone for REPARSE_DATA_BUFFER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REPARSE_DATA_BUFFER_0_0 {
    pub DataBuffer: [u8; 1],
}
impl ::core::marker::Copy for REPARSE_DATA_BUFFER_0_0 {}
impl ::core::clone::Clone for REPARSE_DATA_BUFFER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REPARSE_DATA_BUFFER_0_1 {
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub PrintNameOffset: u16,
    pub PrintNameLength: u16,
    pub PathBuffer: [u16; 1],
}
impl ::core::marker::Copy for REPARSE_DATA_BUFFER_0_1 {}
impl ::core::clone::Clone for REPARSE_DATA_BUFFER_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REPARSE_DATA_BUFFER_0_2 {
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub PrintNameOffset: u16,
    pub PrintNameLength: u16,
    pub Flags: u32,
    pub PathBuffer: [u16; 1],
}
impl ::core::marker::Copy for REPARSE_DATA_BUFFER_0_2 {}
impl ::core::clone::Clone for REPARSE_DATA_BUFFER_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REPARSE_DATA_BUFFER_EX {
    pub Flags: u32,
    pub ExistingReparseTag: u32,
    pub ExistingReparseGuid: ::windows_sys::core::GUID,
    pub Reserved: u64,
    pub Anonymous: REPARSE_DATA_BUFFER_EX_0,
}
impl ::core::marker::Copy for REPARSE_DATA_BUFFER_EX {}
impl ::core::clone::Clone for REPARSE_DATA_BUFFER_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union REPARSE_DATA_BUFFER_EX_0 {
    pub ReparseDataBuffer: REPARSE_DATA_BUFFER,
    pub ReparseGuidDataBuffer: REPARSE_GUID_DATA_BUFFER,
}
impl ::core::marker::Copy for REPARSE_DATA_BUFFER_EX_0 {}
impl ::core::clone::Clone for REPARSE_DATA_BUFFER_EX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REPARSE_GUID_DATA_BUFFER {
    pub ReparseTag: u32,
    pub ReparseDataLength: u16,
    pub Reserved: u16,
    pub ReparseGuid: ::windows_sys::core::GUID,
    pub GenericReparseBuffer: REPARSE_GUID_DATA_BUFFER_0,
}
impl ::core::marker::Copy for REPARSE_GUID_DATA_BUFFER {}
impl ::core::clone::Clone for REPARSE_GUID_DATA_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REPARSE_GUID_DATA_BUFFER_0 {
    pub DataBuffer: [u8; 1],
}
impl ::core::marker::Copy for REPARSE_GUID_DATA_BUFFER_0 {}
impl ::core::clone::Clone for REPARSE_GUID_DATA_BUFFER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REPARSE_INDEX_KEY {
    pub FileReparseTag: u32,
    pub FileId: i64,
}
impl ::core::marker::Copy for REPARSE_INDEX_KEY {}
impl ::core::clone::Clone for REPARSE_INDEX_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REQUEST_OPLOCK_INPUT_BUFFER {
    pub StructureVersion: u16,
    pub StructureLength: u16,
    pub RequestedOplockLevel: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for REQUEST_OPLOCK_INPUT_BUFFER {}
impl ::core::clone::Clone for REQUEST_OPLOCK_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REQUEST_OPLOCK_OUTPUT_BUFFER {
    pub StructureVersion: u16,
    pub StructureLength: u16,
    pub OriginalOplockLevel: u32,
    pub NewOplockLevel: u32,
    pub Flags: u32,
    pub AccessMode: u32,
    pub ShareMode: u16,
}
impl ::core::marker::Copy for REQUEST_OPLOCK_OUTPUT_BUFFER {}
impl ::core::clone::Clone for REQUEST_OPLOCK_OUTPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct REQUEST_RAW_ENCRYPTED_DATA {
    pub FileOffset: i64,
    pub Length: u32,
}
impl ::core::marker::Copy for REQUEST_RAW_ENCRYPTED_DATA {}
impl ::core::clone::Clone for REQUEST_RAW_ENCRYPTED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    pub ExtentCount: u32,
    pub StartingVcn: i64,
    pub Extents: [RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0; 1],
}
impl ::core::marker::Copy for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {}
impl ::core::clone::Clone for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    pub NextVcn: i64,
    pub Lcn: i64,
    pub ReferenceCount: u32,
}
impl ::core::marker::Copy for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {}
impl ::core::clone::Clone for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct RETRIEVAL_POINTERS_BUFFER {
    pub ExtentCount: u32,
    pub StartingVcn: i64,
    pub Extents: [RETRIEVAL_POINTERS_BUFFER_0; 1],
}
impl ::core::marker::Copy for RETRIEVAL_POINTERS_BUFFER {}
impl ::core::clone::Clone for RETRIEVAL_POINTERS_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct RETRIEVAL_POINTERS_BUFFER_0 {
    pub NextVcn: i64,
    pub Lcn: i64,
}
impl ::core::marker::Copy for RETRIEVAL_POINTERS_BUFFER_0 {}
impl ::core::clone::Clone for RETRIEVAL_POINTERS_BUFFER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct RETRIEVAL_POINTER_BASE {
    pub FileAreaOffset: i64,
}
impl ::core::marker::Copy for RETRIEVAL_POINTER_BASE {}
impl ::core::clone::Clone for RETRIEVAL_POINTER_BASE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct RETRIEVAL_POINTER_COUNT {
    pub ExtentCount: u32,
}
impl ::core::marker::Copy for RETRIEVAL_POINTER_COUNT {}
impl ::core::clone::Clone for RETRIEVAL_POINTER_COUNT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct RKF_BYPASS_ECP_CONTEXT {
    pub Reserved: i32,
    pub Version: i32,
}
impl ::core::marker::Copy for RKF_BYPASS_ECP_CONTEXT {}
impl ::core::clone::Clone for RKF_BYPASS_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct RTL_HEAP_MEMORY_LIMIT_DATA {
    pub CommitLimitBytes: usize,
    pub CommitLimitFailureCode: usize,
    pub MaxAllocationSizeBytes: usize,
    pub AllocationLimitFailureCode: usize,
}
impl ::core::marker::Copy for RTL_HEAP_MEMORY_LIMIT_DATA {}
impl ::core::clone::Clone for RTL_HEAP_MEMORY_LIMIT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct RTL_HEAP_MEMORY_LIMIT_INFO {
    pub Version: u32,
    pub Data: RTL_HEAP_MEMORY_LIMIT_DATA,
}
impl ::core::marker::Copy for RTL_HEAP_MEMORY_LIMIT_INFO {}
impl ::core::clone::Clone for RTL_HEAP_MEMORY_LIMIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RTL_HEAP_PARAMETERS {
    pub Length: u32,
    pub SegmentReserve: usize,
    pub SegmentCommit: usize,
    pub DeCommitFreeBlockThreshold: usize,
    pub DeCommitTotalFreeThreshold: usize,
    pub MaximumAllocationSize: usize,
    pub VirtualMemoryThreshold: usize,
    pub InitialCommit: usize,
    pub InitialReserve: usize,
    pub CommitRoutine: PRTL_HEAP_COMMIT_ROUTINE,
    pub Reserved: [usize; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RTL_HEAP_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RTL_HEAP_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct RTL_NLS_STATE {
    pub DefaultAcpTableInfo: CPTABLEINFO,
    pub DefaultOemTableInfo: CPTABLEINFO,
    pub ActiveCodePageData: *mut u16,
    pub OemCodePageData: *mut u16,
    pub LeadByteInfo: *mut u16,
    pub OemLeadByteInfo: *mut u16,
    pub CaseMappingData: *mut u16,
    pub UnicodeUpcaseTable844: *mut u16,
    pub UnicodeLowercaseTable844: *mut u16,
}
impl ::core::marker::Copy for RTL_NLS_STATE {}
impl ::core::clone::Clone for RTL_NLS_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RTL_SEGMENT_HEAP_MEMORY_SOURCE {
    pub Flags: u32,
    pub MemoryTypeMask: u32,
    pub NumaNode: u32,
    pub Anonymous: RTL_SEGMENT_HEAP_MEMORY_SOURCE_0,
    pub Reserved: [usize; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RTL_SEGMENT_HEAP_MEMORY_SOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RTL_SEGMENT_HEAP_MEMORY_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union RTL_SEGMENT_HEAP_MEMORY_SOURCE_0 {
    pub PartitionHandle: super::super::super::Win32::Foundation::HANDLE,
    pub Callbacks: *mut RTL_SEGMENT_HEAP_VA_CALLBACKS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RTL_SEGMENT_HEAP_MEMORY_SOURCE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RTL_SEGMENT_HEAP_MEMORY_SOURCE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RTL_SEGMENT_HEAP_PARAMETERS {
    pub Version: u16,
    pub Size: u16,
    pub Flags: u32,
    pub MemorySource: RTL_SEGMENT_HEAP_MEMORY_SOURCE,
    pub Reserved: [usize; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RTL_SEGMENT_HEAP_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RTL_SEGMENT_HEAP_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RTL_SEGMENT_HEAP_VA_CALLBACKS {
    pub CallbackContext: super::super::super::Win32::Foundation::HANDLE,
    pub AllocateVirtualMemory: PALLOCATE_VIRTUAL_MEMORY_EX_CALLBACK,
    pub FreeVirtualMemory: PFREE_VIRTUAL_MEMORY_EX_CALLBACK,
    pub QueryVirtualMemory: PQUERY_VIRTUAL_MEMORY_CALLBACK,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RTL_SEGMENT_HEAP_VA_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RTL_SEGMENT_HEAP_VA_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SCRUB_DATA_INPUT {
    pub Size: u32,
    pub Flags: u32,
    pub MaximumIos: u32,
    pub ObjectId: [u32; 4],
    pub Reserved: [u32; 41],
    pub ResumeContext: [u8; 1040],
}
impl ::core::marker::Copy for SCRUB_DATA_INPUT {}
impl ::core::clone::Clone for SCRUB_DATA_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SCRUB_DATA_OUTPUT {
    pub Size: u32,
    pub Flags: u32,
    pub Status: u32,
    pub ErrorFileOffset: u64,
    pub ErrorLength: u64,
    pub NumberOfBytesRepaired: u64,
    pub NumberOfBytesFailed: u64,
    pub InternalFileReference: u64,
    pub ResumeContextLength: u16,
    pub ParityExtentDataOffset: u16,
    pub Reserved: [u32; 9],
    pub NumberOfMetadataBytesProcessed: u64,
    pub NumberOfDataBytesProcessed: u64,
    pub TotalNumberOfMetadataBytesInUse: u64,
    pub TotalNumberOfDataBytesInUse: u64,
    pub DataBytesSkippedDueToNoAllocation: u64,
    pub DataBytesSkippedDueToInvalidRun: u64,
    pub DataBytesSkippedDueToIntegrityStream: u64,
    pub DataBytesSkippedDueToRegionBeingClean: u64,
    pub DataBytesSkippedDueToLockConflict: u64,
    pub DataBytesSkippedDueToNoScrubDataFlag: u64,
    pub DataBytesSkippedDueToNoScrubNonIntegrityStreamFlag: u64,
    pub DataBytesScrubbed: u64,
    pub ResumeContext: [u8; 1040],
}
impl ::core::marker::Copy for SCRUB_DATA_OUTPUT {}
impl ::core::clone::Clone for SCRUB_DATA_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SCRUB_PARITY_EXTENT {
    pub Offset: i64,
    pub Length: u64,
}
impl ::core::marker::Copy for SCRUB_PARITY_EXTENT {}
impl ::core::clone::Clone for SCRUB_PARITY_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SCRUB_PARITY_EXTENT_DATA {
    pub Size: u16,
    pub Flags: u16,
    pub NumberOfParityExtents: u16,
    pub MaximumNumberOfParityExtents: u16,
    pub ParityExtents: [SCRUB_PARITY_EXTENT; 1],
}
impl ::core::marker::Copy for SCRUB_PARITY_EXTENT_DATA {}
impl ::core::clone::Clone for SCRUB_PARITY_EXTENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SD_CHANGE_MACHINE_SID_INPUT {
    pub CurrentMachineSIDOffset: u16,
    pub CurrentMachineSIDLength: u16,
    pub NewMachineSIDOffset: u16,
    pub NewMachineSIDLength: u16,
}
impl ::core::marker::Copy for SD_CHANGE_MACHINE_SID_INPUT {}
impl ::core::clone::Clone for SD_CHANGE_MACHINE_SID_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SD_CHANGE_MACHINE_SID_OUTPUT {
    pub NumSDChangedSuccess: u64,
    pub NumSDChangedFail: u64,
    pub NumSDUnused: u64,
    pub NumSDTotal: u64,
    pub NumMftSDChangedSuccess: u64,
    pub NumMftSDChangedFail: u64,
    pub NumMftSDTotal: u64,
}
impl ::core::marker::Copy for SD_CHANGE_MACHINE_SID_OUTPUT {}
impl ::core::clone::Clone for SD_CHANGE_MACHINE_SID_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SD_ENUM_SDS_ENTRY {
    pub Hash: u32,
    pub SecurityId: u32,
    pub Offset: u64,
    pub Length: u32,
    pub Descriptor: [u8; 1],
}
impl ::core::marker::Copy for SD_ENUM_SDS_ENTRY {}
impl ::core::clone::Clone for SD_ENUM_SDS_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SD_ENUM_SDS_INPUT {
    pub StartingOffset: u64,
    pub MaxSDEntriesToReturn: u64,
}
impl ::core::marker::Copy for SD_ENUM_SDS_INPUT {}
impl ::core::clone::Clone for SD_ENUM_SDS_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SD_ENUM_SDS_OUTPUT {
    pub NextOffset: u64,
    pub NumSDEntriesReturned: u64,
    pub NumSDBytesReturned: u64,
    pub SDEntry: [SD_ENUM_SDS_ENTRY; 1],
}
impl ::core::marker::Copy for SD_ENUM_SDS_OUTPUT {}
impl ::core::clone::Clone for SD_ENUM_SDS_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SD_GLOBAL_CHANGE_INPUT {
    pub Flags: u32,
    pub ChangeType: u32,
    pub Anonymous: SD_GLOBAL_CHANGE_INPUT_0,
}
impl ::core::marker::Copy for SD_GLOBAL_CHANGE_INPUT {}
impl ::core::clone::Clone for SD_GLOBAL_CHANGE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union SD_GLOBAL_CHANGE_INPUT_0 {
    pub SdChange: SD_CHANGE_MACHINE_SID_INPUT,
    pub SdQueryStats: SD_QUERY_STATS_INPUT,
    pub SdEnumSds: SD_ENUM_SDS_INPUT,
}
impl ::core::marker::Copy for SD_GLOBAL_CHANGE_INPUT_0 {}
impl ::core::clone::Clone for SD_GLOBAL_CHANGE_INPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SD_GLOBAL_CHANGE_OUTPUT {
    pub Flags: u32,
    pub ChangeType: u32,
    pub Anonymous: SD_GLOBAL_CHANGE_OUTPUT_0,
}
impl ::core::marker::Copy for SD_GLOBAL_CHANGE_OUTPUT {}
impl ::core::clone::Clone for SD_GLOBAL_CHANGE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union SD_GLOBAL_CHANGE_OUTPUT_0 {
    pub SdChange: SD_CHANGE_MACHINE_SID_OUTPUT,
    pub SdQueryStats: SD_QUERY_STATS_OUTPUT,
    pub SdEnumSds: SD_ENUM_SDS_OUTPUT,
}
impl ::core::marker::Copy for SD_GLOBAL_CHANGE_OUTPUT_0 {}
impl ::core::clone::Clone for SD_GLOBAL_CHANGE_OUTPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SD_QUERY_STATS_INPUT {
    pub Reserved: u32,
}
impl ::core::marker::Copy for SD_QUERY_STATS_INPUT {}
impl ::core::clone::Clone for SD_QUERY_STATS_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SD_QUERY_STATS_OUTPUT {
    pub SdsStreamSize: u64,
    pub SdsAllocationSize: u64,
    pub SiiStreamSize: u64,
    pub SiiAllocationSize: u64,
    pub SdhStreamSize: u64,
    pub SdhAllocationSize: u64,
    pub NumSDTotal: u64,
    pub NumSDUnused: u64,
}
impl ::core::marker::Copy for SD_QUERY_STATS_OUTPUT {}
impl ::core::clone::Clone for SD_QUERY_STATS_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SECURITY_CLIENT_CONTEXT {
    pub SecurityQos: super::super::super::Win32::Security::SECURITY_QUALITY_OF_SERVICE,
    pub ClientToken: *mut ::core::ffi::c_void,
    pub DirectlyAccessClientToken: super::super::super::Win32::Foundation::BOOLEAN,
    pub DirectAccessEffectiveOnly: super::super::super::Win32::Foundation::BOOLEAN,
    pub ServerIsRemote: super::super::super::Win32::Foundation::BOOLEAN,
    pub ClientTokenControl: TOKEN_CONTROL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for SECURITY_CLIENT_CONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for SECURITY_CLIENT_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SECURITY_DESCRIPTOR {
    pub Revision: u8,
    pub Sbz1: u8,
    pub Control: u16,
    pub Owner: super::super::super::Win32::Foundation::PSID,
    pub Group: super::super::super::Win32::Foundation::PSID,
    pub Sacl: *mut super::super::super::Win32::Security::ACL,
    pub Dacl: *mut super::super::super::Win32::Security::ACL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for SECURITY_DESCRIPTOR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for SECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SECURITY_DESCRIPTOR_RELATIVE {
    pub Revision: u8,
    pub Sbz1: u8,
    pub Control: u16,
    pub Owner: u32,
    pub Group: u32,
    pub Sacl: u32,
    pub Dacl: u32,
}
impl ::core::marker::Copy for SECURITY_DESCRIPTOR_RELATIVE {}
impl ::core::clone::Clone for SECURITY_DESCRIPTOR_RELATIVE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SECURITY_OBJECT_AI_PARAMS {
    pub Size: u32,
    pub ConstraintMask: u32,
}
impl ::core::marker::Copy for SECURITY_OBJECT_AI_PARAMS {}
impl ::core::clone::Clone for SECURITY_OBJECT_AI_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_USER_DATA {
    pub UserName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub LogonDomainName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub LogonServer: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub pSid: super::super::super::Win32::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECURITY_USER_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECURITY_USER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_APPLICATION_PROTOCOLS {
    pub ProtocolListsSize: u32,
    pub ProtocolLists: [SEC_APPLICATION_PROTOCOL_LIST; 1],
}
impl ::core::marker::Copy for SEC_APPLICATION_PROTOCOLS {}
impl ::core::clone::Clone for SEC_APPLICATION_PROTOCOLS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_APPLICATION_PROTOCOL_LIST {
    pub ProtoNegoExt: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT,
    pub ProtocolListSize: u16,
    pub ProtocolList: [u8; 1],
}
impl ::core::marker::Copy for SEC_APPLICATION_PROTOCOL_LIST {}
impl ::core::clone::Clone for SEC_APPLICATION_PROTOCOL_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_CERTIFICATE_REQUEST_CONTEXT {
    pub cbCertificateRequestContext: u8,
    pub rgCertificateRequestContext: [u8; 1],
}
impl ::core::marker::Copy for SEC_CERTIFICATE_REQUEST_CONTEXT {}
impl ::core::clone::Clone for SEC_CERTIFICATE_REQUEST_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_CHANNEL_BINDINGS {
    pub dwInitiatorAddrType: u32,
    pub cbInitiatorLength: u32,
    pub dwInitiatorOffset: u32,
    pub dwAcceptorAddrType: u32,
    pub cbAcceptorLength: u32,
    pub dwAcceptorOffset: u32,
    pub cbApplicationDataLength: u32,
    pub dwApplicationDataOffset: u32,
}
impl ::core::marker::Copy for SEC_CHANNEL_BINDINGS {}
impl ::core::clone::Clone for SEC_CHANNEL_BINDINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_DTLS_MTU {
    pub PathMTU: u16,
}
impl ::core::marker::Copy for SEC_DTLS_MTU {}
impl ::core::clone::Clone for SEC_DTLS_MTU {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_FLAGS {
    pub Flags: u64,
}
impl ::core::marker::Copy for SEC_FLAGS {}
impl ::core::clone::Clone for SEC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_NEGOTIATION_INFO {
    pub Size: u32,
    pub NameLength: u32,
    pub Name: *mut u16,
    pub Reserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SEC_NEGOTIATION_INFO {}
impl ::core::clone::Clone for SEC_NEGOTIATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_PRESHAREDKEY {
    pub KeySize: u16,
    pub Key: [u8; 1],
}
impl ::core::marker::Copy for SEC_PRESHAREDKEY {}
impl ::core::clone::Clone for SEC_PRESHAREDKEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_PRESHAREDKEY_IDENTITY {
    pub KeyIdentitySize: u16,
    pub KeyIdentity: [u8; 1],
}
impl ::core::marker::Copy for SEC_PRESHAREDKEY_IDENTITY {}
impl ::core::clone::Clone for SEC_PRESHAREDKEY_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_SRTP_MASTER_KEY_IDENTIFIER {
    pub MasterKeyIdentifierSize: u8,
    pub MasterKeyIdentifier: [u8; 1],
}
impl ::core::marker::Copy for SEC_SRTP_MASTER_KEY_IDENTIFIER {}
impl ::core::clone::Clone for SEC_SRTP_MASTER_KEY_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_SRTP_PROTECTION_PROFILES {
    pub ProfilesSize: u16,
    pub ProfilesList: [u16; 1],
}
impl ::core::marker::Copy for SEC_SRTP_PROTECTION_PROFILES {}
impl ::core::clone::Clone for SEC_SRTP_PROTECTION_PROFILES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_TOKEN_BINDING {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub KeyParametersSize: u16,
    pub KeyParameters: [u8; 1],
}
impl ::core::marker::Copy for SEC_TOKEN_BINDING {}
impl ::core::clone::Clone for SEC_TOKEN_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_TRAFFIC_SECRETS {
    pub SymmetricAlgId: [u16; 64],
    pub ChainingMode: [u16; 64],
    pub HashAlgId: [u16; 64],
    pub KeySize: u16,
    pub IvSize: u16,
    pub MsgSequenceStart: u16,
    pub MsgSequenceEnd: u16,
    pub TrafficSecretType: SEC_TRAFFIC_SECRET_TYPE,
    pub TrafficSecretSize: u16,
    pub TrafficSecret: [u8; 1],
}
impl ::core::marker::Copy for SEC_TRAFFIC_SECRETS {}
impl ::core::clone::Clone for SEC_TRAFFIC_SECRETS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_WINNT_AUTH_IDENTITY_EX2 {
    pub Version: u32,
    pub cbHeaderLength: u16,
    pub cbStructureLength: u32,
    pub UserOffset: u32,
    pub UserLength: u16,
    pub DomainOffset: u32,
    pub DomainLength: u16,
    pub PackedCredentialsOffset: u32,
    pub PackedCredentialsLength: u16,
    pub Flags: u32,
    pub PackageListOffset: u32,
    pub PackageListLength: u16,
}
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY_EX2 {}
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY_EX2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_WINNT_AUTH_IDENTITY_EXW {
    pub Version: u32,
    pub Length: u32,
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
    pub PackageList: *mut u16,
    pub PackageListLength: u32,
}
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY_EXW {}
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY_EXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SEC_WINNT_AUTH_IDENTITY_W {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY_W {}
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SET_CACHED_RUNS_STATE_INPUT_BUFFER {
    pub Enable: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SET_CACHED_RUNS_STATE_INPUT_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SET_CACHED_RUNS_STATE_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {
    pub Flags: u32,
    pub AlignmentShift: u32,
    pub FileOffsetToAlign: u64,
    pub FallbackAlignmentShift: u32,
}
impl ::core::marker::Copy for SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {}
impl ::core::clone::Clone for SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SET_PURGE_FAILURE_MODE_INPUT {
    pub Flags: u32,
}
impl ::core::marker::Copy for SET_PURGE_FAILURE_MODE_INPUT {}
impl ::core::clone::Clone for SET_PURGE_FAILURE_MODE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SE_ACCESS_REPLY {
    pub Size: u32,
    pub ResultListCount: u32,
    pub GrantedAccess: *mut u32,
    pub AccessStatus: *mut i32,
    pub AccessReason: *mut ACCESS_REASONS,
    pub Privileges: *mut *mut super::super::super::Win32::Security::PRIVILEGE_SET,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for SE_ACCESS_REPLY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for SE_ACCESS_REPLY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SE_ACCESS_REQUEST {
    pub Size: u32,
    pub SeSecurityDescriptor: *mut SE_SECURITY_DESCRIPTOR,
    pub DesiredAccess: u32,
    pub PreviouslyGrantedAccess: u32,
    pub PrincipalSelfSid: super::super::super::Win32::Foundation::PSID,
    pub GenericMapping: *mut super::super::super::Win32::Security::GENERIC_MAPPING,
    pub ObjectTypeListCount: u32,
    pub ObjectTypeList: *mut OBJECT_TYPE_LIST,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for SE_ACCESS_REQUEST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for SE_ACCESS_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SE_AUDIT_INFO {
    pub Size: u32,
    pub AuditType: AUDIT_EVENT_TYPE,
    pub AuditOperation: SE_AUDIT_OPERATION,
    pub AuditFlags: u32,
    pub SubsystemName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ObjectTypeName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ObjectName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub HandleId: *mut ::core::ffi::c_void,
    pub TransactionId: *mut ::windows_sys::core::GUID,
    pub OperationId: *mut super::super::super::Win32::Foundation::LUID,
    pub ObjectCreation: super::super::super::Win32::Foundation::BOOLEAN,
    pub GenerateOnClose: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SE_AUDIT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SE_AUDIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SE_EXPORTS {
    pub SeCreateTokenPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeAssignPrimaryTokenPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeLockMemoryPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeIncreaseQuotaPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeUnsolicitedInputPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeTcbPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeSecurityPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeTakeOwnershipPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeLoadDriverPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeCreatePagefilePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeIncreaseBasePriorityPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeSystemProfilePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeSystemtimePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeProfileSingleProcessPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeCreatePermanentPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeBackupPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeRestorePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeShutdownPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeDebugPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeAuditPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeSystemEnvironmentPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeChangeNotifyPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeRemoteShutdownPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeNullSid: super::super::super::Win32::Foundation::PSID,
    pub SeWorldSid: super::super::super::Win32::Foundation::PSID,
    pub SeLocalSid: super::super::super::Win32::Foundation::PSID,
    pub SeCreatorOwnerSid: super::super::super::Win32::Foundation::PSID,
    pub SeCreatorGroupSid: super::super::super::Win32::Foundation::PSID,
    pub SeNtAuthoritySid: super::super::super::Win32::Foundation::PSID,
    pub SeDialupSid: super::super::super::Win32::Foundation::PSID,
    pub SeNetworkSid: super::super::super::Win32::Foundation::PSID,
    pub SeBatchSid: super::super::super::Win32::Foundation::PSID,
    pub SeInteractiveSid: super::super::super::Win32::Foundation::PSID,
    pub SeLocalSystemSid: super::super::super::Win32::Foundation::PSID,
    pub SeAliasAdminsSid: super::super::super::Win32::Foundation::PSID,
    pub SeAliasUsersSid: super::super::super::Win32::Foundation::PSID,
    pub SeAliasGuestsSid: super::super::super::Win32::Foundation::PSID,
    pub SeAliasPowerUsersSid: super::super::super::Win32::Foundation::PSID,
    pub SeAliasAccountOpsSid: super::super::super::Win32::Foundation::PSID,
    pub SeAliasSystemOpsSid: super::super::super::Win32::Foundation::PSID,
    pub SeAliasPrintOpsSid: super::super::super::Win32::Foundation::PSID,
    pub SeAliasBackupOpsSid: super::super::super::Win32::Foundation::PSID,
    pub SeAuthenticatedUsersSid: super::super::super::Win32::Foundation::PSID,
    pub SeRestrictedSid: super::super::super::Win32::Foundation::PSID,
    pub SeAnonymousLogonSid: super::super::super::Win32::Foundation::PSID,
    pub SeUndockPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeSyncAgentPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeEnableDelegationPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeLocalServiceSid: super::super::super::Win32::Foundation::PSID,
    pub SeNetworkServiceSid: super::super::super::Win32::Foundation::PSID,
    pub SeManageVolumePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeImpersonatePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeCreateGlobalPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeTrustedCredManAccessPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeRelabelPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeIncreaseWorkingSetPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeTimeZonePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeCreateSymbolicLinkPrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeIUserSid: super::super::super::Win32::Foundation::PSID,
    pub SeUntrustedMandatorySid: super::super::super::Win32::Foundation::PSID,
    pub SeLowMandatorySid: super::super::super::Win32::Foundation::PSID,
    pub SeMediumMandatorySid: super::super::super::Win32::Foundation::PSID,
    pub SeHighMandatorySid: super::super::super::Win32::Foundation::PSID,
    pub SeSystemMandatorySid: super::super::super::Win32::Foundation::PSID,
    pub SeOwnerRightsSid: super::super::super::Win32::Foundation::PSID,
    pub SeAllAppPackagesSid: super::super::super::Win32::Foundation::PSID,
    pub SeUserModeDriversSid: super::super::super::Win32::Foundation::PSID,
    pub SeProcTrustWinTcbSid: super::super::super::Win32::Foundation::PSID,
    pub SeTrustedInstallerSid: super::super::super::Win32::Foundation::PSID,
    pub SeDelegateSessionUserImpersonatePrivilege: super::super::super::Win32::Foundation::LUID,
    pub SeAppSiloSid: super::super::super::Win32::Foundation::PSID,
    pub SeAppSiloVolumeRootMinimalCapabilitySid: super::super::super::Win32::Foundation::PSID,
    pub SeAppSiloProfilesRootMinimalCapabilitySid: super::super::super::Win32::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SE_EXPORTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SE_EXPORTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct SE_SECURITY_DESCRIPTOR {
    pub Size: u32,
    pub Flags: u32,
    pub SecurityDescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for SE_SECURITY_DESCRIPTOR {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for SE_SECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub union SE_SID {
    pub Sid: super::super::super::Win32::Security::SID,
    pub Buffer: [u8; 68],
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for SE_SID {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for SE_SID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SE_TOKEN_USER {
    pub Anonymous1: SE_TOKEN_USER_0,
    pub Anonymous2: SE_TOKEN_USER_1,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for SE_TOKEN_USER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for SE_TOKEN_USER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union SE_TOKEN_USER_0 {
    pub TokenUser: TOKEN_USER,
    pub User: SID_AND_ATTRIBUTES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for SE_TOKEN_USER_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for SE_TOKEN_USER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union SE_TOKEN_USER_1 {
    pub Sid: super::super::super::Win32::Security::SID,
    pub Buffer: [u8; 68],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for SE_TOKEN_USER_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for SE_TOKEN_USER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SHARED_VIRTUAL_DISK_SUPPORT {
    pub SharedVirtualDiskSupport: SharedVirtualDiskSupportType,
    pub HandleState: SharedVirtualDiskHandleState,
}
impl ::core::marker::Copy for SHARED_VIRTUAL_DISK_SUPPORT {}
impl ::core::clone::Clone for SHARED_VIRTUAL_DISK_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SHRINK_VOLUME_INFORMATION {
    pub ShrinkRequestType: SHRINK_VOLUME_REQUEST_TYPES,
    pub Flags: u64,
    pub NewNumberOfSectors: i64,
}
impl ::core::marker::Copy for SHRINK_VOLUME_INFORMATION {}
impl ::core::clone::Clone for SHRINK_VOLUME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SHUFFLE_FILE_DATA {
    pub StartingOffset: i64,
    pub Length: i64,
    pub Flags: u32,
}
impl ::core::marker::Copy for SHUFFLE_FILE_DATA {}
impl ::core::clone::Clone for SHUFFLE_FILE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SID_AND_ATTRIBUTES {
    pub Sid: super::super::super::Win32::Foundation::PSID,
    pub Attributes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SID_AND_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SID_AND_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SID_AND_ATTRIBUTES_HASH {
    pub SidCount: u32,
    pub SidAttr: *mut SID_AND_ATTRIBUTES,
    pub Hash: [usize; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SID_AND_ATTRIBUTES_HASH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SID_AND_ATTRIBUTES_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SID_IDENTIFIER_AUTHORITY {
    pub Value: [u8; 6],
}
impl ::core::marker::Copy for SID_IDENTIFIER_AUTHORITY {}
impl ::core::clone::Clone for SID_IDENTIFIER_AUTHORITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SI_COPYFILE {
    pub SourceFileNameLength: u32,
    pub DestinationFileNameLength: u32,
    pub Flags: u32,
    pub FileNameBuffer: [u16; 1],
}
impl ::core::marker::Copy for SI_COPYFILE {}
impl ::core::clone::Clone for SI_COPYFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SMB_SHARE_FLUSH_AND_PURGE_INPUT {
    pub Version: u16,
}
impl ::core::marker::Copy for SMB_SHARE_FLUSH_AND_PURGE_INPUT {}
impl ::core::clone::Clone for SMB_SHARE_FLUSH_AND_PURGE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {
    pub cEntriesPurged: u32,
}
impl ::core::marker::Copy for SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {}
impl ::core::clone::Clone for SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKADDR_STORAGE(pub u8);
impl ::core::marker::Copy for SOCKADDR_STORAGE {}
impl ::core::clone::Clone for SOCKADDR_STORAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SRV_OPEN_ECP_CONTEXT {
    pub ShareName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub SocketAddress: *mut SOCKADDR_STORAGE,
    pub OplockBlockState: super::super::super::Win32::Foundation::BOOLEAN,
    pub OplockAppState: super::super::super::Win32::Foundation::BOOLEAN,
    pub OplockFinalState: super::super::super::Win32::Foundation::BOOLEAN,
    pub Version: u16,
    pub InstanceType: SRV_INSTANCE_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SRV_OPEN_ECP_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SRV_OPEN_ECP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STARTING_LCN_INPUT_BUFFER {
    pub StartingLcn: i64,
}
impl ::core::marker::Copy for STARTING_LCN_INPUT_BUFFER {}
impl ::core::clone::Clone for STARTING_LCN_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STARTING_LCN_INPUT_BUFFER_EX {
    pub StartingLcn: i64,
    pub Flags: u32,
}
impl ::core::marker::Copy for STARTING_LCN_INPUT_BUFFER_EX {}
impl ::core::clone::Clone for STARTING_LCN_INPUT_BUFFER_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STARTING_VCN_INPUT_BUFFER {
    pub StartingVcn: i64,
}
impl ::core::marker::Copy for STARTING_VCN_INPUT_BUFFER {}
impl ::core::clone::Clone for STARTING_VCN_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {
    pub EntryLength: u32,
    pub DependencyTypeFlags: u32,
    pub ProviderSpecificFlags: u32,
    pub VirtualStorageType: VIRTUAL_STORAGE_TYPE,
}
impl ::core::marker::Copy for STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {}
impl ::core::clone::Clone for STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {
    pub EntryLength: u32,
    pub DependencyTypeFlags: u32,
    pub ProviderSpecificFlags: u32,
    pub VirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub AncestorLevel: u32,
    pub HostVolumeNameOffset: u32,
    pub HostVolumeNameSize: u32,
    pub DependentVolumeNameOffset: u32,
    pub DependentVolumeNameSize: u32,
    pub RelativePathOffset: u32,
    pub RelativePathSize: u32,
    pub DependentDeviceNameOffset: u32,
    pub DependentDeviceNameSize: u32,
}
impl ::core::marker::Copy for STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {}
impl ::core::clone::Clone for STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {
    pub RequestLevel: u32,
    pub RequestFlags: u32,
}
impl ::core::marker::Copy for STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {}
impl ::core::clone::Clone for STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE {
    pub ResponseLevel: u32,
    pub NumberEntries: u32,
    pub Anonymous: STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0,
}
impl ::core::marker::Copy for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE {}
impl ::core::clone::Clone for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0 {
    pub Lev1Depends: [STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY; 1],
    pub Lev2Depends: [STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY; 1],
}
impl ::core::marker::Copy for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0 {}
impl ::core::clone::Clone for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STREAMS_ASSOCIATE_ID_INPUT_BUFFER {
    pub Flags: u32,
    pub StreamId: u32,
}
impl ::core::marker::Copy for STREAMS_ASSOCIATE_ID_INPUT_BUFFER {}
impl ::core::clone::Clone for STREAMS_ASSOCIATE_ID_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STREAMS_QUERY_ID_OUTPUT_BUFFER {
    pub StreamId: u32,
}
impl ::core::marker::Copy for STREAMS_QUERY_ID_OUTPUT_BUFFER {}
impl ::core::clone::Clone for STREAMS_QUERY_ID_OUTPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {
    pub OptimalWriteSize: u32,
    pub StreamGranularitySize: u32,
    pub StreamIdMin: u32,
    pub StreamIdMax: u32,
}
impl ::core::marker::Copy for STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {}
impl ::core::clone::Clone for STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STREAM_EXTENT_ENTRY {
    pub Flags: u32,
    pub ExtentInformation: STREAM_EXTENT_ENTRY_0,
}
impl ::core::marker::Copy for STREAM_EXTENT_ENTRY {}
impl ::core::clone::Clone for STREAM_EXTENT_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union STREAM_EXTENT_ENTRY_0 {
    pub RetrievalPointers: RETRIEVAL_POINTERS_BUFFER,
}
impl ::core::marker::Copy for STREAM_EXTENT_ENTRY_0 {}
impl ::core::clone::Clone for STREAM_EXTENT_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STREAM_INFORMATION_ENTRY {
    pub Version: u32,
    pub Flags: u32,
    pub StreamInformation: STREAM_INFORMATION_ENTRY_0,
}
impl ::core::marker::Copy for STREAM_INFORMATION_ENTRY {}
impl ::core::clone::Clone for STREAM_INFORMATION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union STREAM_INFORMATION_ENTRY_0 {
    pub DesiredStorageClass: STREAM_INFORMATION_ENTRY_0_1,
    pub DataStream: STREAM_INFORMATION_ENTRY_0_0,
    pub Reparse: STREAM_INFORMATION_ENTRY_0_3,
    pub Ea: STREAM_INFORMATION_ENTRY_0_2,
}
impl ::core::marker::Copy for STREAM_INFORMATION_ENTRY_0 {}
impl ::core::clone::Clone for STREAM_INFORMATION_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STREAM_INFORMATION_ENTRY_0_0 {
    pub Length: u16,
    pub Flags: u16,
    pub Reserved: u32,
    pub Vdl: u64,
}
impl ::core::marker::Copy for STREAM_INFORMATION_ENTRY_0_0 {}
impl ::core::clone::Clone for STREAM_INFORMATION_ENTRY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STREAM_INFORMATION_ENTRY_0_1 {
    pub Class: FILE_STORAGE_TIER_CLASS,
    pub Flags: u32,
}
impl ::core::marker::Copy for STREAM_INFORMATION_ENTRY_0_1 {}
impl ::core::clone::Clone for STREAM_INFORMATION_ENTRY_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STREAM_INFORMATION_ENTRY_0_2 {
    pub Length: u16,
    pub Flags: u16,
    pub EaSize: u32,
    pub EaInformationOffset: u32,
}
impl ::core::marker::Copy for STREAM_INFORMATION_ENTRY_0_2 {}
impl ::core::clone::Clone for STREAM_INFORMATION_ENTRY_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STREAM_INFORMATION_ENTRY_0_3 {
    pub Length: u16,
    pub Flags: u16,
    pub ReparseDataSize: u32,
    pub ReparseDataOffset: u32,
}
impl ::core::marker::Copy for STREAM_INFORMATION_ENTRY_0_3 {}
impl ::core::clone::Clone for STREAM_INFORMATION_ENTRY_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct STREAM_LAYOUT_ENTRY {
    pub Version: u32,
    pub NextStreamOffset: u32,
    pub Flags: u32,
    pub ExtentInformationOffset: u32,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub StreamInformationOffset: u32,
    pub AttributeTypeCode: u32,
    pub AttributeFlags: u32,
    pub StreamIdentifierLength: u32,
    pub StreamIdentifier: [u16; 1],
}
impl ::core::marker::Copy for STREAM_LAYOUT_ENTRY {}
impl ::core::clone::Clone for STREAM_LAYOUT_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SYSTEM_ACCESS_FILTER_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ACCESS_FILTER_ACE {}
impl ::core::clone::Clone for SYSTEM_ACCESS_FILTER_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SYSTEM_ALARM_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ALARM_ACE {}
impl ::core::clone::Clone for SYSTEM_ALARM_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SYSTEM_AUDIT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_AUDIT_ACE {}
impl ::core::clone::Clone for SYSTEM_AUDIT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SYSTEM_MANDATORY_LABEL_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_MANDATORY_LABEL_ACE {}
impl ::core::clone::Clone for SYSTEM_MANDATORY_LABEL_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SYSTEM_PROCESS_TRUST_LABEL_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_PROCESS_TRUST_LABEL_ACE {}
impl ::core::clone::Clone for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_RESOURCE_ATTRIBUTE_ACE {}
impl ::core::clone::Clone for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SYSTEM_SCOPED_POLICY_ID_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_SCOPED_POLICY_ID_ACE {}
impl ::core::clone::Clone for SYSTEM_SCOPED_POLICY_ID_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecBuffer {
    pub cbBuffer: u32,
    pub BufferType: u32,
    pub pvBuffer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SecBuffer {}
impl ::core::clone::Clone for SecBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecBufferDesc {
    pub ulVersion: u32,
    pub cBuffers: u32,
    pub pBuffers: *mut SecBuffer,
}
impl ::core::marker::Copy for SecBufferDesc {}
impl ::core::clone::Clone for SecBufferDesc {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecHandle {
    pub dwLower: usize,
    pub dwUpper: usize,
}
impl ::core::marker::Copy for SecHandle {}
impl ::core::clone::Clone for SecHandle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_AuthorityW {
    pub sAuthorityName: *mut u16,
}
impl ::core::marker::Copy for SecPkgContext_AuthorityW {}
impl ::core::clone::Clone for SecPkgContext_AuthorityW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_CredInfo {
    pub CredClass: SECPKG_CRED_CLASS,
    pub IsPromptingNeeded: u32,
}
impl ::core::marker::Copy for SecPkgContext_CredInfo {}
impl ::core::clone::Clone for SecPkgContext_CredInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_CredentialNameW {
    pub CredentialType: u32,
    pub sCredentialName: *mut u16,
}
impl ::core::marker::Copy for SecPkgContext_CredentialNameW {}
impl ::core::clone::Clone for SecPkgContext_CredentialNameW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_DceInfo {
    pub AuthzSvc: u32,
    pub pPac: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SecPkgContext_DceInfo {}
impl ::core::clone::Clone for SecPkgContext_DceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_Flags {
    pub Flags: u32,
}
impl ::core::marker::Copy for SecPkgContext_Flags {}
impl ::core::clone::Clone for SecPkgContext_Flags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_KeyInfoW {
    pub sSignatureAlgorithmName: *mut u16,
    pub sEncryptAlgorithmName: *mut u16,
    pub KeySize: u32,
    pub SignatureAlgorithm: u32,
    pub EncryptAlgorithm: u32,
}
impl ::core::marker::Copy for SecPkgContext_KeyInfoW {}
impl ::core::clone::Clone for SecPkgContext_KeyInfoW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_Lifespan {
    pub tsStart: i64,
    pub tsExpiry: i64,
}
impl ::core::marker::Copy for SecPkgContext_Lifespan {}
impl ::core::clone::Clone for SecPkgContext_Lifespan {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_LogoffTime {
    pub tsLogoffTime: i64,
}
impl ::core::marker::Copy for SecPkgContext_LogoffTime {}
impl ::core::clone::Clone for SecPkgContext_LogoffTime {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_NamesW {
    pub sUserName: *mut u16,
}
impl ::core::marker::Copy for SecPkgContext_NamesW {}
impl ::core::clone::Clone for SecPkgContext_NamesW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_NegoKeys {
    pub KeyType: u32,
    pub KeyLength: u16,
    pub KeyValue: *mut u8,
    pub VerifyKeyType: u32,
    pub VerifyKeyLength: u16,
    pub VerifyKeyValue: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_NegoKeys {}
impl ::core::clone::Clone for SecPkgContext_NegoKeys {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_NegoPackageInfo {
    pub PackageMask: u32,
}
impl ::core::marker::Copy for SecPkgContext_NegoPackageInfo {}
impl ::core::clone::Clone for SecPkgContext_NegoPackageInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_NegoStatus {
    pub LastStatus: u32,
}
impl ::core::marker::Copy for SecPkgContext_NegoStatus {}
impl ::core::clone::Clone for SecPkgContext_NegoStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_NegotiationInfoW {
    pub PackageInfo: *mut SecPkgInfoW,
    pub NegotiationState: u32,
}
impl ::core::marker::Copy for SecPkgContext_NegotiationInfoW {}
impl ::core::clone::Clone for SecPkgContext_NegotiationInfoW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_PackageInfoW {
    pub PackageInfo: *mut SecPkgInfoW,
}
impl ::core::marker::Copy for SecPkgContext_PackageInfoW {}
impl ::core::clone::Clone for SecPkgContext_PackageInfoW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_PasswordExpiry {
    pub tsPasswordExpires: i64,
}
impl ::core::marker::Copy for SecPkgContext_PasswordExpiry {}
impl ::core::clone::Clone for SecPkgContext_PasswordExpiry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_ProtoInfoW {
    pub sProtocolName: *mut u16,
    pub majorVersion: u32,
    pub minorVersion: u32,
}
impl ::core::marker::Copy for SecPkgContext_ProtoInfoW {}
impl ::core::clone::Clone for SecPkgContext_ProtoInfoW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_SessionKey {
    pub SessionKeyLength: u32,
    pub SessionKey: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_SessionKey {}
impl ::core::clone::Clone for SecPkgContext_SessionKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_Sizes {
    pub cbMaxToken: u32,
    pub cbMaxSignature: u32,
    pub cbBlockSize: u32,
    pub cbSecurityTrailer: u32,
}
impl ::core::marker::Copy for SecPkgContext_Sizes {}
impl ::core::clone::Clone for SecPkgContext_Sizes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_StreamSizes {
    pub cbHeader: u32,
    pub cbTrailer: u32,
    pub cbMaximumMessage: u32,
    pub cBuffers: u32,
    pub cbBlockSize: u32,
}
impl ::core::marker::Copy for SecPkgContext_StreamSizes {}
impl ::core::clone::Clone for SecPkgContext_StreamSizes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_SubjectAttributes {
    pub AttributeInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SecPkgContext_SubjectAttributes {}
impl ::core::clone::Clone for SecPkgContext_SubjectAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgContext_UserFlags {
    pub UserFlags: u32,
}
impl ::core::marker::Copy for SecPkgContext_UserFlags {}
impl ::core::clone::Clone for SecPkgContext_UserFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgCredentials_Cert {
    pub EncodedCertSize: u32,
    pub EncodedCert: *mut u8,
}
impl ::core::marker::Copy for SecPkgCredentials_Cert {}
impl ::core::clone::Clone for SecPkgCredentials_Cert {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgCredentials_KdcProxySettingsW {
    pub Version: u32,
    pub Flags: u32,
    pub ProxyServerOffset: u16,
    pub ProxyServerLength: u16,
    pub ClientTlsCredOffset: u16,
    pub ClientTlsCredLength: u16,
}
impl ::core::marker::Copy for SecPkgCredentials_KdcProxySettingsW {}
impl ::core::clone::Clone for SecPkgCredentials_KdcProxySettingsW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgCredentials_NamesW {
    pub sUserName: *mut u16,
}
impl ::core::marker::Copy for SecPkgCredentials_NamesW {}
impl ::core::clone::Clone for SecPkgCredentials_NamesW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgCredentials_SSIProviderW {
    pub sProviderName: *mut u16,
    pub ProviderInfoLength: u32,
    pub ProviderInfo: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for SecPkgCredentials_SSIProviderW {}
impl ::core::clone::Clone for SecPkgCredentials_SSIProviderW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct SecPkgInfoW {
    pub fCapabilities: u32,
    pub wVersion: u16,
    pub wRPCID: u16,
    pub cbMaxToken: u32,
    pub Name: *mut u16,
    pub Comment: *mut u16,
}
impl ::core::marker::Copy for SecPkgInfoW {}
impl ::core::clone::Clone for SecPkgInfoW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SecurityFunctionTableW {
    pub dwVersion: u32,
    pub EnumerateSecurityPackagesW: ENUMERATE_SECURITY_PACKAGES_FN_W,
    pub QueryCredentialsAttributesW: QUERY_CREDENTIALS_ATTRIBUTES_FN_W,
    pub AcquireCredentialsHandleW: ACQUIRE_CREDENTIALS_HANDLE_FN_W,
    pub FreeCredentialsHandle: FREE_CREDENTIALS_HANDLE_FN,
    pub Reserved2: *mut ::core::ffi::c_void,
    pub InitializeSecurityContextW: INITIALIZE_SECURITY_CONTEXT_FN_W,
    pub AcceptSecurityContext: ACCEPT_SECURITY_CONTEXT_FN,
    pub CompleteAuthToken: COMPLETE_AUTH_TOKEN_FN,
    pub DeleteSecurityContext: DELETE_SECURITY_CONTEXT_FN,
    pub ApplyControlToken: APPLY_CONTROL_TOKEN_FN,
    pub QueryContextAttributesW: QUERY_CONTEXT_ATTRIBUTES_FN_W,
    pub ImpersonateSecurityContext: IMPERSONATE_SECURITY_CONTEXT_FN,
    pub RevertSecurityContext: REVERT_SECURITY_CONTEXT_FN,
    pub MakeSignature: MAKE_SIGNATURE_FN,
    pub VerifySignature: VERIFY_SIGNATURE_FN,
    pub FreeContextBuffer: FREE_CONTEXT_BUFFER_FN,
    pub QuerySecurityPackageInfoW: QUERY_SECURITY_PACKAGE_INFO_FN_W,
    pub Reserved3: *mut ::core::ffi::c_void,
    pub Reserved4: *mut ::core::ffi::c_void,
    pub ExportSecurityContext: EXPORT_SECURITY_CONTEXT_FN,
    pub ImportSecurityContextW: IMPORT_SECURITY_CONTEXT_FN_W,
    pub AddCredentialsW: ADD_CREDENTIALS_FN_W,
    pub Reserved8: *mut ::core::ffi::c_void,
    pub QuerySecurityContextToken: QUERY_SECURITY_CONTEXT_TOKEN_FN,
    pub EncryptMessage: ENCRYPT_MESSAGE_FN,
    pub DecryptMessage: DECRYPT_MESSAGE_FN,
    pub SetContextAttributesW: SET_CONTEXT_ATTRIBUTES_FN_W,
    pub SetCredentialsAttributesW: SET_CREDENTIALS_ATTRIBUTES_FN_W,
    pub Reserved9: *mut ::core::ffi::c_void,
    pub QueryContextAttributesExW: QUERY_CONTEXT_ATTRIBUTES_EX_FN_W,
    pub QueryCredentialsAttributesExW: QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_W,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SecurityFunctionTableW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SecurityFunctionTableW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SspiAsyncContext(pub u8);
impl ::core::marker::Copy for SspiAsyncContext {}
impl ::core::clone::Clone for SspiAsyncContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct TOKEN_ACCESS_INFORMATION {
    pub SidHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub RestrictedSidHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub Privileges: *mut TOKEN_PRIVILEGES,
    pub AuthenticationId: super::super::super::Win32::Foundation::LUID,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: super::super::super::Win32::Security::SECURITY_IMPERSONATION_LEVEL,
    pub MandatoryPolicy: TOKEN_MANDATORY_POLICY,
    pub Flags: u32,
    pub AppContainerNumber: u32,
    pub PackageSid: super::super::super::Win32::Foundation::PSID,
    pub CapabilitiesHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub TrustLevelSid: super::super::super::Win32::Foundation::PSID,
    pub SecurityAttributes: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for TOKEN_ACCESS_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for TOKEN_ACCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_APPCONTAINER_INFORMATION {
    pub TokenAppContainer: super::super::super::Win32::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_APPCONTAINER_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_APPCONTAINER_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TOKEN_AUDIT_POLICY {
    pub PerUserPolicy: [u8; 30],
}
impl ::core::marker::Copy for TOKEN_AUDIT_POLICY {}
impl ::core::clone::Clone for TOKEN_AUDIT_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_BNO_ISOLATION_INFORMATION {
    pub IsolationPrefix: ::windows_sys::core::PWSTR,
    pub IsolationEnabled: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_BNO_ISOLATION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_BNO_ISOLATION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_CONTROL {
    pub TokenId: super::super::super::Win32::Foundation::LUID,
    pub AuthenticationId: super::super::super::Win32::Foundation::LUID,
    pub ModifiedId: super::super::super::Win32::Foundation::LUID,
    pub TokenSource: TOKEN_SOURCE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_CONTROL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct TOKEN_DEFAULT_DACL {
    pub DefaultDacl: *mut super::super::super::Win32::Security::ACL,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for TOKEN_DEFAULT_DACL {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for TOKEN_DEFAULT_DACL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TOKEN_DEVICE_CLAIMS {
    pub DeviceClaims: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for TOKEN_DEVICE_CLAIMS {}
impl ::core::clone::Clone for TOKEN_DEVICE_CLAIMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TOKEN_ELEVATION {
    pub TokenIsElevated: u32,
}
impl ::core::marker::Copy for TOKEN_ELEVATION {}
impl ::core::clone::Clone for TOKEN_ELEVATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_GROUPS {
    pub GroupCount: u32,
    pub Groups: [SID_AND_ATTRIBUTES; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_GROUPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_GROUPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct TOKEN_GROUPS_AND_PRIVILEGES {
    pub SidCount: u32,
    pub SidLength: u32,
    pub Sids: *mut SID_AND_ATTRIBUTES,
    pub RestrictedSidCount: u32,
    pub RestrictedSidLength: u32,
    pub RestrictedSids: *mut SID_AND_ATTRIBUTES,
    pub PrivilegeCount: u32,
    pub PrivilegeLength: u32,
    pub Privileges: *mut super::super::super::Win32::Security::LUID_AND_ATTRIBUTES,
    pub AuthenticationId: super::super::super::Win32::Foundation::LUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for TOKEN_GROUPS_AND_PRIVILEGES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for TOKEN_GROUPS_AND_PRIVILEGES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_LINKED_TOKEN {
    pub LinkedToken: super::super::super::Win32::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_LINKED_TOKEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_LINKED_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_MANDATORY_LABEL {
    pub Label: SID_AND_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_MANDATORY_LABEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_MANDATORY_LABEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TOKEN_MANDATORY_POLICY {
    pub Policy: u32,
}
impl ::core::marker::Copy for TOKEN_MANDATORY_POLICY {}
impl ::core::clone::Clone for TOKEN_MANDATORY_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_ORIGIN {
    pub OriginatingLogonSession: super::super::super::Win32::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_ORIGIN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_ORIGIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_OWNER {
    pub Owner: super::super::super::Win32::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_OWNER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_OWNER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_PRIMARY_GROUP {
    pub PrimaryGroup: super::super::super::Win32::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_PRIMARY_GROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_PRIMARY_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct TOKEN_PRIVILEGES {
    pub PrivilegeCount: u32,
    pub Privileges: [super::super::super::Win32::Security::LUID_AND_ATTRIBUTES; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for TOKEN_PRIVILEGES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for TOKEN_PRIVILEGES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_SID_INFORMATION {
    pub Sid: super::super::super::Win32::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_SID_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_SID_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_SOURCE {
    pub SourceName: [u8; 8],
    pub SourceIdentifier: super::super::super::Win32::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_SOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct TOKEN_STATISTICS {
    pub TokenId: super::super::super::Win32::Foundation::LUID,
    pub AuthenticationId: super::super::super::Win32::Foundation::LUID,
    pub ExpirationTime: i64,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: super::super::super::Win32::Security::SECURITY_IMPERSONATION_LEVEL,
    pub DynamicCharged: u32,
    pub DynamicAvailable: u32,
    pub GroupCount: u32,
    pub PrivilegeCount: u32,
    pub ModifiedId: super::super::super::Win32::Foundation::LUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for TOKEN_STATISTICS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for TOKEN_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_USER {
    pub User: SID_AND_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_USER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_USER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TOKEN_USER_CLAIMS {
    pub UserClaims: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for TOKEN_USER_CLAIMS {}
impl ::core::clone::Clone for TOKEN_USER_CLAIMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct TUNNEL {
    pub Mutex: super::super::Foundation::FAST_MUTEX,
    pub Cache: *mut super::super::Foundation::RTL_SPLAY_LINKS,
    pub TimerQueue: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub NumEntries: u16,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for TUNNEL {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for TUNNEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TXFS_CREATE_MINIVERSION_INFO {
    pub StructureVersion: u16,
    pub StructureLength: u16,
    pub BaseVersion: u32,
    pub MiniVersion: u16,
}
impl ::core::marker::Copy for TXFS_CREATE_MINIVERSION_INFO {}
impl ::core::clone::Clone for TXFS_CREATE_MINIVERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TXFS_GET_METADATA_INFO_OUT {
    pub TxfFileId: TXFS_GET_METADATA_INFO_OUT_0,
    pub LockingTransaction: ::windows_sys::core::GUID,
    pub LastLsn: u64,
    pub TransactionState: u32,
}
impl ::core::marker::Copy for TXFS_GET_METADATA_INFO_OUT {}
impl ::core::clone::Clone for TXFS_GET_METADATA_INFO_OUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TXFS_GET_METADATA_INFO_OUT_0 {
    pub LowPart: i64,
    pub HighPart: i64,
}
impl ::core::marker::Copy for TXFS_GET_METADATA_INFO_OUT_0 {}
impl ::core::clone::Clone for TXFS_GET_METADATA_INFO_OUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TXFS_GET_TRANSACTED_VERSION {
    pub ThisBaseVersion: u32,
    pub LatestVersion: u32,
    pub ThisMiniVersion: u16,
    pub FirstMiniVersion: u16,
    pub LatestMiniVersion: u16,
}
impl ::core::marker::Copy for TXFS_GET_TRANSACTED_VERSION {}
impl ::core::clone::Clone for TXFS_GET_TRANSACTED_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TXFS_LIST_TRANSACTIONS {
    pub NumberOfTransactions: u64,
    pub BufferSizeRequired: u64,
}
impl ::core::marker::Copy for TXFS_LIST_TRANSACTIONS {}
impl ::core::clone::Clone for TXFS_LIST_TRANSACTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TXFS_LIST_TRANSACTIONS_ENTRY {
    pub TransactionId: ::windows_sys::core::GUID,
    pub TransactionState: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: i64,
}
impl ::core::marker::Copy for TXFS_LIST_TRANSACTIONS_ENTRY {}
impl ::core::clone::Clone for TXFS_LIST_TRANSACTIONS_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TXFS_LIST_TRANSACTION_LOCKED_FILES {
    pub KtmTransaction: ::windows_sys::core::GUID,
    pub NumberOfFiles: u64,
    pub BufferSizeRequired: u64,
    pub Offset: u64,
}
impl ::core::marker::Copy for TXFS_LIST_TRANSACTION_LOCKED_FILES {}
impl ::core::clone::Clone for TXFS_LIST_TRANSACTION_LOCKED_FILES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    pub Offset: u64,
    pub NameFlags: u32,
    pub FileId: i64,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: i64,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {}
impl ::core::clone::Clone for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TXFS_MODIFY_RM {
    pub Flags: u32,
    pub LogContainerCountMax: u32,
    pub LogContainerCountMin: u32,
    pub LogContainerCount: u32,
    pub LogGrowthIncrement: u32,
    pub LogAutoShrinkPercentage: u32,
    pub Reserved: u64,
    pub LoggingMode: u16,
}
impl ::core::marker::Copy for TXFS_MODIFY_RM {}
impl ::core::clone::Clone for TXFS_MODIFY_RM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TXFS_QUERY_RM_INFORMATION {
    pub BytesRequired: u32,
    pub TailLsn: u64,
    pub CurrentLsn: u64,
    pub ArchiveTailLsn: u64,
    pub LogContainerSize: u64,
    pub HighestVirtualClock: i64,
    pub LogContainerCount: u32,
    pub LogContainerCountMax: u32,
    pub LogContainerCountMin: u32,
    pub LogGrowthIncrement: u32,
    pub LogAutoShrinkPercentage: u32,
    pub Flags: u32,
    pub LoggingMode: u16,
    pub Reserved: u16,
    pub RmState: u32,
    pub LogCapacity: u64,
    pub LogFree: u64,
    pub TopsSize: u64,
    pub TopsUsed: u64,
    pub TransactionCount: u64,
    pub OnePCCount: u64,
    pub TwoPCCount: u64,
    pub NumberLogFileFull: u64,
    pub OldestTransactionAge: u64,
    pub RMName: ::windows_sys::core::GUID,
    pub TmLogPathOffset: u32,
}
impl ::core::marker::Copy for TXFS_QUERY_RM_INFORMATION {}
impl ::core::clone::Clone for TXFS_QUERY_RM_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TXFS_READ_BACKUP_INFORMATION_OUT {
    pub Anonymous: TXFS_READ_BACKUP_INFORMATION_OUT_0,
}
impl ::core::marker::Copy for TXFS_READ_BACKUP_INFORMATION_OUT {}
impl ::core::clone::Clone for TXFS_READ_BACKUP_INFORMATION_OUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union TXFS_READ_BACKUP_INFORMATION_OUT_0 {
    pub BufferLength: u32,
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for TXFS_READ_BACKUP_INFORMATION_OUT_0 {}
impl ::core::clone::Clone for TXFS_READ_BACKUP_INFORMATION_OUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TXFS_ROLLFORWARD_REDO_INFORMATION {
    pub LastVirtualClock: i64,
    pub LastRedoLsn: u64,
    pub HighestRecoveryLsn: u64,
    pub Flags: u32,
}
impl ::core::marker::Copy for TXFS_ROLLFORWARD_REDO_INFORMATION {}
impl ::core::clone::Clone for TXFS_ROLLFORWARD_REDO_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TXFS_SAVEPOINT_INFORMATION {
    pub KtmTransaction: super::super::super::Win32::Foundation::HANDLE,
    pub ActionCode: u32,
    pub SavepointId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TXFS_SAVEPOINT_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TXFS_SAVEPOINT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TXFS_START_RM_INFORMATION {
    pub Flags: u32,
    pub LogContainerSize: u64,
    pub LogContainerCountMin: u32,
    pub LogContainerCountMax: u32,
    pub LogGrowthIncrement: u32,
    pub LogAutoShrinkPercentage: u32,
    pub TmLogPathOffset: u32,
    pub TmLogPathLength: u16,
    pub LoggingMode: u16,
    pub LogPathLength: u16,
    pub Reserved: u16,
    pub LogPath: [u16; 1],
}
impl ::core::marker::Copy for TXFS_START_RM_INFORMATION {}
impl ::core::clone::Clone for TXFS_START_RM_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TXFS_TRANSACTION_ACTIVE_INFO {
    pub TransactionsActiveAtSnapshot: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TXFS_TRANSACTION_ACTIVE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TXFS_TRANSACTION_ACTIVE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct TXFS_WRITE_BACKUP_INFORMATION {
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for TXFS_WRITE_BACKUP_INFORMATION {}
impl ::core::clone::Clone for TXFS_WRITE_BACKUP_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
pub struct UNICODE_PREFIX_TABLE {
    pub NodeTypeCode: i16,
    pub NameLength: i16,
    pub NextPrefixTree: *mut UNICODE_PREFIX_TABLE_ENTRY,
    pub LastNextEntry: *mut UNICODE_PREFIX_TABLE_ENTRY,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
impl ::core::marker::Copy for UNICODE_PREFIX_TABLE {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
impl ::core::clone::Clone for UNICODE_PREFIX_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
pub struct UNICODE_PREFIX_TABLE_ENTRY {
    pub NodeTypeCode: i16,
    pub NameLength: i16,
    pub NextPrefixTree: *mut UNICODE_PREFIX_TABLE_ENTRY,
    pub CaseMatch: *mut UNICODE_PREFIX_TABLE_ENTRY,
    pub Links: super::super::Foundation::RTL_SPLAY_LINKS,
    pub Prefix: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
impl ::core::marker::Copy for UNICODE_PREFIX_TABLE_ENTRY {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
impl ::core::clone::Clone for UNICODE_PREFIX_TABLE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct USN_JOURNAL_DATA {
    pub UsnJournalID: u64,
    pub FirstUsn: i64,
    pub NextUsn: i64,
    pub LowestValidUsn: i64,
    pub MaxUsn: i64,
    pub MaximumSize: u64,
    pub AllocationDelta: u64,
    pub MinSupportedMajorVersion: u16,
    pub MaxSupportedMajorVersion: u16,
}
impl ::core::marker::Copy for USN_JOURNAL_DATA {}
impl ::core::clone::Clone for USN_JOURNAL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct USN_JOURNAL_DATA_V0 {
    pub UsnJournalID: u64,
    pub FirstUsn: i64,
    pub NextUsn: i64,
    pub LowestValidUsn: i64,
    pub MaxUsn: i64,
    pub MaximumSize: u64,
    pub AllocationDelta: u64,
}
impl ::core::marker::Copy for USN_JOURNAL_DATA_V0 {}
impl ::core::clone::Clone for USN_JOURNAL_DATA_V0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct USN_JOURNAL_DATA_V2 {
    pub UsnJournalID: u64,
    pub FirstUsn: i64,
    pub NextUsn: i64,
    pub LowestValidUsn: i64,
    pub MaxUsn: i64,
    pub MaximumSize: u64,
    pub AllocationDelta: u64,
    pub MinSupportedMajorVersion: u16,
    pub MaxSupportedMajorVersion: u16,
    pub Flags: u32,
    pub RangeTrackChunkSize: u64,
    pub RangeTrackFileSizeThreshold: i64,
}
impl ::core::marker::Copy for USN_JOURNAL_DATA_V2 {}
impl ::core::clone::Clone for USN_JOURNAL_DATA_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct USN_RANGE_TRACK_OUTPUT {
    pub Usn: i64,
}
impl ::core::marker::Copy for USN_RANGE_TRACK_OUTPUT {}
impl ::core::clone::Clone for USN_RANGE_TRACK_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct USN_RECORD {
    pub RecordLength: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub FileReferenceNumber: u64,
    pub ParentFileReferenceNumber: u64,
    pub Usn: i64,
    pub TimeStamp: i64,
    pub Reason: u32,
    pub SourceInfo: u32,
    pub SecurityId: u32,
    pub FileAttributes: u32,
    pub FileNameLength: u16,
    pub FileNameOffset: u16,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for USN_RECORD {}
impl ::core::clone::Clone for USN_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct USN_RECORD_COMMON_HEADER {
    pub RecordLength: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl ::core::marker::Copy for USN_RECORD_COMMON_HEADER {}
impl ::core::clone::Clone for USN_RECORD_COMMON_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct USN_RECORD_EXTENT {
    pub Offset: i64,
    pub Length: i64,
}
impl ::core::marker::Copy for USN_RECORD_EXTENT {}
impl ::core::clone::Clone for USN_RECORD_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub union USN_RECORD_UNION {
    pub Header: USN_RECORD_COMMON_HEADER,
    pub V2: USN_RECORD,
    pub V3: USN_RECORD_V3,
    pub V4: USN_RECORD_V4,
}
impl ::core::marker::Copy for USN_RECORD_UNION {}
impl ::core::clone::Clone for USN_RECORD_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct USN_RECORD_V3 {
    pub RecordLength: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub FileReferenceNumber: FILE_ID_128,
    pub ParentFileReferenceNumber: FILE_ID_128,
    pub Usn: i64,
    pub TimeStamp: i64,
    pub Reason: u32,
    pub SourceInfo: u32,
    pub SecurityId: u32,
    pub FileAttributes: u32,
    pub FileNameLength: u16,
    pub FileNameOffset: u16,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for USN_RECORD_V3 {}
impl ::core::clone::Clone for USN_RECORD_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct USN_RECORD_V4 {
    pub Header: USN_RECORD_COMMON_HEADER,
    pub FileReferenceNumber: FILE_ID_128,
    pub ParentFileReferenceNumber: FILE_ID_128,
    pub Usn: i64,
    pub Reason: u32,
    pub SourceInfo: u32,
    pub RemainingExtents: u32,
    pub NumberOfExtents: u16,
    pub ExtentSize: u16,
    pub Extents: [USN_RECORD_EXTENT; 1],
}
impl ::core::marker::Copy for USN_RECORD_V4 {}
impl ::core::clone::Clone for USN_RECORD_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct USN_TRACK_MODIFIED_RANGES {
    pub Flags: u32,
    pub Unused: u32,
    pub ChunkSize: u64,
    pub FileSizeThreshold: i64,
}
impl ::core::marker::Copy for USN_TRACK_MODIFIED_RANGES {}
impl ::core::clone::Clone for USN_TRACK_MODIFIED_RANGES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct VCN_RANGE_INPUT_BUFFER {
    pub StartingVcn: i64,
    pub ClusterCount: i64,
}
impl ::core::marker::Copy for VCN_RANGE_INPUT_BUFFER {}
impl ::core::clone::Clone for VCN_RANGE_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct VIRTUALIZATION_INSTANCE_INFO_INPUT {
    pub NumberOfWorkerThreads: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for VIRTUALIZATION_INSTANCE_INFO_INPUT {}
impl ::core::clone::Clone for VIRTUALIZATION_INSTANCE_INFO_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {
    pub HeaderSize: u16,
    pub Flags: u32,
    pub NotificationInfoSize: u32,
    pub NotificationInfoOffset: u16,
    pub ProviderMajorVersion: u16,
}
impl ::core::marker::Copy for VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {}
impl ::core::clone::Clone for VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    pub VirtualizationInstanceID: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {}
impl ::core::clone::Clone for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {
    pub Size: u32,
    pub BehaviorCode: VIRTUAL_STORAGE_BEHAVIOR_CODE,
}
impl ::core::marker::Copy for VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {}
impl ::core::clone::Clone for VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct VIRTUAL_STORAGE_TYPE {
    pub DeviceId: u32,
    pub VendorId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VIRTUAL_STORAGE_TYPE {}
impl ::core::clone::Clone for VIRTUAL_STORAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct VOLUME_BITMAP_BUFFER {
    pub StartingLcn: i64,
    pub BitmapSize: i64,
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for VOLUME_BITMAP_BUFFER {}
impl ::core::clone::Clone for VOLUME_BITMAP_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct VOLUME_REFS_INFO_BUFFER {
    pub CacheSizeInBytes: i64,
    pub AllocatedCacheInBytes: i64,
    pub PopulatedCacheInBytes: i64,
    pub InErrorCacheInBytes: i64,
    pub MemoryUsedForCacheMetadata: i64,
    pub CacheLineSize: u32,
    pub CacheTransactionsOutstanding: i32,
    pub CacheLinesFree: i32,
    pub CacheLinesInError: i32,
    pub CacheHitsInBytes: i64,
    pub CacheMissesInBytes: i64,
    pub CachePopulationUpdatesInBytes: i64,
    pub CacheWriteThroughUpdatesInBytes: i64,
    pub CacheInvalidationsInBytes: i64,
    pub CacheOverReadsInBytes: i64,
    pub MetadataWrittenBytes: i64,
    pub CacheHitCounter: i32,
    pub CacheMissCounter: i32,
    pub CacheLineAllocationCounter: i32,
    pub CacheInvalidationsCounter: i32,
    pub CachePopulationUpdatesCounter: i32,
    pub CacheWriteThroughUpdatesCounter: i32,
    pub MaxCacheTransactionsOutstanding: i32,
    pub DataWritesReallocationCount: i32,
    pub DataInPlaceWriteCount: i32,
    pub MetadataAllocationsFastTierCount: i32,
    pub MetadataAllocationsSlowTierCount: i32,
    pub DataAllocationsFastTierCount: i32,
    pub DataAllocationsSlowTierCount: i32,
    pub DestagesSlowTierToFastTier: i32,
    pub DestagesFastTierToSlowTier: i32,
    pub SlowTierDataFillRatio: i32,
    pub FastTierDataFillRatio: i32,
    pub SlowTierMetadataFillRatio: i32,
    pub FastTierMetadataFillRatio: i32,
    pub SlowToFastDestageReadLatency: i32,
    pub SlowToFastDestageReadLatencyBase: i32,
    pub SlowToFastDestageWriteLatency: i32,
    pub SlowToFastDestageWriteLatencyBase: i32,
    pub FastToSlowDestageReadLatency: i32,
    pub FastToSlowDestageReadLatencyBase: i32,
    pub FastToSlowDestageWriteLatency: i32,
    pub FastToSlowDestageWriteLatencyBase: i32,
    pub SlowTierContainerFillRatio: i32,
    pub SlowTierContainerFillRatioBase: i32,
    pub FastTierContainerFillRatio: i32,
    pub FastTierContainerFillRatioBase: i32,
    pub TreeUpdateLatency: i32,
    pub TreeUpdateLatencyBase: i32,
    pub CheckpointLatency: i32,
    pub CheckpointLatencyBase: i32,
    pub TreeUpdateCount: i32,
    pub CheckpointCount: i32,
    pub LogWriteCount: i32,
    pub LogFillRatio: i32,
    pub ReadCacheInvalidationsForOverwrite: i32,
    pub ReadCacheInvalidationsForReuse: i32,
    pub ReadCacheInvalidationsGeneral: i32,
    pub ReadCacheChecksOnMount: i32,
    pub ReadCacheIssuesOnMount: i32,
    pub TrimLatency: i32,
    pub TrimLatencyBase: i32,
    pub DataCompactionCount: i32,
    pub CompactionReadLatency: i32,
    pub CompactionReadLatencyBase: i32,
    pub CompactionWriteLatency: i32,
    pub CompactionWriteLatencyBase: i32,
    pub DataInPlaceWriteClusterCount: i64,
    pub CompactionFailedDueToIneligibleContainer: i32,
    pub CompactionFailedDueToMaxFragmentation: i32,
    pub CompactedContainerFillRatio: i32,
    pub CompactedContainerFillRatioBase: i32,
    pub ContainerMoveRetryCount: i32,
    pub ContainerMoveFailedDueToIneligibleContainer: i32,
    pub CompactionFailureCount: i32,
    pub ContainerMoveFailureCount: i32,
    pub NumberOfDirtyMetadataPages: i64,
    pub NumberOfDirtyTableListEntries: i32,
    pub NumberOfDeleteQueueEntries: i32,
}
impl ::core::marker::Copy for VOLUME_REFS_INFO_BUFFER {}
impl ::core::clone::Clone for VOLUME_REFS_INFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct WIM_PROVIDER_ADD_OVERLAY_INPUT {
    pub WimType: u32,
    pub WimIndex: u32,
    pub WimFileNameOffset: u32,
    pub WimFileNameLength: u32,
}
impl ::core::marker::Copy for WIM_PROVIDER_ADD_OVERLAY_INPUT {}
impl ::core::clone::Clone for WIM_PROVIDER_ADD_OVERLAY_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct WIM_PROVIDER_EXTERNAL_INFO {
    pub Version: u32,
    pub Flags: u32,
    pub DataSourceId: i64,
    pub ResourceHash: [u8; 20],
}
impl ::core::marker::Copy for WIM_PROVIDER_EXTERNAL_INFO {}
impl ::core::clone::Clone for WIM_PROVIDER_EXTERNAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct WIM_PROVIDER_OVERLAY_ENTRY {
    pub NextEntryOffset: u32,
    pub DataSourceId: i64,
    pub WimGuid: ::windows_sys::core::GUID,
    pub WimFileNameOffset: u32,
    pub WimType: u32,
    pub WimIndex: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for WIM_PROVIDER_OVERLAY_ENTRY {}
impl ::core::clone::Clone for WIM_PROVIDER_OVERLAY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    pub DataSourceId: i64,
}
impl ::core::marker::Copy for WIM_PROVIDER_REMOVE_OVERLAY_INPUT {}
impl ::core::clone::Clone for WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    pub DataSourceId: i64,
}
impl ::core::marker::Copy for WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {}
impl ::core::clone::Clone for WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    pub DataSourceId: i64,
    pub WimFileNameOffset: u32,
    pub WimFileNameLength: u32,
}
impl ::core::marker::Copy for WIM_PROVIDER_UPDATE_OVERLAY_INPUT {}
impl ::core::clone::Clone for WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct WOF_EXTERNAL_FILE_ID {
    pub FileId: FILE_ID_128,
}
impl ::core::marker::Copy for WOF_EXTERNAL_FILE_ID {}
impl ::core::clone::Clone for WOF_EXTERNAL_FILE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct WOF_EXTERNAL_INFO {
    pub Version: u32,
    pub Provider: u32,
}
impl ::core::marker::Copy for WOF_EXTERNAL_INFO {}
impl ::core::clone::Clone for WOF_EXTERNAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct WOF_VERSION_INFO {
    pub WofVersion: u32,
}
impl ::core::marker::Copy for WOF_VERSION_INFO {}
impl ::core::clone::Clone for WOF_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub struct WRITE_USN_REASON_INPUT {
    pub Flags: u32,
    pub UsnReasonToWrite: u32,
}
impl ::core::marker::Copy for WRITE_USN_REASON_INPUT {}
impl ::core::clone::Clone for WRITE_USN_REASON_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct _REAL_NOTIFY_SYNC(pub u8);
impl ::core::marker::Copy for _REAL_NOTIFY_SYNC {}
impl ::core::clone::Clone for _REAL_NOTIFY_SYNC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type ACCEPT_SECURITY_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: *mut SecHandle, param2: *mut SecBufferDesc, param3: u32, param4: u32, param5: *mut SecHandle, param6: *mut SecBufferDesc, param7: *mut u32, param8: *mut i64) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACQUIRE_CREDENTIALS_HANDLE_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::super::Win32::Foundation::UNICODE_STRING, param1: *mut super::super::super::Win32::Foundation::UNICODE_STRING, param2: u32, param3: *mut ::core::ffi::c_void, param4: *mut ::core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut ::core::ffi::c_void, param7: *mut SecHandle, param8: *mut i64) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type ADD_CREDENTIALS_FN_A = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: *mut i8, param2: *mut i8, param3: u32, param4: *mut ::core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut ::core::ffi::c_void, param7: *mut i64) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ADD_CREDENTIALS_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: *mut super::super::super::Win32::Foundation::UNICODE_STRING, param2: *mut super::super::super::Win32::Foundation::UNICODE_STRING, param3: u32, param4: *mut ::core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut ::core::ffi::c_void, param7: *mut i64) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Memory\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
pub type ALLOCATE_VIRTUAL_MEMORY_EX_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackcontext: super::super::super::Win32::Foundation::HANDLE, processhandle: super::super::super::Win32::Foundation::HANDLE, baseaddress: *mut *mut ::core::ffi::c_void, regionsize: *mut usize, allocationtype: u32, pageprotection: u32, extendedparameters: *mut super::super::super::Win32::System::Memory::MEM_EXTENDED_PARAMETER, extendedparametercount: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type APPLY_CONTROL_TOKEN_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: *mut SecBufferDesc) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type COMPLETE_AUTH_TOKEN_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: *mut SecBufferDesc) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type DECRYPT_MESSAGE_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: *mut SecBufferDesc, param2: u32, param3: *mut u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type DELETE_SECURITY_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub type DRIVER_FS_NOTIFICATION = ::core::option::Option<unsafe extern "system" fn(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, fsactive: super::super::super::Win32::Foundation::BOOLEAN) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type ENCRYPT_MESSAGE_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: u32, param2: *mut SecBufferDesc, param3: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type ENUMERATE_SECURITY_PACKAGES_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut *mut SecPkgInfoW) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type EXPORT_SECURITY_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: u32, param2: *mut SecBuffer, param3: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FREE_CONTEXT_BUFFER_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type FREE_CREDENTIALS_HANDLE_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FREE_VIRTUAL_MEMORY_EX_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackcontext: super::super::super::Win32::Foundation::HANDLE, processhandle: super::super::super::Win32::Foundation::HANDLE, baseaddress: *mut *mut ::core::ffi::c_void, regionsize: *mut usize, freetype: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type IMPERSONATE_SECURITY_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type IMPORT_SECURITY_CONTEXT_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::super::Win32::Foundation::UNICODE_STRING, param1: *mut SecBuffer, param2: *mut ::core::ffi::c_void, param3: *mut SecHandle) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type INITIALIZE_SECURITY_CONTEXT_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: *mut SecHandle, param2: *mut super::super::super::Win32::Foundation::UNICODE_STRING, param3: u32, param4: u32, param5: u32, param6: *mut SecBufferDesc, param7: u32, param8: *mut SecHandle, param9: *mut SecBufferDesc, param10: *mut u32, param11: *mut i64) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type INIT_SECURITY_INTERFACE_W = ::core::option::Option<unsafe extern "system" fn() -> *mut SecurityFunctionTableW>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type MAKE_SIGNATURE_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: u32, param2: *mut SecBufferDesc, param3: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PACQUIRE_FOR_LAZY_WRITE = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, wait: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PACQUIRE_FOR_LAZY_WRITE_EX = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, inflags: u32, outflags: *mut u32) -> super::super::super::Win32::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PACQUIRE_FOR_READ_AHEAD = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, wait: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PALLOCATE_VIRTUAL_MEMORY_EX_CALLBACK = ::core::option::Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PASYNC_READ_COMPLETION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type PCC_POST_DEFERRED_WRITE = ::core::option::Option<unsafe extern "system" fn(context1: *const ::core::ffi::c_void, context2: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type PCHECK_FOR_TRAVERSE_ACCESS = ::core::option::Option<unsafe extern "system" fn(notifycontext: *const ::core::ffi::c_void, targetcontext: *const ::core::ffi::c_void, subjectcontext: *const super::super::Foundation::SECURITY_SUBJECT_CONTEXT) -> super::super::super::Win32::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub type PCOMPLETE_LOCK_IRP_ROUTINE = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, irp: *const super::super::Foundation::IRP) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub type PDIRTY_PAGE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(fileobject: *const super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, oldestlsn: *const i64, newestlsn: *const i64, context1: *const ::core::ffi::c_void, context2: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type PDRIVER_FS_NOTIFICATION = ::core::option::Option<unsafe extern "system" fn() -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFILTER_REPORT_CHANGE = ::core::option::Option<unsafe extern "system" fn(notifycontext: *const ::core::ffi::c_void, filtercontext: *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type PFLUSH_TO_LSN = ::core::option::Option<unsafe extern "system" fn(loghandle: *const ::core::ffi::c_void, lsn: i64) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type PFN_FSRTLTEARDOWNPERSTREAMCONTEXTS = ::core::option::Option<unsafe extern "system" fn(advancedheader: *const FSRTL_ADVANCED_FCB_HEADER) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFREE_VIRTUAL_MEMORY_EX_CALLBACK = ::core::option::Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK = ::core::option::Option<unsafe extern "system" fn(ecpcontext: *mut ::core::ffi::c_void, ecptype: *const ::windows_sys::core::GUID) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type PFSRTL_STACK_OVERFLOW_ROUTINE = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, event: *const super::super::Foundation::KEVENT) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub type PFS_FILTER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(data: *const FS_FILTER_CALLBACK_DATA, completioncontext: *mut *mut ::core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub type PFS_FILTER_COMPLETION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(data: *const FS_FILTER_CALLBACK_DATA, operationstatus: super::super::super::Win32::Foundation::NTSTATUS, completioncontext: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type PIO_IRP_EXT_PROCESS_TRACKED_OFFSET_CALLBACK = ::core::option::Option<unsafe extern "system" fn(sourcecontext: *const IO_IRP_EXT_TRACK_OFFSET_HEADER, targetcontext: *mut IO_IRP_EXT_TRACK_OFFSET_HEADER, relativeoffset: i64) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub type POPLOCK_FS_PREPOST_IRP = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, irp: *const super::super::Foundation::IRP) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub type POPLOCK_NOTIFY_ROUTINE = ::core::option::Option<unsafe extern "system" fn(notifyparams: *const OPLOCK_NOTIFY_PARAMS) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub type POPLOCK_WAIT_COMPLETE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, irp: *const super::super::Foundation::IRP) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type PQUERY_LOG_USAGE = ::core::option::Option<unsafe extern "system" fn(loghandle: *const ::core::ffi::c_void, percentagefull: *mut u16) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PQUERY_VIRTUAL_MEMORY_CALLBACK = ::core::option::Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type PRELEASE_FROM_LAZY_WRITE = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type PRELEASE_FROM_READ_AHEAD = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type PRTL_ALLOCATE_STRING_ROUTINE = ::core::option::Option<unsafe extern "system" fn() -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type PRTL_FREE_STRING_ROUTINE = ::core::option::Option<unsafe extern "system" fn() -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRTL_HEAP_COMMIT_ROUTINE = ::core::option::Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type PRTL_REALLOCATE_STRING_ROUTINE = ::core::option::Option<unsafe extern "system" fn() -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSE_LOGON_SESSION_TERMINATED_ROUTINE = ::core::option::Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSE_LOGON_SESSION_TERMINATED_ROUTINE_EX = ::core::option::Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Wdk_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Security\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Power\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Security", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub type PUNLOCK_ROUTINE = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, filelockinfo: *const FILE_LOCK_INFO) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type QUERY_CONTEXT_ATTRIBUTES_EX_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: u32, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type QUERY_CONTEXT_ATTRIBUTES_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: u32, param2: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: u32, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type QUERY_CREDENTIALS_ATTRIBUTES_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: u32, param2: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type QUERY_SECURITY_CONTEXT_TOKEN_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type QUERY_SECURITY_PACKAGE_INFO_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::super::Win32::Foundation::UNICODE_STRING, param1: *mut *mut SecPkgInfoW) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type QUERY_VIRTUAL_MEMORY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackcontext: super::super::super::Win32::Foundation::HANDLE, processhandle: super::super::super::Win32::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, memoryinformationclass: HEAP_MEMORY_INFO_CLASS, memoryinformation: *mut ::core::ffi::c_void, memoryinformationlength: usize, returnlength: *mut usize) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type REVERT_SECURITY_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type RTL_ALLOCATE_STRING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(numberofbytes: usize) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type RTL_FREE_STRING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(buffer: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type RTL_HEAP_COMMIT_ROUTINE = ::core::option::Option<unsafe extern "system" fn(base: *const ::core::ffi::c_void, commitaddress: *mut *mut ::core::ffi::c_void, commitsize: *mut usize) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type RTL_REALLOCATE_STRING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(numberofbytes: usize, buffer: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type SEC_GET_KEY_FN = ::core::option::Option<unsafe extern "system" fn(arg: *mut ::core::ffi::c_void, principal: *mut ::core::ffi::c_void, keyver: u32, key: *mut *mut ::core::ffi::c_void, status: *mut ::windows_sys::core::HRESULT) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type SET_CONTEXT_ATTRIBUTES_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: u32, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type SET_CREDENTIALS_ATTRIBUTES_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: u32, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SE_LOGON_SESSION_TERMINATED_ROUTINE = ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Win32::Foundation::LUID) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation"))]
pub type SE_LOGON_SESSION_TERMINATED_ROUTINE_EX = ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Win32::Foundation::LUID, pserversilo: *const super::super::Foundation::EJOB, context: *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type SspiAsyncNotifyCallback = ::core::option::Option<unsafe extern "system" fn(handle: *const SspiAsyncContext, callbackdata: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Wdk_Storage_FileSystem\"`*"]
pub type VERIFY_SIGNATURE_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut SecHandle, param1: *mut SecBufferDesc, param2: u32, param3: *mut u32) -> ::windows_sys::core::HRESULT>;
