#[doc(hidden)]
#[repr(transparent)]
pub struct IRequestingFocusOnKeyboardInputEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRequestingFocusOnKeyboardInputEventArgs {
    type Vtable = IRequestingFocusOnKeyboardInputEventArgs_Vtbl;
}
impl ::core::clone::Clone for IRequestingFocusOnKeyboardInputEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRequestingFocusOnKeyboardInputEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1195f27_b1a7_41a2_879d_6a68687e5985);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRequestingFocusOnKeyboardInputEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchSuggestion {
    type Vtable = ISearchSuggestion_Vtbl;
}
impl ::core::clone::Clone for ISearchSuggestion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISearchSuggestion {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b5554b0_1527_437b_95c5_8d18d2b8af55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestion_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SearchSuggestionKind) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DetailText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Image: usize,
    pub ImageAlternateText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestionManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchSuggestionManager {
    type Vtable = ISearchSuggestionManager_Vtbl;
}
impl ::core::clone::Clone for ISearchSuggestionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISearchSuggestionManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f0c50a1_cb9d_497b_b500_3c04ac959ad2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SearchHistoryEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSearchHistoryEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SearchHistoryContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSearchHistoryContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLocalContentSuggestionSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytext: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetQueryWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytext: ::std::mem::MaybeUninit<::windows_core::HSTRING>, language: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetQueryWithSearchQueryLinguisticDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytext: ::std::mem::MaybeUninit<::windows_core::HSTRING>, language: ::std::mem::MaybeUninit<::windows_core::HSTRING>, linguisticdetails: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Suggestions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Suggestions: usize,
    pub AddToHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytext: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddToHistoryWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytext: ::std::mem::MaybeUninit<::windows_core::HSTRING>, language: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ClearHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SuggestionsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SuggestionsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSuggestionsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSuggestionsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RequestingFocusOnKeyboardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestingFocusOnKeyboardInput: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRequestingFocusOnKeyboardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRequestingFocusOnKeyboardInput: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestionsRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchSuggestionsRequestedEventArgs {
    type Vtable = ISearchSuggestionsRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISearchSuggestionsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISearchSuggestionsRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6fd519e5_9e7e_4ab4_8be3_c76b1bd4344a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub QueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Search_Core\"`*"]
#[repr(transparent)]
pub struct RequestingFocusOnKeyboardInputEventArgs(::windows_core::IUnknown);
impl RequestingFocusOnKeyboardInputEventArgs {}
impl ::core::cmp::PartialEq for RequestingFocusOnKeyboardInputEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RequestingFocusOnKeyboardInputEventArgs {}
impl ::core::fmt::Debug for RequestingFocusOnKeyboardInputEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RequestingFocusOnKeyboardInputEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RequestingFocusOnKeyboardInputEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.Core.RequestingFocusOnKeyboardInputEventArgs;{a1195f27-b1a7-41a2-879d-6a68687e5985})");
}
impl ::core::clone::Clone for RequestingFocusOnKeyboardInputEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RequestingFocusOnKeyboardInputEventArgs {
    type Vtable = IRequestingFocusOnKeyboardInputEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RequestingFocusOnKeyboardInputEventArgs {
    const IID: ::windows_core::GUID = <IRequestingFocusOnKeyboardInputEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RequestingFocusOnKeyboardInputEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.Core.RequestingFocusOnKeyboardInputEventArgs";
}
::windows_core::imp::interface_hierarchy!(RequestingFocusOnKeyboardInputEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RequestingFocusOnKeyboardInputEventArgs {}
unsafe impl ::core::marker::Sync for RequestingFocusOnKeyboardInputEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Search_Core\"`*"]
#[repr(transparent)]
pub struct SearchSuggestion(::windows_core::IUnknown);
impl SearchSuggestion {
    pub fn Kind(&self) -> ::windows_core::Result<SearchSuggestionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<SearchSuggestionKind>();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DetailText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).DetailText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Image(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::super::Storage::Streams::IRandomAccessStreamReference>();
            (::windows_core::Interface::vtable(this).Image)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ImageAlternateText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).ImageAlternateText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SearchSuggestion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestion {}
impl ::core::fmt::Debug for SearchSuggestion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchSuggestion").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SearchSuggestion {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.Core.SearchSuggestion;{5b5554b0-1527-437b-95c5-8d18d2b8af55})");
}
impl ::core::clone::Clone for SearchSuggestion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SearchSuggestion {
    type Vtable = ISearchSuggestion_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SearchSuggestion {
    const IID: ::windows_core::GUID = <ISearchSuggestion as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SearchSuggestion {
    const NAME: &'static str = "Windows.ApplicationModel.Search.Core.SearchSuggestion";
}
::windows_core::imp::interface_hierarchy!(SearchSuggestion, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Search_Core\"`*"]
#[repr(transparent)]
pub struct SearchSuggestionManager(::windows_core::IUnknown);
impl SearchSuggestionManager {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SearchSuggestionManager, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SearchHistoryEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).SearchHistoryEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSearchHistoryEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSearchHistoryEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SearchHistoryContext(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).SearchHistoryContext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSearchHistoryContext(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSearchHistoryContext)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SetLocalContentSuggestionSettings(&self, settings: &super::LocalContentSuggestionSettings) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLocalContentSuggestionSettings)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(settings)).ok() }
    }
    pub fn SetQuery(&self, querytext: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetQuery)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(querytext)).ok() }
    }
    pub fn SetQueryWithLanguage(&self, querytext: &::windows_core::HSTRING, language: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetQueryWithLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(querytext), ::core::mem::transmute_copy(language)).ok() }
    }
    pub fn SetQueryWithSearchQueryLinguisticDetails(&self, querytext: &::windows_core::HSTRING, language: &::windows_core::HSTRING, linguisticdetails: &super::SearchQueryLinguisticDetails) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetQueryWithSearchQueryLinguisticDetails)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(querytext), ::core::mem::transmute_copy(language), ::core::mem::transmute_copy(linguisticdetails)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Suggestions(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IObservableVector<SearchSuggestion>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::super::Foundation::Collections::IObservableVector<SearchSuggestion>>();
            (::windows_core::Interface::vtable(this).Suggestions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddToHistory(&self, querytext: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddToHistory)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(querytext)).ok() }
    }
    pub fn AddToHistoryWithLanguage(&self, querytext: &::windows_core::HSTRING, language: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddToHistoryWithLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(querytext), ::core::mem::transmute_copy(language)).ok() }
    }
    pub fn ClearHistory(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearHistory)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SuggestionsRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<SearchSuggestionManager, SearchSuggestionsRequestedEventArgs>) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows_core::Interface::vtable(this).SuggestionsRequested)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSuggestionsRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSuggestionsRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestingFocusOnKeyboardInput(&self, handler: &super::super::super::Foundation::TypedEventHandler<SearchSuggestionManager, RequestingFocusOnKeyboardInputEventArgs>) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows_core::Interface::vtable(this).RequestingFocusOnKeyboardInput)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestingFocusOnKeyboardInput(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRequestingFocusOnKeyboardInput)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for SearchSuggestionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestionManager {}
impl ::core::fmt::Debug for SearchSuggestionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchSuggestionManager").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SearchSuggestionManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.Core.SearchSuggestionManager;{3f0c50a1-cb9d-497b-b500-3c04ac959ad2})");
}
impl ::core::clone::Clone for SearchSuggestionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SearchSuggestionManager {
    type Vtable = ISearchSuggestionManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SearchSuggestionManager {
    const IID: ::windows_core::GUID = <ISearchSuggestionManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SearchSuggestionManager {
    const NAME: &'static str = "Windows.ApplicationModel.Search.Core.SearchSuggestionManager";
}
::windows_core::imp::interface_hierarchy!(SearchSuggestionManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Search_Core\"`*"]
#[repr(transparent)]
pub struct SearchSuggestionsRequestedEventArgs(::windows_core::IUnknown);
impl SearchSuggestionsRequestedEventArgs {
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<super::SearchQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::SearchQueryLinguisticDetails>();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Request(&self) -> ::windows_core::Result<super::SearchSuggestionsRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::SearchSuggestionsRequest>();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SearchSuggestionsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestionsRequestedEventArgs {}
impl ::core::fmt::Debug for SearchSuggestionsRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchSuggestionsRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SearchSuggestionsRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.Core.SearchSuggestionsRequestedEventArgs;{6fd519e5-9e7e-4ab4-8be3-c76b1bd4344a})");
}
impl ::core::clone::Clone for SearchSuggestionsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SearchSuggestionsRequestedEventArgs {
    type Vtable = ISearchSuggestionsRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SearchSuggestionsRequestedEventArgs {
    const IID: ::windows_core::GUID = <ISearchSuggestionsRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SearchSuggestionsRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.Core.SearchSuggestionsRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SearchSuggestionsRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SearchSuggestionsRequestedEventArgs {}
unsafe impl ::core::marker::Sync for SearchSuggestionsRequestedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Search_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SearchSuggestionKind(pub i32);
impl SearchSuggestionKind {
    pub const Query: Self = Self(0i32);
    pub const Result: Self = Self(1i32);
    pub const Separator: Self = Self(2i32);
}
impl ::core::marker::Copy for SearchSuggestionKind {}
impl ::core::clone::Clone for SearchSuggestionKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SearchSuggestionKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SearchSuggestionKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SearchSuggestionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchSuggestionKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SearchSuggestionKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Search.Core.SearchSuggestionKind;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
