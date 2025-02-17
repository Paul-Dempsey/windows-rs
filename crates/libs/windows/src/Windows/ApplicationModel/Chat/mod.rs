#[doc(hidden)]
#[repr(transparent)]
pub struct IChatCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatCapabilities {
    type Vtable = IChatCapabilities_Vtbl;
}
impl ::core::clone::Clone for IChatCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatCapabilities {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3aff77bc_39c9_4dd1_ad2d_3964dd9d403f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsOnline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsChatCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsFileTransferCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsGeoLocationPushCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsIntegratedMessagingCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatCapabilitiesManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatCapabilitiesManagerStatics {
    type Vtable = IChatCapabilitiesManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IChatCapabilitiesManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatCapabilitiesManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb57a2f30_7041_458e_b0cf_7c0d9fea333a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatCapabilitiesManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetCachedCapabilitiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCachedCapabilitiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCapabilitiesFromNetworkAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCapabilitiesFromNetworkAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatCapabilitiesManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatCapabilitiesManagerStatics2 {
    type Vtable = IChatCapabilitiesManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IChatCapabilitiesManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatCapabilitiesManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe30d4274_d5c1_4ac9_9ffc_40e69184fec8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatCapabilitiesManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetCachedCapabilitiesForTransportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: ::std::mem::MaybeUninit<::windows_core::HSTRING>, transportid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCachedCapabilitiesForTransportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCapabilitiesFromNetworkForTransportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: ::std::mem::MaybeUninit<::windows_core::HSTRING>, transportid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCapabilitiesFromNetworkForTransportAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatConversation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatConversation {
    type Vtable = IChatConversation_Vtbl;
}
impl ::core::clone::Clone for IChatConversation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatConversation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa58c080d_1a6f_46dc_8f3d_f5028660b6ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HasUnreadMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsConversationMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsConversationMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub MostRecentMessageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Participants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Participants: usize,
    pub ThreadingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    pub GetMessageReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MarkAllMessagesAsReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkAllMessagesAsReadAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkMessagesAsReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessagesAsReadAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    pub NotifyLocalParticipantComposing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, participantaddress: ::std::mem::MaybeUninit<::windows_core::HSTRING>, iscomposing: bool) -> ::windows_core::HRESULT,
    pub NotifyRemoteParticipantComposing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, participantaddress: ::std::mem::MaybeUninit<::windows_core::HSTRING>, iscomposing: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemoteParticipantComposingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoteParticipantComposingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoteParticipantComposingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoteParticipantComposingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatConversation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatConversation2 {
    type Vtable = IChatConversation2_Vtbl;
}
impl ::core::clone::Clone for IChatConversation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatConversation2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a030cd1_983a_47aa_9a90_ee48ee997b59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversation2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanModifyParticipants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanModifyParticipants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatConversationReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatConversationReader {
    type Vtable = IChatConversationReader_Vtbl;
}
impl ::core::clone::Clone for IChatConversationReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatConversationReader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x055136d2_de32_4a47_a93a_b3dc0833852b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversationReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchWithCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchWithCountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatConversationThreadingInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatConversationThreadingInfo {
    type Vtable = IChatConversationThreadingInfo_Vtbl;
}
impl ::core::clone::Clone for IChatConversationThreadingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatConversationThreadingInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x331c21dc_7a07_4422_a32c_24be7c6dab24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversationThreadingInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ContactId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetContactId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Custom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ConversationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetConversationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Participants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Participants: usize,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatConversationThreadingKind) -> ::windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatConversationThreadingKind) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct IChatItem(::windows_core::IUnknown);
impl IChatItem {
    pub fn ItemKind(&self) -> ::windows_core::Result<ChatItemKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatItemKind>();
            (::windows_core::Interface::vtable(this).ItemKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IChatItem, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IChatItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChatItem {}
impl ::core::fmt::Debug for IChatItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChatItem").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IChatItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{8751d000-ceb1-4243-b803-15d45a1dd428}");
}
unsafe impl ::windows_core::Interface for IChatItem {
    type Vtable = IChatItem_Vtbl;
}
impl ::core::clone::Clone for IChatItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8751d000_ceb1_4243_b803_15d45a1dd428);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ItemKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatItemKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessage {
    type Vtable = IChatMessage_Vtbl;
}
impl ::core::clone::Clone for IChatMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b39052a_1142_5089_76da_f2db3d17cd05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Attachments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Attachments: usize,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsForwardingDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LocalTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocalTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub NetworkTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NetworkTimestamp: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Recipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Recipients: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RecipientSendStatuses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RecipientSendStatuses: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageStatus) -> ::windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TransportFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TransportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTransportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessage2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessage2 {
    type Vtable = IChatMessage2_Vtbl;
}
impl ::core::clone::Clone for IChatMessage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessage2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86668332_543f_49f5_ac71_6c2afc6565fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EstimatedDownloadSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SetEstimatedDownloadSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsAutoReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsAutoReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetIsForwardingDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsReplyDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetIsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsSeen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsSeen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsSimMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetLocalTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLocalTimestamp: usize,
    pub MessageKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageKind) -> ::windows_core::HRESULT,
    pub SetMessageKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatMessageKind) -> ::windows_core::HRESULT,
    pub MessageOperatorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageOperatorKind) -> ::windows_core::HRESULT,
    pub SetMessageOperatorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatMessageOperatorKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetNetworkTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNetworkTimestamp: usize,
    pub IsReceivedDuringQuietHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsReceivedDuringQuietHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatMessageStatus) -> ::windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ShouldSuppressNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShouldSuppressNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ThreadingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetThreadingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RecipientsDeliveryInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RecipientsDeliveryInfos: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessage3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessage3 {
    type Vtable = IChatMessage3_Vtbl;
}
impl ::core::clone::Clone for IChatMessage3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessage3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74eb2fb0_3ba7_459f_8e0b_e8af0febd9ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessage4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessage4 {
    type Vtable = IChatMessage4_Vtbl;
}
impl ::core::clone::Clone for IChatMessage4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessage4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d144b0f_d2bf_460c_aa68_6d3f8483c9bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SyncId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSyncId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageAttachment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageAttachment {
    type Vtable = IChatMessageAttachment_Vtbl;
}
impl ::core::clone::Clone for IChatMessageAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageAttachment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7c4fd74_bf63_58eb_508c_8b863ff16b67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageAttachment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub DataStreamReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DataStreamReference: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetDataStreamReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetDataStreamReference: usize,
    pub GroupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetGroupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub MimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetMimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageAttachment2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageAttachment2 {
    type Vtable = IChatMessageAttachment2_Vtbl;
}
impl ::core::clone::Clone for IChatMessageAttachment2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageAttachment2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ed99270_7dd1_4a87_a8ce_acdd87d80dc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageAttachment2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub TransferProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetTransferProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub OriginalFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetOriginalFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageAttachmentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageAttachmentFactory {
    type Vtable = IChatMessageAttachmentFactory_Vtbl;
}
impl ::core::clone::Clone for IChatMessageAttachmentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageAttachmentFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x205852a2_a356_5b71_6ca9_66c985b7d0d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageAttachmentFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateChatMessageAttachment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mimetype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, datastreamreference: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateChatMessageAttachment: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageBlockingStatic(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageBlockingStatic {
    type Vtable = IChatMessageBlockingStatic_Vtbl;
}
impl ::core::clone::Clone for IChatMessageBlockingStatic {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageBlockingStatic {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6b9a380_cdea_11e4_8830_0800200c9a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageBlockingStatic_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MarkMessageAsBlockedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, blocked: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessageAsBlockedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageChange(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageChange {
    type Vtable = IChatMessageChange_Vtbl;
}
impl ::core::clone::Clone for IChatMessageChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageChange {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c18c355_421e_54b8_6d38_6b3a6c82fccc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChange_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageChangeType) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageChangeReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageChangeReader {
    type Vtable = IChatMessageChangeReader_Vtbl;
}
impl ::core::clone::Clone for IChatMessageChangeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageChangeReader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14267020_28ce_5f26_7b05_9a5c7cce87ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangeReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AcceptChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AcceptChangesThrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastchangetoacknowledge: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageChangeTracker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageChangeTracker {
    type Vtable = IChatMessageChangeTracker_Vtbl;
}
impl ::core::clone::Clone for IChatMessageChangeTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageChangeTracker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x60b7f066_70a0_5224_508c_242ef7c1d06f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangeTracker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageChangedDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageChangedDeferral {
    type Vtable = IChatMessageChangedDeferral_Vtbl;
}
impl ::core::clone::Clone for IChatMessageChangedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageChangedDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbc6b30c_788c_4dcc_ace7_6282382968cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangedDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageChangedEventArgs {
    type Vtable = IChatMessageChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IChatMessageChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6b73e2d_691c_4edf_8660_6eb9896892e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageManager2Statics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageManager2Statics {
    type Vtable = IChatMessageManager2Statics_Vtbl;
}
impl ::core::clone::Clone for IChatMessageManager2Statics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageManager2Statics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d45390f_9f4f_4e35_964e_1b9ca61ac044);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageManager2Statics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RegisterTransportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterTransportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetTransportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTransportAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageManagerStatic(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageManagerStatic {
    type Vtable = IChatMessageManagerStatic_Vtbl;
}
impl ::core::clone::Clone for IChatMessageManagerStatic {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageManagerStatic {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf15c60f7_d5e8_5e92_556d_e03b60253104);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageManagerStatic_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTransportsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTransportsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowComposeSmsMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowComposeSmsMessageAsync: usize,
    pub ShowSmsSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageManagerStatics3 {
    type Vtable = IChatMessageManagerStatics3_Vtbl;
}
impl ::core::clone::Clone for IChatMessageManagerStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageManagerStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x208b830d_6755_48cc_9ab3_fd03c463fc92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestSyncManagerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSyncManagerAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageNotificationTriggerDetails {
    type Vtable = IChatMessageNotificationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IChatMessageNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageNotificationTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd344dfb_3063_4e17_8586_c6c08262e6c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChatMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageNotificationTriggerDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageNotificationTriggerDetails2 {
    type Vtable = IChatMessageNotificationTriggerDetails2_Vtbl;
}
impl ::core::clone::Clone for IChatMessageNotificationTriggerDetails2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageNotificationTriggerDetails2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6bb522e0_aa07_4fd1_9471_77934fb75ee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShouldDisplayToast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ShouldUpdateDetailText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ShouldUpdateBadge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ShouldUpdateActionCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageReader {
    type Vtable = IChatMessageReader_Vtbl;
}
impl ::core::clone::Clone for IChatMessageReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageReader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6ea78ce_4489_56f9_76aa_e204682514cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageReader2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageReader2 {
    type Vtable = IChatMessageReader2_Vtbl;
}
impl ::core::clone::Clone for IChatMessageReader2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageReader2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89643683_64bb_470d_9df4_0de8be1a05bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageReader2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchWithCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchWithCountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageStore {
    type Vtable = IChatMessageStore_Vtbl;
}
impl ::core::clone::Clone for IChatMessageStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x31f2fd01_ccf6_580b_4976_0a07dd5d3b47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStore_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localmessageid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageAsync: usize,
    pub GetMessageReader1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetMessageReader2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recenttimelimit: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageReader2: usize,
    #[cfg(feature = "Foundation")]
    pub MarkMessageReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessageReadAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RetrySendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RetrySendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chatmessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendMessageAsync: usize,
    pub ValidateMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chatmessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MessageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageStore2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageStore2 {
    type Vtable = IChatMessageStore2_Vtbl;
}
impl ::core::clone::Clone for IChatMessageStore2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageStore2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad4dc4ee_3ad4_491b_b311_abdf9bb22768);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStore2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ForwardMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, addresses: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ForwardMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConversationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conversationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConversationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConversationForTransportsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conversationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, transportids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConversationForTransportsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConversationFromThreadingInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadinginfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConversationFromThreadingInfoAsync: usize,
    pub GetConversationReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConversationForTransportsReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConversationForTransportsReader: usize,
    #[cfg(feature = "Foundation")]
    pub GetMessageByRemoteIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, remoteid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageByRemoteIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetUnseenCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetUnseenCountAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnseenCountForTransportsReaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnseenCountForTransportsReaderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkAsSeenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkAsSeenAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MarkAsSeenForTransportsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MarkAsSeenForTransportsAsync: usize,
    pub GetSearchReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chatmessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCancelDownloadMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCancelDownloadMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCancelSendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCancelSendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StoreChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StoreChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStoreChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStoreChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageStore3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageStore3 {
    type Vtable = IChatMessageStore3_Vtbl;
}
impl ::core::clone::Clone for IChatMessageStore3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageStore3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9adbbb09_4345_4ec1_8b74_b7338243719c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStore3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetMessageBySyncIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageBySyncIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageStoreChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageStoreChangedEventArgs {
    type Vtable = IChatMessageStoreChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IChatMessageStoreChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageStoreChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65c66fac_fe8c_46d4_9119_57b8410311d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStoreChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatStoreChangedEventKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageTransport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageTransport {
    type Vtable = IChatMessageTransport_Vtbl;
}
impl ::core::clone::Clone for IChatMessageTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageTransport {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63a9dbf8_e6b3_5c9a_5f85_d47925b9bd18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageTransport_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsAppSetAsNotificationProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TransportFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TransportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestSetAsNotificationProviderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetAsNotificationProviderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageTransport2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageTransport2 {
    type Vtable = IChatMessageTransport2_Vtbl;
}
impl ::core::clone::Clone for IChatMessageTransport2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageTransport2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90a75622_d84a_4c22_a94d_544444edc8a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageTransport2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TransportKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageTransportKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageTransportConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageTransportConfiguration {
    type Vtable = IChatMessageTransportConfiguration_Vtbl;
}
impl ::core::clone::Clone for IChatMessageTransportConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageTransportConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x879ff725_1a08_4aca_a075_3355126312e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageTransportConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MaxAttachmentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaxMessageSizeInKilobytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaxRecipientCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub SupportedVideoFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SupportedVideoFormat: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageValidationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageValidationResult {
    type Vtable = IChatMessageValidationResult_Vtbl;
}
impl ::core::clone::Clone for IChatMessageValidationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatMessageValidationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25e93a03_28ec_5889_569b_7e486b126f18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageValidationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MaxPartCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPartCount: usize,
    #[cfg(feature = "Foundation")]
    pub PartCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PartCount: usize,
    #[cfg(feature = "Foundation")]
    pub RemainingCharacterCountInPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingCharacterCountInPart: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageValidationStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatQueryOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatQueryOptions {
    type Vtable = IChatQueryOptions_Vtbl;
}
impl ::core::clone::Clone for IChatQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatQueryOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2fd364a6_bf36_42f7_b7e7_923c0aabfe16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatQueryOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SearchString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSearchString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatRecipientDeliveryInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatRecipientDeliveryInfo {
    type Vtable = IChatRecipientDeliveryInfo_Vtbl;
}
impl ::core::clone::Clone for IChatRecipientDeliveryInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatRecipientDeliveryInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xffc7b2a2_283c_4c0a_8a0e_8c33bdbf0545);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatRecipientDeliveryInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeliveryTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeliveryTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDeliveryTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDeliveryTime: usize,
    #[cfg(feature = "Foundation")]
    pub ReadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetReadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReadTime: usize,
    pub TransportErrorCodeCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatTransportErrorCodeCategory) -> ::windows_core::HRESULT,
    pub TransportInterpretedErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatTransportInterpretedErrorCode) -> ::windows_core::HRESULT,
    pub TransportErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub IsErrorPermanent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatSearchReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatSearchReader {
    type Vtable = IChatSearchReader_Vtbl;
}
impl ::core::clone::Clone for IChatSearchReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatSearchReader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4665fe49_9020_4752_980d_39612325f589);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatSearchReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchWithCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchWithCountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatSyncConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatSyncConfiguration {
    type Vtable = IChatSyncConfiguration_Vtbl;
}
impl ::core::clone::Clone for IChatSyncConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatSyncConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09f869b2_69f4_4aff_82b6_06992ff402d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatSyncConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RestoreHistorySpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatRestoreHistorySpan) -> ::windows_core::HRESULT,
    pub SetRestoreHistorySpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatRestoreHistorySpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatSyncManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatSyncManager {
    type Vtable = IChatSyncManager_Vtbl;
}
impl ::core::clone::Clone for IChatSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChatSyncManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ba52c63_2650_486f_b4b4_6bd9d3d63c84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatSyncManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub AssociateAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    AssociateAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnassociateAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnassociateAccountAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub IsAccountAssociated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    IsAccountAssociated: usize,
    pub StartSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetConfigurationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRcsEndUserMessage {
    type Vtable = IRcsEndUserMessage_Vtbl;
}
impl ::core::clone::Clone for IRcsEndUserMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRcsEndUserMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7cda5eb_cbd7_4f3b_8526_b506dec35c53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TransportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsPinRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Actions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Actions: usize,
    #[cfg(feature = "Foundation")]
    pub SendResponseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendResponseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendResponseWithPinAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: *mut ::core::ffi::c_void, pin: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendResponseWithPinAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessageAction(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRcsEndUserMessageAction {
    type Vtable = IRcsEndUserMessageAction_Vtbl;
}
impl ::core::clone::Clone for IRcsEndUserMessageAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRcsEndUserMessageAction {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92378737_9b42_46d3_9d5e_3c1b2dae7cb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAction_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessageAvailableEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRcsEndUserMessageAvailableEventArgs {
    type Vtable = IRcsEndUserMessageAvailableEventArgs_Vtbl;
}
impl ::core::clone::Clone for IRcsEndUserMessageAvailableEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRcsEndUserMessageAvailableEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d45ae01_3f89_41ea_9702_9e9ed411aa98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsMessageAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessageAvailableTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRcsEndUserMessageAvailableTriggerDetails {
    type Vtable = IRcsEndUserMessageAvailableTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IRcsEndUserMessageAvailableTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRcsEndUserMessageAvailableTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b97742d_351f_4692_b41e_1b035dc18986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessageManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRcsEndUserMessageManager {
    type Vtable = IRcsEndUserMessageManager_Vtbl;
}
impl ::core::clone::Clone for IRcsEndUserMessageManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRcsEndUserMessageManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3054ae5a_4d1f_4b59_9433_126c734e86a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MessageAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageAvailableChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageAvailableChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRcsManagerStatics {
    type Vtable = IRcsManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IRcsManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRcsManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d270ac5_0abd_4f31_9b99_a59e71a7b731);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetEndUserMessageManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTransportsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTransportsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetTransportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTransportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LeaveConversationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conversation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LeaveConversationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRcsManagerStatics2 {
    type Vtable = IRcsManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IRcsManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRcsManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd49ad18_ad8a_42aa_8eeb_a798a8808959);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TransportListChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransportListChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransportListChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransportListChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsServiceKindSupportedChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRcsServiceKindSupportedChangedEventArgs {
    type Vtable = IRcsServiceKindSupportedChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IRcsServiceKindSupportedChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRcsServiceKindSupportedChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf47ea244_e783_4866_b3a7_4e5ccf023070);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsServiceKindSupportedChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ServiceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RcsServiceKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsTransport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRcsTransport {
    type Vtable = IRcsTransport_Vtbl;
}
impl ::core::clone::Clone for IRcsTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRcsTransport {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfea34759_f37c_4319_8546_ec84d21d30ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsTransport_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TransportFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TransportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsStoreAndForwardEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicekind: RcsServiceKind, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsServiceKindSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicekind: RcsServiceKind, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ServiceKindSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceKindSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServiceKindSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServiceKindSupportedChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsTransportConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRcsTransportConfiguration {
    type Vtable = IRcsTransportConfiguration_Vtbl;
}
impl ::core::clone::Clone for IRcsTransportConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRcsTransportConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1fccb102_2472_4bb9_9988_c1211c83e8a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsTransportConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MaxAttachmentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaxMessageSizeInKilobytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaxGroupMessageSizeInKilobytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaxRecipientCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaxFileSizeInKilobytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub WarningFileSizeInKilobytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteParticipantComposingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteParticipantComposingChangedEventArgs {
    type Vtable = IRemoteParticipantComposingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IRemoteParticipantComposingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRemoteParticipantComposingChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ec045a7_cfc9_45c9_9876_449f2bc180f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteParticipantComposingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TransportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ParticipantAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsComposing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatCapabilities(::windows_core::IUnknown);
impl ChatCapabilities {
    pub fn IsOnline(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsOnline)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsChatCapable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsChatCapable)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsFileTransferCapable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsFileTransferCapable)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsGeoLocationPushCapable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsGeoLocationPushCapable)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsIntegratedMessagingCapable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsIntegratedMessagingCapable)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatCapabilities {}
impl ::core::fmt::Debug for ChatCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatCapabilities").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatCapabilities {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatCapabilities;{3aff77bc-39c9-4dd1-ad2d-3964dd9d403f})");
}
impl ::core::clone::Clone for ChatCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatCapabilities {
    type Vtable = IChatCapabilities_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatCapabilities {
    const IID: ::windows_core::GUID = <IChatCapabilities as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatCapabilities";
}
::windows_core::imp::interface_hierarchy!(ChatCapabilities, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatCapabilities {}
unsafe impl ::core::marker::Sync for ChatCapabilities {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
pub struct ChatCapabilitiesManager;
impl ChatCapabilitiesManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCachedCapabilitiesAsync(address: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<ChatCapabilities>>();
            (::windows_core::Interface::vtable(this).GetCachedCapabilitiesAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(address), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCapabilitiesFromNetworkAsync(address: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<ChatCapabilities>>();
            (::windows_core::Interface::vtable(this).GetCapabilitiesFromNetworkAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(address), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCachedCapabilitiesForTransportAsync(address: &::windows_core::HSTRING, transportid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics2(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<ChatCapabilities>>();
            (::windows_core::Interface::vtable(this).GetCachedCapabilitiesForTransportAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(address), ::core::mem::transmute_copy(transportid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCapabilitiesFromNetworkForTransportAsync(address: &::windows_core::HSTRING, transportid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics2(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<ChatCapabilities>>();
            (::windows_core::Interface::vtable(this).GetCapabilitiesFromNetworkForTransportAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(address), ::core::mem::transmute_copy(transportid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IChatCapabilitiesManagerStatics<R, F: FnOnce(&IChatCapabilitiesManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChatCapabilitiesManager, IChatCapabilitiesManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IChatCapabilitiesManagerStatics2<R, F: FnOnce(&IChatCapabilitiesManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChatCapabilitiesManager, IChatCapabilitiesManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ChatCapabilitiesManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatCapabilitiesManager";
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatConversation(::windows_core::IUnknown);
impl ChatConversation {
    pub fn HasUnreadMessages(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).HasUnreadMessages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Subject(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Subject)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSubject(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubject)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsConversationMuted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsConversationMuted)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsConversationMuted(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsConversationMuted)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MostRecentMessageId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).MostRecentMessageId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Participants(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>();
            (::windows_core::Interface::vtable(this).Participants)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ThreadingInfo(&self) -> ::windows_core::Result<ChatConversationThreadingInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatConversationThreadingInfo>();
            (::windows_core::Interface::vtable(this).ThreadingInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetMessageReader(&self) -> ::windows_core::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageReader>();
            (::windows_core::Interface::vtable(this).GetMessageReader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkAllMessagesAsReadAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).MarkAllMessagesAsReadAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkMessagesAsReadAsync(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).MarkMessagesAsReadAsync)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).SaveAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NotifyLocalParticipantComposing(&self, transportid: &::windows_core::HSTRING, participantaddress: &::windows_core::HSTRING, iscomposing: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyLocalParticipantComposing)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(transportid), ::core::mem::transmute_copy(participantaddress), iscomposing).ok() }
    }
    pub fn NotifyRemoteParticipantComposing(&self, transportid: &::windows_core::HSTRING, participantaddress: &::windows_core::HSTRING, iscomposing: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyRemoteParticipantComposing)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(transportid), ::core::mem::transmute_copy(participantaddress), iscomposing).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoteParticipantComposingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<ChatConversation, RemoteParticipantComposingChangedEventArgs>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows_core::Interface::vtable(this).RemoteParticipantComposingChanged)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoteParticipantComposingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoteParticipantComposingChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CanModifyParticipants(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IChatConversation2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).CanModifyParticipants)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCanModifyParticipants(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatConversation2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanModifyParticipants)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ItemKind(&self) -> ::windows_core::Result<ChatItemKind> {
        let this = &::windows_core::ComInterface::cast::<IChatItem>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatItemKind>();
            (::windows_core::Interface::vtable(this).ItemKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatConversation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatConversation {}
impl ::core::fmt::Debug for ChatConversation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatConversation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatConversation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatConversation;{a58c080d-1a6f-46dc-8f3d-f5028660b6ee})");
}
impl ::core::clone::Clone for ChatConversation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatConversation {
    type Vtable = IChatConversation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatConversation {
    const IID: ::windows_core::GUID = <IChatConversation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatConversation {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversation";
}
::windows_core::imp::interface_hierarchy!(ChatConversation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IChatItem> for ChatConversation {}
unsafe impl ::core::marker::Send for ChatConversation {}
unsafe impl ::core::marker::Sync for ChatConversation {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatConversationReader(::windows_core::IUnknown);
impl ChatConversationReader {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>>();
            (::windows_core::Interface::vtable(this).ReadBatchAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>>();
            (::windows_core::Interface::vtable(this).ReadBatchWithCountAsync)(::windows_core::Interface::as_raw(this), count, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatConversationReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatConversationReader {}
impl ::core::fmt::Debug for ChatConversationReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatConversationReader").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatConversationReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatConversationReader;{055136d2-de32-4a47-a93a-b3dc0833852b})");
}
impl ::core::clone::Clone for ChatConversationReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatConversationReader {
    type Vtable = IChatConversationReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatConversationReader {
    const IID: ::windows_core::GUID = <IChatConversationReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatConversationReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversationReader";
}
::windows_core::imp::interface_hierarchy!(ChatConversationReader, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatConversationReader {}
unsafe impl ::core::marker::Sync for ChatConversationReader {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatConversationThreadingInfo(::windows_core::IUnknown);
impl ChatConversationThreadingInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChatConversationThreadingInfo, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ContactId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).ContactId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContactId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContactId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Custom(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Custom)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCustom(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustom)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ConversationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).ConversationId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetConversationId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetConversationId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Participants(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>();
            (::windows_core::Interface::vtable(this).Participants)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ChatConversationThreadingKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatConversationThreadingKind>();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetKind(&self, value: ChatConversationThreadingKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for ChatConversationThreadingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatConversationThreadingInfo {}
impl ::core::fmt::Debug for ChatConversationThreadingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatConversationThreadingInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatConversationThreadingInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatConversationThreadingInfo;{331c21dc-7a07-4422-a32c-24be7c6dab24})");
}
impl ::core::clone::Clone for ChatConversationThreadingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatConversationThreadingInfo {
    type Vtable = IChatConversationThreadingInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatConversationThreadingInfo {
    const IID: ::windows_core::GUID = <IChatConversationThreadingInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatConversationThreadingInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversationThreadingInfo";
}
::windows_core::imp::interface_hierarchy!(ChatConversationThreadingInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatConversationThreadingInfo {}
unsafe impl ::core::marker::Sync for ChatConversationThreadingInfo {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessage(::windows_core::IUnknown);
impl ChatMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChatMessage, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ItemKind(&self) -> ::windows_core::Result<ChatItemKind> {
        let this = &::windows_core::ComInterface::cast::<IChatItem>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatItemKind>();
            (::windows_core::Interface::vtable(this).ItemKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Attachments(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ChatMessageAttachment>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::Collections::IVector<ChatMessageAttachment>>();
            (::windows_core::Interface::vtable(this).Attachments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Body(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Body)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBody(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBody)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn From(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).From)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsForwardingDisabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsForwardingDisabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsIncoming(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsIncoming)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRead(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsRead)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LocalTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::DateTime>();
            (::windows_core::Interface::vtable(this).LocalTimestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NetworkTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::DateTime>();
            (::windows_core::Interface::vtable(this).NetworkTimestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Recipients(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>();
            (::windows_core::Interface::vtable(this).Recipients)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecipientSendStatuses(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ChatMessageStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ChatMessageStatus>>();
            (::windows_core::Interface::vtable(this).RecipientSendStatuses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<ChatMessageStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageStatus>();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Subject(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Subject)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TransportFriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).TransportFriendlyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TransportId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).TransportId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTransportId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTransportId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn EstimatedDownloadSize(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<u64>();
            (::windows_core::Interface::vtable(this).EstimatedDownloadSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEstimatedDownloadSize(&self, value: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetEstimatedDownloadSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetFrom(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFrom)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsAutoReply(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsAutoReply)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsAutoReply(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsAutoReply)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetIsForwardingDisabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsForwardingDisabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsReplyDisabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsReplyDisabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsIncoming(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsIncoming)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetIsRead(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsRead)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSeen(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsSeen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsSeen(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsSeen)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSimMessage(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsSimMessage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLocalTimestamp(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLocalTimestamp)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MessageKind(&self) -> ::windows_core::Result<ChatMessageKind> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageKind>();
            (::windows_core::Interface::vtable(this).MessageKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMessageKind(&self, value: ChatMessageKind) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMessageKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MessageOperatorKind(&self) -> ::windows_core::Result<ChatMessageOperatorKind> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageOperatorKind>();
            (::windows_core::Interface::vtable(this).MessageOperatorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMessageOperatorKind(&self, value: ChatMessageOperatorKind) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMessageOperatorKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetNetworkTimestamp(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNetworkTimestamp)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsReceivedDuringQuietHours(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsReceivedDuringQuietHours)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsReceivedDuringQuietHours(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsReceivedDuringQuietHours)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetRemoteId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SetStatus(&self, value: ChatMessageStatus) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetSubject(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSubject)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ShouldSuppressNotification(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).ShouldSuppressNotification)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShouldSuppressNotification(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetShouldSuppressNotification)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ThreadingInfo(&self) -> ::windows_core::Result<ChatConversationThreadingInfo> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatConversationThreadingInfo>();
            (::windows_core::Interface::vtable(this).ThreadingInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetThreadingInfo(&self, value: &ChatConversationThreadingInfo) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetThreadingInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecipientsDeliveryInfos(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ChatRecipientDeliveryInfo>> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::Collections::IVector<ChatRecipientDeliveryInfo>>();
            (::windows_core::Interface::vtable(this).RecipientsDeliveryInfos)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage3>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).RemoteId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SyncId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage4>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).SyncId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSyncId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessage4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSyncId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for ChatMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessage {}
impl ::core::fmt::Debug for ChatMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessage").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessage;{4b39052a-1142-5089-76da-f2db3d17cd05})");
}
impl ::core::clone::Clone for ChatMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatMessage {
    type Vtable = IChatMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessage {
    const IID: ::windows_core::GUID = <IChatMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessage";
}
::windows_core::imp::interface_hierarchy!(ChatMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IChatItem> for ChatMessage {}
unsafe impl ::core::marker::Send for ChatMessage {}
unsafe impl ::core::marker::Sync for ChatMessage {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageAttachment(::windows_core::IUnknown);
impl ChatMessageAttachment {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DataStreamReference(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Storage::Streams::IRandomAccessStreamReference>();
            (::windows_core::Interface::vtable(this).DataStreamReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetDataStreamReference<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDataStreamReference)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn GroupId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<u32>();
            (::windows_core::Interface::vtable(this).GroupId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGroupId(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGroupId)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MimeType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).MimeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMimeType(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMimeType)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Storage::Streams::IRandomAccessStreamReference>();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = &::windows_core::ComInterface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnail)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn TransferProgress(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<f64>();
            (::windows_core::Interface::vtable(this).TransferProgress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTransferProgress(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTransferProgress)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OriginalFileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).OriginalFileName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOriginalFileName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOriginalFileName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateChatMessageAttachment<P0>(mimetype: &::windows_core::HSTRING, datastreamreference: P0) -> ::windows_core::Result<ChatMessageAttachment>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::IChatMessageAttachmentFactory(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageAttachment>();
            (::windows_core::Interface::vtable(this).CreateChatMessageAttachment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(mimetype), datastreamreference.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IChatMessageAttachmentFactory<R, F: FnOnce(&IChatMessageAttachmentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChatMessageAttachment, IChatMessageAttachmentFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ChatMessageAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageAttachment {}
impl ::core::fmt::Debug for ChatMessageAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageAttachment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageAttachment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageAttachment;{c7c4fd74-bf63-58eb-508c-8b863ff16b67})");
}
impl ::core::clone::Clone for ChatMessageAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatMessageAttachment {
    type Vtable = IChatMessageAttachment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageAttachment {
    const IID: ::windows_core::GUID = <IChatMessageAttachment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageAttachment {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageAttachment";
}
::windows_core::imp::interface_hierarchy!(ChatMessageAttachment, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageAttachment {}
unsafe impl ::core::marker::Sync for ChatMessageAttachment {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
pub struct ChatMessageBlocking;
impl ChatMessageBlocking {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkMessageAsBlockedAsync(localchatmessageid: &::windows_core::HSTRING, blocked: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IChatMessageBlockingStatic(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).MarkMessageAsBlockedAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), blocked, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IChatMessageBlockingStatic<R, F: FnOnce(&IChatMessageBlockingStatic) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChatMessageBlocking, IChatMessageBlockingStatic> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ChatMessageBlocking {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageBlocking";
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageChange(::windows_core::IUnknown);
impl ChatMessageChange {
    pub fn ChangeType(&self) -> ::windows_core::Result<ChatMessageChangeType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageChangeType>();
            (::windows_core::Interface::vtable(this).ChangeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Message(&self) -> ::windows_core::Result<ChatMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessage>();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatMessageChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChange {}
impl ::core::fmt::Debug for ChatMessageChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChange").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageChange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChange;{1c18c355-421e-54b8-6d38-6b3a6c82fccc})");
}
impl ::core::clone::Clone for ChatMessageChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatMessageChange {
    type Vtable = IChatMessageChange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageChange {
    const IID: ::windows_core::GUID = <IChatMessageChange as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageChange {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChange";
}
::windows_core::imp::interface_hierarchy!(ChatMessageChange, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageChange {}
unsafe impl ::core::marker::Sync for ChatMessageChange {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageChangeReader(::windows_core::IUnknown);
impl ChatMessageChangeReader {
    pub fn AcceptChanges(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AcceptChanges)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AcceptChangesThrough(&self, lastchangetoacknowledge: &ChatMessageChange) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AcceptChangesThrough)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(lastchangetoacknowledge)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageChange>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageChange>>>();
            (::windows_core::Interface::vtable(this).ReadBatchAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatMessageChangeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChangeReader {}
impl ::core::fmt::Debug for ChatMessageChangeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangeReader").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageChangeReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangeReader;{14267020-28ce-5f26-7b05-9a5c7cce87ca})");
}
impl ::core::clone::Clone for ChatMessageChangeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatMessageChangeReader {
    type Vtable = IChatMessageChangeReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageChangeReader {
    const IID: ::windows_core::GUID = <IChatMessageChangeReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangeReader";
}
::windows_core::imp::interface_hierarchy!(ChatMessageChangeReader, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageChangeReader {}
unsafe impl ::core::marker::Sync for ChatMessageChangeReader {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageChangeTracker(::windows_core::IUnknown);
impl ChatMessageChangeTracker {
    pub fn Enable(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Enable)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetChangeReader(&self) -> ::windows_core::Result<ChatMessageChangeReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageChangeReader>();
            (::windows_core::Interface::vtable(this).GetChangeReader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for ChatMessageChangeTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChangeTracker {}
impl ::core::fmt::Debug for ChatMessageChangeTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangeTracker").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageChangeTracker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangeTracker;{60b7f066-70a0-5224-508c-242ef7c1d06f})");
}
impl ::core::clone::Clone for ChatMessageChangeTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatMessageChangeTracker {
    type Vtable = IChatMessageChangeTracker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageChangeTracker {
    const IID: ::windows_core::GUID = <IChatMessageChangeTracker as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangeTracker";
}
::windows_core::imp::interface_hierarchy!(ChatMessageChangeTracker, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageChangeTracker {}
unsafe impl ::core::marker::Sync for ChatMessageChangeTracker {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageChangedDeferral(::windows_core::IUnknown);
impl ChatMessageChangedDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for ChatMessageChangedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChangedDeferral {}
impl ::core::fmt::Debug for ChatMessageChangedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangedDeferral").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageChangedDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangedDeferral;{fbc6b30c-788c-4dcc-ace7-6282382968cf})");
}
impl ::core::clone::Clone for ChatMessageChangedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatMessageChangedDeferral {
    type Vtable = IChatMessageChangedDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageChangedDeferral {
    const IID: ::windows_core::GUID = <IChatMessageChangedDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangedDeferral";
}
::windows_core::imp::interface_hierarchy!(ChatMessageChangedDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageChangedDeferral {}
unsafe impl ::core::marker::Sync for ChatMessageChangedDeferral {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageChangedEventArgs(::windows_core::IUnknown);
impl ChatMessageChangedEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<ChatMessageChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageChangedDeferral>();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatMessageChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChangedEventArgs {}
impl ::core::fmt::Debug for ChatMessageChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs;{b6b73e2d-691c-4edf-8660-6eb9896892e3})");
}
impl ::core::clone::Clone for ChatMessageChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatMessageChangedEventArgs {
    type Vtable = IChatMessageChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageChangedEventArgs {
    const IID: ::windows_core::GUID = <IChatMessageChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ChatMessageChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageChangedEventArgs {}
unsafe impl ::core::marker::Sync for ChatMessageChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
pub struct ChatMessageManager;
impl ChatMessageManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RegisterTransportAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IChatMessageManager2Statics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>();
            (::windows_core::Interface::vtable(this).RegisterTransportAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetTransportAsync(transportid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ChatMessageTransport>> {
        Self::IChatMessageManager2Statics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<ChatMessageTransport>>();
            (::windows_core::Interface::vtable(this).GetTransportAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(transportid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTransportsAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageTransport>>> {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageTransport>>>();
            (::windows_core::Interface::vtable(this).GetTransportsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ChatMessageStore>> {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<ChatMessageStore>>();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowComposeSmsMessageAsync(message: &ChatMessage) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).ShowComposeSmsMessageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(message), &mut result__).from_abi(result__)
        })
    }
    pub fn ShowSmsSettings() -> ::windows_core::Result<()> {
        Self::IChatMessageManagerStatic(|this| unsafe { (::windows_core::Interface::vtable(this).ShowSmsSettings)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSyncManagerAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ChatSyncManager>> {
        Self::IChatMessageManagerStatics3(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<ChatSyncManager>>();
            (::windows_core::Interface::vtable(this).RequestSyncManagerAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IChatMessageManager2Statics<R, F: FnOnce(&IChatMessageManager2Statics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChatMessageManager, IChatMessageManager2Statics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IChatMessageManagerStatic<R, F: FnOnce(&IChatMessageManagerStatic) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChatMessageManager, IChatMessageManagerStatic> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IChatMessageManagerStatics3<R, F: FnOnce(&IChatMessageManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChatMessageManager, IChatMessageManagerStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ChatMessageManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageManager";
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageNotificationTriggerDetails(::windows_core::IUnknown);
impl ChatMessageNotificationTriggerDetails {
    pub fn ChatMessage(&self) -> ::windows_core::Result<ChatMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessage>();
            (::windows_core::Interface::vtable(this).ChatMessage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShouldDisplayToast(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).ShouldDisplayToast)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShouldUpdateDetailText(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).ShouldUpdateDetailText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShouldUpdateBadge(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).ShouldUpdateBadge)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShouldUpdateActionCenter(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).ShouldUpdateActionCenter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatMessageNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageNotificationTriggerDetails {}
impl ::core::fmt::Debug for ChatMessageNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails;{fd344dfb-3063-4e17-8586-c6c08262e6c0})");
}
impl ::core::clone::Clone for ChatMessageNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatMessageNotificationTriggerDetails {
    type Vtable = IChatMessageNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageNotificationTriggerDetails {
    const IID: ::windows_core::GUID = <IChatMessageNotificationTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(ChatMessageNotificationTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for ChatMessageNotificationTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageReader(::windows_core::IUnknown);
impl ChatMessageReader {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>>();
            (::windows_core::Interface::vtable(this).ReadBatchAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageReader2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>>();
            (::windows_core::Interface::vtable(this).ReadBatchWithCountAsync)(::windows_core::Interface::as_raw(this), count, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatMessageReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageReader {}
impl ::core::fmt::Debug for ChatMessageReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageReader").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageReader;{b6ea78ce-4489-56f9-76aa-e204682514cf})");
}
impl ::core::clone::Clone for ChatMessageReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatMessageReader {
    type Vtable = IChatMessageReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageReader {
    const IID: ::windows_core::GUID = <IChatMessageReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageReader";
}
::windows_core::imp::interface_hierarchy!(ChatMessageReader, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageReader {}
unsafe impl ::core::marker::Sync for ChatMessageReader {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageStore(::windows_core::IUnknown);
impl ChatMessageStore {
    pub fn ChangeTracker(&self) -> ::windows_core::Result<ChatMessageChangeTracker> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageChangeTracker>();
            (::windows_core::Interface::vtable(this).ChangeTracker)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteMessageAsync(&self, localmessageid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).DeleteMessageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(localmessageid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DownloadMessageAsync(&self, localchatmessageid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).DownloadMessageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageAsync(&self, localchatmessageid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<ChatMessage>>();
            (::windows_core::Interface::vtable(this).GetMessageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), &mut result__).from_abi(result__)
        }
    }
    pub fn GetMessageReader1(&self) -> ::windows_core::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageReader>();
            (::windows_core::Interface::vtable(this).GetMessageReader1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageReader2(&self, recenttimelimit: super::super::Foundation::TimeSpan) -> ::windows_core::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageReader>();
            (::windows_core::Interface::vtable(this).GetMessageReader2)(::windows_core::Interface::as_raw(this), recenttimelimit, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkMessageReadAsync(&self, localchatmessageid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).MarkMessageReadAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RetrySendMessageAsync(&self, localchatmessageid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).RetrySendMessageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendMessageAsync(&self, chatmessage: &ChatMessage) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).SendMessageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(chatmessage), &mut result__).from_abi(result__)
        }
    }
    pub fn ValidateMessage(&self, chatmessage: &ChatMessage) -> ::windows_core::Result<ChatMessageValidationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageValidationResult>();
            (::windows_core::Interface::vtable(this).ValidateMessage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(chatmessage), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageChanged(&self, value: &super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageChangedEventArgs>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows_core::Interface::vtable(this).MessageChanged)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageChanged(&self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMessageChanged)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ForwardMessageAsync<P0>(&self, localchatmessageid: &::windows_core::HSTRING, addresses: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<ChatMessage>>();
            (::windows_core::Interface::vtable(this).ForwardMessageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), addresses.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetConversationAsync(&self, conversationid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<ChatConversation>>();
            (::windows_core::Interface::vtable(this).GetConversationAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(conversationid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConversationForTransportsAsync<P0>(&self, conversationid: &::windows_core::HSTRING, transportids: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<ChatConversation>>();
            (::windows_core::Interface::vtable(this).GetConversationForTransportsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(conversationid), transportids.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetConversationFromThreadingInfoAsync(&self, threadinginfo: &ChatConversationThreadingInfo) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<ChatConversation>>();
            (::windows_core::Interface::vtable(this).GetConversationFromThreadingInfoAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(threadinginfo), &mut result__).from_abi(result__)
        }
    }
    pub fn GetConversationReader(&self) -> ::windows_core::Result<ChatConversationReader> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatConversationReader>();
            (::windows_core::Interface::vtable(this).GetConversationReader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConversationForTransportsReader<P0>(&self, transportids: P0) -> ::windows_core::Result<ChatConversationReader>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatConversationReader>();
            (::windows_core::Interface::vtable(this).GetConversationForTransportsReader)(::windows_core::Interface::as_raw(this), transportids.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageByRemoteIdAsync(&self, transportid: &::windows_core::HSTRING, remoteid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<ChatMessage>>();
            (::windows_core::Interface::vtable(this).GetMessageByRemoteIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(transportid), ::core::mem::transmute_copy(remoteid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetUnseenCountAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<i32>> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<i32>>();
            (::windows_core::Interface::vtable(this).GetUnseenCountAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUnseenCountForTransportsReaderAsync<P0>(&self, transportids: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<i32>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<i32>>();
            (::windows_core::Interface::vtable(this).GetUnseenCountForTransportsReaderAsync)(::windows_core::Interface::as_raw(this), transportids.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkAsSeenAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).MarkAsSeenAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MarkAsSeenForTransportsAsync<P0>(&self, transportids: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).MarkAsSeenForTransportsAsync)(::windows_core::Interface::as_raw(this), transportids.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetSearchReader(&self, value: &ChatQueryOptions) -> ::windows_core::Result<ChatSearchReader> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatSearchReader>();
            (::windows_core::Interface::vtable(this).GetSearchReader)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveMessageAsync(&self, chatmessage: &ChatMessage) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).SaveMessageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(chatmessage), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCancelDownloadMessageAsync(&self, localchatmessageid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows_core::Interface::vtable(this).TryCancelDownloadMessageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCancelSendMessageAsync(&self, localchatmessageid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows_core::Interface::vtable(this).TryCancelSendMessageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StoreChanged(&self, handler: &super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageStoreChangedEventArgs>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows_core::Interface::vtable(this).StoreChanged)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStoreChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStoreChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageBySyncIdAsync(&self, syncid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageStore3>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<ChatMessage>>();
            (::windows_core::Interface::vtable(this).GetMessageBySyncIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(syncid), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatMessageStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageStore {}
impl ::core::fmt::Debug for ChatMessageStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageStore").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageStore {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageStore;{31f2fd01-ccf6-580b-4976-0a07dd5d3b47})");
}
impl ::core::clone::Clone for ChatMessageStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatMessageStore {
    type Vtable = IChatMessageStore_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageStore {
    const IID: ::windows_core::GUID = <IChatMessageStore as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageStore {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageStore";
}
::windows_core::imp::interface_hierarchy!(ChatMessageStore, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageStore {}
unsafe impl ::core::marker::Sync for ChatMessageStore {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageStoreChangedEventArgs(::windows_core::IUnknown);
impl ChatMessageStoreChangedEventArgs {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ChatStoreChangedEventKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatStoreChangedEventKind>();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatMessageStoreChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageStoreChangedEventArgs {}
impl ::core::fmt::Debug for ChatMessageStoreChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageStoreChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageStoreChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs;{65c66fac-fe8c-46d4-9119-57b8410311d5})");
}
impl ::core::clone::Clone for ChatMessageStoreChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatMessageStoreChangedEventArgs {
    type Vtable = IChatMessageStoreChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageStoreChangedEventArgs {
    const IID: ::windows_core::GUID = <IChatMessageStoreChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ChatMessageStoreChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageStoreChangedEventArgs {}
unsafe impl ::core::marker::Sync for ChatMessageStoreChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageTransport(::windows_core::IUnknown);
impl ChatMessageTransport {
    pub fn IsAppSetAsNotificationProvider(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsAppSetAsNotificationProvider)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsActive)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TransportFriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).TransportFriendlyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TransportId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).TransportId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSetAsNotificationProviderAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).RequestSetAsNotificationProviderAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Configuration(&self) -> ::windows_core::Result<ChatMessageTransportConfiguration> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageTransport2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageTransportConfiguration>();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TransportKind(&self) -> ::windows_core::Result<ChatMessageTransportKind> {
        let this = &::windows_core::ComInterface::cast::<IChatMessageTransport2>(self)?;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageTransportKind>();
            (::windows_core::Interface::vtable(this).TransportKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatMessageTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageTransport {}
impl ::core::fmt::Debug for ChatMessageTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageTransport").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageTransport {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageTransport;{63a9dbf8-e6b3-5c9a-5f85-d47925b9bd18})");
}
impl ::core::clone::Clone for ChatMessageTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatMessageTransport {
    type Vtable = IChatMessageTransport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageTransport {
    const IID: ::windows_core::GUID = <IChatMessageTransport as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageTransport {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageTransport";
}
::windows_core::imp::interface_hierarchy!(ChatMessageTransport, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageTransport {}
unsafe impl ::core::marker::Sync for ChatMessageTransport {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageTransportConfiguration(::windows_core::IUnknown);
impl ChatMessageTransportConfiguration {
    pub fn MaxAttachmentCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<i32>();
            (::windows_core::Interface::vtable(this).MaxAttachmentCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxMessageSizeInKilobytes(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<i32>();
            (::windows_core::Interface::vtable(this).MaxMessageSizeInKilobytes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxRecipientCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<i32>();
            (::windows_core::Interface::vtable(this).MaxRecipientCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SupportedVideoFormat(&self) -> ::windows_core::Result<super::super::Media::MediaProperties::MediaEncodingProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Media::MediaProperties::MediaEncodingProfile>();
            (::windows_core::Interface::vtable(this).SupportedVideoFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatMessageTransportConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageTransportConfiguration {}
impl ::core::fmt::Debug for ChatMessageTransportConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageTransportConfiguration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageTransportConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration;{879ff725-1a08-4aca-a075-3355126312e6})");
}
impl ::core::clone::Clone for ChatMessageTransportConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatMessageTransportConfiguration {
    type Vtable = IChatMessageTransportConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageTransportConfiguration {
    const IID: ::windows_core::GUID = <IChatMessageTransportConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageTransportConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration";
}
::windows_core::imp::interface_hierarchy!(ChatMessageTransportConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageTransportConfiguration {}
unsafe impl ::core::marker::Sync for ChatMessageTransportConfiguration {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageValidationResult(::windows_core::IUnknown);
impl ChatMessageValidationResult {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxPartCount(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IReference<u32>>();
            (::windows_core::Interface::vtable(this).MaxPartCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PartCount(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IReference<u32>>();
            (::windows_core::Interface::vtable(this).PartCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingCharacterCountInPart(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IReference<u32>>();
            (::windows_core::Interface::vtable(this).RemainingCharacterCountInPart)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<ChatMessageValidationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageValidationStatus>();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatMessageValidationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageValidationResult {}
impl ::core::fmt::Debug for ChatMessageValidationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageValidationResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageValidationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageValidationResult;{25e93a03-28ec-5889-569b-7e486b126f18})");
}
impl ::core::clone::Clone for ChatMessageValidationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatMessageValidationResult {
    type Vtable = IChatMessageValidationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageValidationResult {
    const IID: ::windows_core::GUID = <IChatMessageValidationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageValidationResult {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageValidationResult";
}
::windows_core::imp::interface_hierarchy!(ChatMessageValidationResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageValidationResult {}
unsafe impl ::core::marker::Sync for ChatMessageValidationResult {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatQueryOptions(::windows_core::IUnknown);
impl ChatQueryOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChatQueryOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SearchString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).SearchString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSearchString(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSearchString)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for ChatQueryOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatQueryOptions {}
impl ::core::fmt::Debug for ChatQueryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatQueryOptions").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatQueryOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatQueryOptions;{2fd364a6-bf36-42f7-b7e7-923c0aabfe16})");
}
impl ::core::clone::Clone for ChatQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatQueryOptions {
    type Vtable = IChatQueryOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatQueryOptions {
    const IID: ::windows_core::GUID = <IChatQueryOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatQueryOptions";
}
::windows_core::imp::interface_hierarchy!(ChatQueryOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatQueryOptions {}
unsafe impl ::core::marker::Sync for ChatQueryOptions {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatRecipientDeliveryInfo(::windows_core::IUnknown);
impl ChatRecipientDeliveryInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChatRecipientDeliveryInfo, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TransportAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).TransportAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTransportAddress(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTransportAddress)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeliveryTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>();
            (::windows_core::Interface::vtable(this).DeliveryTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDeliveryTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDeliveryTime)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>();
            (::windows_core::Interface::vtable(this).ReadTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetReadTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReadTime)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn TransportErrorCodeCategory(&self) -> ::windows_core::Result<ChatTransportErrorCodeCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatTransportErrorCodeCategory>();
            (::windows_core::Interface::vtable(this).TransportErrorCodeCategory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TransportInterpretedErrorCode(&self) -> ::windows_core::Result<ChatTransportInterpretedErrorCode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatTransportInterpretedErrorCode>();
            (::windows_core::Interface::vtable(this).TransportInterpretedErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TransportErrorCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<i32>();
            (::windows_core::Interface::vtable(this).TransportErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsErrorPermanent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsErrorPermanent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<ChatMessageStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatMessageStatus>();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatRecipientDeliveryInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatRecipientDeliveryInfo {}
impl ::core::fmt::Debug for ChatRecipientDeliveryInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatRecipientDeliveryInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatRecipientDeliveryInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo;{ffc7b2a2-283c-4c0a-8a0e-8c33bdbf0545})");
}
impl ::core::clone::Clone for ChatRecipientDeliveryInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatRecipientDeliveryInfo {
    type Vtable = IChatRecipientDeliveryInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatRecipientDeliveryInfo {
    const IID: ::windows_core::GUID = <IChatRecipientDeliveryInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatRecipientDeliveryInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo";
}
::windows_core::imp::interface_hierarchy!(ChatRecipientDeliveryInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatRecipientDeliveryInfo {}
unsafe impl ::core::marker::Sync for ChatRecipientDeliveryInfo {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatSearchReader(::windows_core::IUnknown);
impl ChatSearchReader {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>>();
            (::windows_core::Interface::vtable(this).ReadBatchAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>>();
            (::windows_core::Interface::vtable(this).ReadBatchWithCountAsync)(::windows_core::Interface::as_raw(this), count, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatSearchReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatSearchReader {}
impl ::core::fmt::Debug for ChatSearchReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatSearchReader").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatSearchReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatSearchReader;{4665fe49-9020-4752-980d-39612325f589})");
}
impl ::core::clone::Clone for ChatSearchReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatSearchReader {
    type Vtable = IChatSearchReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatSearchReader {
    const IID: ::windows_core::GUID = <IChatSearchReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatSearchReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSearchReader";
}
::windows_core::imp::interface_hierarchy!(ChatSearchReader, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatSearchReader {}
unsafe impl ::core::marker::Sync for ChatSearchReader {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatSyncConfiguration(::windows_core::IUnknown);
impl ChatSyncConfiguration {
    pub fn IsSyncEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsSyncEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsSyncEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsSyncEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RestoreHistorySpan(&self) -> ::windows_core::Result<ChatRestoreHistorySpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatRestoreHistorySpan>();
            (::windows_core::Interface::vtable(this).RestoreHistorySpan)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRestoreHistorySpan(&self, value: ChatRestoreHistorySpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRestoreHistorySpan)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for ChatSyncConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatSyncConfiguration {}
impl ::core::fmt::Debug for ChatSyncConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatSyncConfiguration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatSyncConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatSyncConfiguration;{09f869b2-69f4-4aff-82b6-06992ff402d2})");
}
impl ::core::clone::Clone for ChatSyncConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatSyncConfiguration {
    type Vtable = IChatSyncConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatSyncConfiguration {
    const IID: ::windows_core::GUID = <IChatSyncConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatSyncConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSyncConfiguration";
}
::windows_core::imp::interface_hierarchy!(ChatSyncConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatSyncConfiguration {}
unsafe impl ::core::marker::Sync for ChatSyncConfiguration {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatSyncManager(::windows_core::IUnknown);
impl ChatSyncManager {
    pub fn Configuration(&self) -> ::windows_core::Result<ChatSyncConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<ChatSyncConfiguration>();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn AssociateAccountAsync(&self, webaccount: &super::super::Security::Credentials::WebAccount) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).AssociateAccountAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccount), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnassociateAccountAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).UnassociateAccountAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn IsAccountAssociated(&self, webaccount: &super::super::Security::Credentials::WebAccount) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsAccountAssociated)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccount), &mut result__).from_abi(result__)
        }
    }
    pub fn StartSync(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartSync)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetConfigurationAsync(&self, configuration: &ChatSyncConfiguration) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).SetConfigurationAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(configuration), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChatSyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatSyncManager {}
impl ::core::fmt::Debug for ChatSyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatSyncManager").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatSyncManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatSyncManager;{7ba52c63-2650-486f-b4b4-6bd9d3d63c84})");
}
impl ::core::clone::Clone for ChatSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChatSyncManager {
    type Vtable = IChatSyncManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatSyncManager {
    const IID: ::windows_core::GUID = <IChatSyncManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSyncManager";
}
::windows_core::imp::interface_hierarchy!(ChatSyncManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ChatSyncManager {}
unsafe impl ::core::marker::Sync for ChatSyncManager {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsEndUserMessage(::windows_core::IUnknown);
impl RcsEndUserMessage {
    pub fn TransportId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).TransportId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPinRequired(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsPinRequired)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Actions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<RcsEndUserMessageAction>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::Collections::IVectorView<RcsEndUserMessageAction>>();
            (::windows_core::Interface::vtable(this).Actions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendResponseAsync(&self, action: &RcsEndUserMessageAction) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).SendResponseAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(action), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendResponseWithPinAsync(&self, action: &RcsEndUserMessageAction, pin: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).SendResponseWithPinAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(action), ::core::mem::transmute_copy(pin), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessage {}
impl ::core::fmt::Debug for RcsEndUserMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessage").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RcsEndUserMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessage;{d7cda5eb-cbd7-4f3b-8526-b506dec35c53})");
}
impl ::core::clone::Clone for RcsEndUserMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RcsEndUserMessage {
    type Vtable = IRcsEndUserMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RcsEndUserMessage {
    const IID: ::windows_core::GUID = <IRcsEndUserMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RcsEndUserMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessage";
}
::windows_core::imp::interface_hierarchy!(RcsEndUserMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RcsEndUserMessage {}
unsafe impl ::core::marker::Sync for RcsEndUserMessage {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsEndUserMessageAction(::windows_core::IUnknown);
impl RcsEndUserMessageAction {
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessageAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessageAction {}
impl ::core::fmt::Debug for RcsEndUserMessageAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessageAction").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RcsEndUserMessageAction {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageAction;{92378737-9b42-46d3-9d5e-3c1b2dae7cb8})");
}
impl ::core::clone::Clone for RcsEndUserMessageAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RcsEndUserMessageAction {
    type Vtable = IRcsEndUserMessageAction_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RcsEndUserMessageAction {
    const IID: ::windows_core::GUID = <IRcsEndUserMessageAction as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RcsEndUserMessageAction {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAction";
}
::windows_core::imp::interface_hierarchy!(RcsEndUserMessageAction, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RcsEndUserMessageAction {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageAction {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsEndUserMessageAvailableEventArgs(::windows_core::IUnknown);
impl RcsEndUserMessageAvailableEventArgs {
    pub fn IsMessageAvailable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsMessageAvailable)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Message(&self) -> ::windows_core::Result<RcsEndUserMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<RcsEndUserMessage>();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessageAvailableEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessageAvailableEventArgs {}
impl ::core::fmt::Debug for RcsEndUserMessageAvailableEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessageAvailableEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RcsEndUserMessageAvailableEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs;{2d45ae01-3f89-41ea-9702-9e9ed411aa98})");
}
impl ::core::clone::Clone for RcsEndUserMessageAvailableEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RcsEndUserMessageAvailableEventArgs {
    type Vtable = IRcsEndUserMessageAvailableEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RcsEndUserMessageAvailableEventArgs {
    const IID: ::windows_core::GUID = <IRcsEndUserMessageAvailableEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RcsEndUserMessageAvailableEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs";
}
::windows_core::imp::interface_hierarchy!(RcsEndUserMessageAvailableEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RcsEndUserMessageAvailableEventArgs {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageAvailableEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsEndUserMessageAvailableTriggerDetails(::windows_core::IUnknown);
impl RcsEndUserMessageAvailableTriggerDetails {
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessageAvailableTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessageAvailableTriggerDetails {}
impl ::core::fmt::Debug for RcsEndUserMessageAvailableTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessageAvailableTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RcsEndUserMessageAvailableTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails;{5b97742d-351f-4692-b41e-1b035dc18986})");
}
impl ::core::clone::Clone for RcsEndUserMessageAvailableTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RcsEndUserMessageAvailableTriggerDetails {
    type Vtable = IRcsEndUserMessageAvailableTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RcsEndUserMessageAvailableTriggerDetails {
    const IID: ::windows_core::GUID = <IRcsEndUserMessageAvailableTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RcsEndUserMessageAvailableTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(RcsEndUserMessageAvailableTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RcsEndUserMessageAvailableTriggerDetails {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageAvailableTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsEndUserMessageManager(::windows_core::IUnknown);
impl RcsEndUserMessageManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageAvailableChanged(&self, handler: &super::super::Foundation::TypedEventHandler<RcsEndUserMessageManager, RcsEndUserMessageAvailableEventArgs>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows_core::Interface::vtable(this).MessageAvailableChanged)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageAvailableChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMessageAvailableChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessageManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessageManager {}
impl ::core::fmt::Debug for RcsEndUserMessageManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessageManager").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RcsEndUserMessageManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageManager;{3054ae5a-4d1f-4b59-9433-126c734e86a6})");
}
impl ::core::clone::Clone for RcsEndUserMessageManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RcsEndUserMessageManager {
    type Vtable = IRcsEndUserMessageManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RcsEndUserMessageManager {
    const IID: ::windows_core::GUID = <IRcsEndUserMessageManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RcsEndUserMessageManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageManager";
}
::windows_core::imp::interface_hierarchy!(RcsEndUserMessageManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RcsEndUserMessageManager {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageManager {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
pub struct RcsManager;
impl RcsManager {
    pub fn GetEndUserMessageManager() -> ::windows_core::Result<RcsEndUserMessageManager> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<RcsEndUserMessageManager>();
            (::windows_core::Interface::vtable(this).GetEndUserMessageManager)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTransportsAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<RcsTransport>>> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<RcsTransport>>>();
            (::windows_core::Interface::vtable(this).GetTransportsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetTransportAsync(transportid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<RcsTransport>> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncOperation<RcsTransport>>();
            (::windows_core::Interface::vtable(this).GetTransportAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(transportid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LeaveConversationAsync(conversation: &ChatConversation) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows_core::Interface::vtable(this).LeaveConversationAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(conversation), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TransportListChanged(handler: &super::super::Foundation::EventHandler<::windows_core::IInspectable>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IRcsManagerStatics2(|this| unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows_core::Interface::vtable(this).TransportListChanged)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTransportListChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IRcsManagerStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveTransportListChanged)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IRcsManagerStatics<R, F: FnOnce(&IRcsManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RcsManager, IRcsManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRcsManagerStatics2<R, F: FnOnce(&IRcsManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RcsManager, IRcsManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for RcsManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsManager";
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsServiceKindSupportedChangedEventArgs(::windows_core::IUnknown);
impl RcsServiceKindSupportedChangedEventArgs {
    pub fn ServiceKind(&self) -> ::windows_core::Result<RcsServiceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<RcsServiceKind>();
            (::windows_core::Interface::vtable(this).ServiceKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for RcsServiceKindSupportedChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsServiceKindSupportedChangedEventArgs {}
impl ::core::fmt::Debug for RcsServiceKindSupportedChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsServiceKindSupportedChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RcsServiceKindSupportedChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs;{f47ea244-e783-4866-b3a7-4e5ccf023070})");
}
impl ::core::clone::Clone for RcsServiceKindSupportedChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RcsServiceKindSupportedChangedEventArgs {
    type Vtable = IRcsServiceKindSupportedChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RcsServiceKindSupportedChangedEventArgs {
    const IID: ::windows_core::GUID = <IRcsServiceKindSupportedChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RcsServiceKindSupportedChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(RcsServiceKindSupportedChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RcsServiceKindSupportedChangedEventArgs {}
unsafe impl ::core::marker::Sync for RcsServiceKindSupportedChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsTransport(::windows_core::IUnknown);
impl RcsTransport {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsActive)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TransportFriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).TransportFriendlyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TransportId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).TransportId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Configuration(&self) -> ::windows_core::Result<RcsTransportConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<RcsTransportConfiguration>();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsStoreAndForwardEnabled(&self, servicekind: RcsServiceKind) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsStoreAndForwardEnabled)(::windows_core::Interface::as_raw(this), servicekind, &mut result__).from_abi(result__)
        }
    }
    pub fn IsServiceKindSupported(&self, servicekind: RcsServiceKind) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsServiceKindSupported)(::windows_core::Interface::as_raw(this), servicekind, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceKindSupportedChanged(&self, handler: &super::super::Foundation::TypedEventHandler<RcsTransport, RcsServiceKindSupportedChangedEventArgs>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows_core::Interface::vtable(this).ServiceKindSupportedChanged)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServiceKindSupportedChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveServiceKindSupportedChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for RcsTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsTransport {}
impl ::core::fmt::Debug for RcsTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsTransport").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RcsTransport {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsTransport;{fea34759-f37c-4319-8546-ec84d21d30ff})");
}
impl ::core::clone::Clone for RcsTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RcsTransport {
    type Vtable = IRcsTransport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RcsTransport {
    const IID: ::windows_core::GUID = <IRcsTransport as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RcsTransport {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsTransport";
}
::windows_core::imp::interface_hierarchy!(RcsTransport, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RcsTransport {}
unsafe impl ::core::marker::Sync for RcsTransport {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsTransportConfiguration(::windows_core::IUnknown);
impl RcsTransportConfiguration {
    pub fn MaxAttachmentCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<i32>();
            (::windows_core::Interface::vtable(this).MaxAttachmentCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxMessageSizeInKilobytes(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<i32>();
            (::windows_core::Interface::vtable(this).MaxMessageSizeInKilobytes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxGroupMessageSizeInKilobytes(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<i32>();
            (::windows_core::Interface::vtable(this).MaxGroupMessageSizeInKilobytes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxRecipientCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<i32>();
            (::windows_core::Interface::vtable(this).MaxRecipientCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxFileSizeInKilobytes(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<i32>();
            (::windows_core::Interface::vtable(this).MaxFileSizeInKilobytes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn WarningFileSizeInKilobytes(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<i32>();
            (::windows_core::Interface::vtable(this).WarningFileSizeInKilobytes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for RcsTransportConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsTransportConfiguration {}
impl ::core::fmt::Debug for RcsTransportConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsTransportConfiguration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RcsTransportConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsTransportConfiguration;{1fccb102-2472-4bb9-9988-c1211c83e8a9})");
}
impl ::core::clone::Clone for RcsTransportConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RcsTransportConfiguration {
    type Vtable = IRcsTransportConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RcsTransportConfiguration {
    const IID: ::windows_core::GUID = <IRcsTransportConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RcsTransportConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsTransportConfiguration";
}
::windows_core::imp::interface_hierarchy!(RcsTransportConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RcsTransportConfiguration {}
unsafe impl ::core::marker::Sync for RcsTransportConfiguration {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RemoteParticipantComposingChangedEventArgs(::windows_core::IUnknown);
impl RemoteParticipantComposingChangedEventArgs {
    pub fn TransportId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).TransportId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParticipantAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<::windows_core::HSTRING>();
            (::windows_core::Interface::vtable(this).ParticipantAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsComposing(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows_core::zeroed::<bool>();
            (::windows_core::Interface::vtable(this).IsComposing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for RemoteParticipantComposingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteParticipantComposingChangedEventArgs {}
impl ::core::fmt::Debug for RemoteParticipantComposingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteParticipantComposingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RemoteParticipantComposingChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs;{1ec045a7-cfc9-45c9-9876-449f2bc180f5})");
}
impl ::core::clone::Clone for RemoteParticipantComposingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RemoteParticipantComposingChangedEventArgs {
    type Vtable = IRemoteParticipantComposingChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RemoteParticipantComposingChangedEventArgs {
    const IID: ::windows_core::GUID = <IRemoteParticipantComposingChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RemoteParticipantComposingChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(RemoteParticipantComposingChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RemoteParticipantComposingChangedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteParticipantComposingChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatConversationThreadingKind(pub i32);
impl ChatConversationThreadingKind {
    pub const Participants: Self = Self(0i32);
    pub const ContactId: Self = Self(1i32);
    pub const ConversationId: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatConversationThreadingKind {}
impl ::core::clone::Clone for ChatConversationThreadingKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatConversationThreadingKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ChatConversationThreadingKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ChatConversationThreadingKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatConversationThreadingKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatConversationThreadingKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatConversationThreadingKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatItemKind(pub i32);
impl ChatItemKind {
    pub const Message: Self = Self(0i32);
    pub const Conversation: Self = Self(1i32);
}
impl ::core::marker::Copy for ChatItemKind {}
impl ::core::clone::Clone for ChatItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatItemKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ChatItemKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ChatItemKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatItemKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatItemKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatItemKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatMessageChangeType(pub i32);
impl ChatMessageChangeType {
    pub const MessageCreated: Self = Self(0i32);
    pub const MessageModified: Self = Self(1i32);
    pub const MessageDeleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatMessageChangeType {}
impl ::core::clone::Clone for ChatMessageChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatMessageChangeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ChatMessageChangeType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ChatMessageChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangeType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageChangeType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageChangeType;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatMessageKind(pub i32);
impl ChatMessageKind {
    pub const Standard: Self = Self(0i32);
    pub const FileTransferRequest: Self = Self(1i32);
    pub const TransportCustom: Self = Self(2i32);
    pub const JoinedConversation: Self = Self(3i32);
    pub const LeftConversation: Self = Self(4i32);
    pub const OtherParticipantJoinedConversation: Self = Self(5i32);
    pub const OtherParticipantLeftConversation: Self = Self(6i32);
}
impl ::core::marker::Copy for ChatMessageKind {}
impl ::core::clone::Clone for ChatMessageKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatMessageKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ChatMessageKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ChatMessageKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatMessageOperatorKind(pub i32);
impl ChatMessageOperatorKind {
    pub const Unspecified: Self = Self(0i32);
    pub const Sms: Self = Self(1i32);
    pub const Mms: Self = Self(2i32);
    pub const Rcs: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatMessageOperatorKind {}
impl ::core::clone::Clone for ChatMessageOperatorKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatMessageOperatorKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ChatMessageOperatorKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ChatMessageOperatorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageOperatorKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageOperatorKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageOperatorKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatMessageStatus(pub i32);
impl ChatMessageStatus {
    pub const Draft: Self = Self(0i32);
    pub const Sending: Self = Self(1i32);
    pub const Sent: Self = Self(2i32);
    pub const SendRetryNeeded: Self = Self(3i32);
    pub const SendFailed: Self = Self(4i32);
    pub const Received: Self = Self(5i32);
    pub const ReceiveDownloadNeeded: Self = Self(6i32);
    pub const ReceiveDownloadFailed: Self = Self(7i32);
    pub const ReceiveDownloading: Self = Self(8i32);
    pub const Deleted: Self = Self(9i32);
    pub const Declined: Self = Self(10i32);
    pub const Cancelled: Self = Self(11i32);
    pub const Recalled: Self = Self(12i32);
    pub const ReceiveRetryNeeded: Self = Self(13i32);
}
impl ::core::marker::Copy for ChatMessageStatus {}
impl ::core::clone::Clone for ChatMessageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatMessageStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ChatMessageStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ChatMessageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatMessageTransportKind(pub i32);
impl ChatMessageTransportKind {
    pub const Text: Self = Self(0i32);
    pub const Untriaged: Self = Self(1i32);
    pub const Blocked: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatMessageTransportKind {}
impl ::core::clone::Clone for ChatMessageTransportKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatMessageTransportKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ChatMessageTransportKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ChatMessageTransportKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageTransportKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageTransportKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageTransportKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatMessageValidationStatus(pub i32);
impl ChatMessageValidationStatus {
    pub const Valid: Self = Self(0i32);
    pub const NoRecipients: Self = Self(1i32);
    pub const InvalidData: Self = Self(2i32);
    pub const MessageTooLarge: Self = Self(3i32);
    pub const TooManyRecipients: Self = Self(4i32);
    pub const TransportInactive: Self = Self(5i32);
    pub const TransportNotFound: Self = Self(6i32);
    pub const TooManyAttachments: Self = Self(7i32);
    pub const InvalidRecipients: Self = Self(8i32);
    pub const InvalidBody: Self = Self(9i32);
    pub const InvalidOther: Self = Self(10i32);
    pub const ValidWithLargeMessage: Self = Self(11i32);
    pub const VoiceRoamingRestriction: Self = Self(12i32);
    pub const DataRoamingRestriction: Self = Self(13i32);
}
impl ::core::marker::Copy for ChatMessageValidationStatus {}
impl ::core::clone::Clone for ChatMessageValidationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatMessageValidationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ChatMessageValidationStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ChatMessageValidationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageValidationStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatMessageValidationStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageValidationStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatRestoreHistorySpan(pub i32);
impl ChatRestoreHistorySpan {
    pub const LastMonth: Self = Self(0i32);
    pub const LastYear: Self = Self(1i32);
    pub const AnyTime: Self = Self(2i32);
}
impl ::core::marker::Copy for ChatRestoreHistorySpan {}
impl ::core::clone::Clone for ChatRestoreHistorySpan {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatRestoreHistorySpan {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ChatRestoreHistorySpan {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ChatRestoreHistorySpan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatRestoreHistorySpan").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatRestoreHistorySpan {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatRestoreHistorySpan;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatStoreChangedEventKind(pub i32);
impl ChatStoreChangedEventKind {
    pub const NotificationsMissed: Self = Self(0i32);
    pub const StoreModified: Self = Self(1i32);
    pub const MessageCreated: Self = Self(2i32);
    pub const MessageModified: Self = Self(3i32);
    pub const MessageDeleted: Self = Self(4i32);
    pub const ConversationModified: Self = Self(5i32);
    pub const ConversationDeleted: Self = Self(6i32);
    pub const ConversationTransportDeleted: Self = Self(7i32);
}
impl ::core::marker::Copy for ChatStoreChangedEventKind {}
impl ::core::clone::Clone for ChatStoreChangedEventKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatStoreChangedEventKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ChatStoreChangedEventKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ChatStoreChangedEventKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatStoreChangedEventKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatStoreChangedEventKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatStoreChangedEventKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatTransportErrorCodeCategory(pub i32);
impl ChatTransportErrorCodeCategory {
    pub const None: Self = Self(0i32);
    pub const Http: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
    pub const MmsServer: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatTransportErrorCodeCategory {}
impl ::core::clone::Clone for ChatTransportErrorCodeCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatTransportErrorCodeCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ChatTransportErrorCodeCategory {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ChatTransportErrorCodeCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatTransportErrorCodeCategory").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatTransportErrorCodeCategory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatTransportErrorCodeCategory;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatTransportInterpretedErrorCode(pub i32);
impl ChatTransportInterpretedErrorCode {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const InvalidRecipientAddress: Self = Self(2i32);
    pub const NetworkConnectivity: Self = Self(3i32);
    pub const ServiceDenied: Self = Self(4i32);
    pub const Timeout: Self = Self(5i32);
}
impl ::core::marker::Copy for ChatTransportInterpretedErrorCode {}
impl ::core::clone::Clone for ChatTransportInterpretedErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatTransportInterpretedErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ChatTransportInterpretedErrorCode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ChatTransportInterpretedErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatTransportInterpretedErrorCode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChatTransportInterpretedErrorCode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatTransportInterpretedErrorCode;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RcsServiceKind(pub i32);
impl RcsServiceKind {
    pub const Chat: Self = Self(0i32);
    pub const GroupChat: Self = Self(1i32);
    pub const FileTransfer: Self = Self(2i32);
    pub const Capability: Self = Self(3i32);
}
impl ::core::marker::Copy for RcsServiceKind {}
impl ::core::clone::Clone for RcsServiceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RcsServiceKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RcsServiceKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RcsServiceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsServiceKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RcsServiceKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.RcsServiceKind;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
