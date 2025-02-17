#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalCategoriesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocalCategoriesStatics {
    type Vtable = ILocalCategoriesStatics_Vtbl;
}
impl ::core::clone::Clone for ILocalCategoriesStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILocalCategoriesStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf49399f5_8261_4321_9974_ef92d49a8dca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalCategoriesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BankAndCreditUnions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EatDrink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Hospitals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HotelsAndMotels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub All: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Parking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SeeDo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Shop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocalLocation {
    type Vtable = ILocalLocation_Vtbl;
}
impl ::core::clone::Clone for ILocalLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILocalLocation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb0fe9ab_4502_4f2c_94a9_0d60de0e2163);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Identifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Point: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Point: usize,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DataAttribution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocalLocation2 {
    type Vtable = ILocalLocation2_Vtbl;
}
impl ::core::clone::Clone for ILocalLocation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILocalLocation2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e9e307c_ecb5_4ffc_bb8c_ba50ba8c2dc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocation2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RatingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub HoursOfOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    HoursOfOperation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocationFinderResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocalLocationFinderResult {
    type Vtable = ILocalLocationFinderResult_Vtbl;
}
impl ::core::clone::Clone for ILocalLocationFinderResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILocalLocationFinderResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd09b6cc6_f338_4191_9fd8_5440b9a68f52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationFinderResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub LocalLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LocalLocations: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LocalLocationFinderStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocationFinderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocalLocationFinderStatics {
    type Vtable = ILocalLocationFinderStatics_Vtbl;
}
impl ::core::clone::Clone for ILocalLocationFinderStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILocalLocationFinderStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2ef7344_a0de_48ca_81a8_07c7dcfd37ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationFinderStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocalLocationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchterm: ::std::mem::MaybeUninit<::windows_core::HSTRING>, searcharea: *mut ::core::ffi::c_void, localcategory: ::std::mem::MaybeUninit<::windows_core::HSTRING>, maxresults: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocalLocationsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocationHoursOfOperationItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocalLocationHoursOfOperationItem {
    type Vtable = ILocalLocationHoursOfOperationItem_Vtbl;
}
impl ::core::clone::Clone for ILocalLocationHoursOfOperationItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILocalLocationHoursOfOperationItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23548c72_a1c7_43f1_a4f0_1091c39ec640);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationHoursOfOperationItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub Day: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Globalization::DayOfWeek) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    Day: usize,
    #[cfg(feature = "Foundation")]
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Start: usize,
    #[cfg(feature = "Foundation")]
    pub Span: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Span: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocationRatingInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocalLocationRatingInfo {
    type Vtable = ILocalLocationRatingInfo_Vtbl;
}
impl ::core::clone::Clone for ILocalLocationRatingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILocalLocationRatingInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb1dab56_3354_4311_8bc0_a2d4d5eb806e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationRatingInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AggregateRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AggregateRating: usize,
    #[cfg(feature = "Foundation")]
    pub RatingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RatingCount: usize,
    pub ProviderIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaceInfoHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaceInfoHelperStatics {
    type Vtable = IPlaceInfoHelperStatics_Vtbl;
}
impl ::core::clone::Clone for IPlaceInfoHelperStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPlaceInfoHelperStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd1ca9a7_a9c6_491b_bc09_e80fcea48ee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfoHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromLocalLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
pub struct LocalCategories;
impl LocalCategories {
    pub fn BankAndCreditUnions() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).BankAndCreditUnions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn EatDrink() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).EatDrink)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Hospitals() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Hospitals)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HotelsAndMotels() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).HotelsAndMotels)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn All() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).All)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Parking() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Parking)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SeeDo() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).SeeDo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Shop() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Shop)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILocalCategoriesStatics<R, F: FnOnce(&ILocalCategoriesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LocalCategories, ILocalCategoriesStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for LocalCategories {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalCategories";
}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
pub struct LocalLocation(::windows_core::IUnknown);
impl LocalLocation {
    pub fn Address(&self) -> ::windows_core::Result<super::MapAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::MapAddress>();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Identifier(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Identifier)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Point(&self) -> ::windows_core::Result<super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::super::Devices::Geolocation::Geopoint>();
            (::windows_core::Interface::vtable(this).Point)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DataAttribution(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).DataAttribution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Category(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Category)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RatingInfo(&self) -> ::windows_core::Result<LocalLocationRatingInfo> {
        let this = &::windows_core::ComInterface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<LocalLocationRatingInfo>();
            (::windows_core::Interface::vtable(this).RatingInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HoursOfOperation(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocationHoursOfOperationItem>> {
        let this = &::windows_core::ComInterface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::super::Foundation::Collections::IVectorView<LocalLocationHoursOfOperationItem>>();
            (::windows_core::Interface::vtable(this).HoursOfOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LocalLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalLocation {}
impl ::core::fmt::Debug for LocalLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LocalLocation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocation;{bb0fe9ab-4502-4f2c-94a9-0d60de0e2163})");
}
impl ::core::clone::Clone for LocalLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LocalLocation {
    type Vtable = ILocalLocation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LocalLocation {
    const IID: ::windows_core::GUID = <ILocalLocation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LocalLocation {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocation";
}
::windows_core::imp::interface_hierarchy!(LocalLocation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LocalLocation {}
unsafe impl ::core::marker::Sync for LocalLocation {}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
pub struct LocalLocationFinder;
impl LocalLocationFinder {
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindLocalLocationsAsync(searchterm: &::windows_core::HSTRING, searcharea: &super::super::super::Devices::Geolocation::Geocircle, localcategory: &::windows_core::HSTRING, maxresults: u32) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<LocalLocationFinderResult>> {
        Self::ILocalLocationFinderStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::super::Foundation::IAsyncOperation<LocalLocationFinderResult>>();
            (::windows_core::Interface::vtable(this).FindLocalLocationsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(searchterm), ::core::mem::transmute_copy(searcharea), ::core::mem::transmute_copy(localcategory), maxresults, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILocalLocationFinderStatics<R, F: FnOnce(&ILocalLocationFinderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LocalLocationFinder, ILocalLocationFinderStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for LocalLocationFinder {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationFinder";
}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
pub struct LocalLocationFinderResult(::windows_core::IUnknown);
impl LocalLocationFinderResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LocalLocations(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocation>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::super::Foundation::Collections::IVectorView<LocalLocation>>();
            (::windows_core::Interface::vtable(this).LocalLocations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<LocalLocationFinderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<LocalLocationFinderStatus>();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LocalLocationFinderResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalLocationFinderResult {}
impl ::core::fmt::Debug for LocalLocationFinderResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocationFinderResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LocalLocationFinderResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocationFinderResult;{d09b6cc6-f338-4191-9fd8-5440b9a68f52})");
}
impl ::core::clone::Clone for LocalLocationFinderResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LocalLocationFinderResult {
    type Vtable = ILocalLocationFinderResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LocalLocationFinderResult {
    const IID: ::windows_core::GUID = <ILocalLocationFinderResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LocalLocationFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationFinderResult";
}
::windows_core::imp::interface_hierarchy!(LocalLocationFinderResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LocalLocationFinderResult {}
unsafe impl ::core::marker::Sync for LocalLocationFinderResult {}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
pub struct LocalLocationHoursOfOperationItem(::windows_core::IUnknown);
impl LocalLocationHoursOfOperationItem {
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn Day(&self) -> ::windows_core::Result<super::super::super::Globalization::DayOfWeek> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::super::Globalization::DayOfWeek>();
            (::windows_core::Interface::vtable(this).Day)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Start(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::super::Foundation::TimeSpan>();
            (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Span(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::super::Foundation::TimeSpan>();
            (::windows_core::Interface::vtable(this).Span)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LocalLocationHoursOfOperationItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalLocationHoursOfOperationItem {}
impl ::core::fmt::Debug for LocalLocationHoursOfOperationItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocationHoursOfOperationItem").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LocalLocationHoursOfOperationItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem;{23548c72-a1c7-43f1-a4f0-1091c39ec640})");
}
impl ::core::clone::Clone for LocalLocationHoursOfOperationItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LocalLocationHoursOfOperationItem {
    type Vtable = ILocalLocationHoursOfOperationItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LocalLocationHoursOfOperationItem {
    const IID: ::windows_core::GUID = <ILocalLocationHoursOfOperationItem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LocalLocationHoursOfOperationItem {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem";
}
::windows_core::imp::interface_hierarchy!(LocalLocationHoursOfOperationItem, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LocalLocationHoursOfOperationItem {}
unsafe impl ::core::marker::Sync for LocalLocationHoursOfOperationItem {}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
pub struct LocalLocationRatingInfo(::windows_core::IUnknown);
impl LocalLocationRatingInfo {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AggregateRating(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::super::Foundation::IReference<f64>>();
            (::windows_core::Interface::vtable(this).AggregateRating)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RatingCount(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::super::Foundation::IReference<i32>>();
            (::windows_core::Interface::vtable(this).RatingCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderIdentifier(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).ProviderIdentifier)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LocalLocationRatingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalLocationRatingInfo {}
impl ::core::fmt::Debug for LocalLocationRatingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocationRatingInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LocalLocationRatingInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo;{cb1dab56-3354-4311-8bc0-a2d4d5eb806e})");
}
impl ::core::clone::Clone for LocalLocationRatingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LocalLocationRatingInfo {
    type Vtable = ILocalLocationRatingInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LocalLocationRatingInfo {
    const IID: ::windows_core::GUID = <ILocalLocationRatingInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LocalLocationRatingInfo {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo";
}
::windows_core::imp::interface_hierarchy!(LocalLocationRatingInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LocalLocationRatingInfo {}
unsafe impl ::core::marker::Sync for LocalLocationRatingInfo {}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
pub struct PlaceInfoHelper;
impl PlaceInfoHelper {
    pub fn CreateFromLocalLocation(location: &LocalLocation) -> ::windows_core::Result<super::PlaceInfo> {
        Self::IPlaceInfoHelperStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::PlaceInfo>();
            (::windows_core::Interface::vtable(this).CreateFromLocalLocation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(location), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlaceInfoHelperStatics<R, F: FnOnce(&IPlaceInfoHelperStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlaceInfoHelper, IPlaceInfoHelperStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for PlaceInfoHelper {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.PlaceInfoHelper";
}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LocalLocationFinderStatus(pub i32);
impl LocalLocationFinderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const InvalidCategory: Self = Self(3i32);
    pub const InvalidSearchTerm: Self = Self(4i32);
    pub const InvalidSearchArea: Self = Self(5i32);
    pub const NetworkFailure: Self = Self(6i32);
    pub const NotSupported: Self = Self(7i32);
}
impl ::core::marker::Copy for LocalLocationFinderStatus {}
impl ::core::clone::Clone for LocalLocationFinderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LocalLocationFinderStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for LocalLocationFinderStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LocalLocationFinderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocationFinderStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LocalLocationFinderStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.LocalSearch.LocalLocationFinderStatus;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
