#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreUserActivityManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreUserActivityManagerStatics {
    type Vtable = ICoreUserActivityManagerStatics_Vtbl;
}
impl ::core::clone::Clone for ICoreUserActivityManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICoreUserActivityManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca3adb02_a4be_4d4d_bfa8_6795f4264efb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreUserActivityManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateUserActivitySessionInBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activity: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteUserActivitySessionsInTimeRangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: *mut ::core::ffi::c_void, starttime: super::super::super::Foundation::DateTime, endtime: super::super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteUserActivitySessionsInTimeRangeAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel_UserActivities_Core\"`*"]
pub struct CoreUserActivityManager;
impl CoreUserActivityManager {
    pub fn CreateUserActivitySessionInBackground(activity: &super::UserActivity) -> ::windows_core::Result<super::UserActivitySession> {
        Self::ICoreUserActivityManagerStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::UserActivitySession>();
            (::windows_core::Interface::vtable(this).CreateUserActivitySessionInBackground)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activity), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteUserActivitySessionsInTimeRangeAsync(channel: &super::UserActivityChannel, starttime: super::super::super::Foundation::DateTime, endtime: super::super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICoreUserActivityManagerStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).DeleteUserActivitySessionsInTimeRangeAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(channel), starttime, endtime, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreUserActivityManagerStatics<R, F: FnOnce(&ICoreUserActivityManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CoreUserActivityManager, ICoreUserActivityManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for CoreUserActivityManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.Core.CoreUserActivityManager";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
