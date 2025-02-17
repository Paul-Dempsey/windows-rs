#[doc = "*Required features: `\"Win32_System_WinRT_Isolation\"`*"]
#[repr(transparent)]
pub struct IIsolatedEnvironmentInterop(::windows_core::IUnknown);
impl IIsolatedEnvironmentInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHostHwndInterop<P0>(&self, containerhwnd: P0) -> ::windows_core::Result<super::super::super::Foundation::HWND>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows_core::zeroed::<super::super::super::Foundation::HWND>();
        (::windows_core::Interface::vtable(self).GetHostHwndInterop)(::windows_core::Interface::as_raw(self), containerhwnd.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IIsolatedEnvironmentInterop, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IIsolatedEnvironmentInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIsolatedEnvironmentInterop {}
impl ::core::fmt::Debug for IIsolatedEnvironmentInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIsolatedEnvironmentInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IIsolatedEnvironmentInterop {
    type Vtable = IIsolatedEnvironmentInterop_Vtbl;
}
impl ::core::clone::Clone for IIsolatedEnvironmentInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIsolatedEnvironmentInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85713c2e_8e62_46c5_8de2_c647e1d54636);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedEnvironmentInterop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHostHwndInterop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHostHwndInterop: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
