#[doc(hidden)]
#[repr(transparent)]
pub struct IDualSimTile(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDualSimTile {
    type Vtable = IDualSimTile_Vtbl;
}
impl ::core::clone::Clone for IDualSimTile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDualSimTile {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x143ab213_d05f_4041_a18c_3e3fcb75b41e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDualSimTile_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsPinnedToStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDualSimTileStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDualSimTileStatics {
    type Vtable = IDualSimTileStatics_Vtbl;
}
impl ::core::clone::Clone for IDualSimTileStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDualSimTileStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50567c9e_c58f_4dc9_b6e8_fa6777eeeb37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDualSimTileStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetTileForSim2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UpdateDisplayNameForSim1Async: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateDisplayNameForSim1Async: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateTileUpdaterForSim1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateTileUpdaterForSim1: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateTileUpdaterForSim2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateTileUpdaterForSim2: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateBadgeUpdaterForSim1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateBadgeUpdaterForSim1: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateBadgeUpdaterForSim2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateBadgeUpdaterForSim2: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateToastNotifierForSim1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateToastNotifierForSim1: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateToastNotifierForSim2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateToastNotifierForSim2: usize,
}
#[doc = "*Required features: `\"Phone_StartScreen\"`*"]
#[repr(transparent)]
pub struct IToastNotificationManagerStatics3(::windows_core::IUnknown);
impl IToastNotificationManagerStatics3 {
    #[doc = "*Required features: `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn CreateToastNotifierForSecondaryTile(&self, tileid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::UI::Notifications::ToastNotifier> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::UI::Notifications::ToastNotifier>();
            (::windows_core::Interface::vtable(this).CreateToastNotifierForSecondaryTile)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(tileid), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IToastNotificationManagerStatics3, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IToastNotificationManagerStatics3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IToastNotificationManagerStatics3 {}
impl ::core::fmt::Debug for IToastNotificationManagerStatics3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IToastNotificationManagerStatics3").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IToastNotificationManagerStatics3 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{2717f54b-50df-4455-8e6e-41e0fc8e13ce}");
}
unsafe impl ::windows_core::Interface for IToastNotificationManagerStatics3 {
    type Vtable = IToastNotificationManagerStatics3_Vtbl;
}
impl ::core::clone::Clone for IToastNotificationManagerStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IToastNotificationManagerStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2717f54b_50df_4455_8e6e_41e0fc8e13ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Notifications")]
    pub CreateToastNotifierForSecondaryTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateToastNotifierForSecondaryTile: usize,
}
#[doc = "*Required features: `\"Phone_StartScreen\"`*"]
#[repr(transparent)]
pub struct DualSimTile(::windows_core::IUnknown);
impl DualSimTile {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DualSimTile, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPinnedToStart(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsPinnedToStart)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows_core::Interface::vtable(this).UpdateAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetTileForSim2() -> ::windows_core::Result<DualSimTile> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<DualSimTile>();
            (::windows_core::Interface::vtable(this).GetTileForSim2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateDisplayNameForSim1Async(name: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows_core::Interface::vtable(this).UpdateDisplayNameForSim1Async)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn CreateTileUpdaterForSim1() -> ::windows_core::Result<super::super::UI::Notifications::TileUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::UI::Notifications::TileUpdater>();
            (::windows_core::Interface::vtable(this).CreateTileUpdaterForSim1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn CreateTileUpdaterForSim2() -> ::windows_core::Result<super::super::UI::Notifications::TileUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::UI::Notifications::TileUpdater>();
            (::windows_core::Interface::vtable(this).CreateTileUpdaterForSim2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn CreateBadgeUpdaterForSim1() -> ::windows_core::Result<super::super::UI::Notifications::BadgeUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::UI::Notifications::BadgeUpdater>();
            (::windows_core::Interface::vtable(this).CreateBadgeUpdaterForSim1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn CreateBadgeUpdaterForSim2() -> ::windows_core::Result<super::super::UI::Notifications::BadgeUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::UI::Notifications::BadgeUpdater>();
            (::windows_core::Interface::vtable(this).CreateBadgeUpdaterForSim2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn CreateToastNotifierForSim1() -> ::windows_core::Result<super::super::UI::Notifications::ToastNotifier> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::UI::Notifications::ToastNotifier>();
            (::windows_core::Interface::vtable(this).CreateToastNotifierForSim1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn CreateToastNotifierForSim2() -> ::windows_core::Result<super::super::UI::Notifications::ToastNotifier> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::UI::Notifications::ToastNotifier>();
            (::windows_core::Interface::vtable(this).CreateToastNotifierForSim2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDualSimTileStatics<R, F: FnOnce(&IDualSimTileStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DualSimTile, IDualSimTileStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for DualSimTile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DualSimTile {}
impl ::core::fmt::Debug for DualSimTile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DualSimTile").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DualSimTile {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.StartScreen.DualSimTile;{143ab213-d05f-4041-a18c-3e3fcb75b41e})");
}
impl ::core::clone::Clone for DualSimTile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DualSimTile {
    type Vtable = IDualSimTile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DualSimTile {
    const IID: ::windows_core::GUID = <IDualSimTile as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DualSimTile {
    const NAME: &'static str = "Windows.Phone.StartScreen.DualSimTile";
}
::windows_core::imp::interface_hierarchy!(DualSimTile, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
