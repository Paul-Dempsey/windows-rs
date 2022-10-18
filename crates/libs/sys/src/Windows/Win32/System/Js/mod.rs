#[cfg_attr(windows, link(name = "windows"))]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsAddRef(r#ref: *mut ::core::ffi::c_void, count: *mut u32) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsBoolToBoolean(value: u8, booleanvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsBooleanToBool(value: *mut ::core::ffi::c_void, boolvalue: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsCallFunction(function: *mut ::core::ffi::c_void, arguments: *mut *mut ::core::ffi::c_void, argumentcount: u16, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsCollectGarbage(runtime: *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsConstructObject(function: *mut ::core::ffi::c_void, arguments: *mut *mut ::core::ffi::c_void, argumentcount: u16, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsConvertValueToBoolean(value: *mut ::core::ffi::c_void, booleanvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsConvertValueToNumber(value: *mut ::core::ffi::c_void, numbervalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsConvertValueToObject(value: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsConvertValueToString(value: *mut ::core::ffi::c_void, stringvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsCreateArray(length: u32, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsCreateContext(runtime: *mut ::core::ffi::c_void, debugapplication: super::Diagnostics::Debug::IDebugApplication64, newcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(target_arch = "x86")]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsCreateContext(runtime: *mut ::core::ffi::c_void, debugapplication: super::Diagnostics::Debug::IDebugApplication32, newcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsCreateError(message: *mut ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsCreateExternalObject(data: *mut ::core::ffi::c_void, finalizecallback: JsFinalizeCallback, object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsCreateFunction(nativefunction: JsNativeFunction, callbackstate: *mut ::core::ffi::c_void, function: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsCreateObject(object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsCreateRangeError(message: *mut ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsCreateReferenceError(message: *mut ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsCreateRuntime(attributes: JsRuntimeAttributes, runtimeversion: JsRuntimeVersion, threadservice: JsThreadServiceCallback, runtime: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsCreateSyntaxError(message: *mut ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsCreateTypeError(message: *mut ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsCreateURIError(message: *mut ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsDefineProperty(object: *mut ::core::ffi::c_void, propertyid: *mut ::core::ffi::c_void, propertydescriptor: *mut ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsDeleteIndexedProperty(object: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsDeleteProperty(object: *mut ::core::ffi::c_void, propertyid: *mut ::core::ffi::c_void, usestrictrules: u8, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsDisableRuntimeExecution(runtime: *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsDisposeRuntime(runtime: *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsDoubleToNumber(doublevalue: f64, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsEnableRuntimeExecution(runtime: *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsEnumerateHeap(enumerator: *mut super::Diagnostics::Debug::IActiveScriptProfilerHeapEnum) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsEquals(object1: *mut ::core::ffi::c_void, object2: *mut ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetAndClearException(exception: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetCurrentContext(currentcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetExtensionAllowed(object: *mut ::core::ffi::c_void, value: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetExternalData(object: *mut ::core::ffi::c_void, externaldata: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetFalseValue(falsevalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetGlobalObject(globalobject: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetIndexedProperty(object: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetNullValue(nullvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetOwnPropertyDescriptor(object: *mut ::core::ffi::c_void, propertyid: *mut ::core::ffi::c_void, propertydescriptor: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetOwnPropertyNames(object: *mut ::core::ffi::c_void, propertynames: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetProperty(object: *mut ::core::ffi::c_void, propertyid: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetPropertyIdFromName(name: ::windows_sys::core::PCWSTR, propertyid: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetPropertyNameFromId(propertyid: *mut ::core::ffi::c_void, name: *const *const u16) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetPrototype(object: *mut ::core::ffi::c_void, prototypeobject: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetRuntime(context: *mut ::core::ffi::c_void, runtime: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetRuntimeMemoryLimit(runtime: *mut ::core::ffi::c_void, memorylimit: *mut usize) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetRuntimeMemoryUsage(runtime: *mut ::core::ffi::c_void, memoryusage: *mut usize) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetStringLength(stringvalue: *mut ::core::ffi::c_void, length: *mut i32) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetTrueValue(truevalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetUndefinedValue(undefinedvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsGetValueType(value: *mut ::core::ffi::c_void, r#type: *mut JsValueType) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsHasException(hasexception: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsHasExternalData(object: *mut ::core::ffi::c_void, value: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsHasIndexedProperty(object: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsHasProperty(object: *mut ::core::ffi::c_void, propertyid: *mut ::core::ffi::c_void, hasproperty: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsIdle(nextidletick: *mut u32) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsIntToNumber(intvalue: i32, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsIsEnumeratingHeap(isenumeratingheap: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsIsRuntimeExecutionDisabled(runtime: *mut ::core::ffi::c_void, isdisabled: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsNumberToDouble(value: *mut ::core::ffi::c_void, doublevalue: *mut f64) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsParseScript(script: ::windows_sys::core::PCWSTR, sourcecontext: usize, sourceurl: ::windows_sys::core::PCWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsParseSerializedScript(script: ::windows_sys::core::PCWSTR, buffer: *mut u8, sourcecontext: usize, sourceurl: ::windows_sys::core::PCWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsPointerToString(stringvalue: ::windows_sys::core::PCWSTR, stringlength: usize, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsPreventExtension(object: *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsRelease(r#ref: *mut ::core::ffi::c_void, count: *mut u32) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsRunScript(script: ::windows_sys::core::PCWSTR, sourcecontext: usize, sourceurl: ::windows_sys::core::PCWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsRunSerializedScript(script: ::windows_sys::core::PCWSTR, buffer: *mut u8, sourcecontext: usize, sourceurl: ::windows_sys::core::PCWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsSerializeScript(script: ::windows_sys::core::PCWSTR, buffer: *mut u8, buffersize: *mut u32) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsSetCurrentContext(context: *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsSetException(exception: *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsSetExternalData(object: *mut ::core::ffi::c_void, externaldata: *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsSetIndexedProperty(object: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsSetProperty(object: *mut ::core::ffi::c_void, propertyid: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, usestrictrules: u8) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsSetPrototype(object: *mut ::core::ffi::c_void, prototypeobject: *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsSetRuntimeBeforeCollectCallback(runtime: *mut ::core::ffi::c_void, callbackstate: *mut ::core::ffi::c_void, beforecollectcallback: JsBeforeCollectCallback) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsSetRuntimeMemoryAllocationCallback(runtime: *mut ::core::ffi::c_void, callbackstate: *mut ::core::ffi::c_void, allocationcallback: JsMemoryAllocationCallback) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsSetRuntimeMemoryLimit(runtime: *mut ::core::ffi::c_void, memorylimit: usize) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsStartDebugging(debugapplication: super::Diagnostics::Debug::IDebugApplication64) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(target_arch = "x86")]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsStartDebugging(debugapplication: super::Diagnostics::Debug::IDebugApplication32) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsStartProfiling(callback: super::Diagnostics::Debug::IActiveScriptProfilerCallback, eventmask: super::Diagnostics::Debug::PROFILER_EVENT_MASK, context: u32) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsStopProfiling(reason: ::windows_sys::core::HRESULT) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsStrictEquals(object1: *mut ::core::ffi::c_void, object2: *mut ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`*"]
    pub fn JsStringToPointer(value: *mut ::core::ffi::c_void, stringvalue: *const *const u16, stringlength: *mut usize) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn JsValueToVariant(object: *mut ::core::ffi::c_void, variant: *mut super::Com::VARIANT) -> JsErrorCode;
    #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn JsVariantToValue(variant: *mut super::Com::VARIANT, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JS_SOURCE_CONTEXT_NONE: u64 = 18446744073709551615u64;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsErrorCode = u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsNoError: JsErrorCode = 0u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCategoryUsage: JsErrorCode = 65536u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInvalidArgument: JsErrorCode = 65537u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorNullArgument: JsErrorCode = 65538u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorNoCurrentContext: JsErrorCode = 65539u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInExceptionState: JsErrorCode = 65540u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorNotImplemented: JsErrorCode = 65541u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorWrongThread: JsErrorCode = 65542u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorRuntimeInUse: JsErrorCode = 65543u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorBadSerializedScript: JsErrorCode = 65544u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInDisabledState: JsErrorCode = 65545u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCannotDisableExecution: JsErrorCode = 65546u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorHeapEnumInProgress: JsErrorCode = 65547u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorArgumentNotObject: JsErrorCode = 65548u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInProfileCallback: JsErrorCode = 65549u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInThreadServiceCallback: JsErrorCode = 65550u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCannotSerializeDebugScript: JsErrorCode = 65551u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorAlreadyDebuggingContext: JsErrorCode = 65552u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorAlreadyProfilingContext: JsErrorCode = 65553u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorIdleNotEnabled: JsErrorCode = 65554u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCategoryEngine: JsErrorCode = 131072u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorOutOfMemory: JsErrorCode = 131073u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCategoryScript: JsErrorCode = 196608u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorScriptException: JsErrorCode = 196609u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorScriptCompile: JsErrorCode = 196610u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorScriptTerminated: JsErrorCode = 196611u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorScriptEvalDisabled: JsErrorCode = 196612u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCategoryFatal: JsErrorCode = 262144u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorFatal: JsErrorCode = 262145u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsMemoryEventType = i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsMemoryAllocate: JsMemoryEventType = 0i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsMemoryFree: JsMemoryEventType = 1i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsMemoryFailure: JsMemoryEventType = 2i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsRuntimeAttributes = i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeNone: JsRuntimeAttributes = 0i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeDisableBackgroundWork: JsRuntimeAttributes = 1i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeAllowScriptInterrupt: JsRuntimeAttributes = 2i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeEnableIdleProcessing: JsRuntimeAttributes = 4i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeDisableNativeCodeGeneration: JsRuntimeAttributes = 8i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeDisableEval: JsRuntimeAttributes = 16i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsRuntimeVersion = i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeVersion10: JsRuntimeVersion = 0i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeVersion11: JsRuntimeVersion = 1i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeVersionEdge: JsRuntimeVersion = -1i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsValueType = i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsUndefined: JsValueType = 0i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsNull: JsValueType = 1i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsNumber: JsValueType = 2i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsString: JsValueType = 3i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsBoolean: JsValueType = 4i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsObject: JsValueType = 5i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsFunction: JsValueType = 6i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsError: JsValueType = 7i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsArray: JsValueType = 8i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsBackgroundWorkItemCallback = ::core::option::Option<unsafe extern "system" fn(callbackstate: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsBeforeCollectCallback = ::core::option::Option<unsafe extern "system" fn(callbackstate: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsFinalizeCallback = ::core::option::Option<unsafe extern "system" fn(data: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsMemoryAllocationCallback = ::core::option::Option<unsafe extern "system" fn(callbackstate: *mut ::core::ffi::c_void, allocationevent: JsMemoryEventType, allocationsize: usize) -> bool>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsNativeFunction = ::core::option::Option<unsafe extern "system" fn(callee: *mut ::core::ffi::c_void, isconstructcall: bool, arguments: *mut *mut ::core::ffi::c_void, argumentcount: u16, callbackstate: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsThreadServiceCallback = ::core::option::Option<unsafe extern "system" fn(callback: JsBackgroundWorkItemCallback, callbackstate: *mut ::core::ffi::c_void) -> bool>;
