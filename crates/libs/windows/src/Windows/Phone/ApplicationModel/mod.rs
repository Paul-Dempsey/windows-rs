#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationProfileStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationProfileStatics {
    type Vtable = IApplicationProfileStatics_Vtbl;
}
impl ::core::clone::Clone for IApplicationProfileStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IApplicationProfileStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5008ab4_7e7a_11e1_a7f2_b0a14824019b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationProfileStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Modes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationProfileModes) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Phone_ApplicationModel\"`*"]
pub struct ApplicationProfile;
impl ApplicationProfile {
    pub fn Modes() -> ::windows_core::Result<ApplicationProfileModes> {
        Self::IApplicationProfileStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<ApplicationProfileModes>();
            (::windows_core::Interface::vtable(this).Modes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IApplicationProfileStatics<R, F: FnOnce(&IApplicationProfileStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ApplicationProfile, IApplicationProfileStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ApplicationProfile {
    const NAME: &'static str = "Windows.Phone.ApplicationModel.ApplicationProfile";
}
#[doc = "*Required features: `\"Phone_ApplicationModel\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ApplicationProfileModes(pub u32);
impl ApplicationProfileModes {
    pub const Default: Self = Self(0u32);
    pub const Alternate: Self = Self(1u32);
}
impl ::core::marker::Copy for ApplicationProfileModes {}
impl ::core::clone::Clone for ApplicationProfileModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationProfileModes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ApplicationProfileModes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ApplicationProfileModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationProfileModes").field(&self.0).finish()
    }
}
impl ApplicationProfileModes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for ApplicationProfileModes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ApplicationProfileModes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ApplicationProfileModes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ApplicationProfileModes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ApplicationProfileModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for ApplicationProfileModes {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.ApplicationModel.ApplicationProfileModes;u4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
