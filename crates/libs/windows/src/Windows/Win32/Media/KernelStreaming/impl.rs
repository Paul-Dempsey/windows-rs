#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"implement\"`*"]
pub trait IKsAggregateControl_Impl: Sized {
    fn KsAddAggregate(&self, aggregateclass: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn KsRemoveAggregate(&self, aggregateclass: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IKsAggregateControl {}
impl IKsAggregateControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAggregateControl_Impl, const OFFSET: isize>() -> IKsAggregateControl_Vtbl {
        unsafe extern "system" fn KsAddAggregate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAggregateControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aggregateclass: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.KsAddAggregate(::core::mem::transmute_copy(&aggregateclass)).into()
        }
        unsafe extern "system" fn KsRemoveAggregate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAggregateControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aggregateclass: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.KsRemoveAggregate(::core::mem::transmute_copy(&aggregateclass)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsAddAggregate: KsAddAggregate::<Identity, Impl, OFFSET>,
            KsRemoveAggregate: KsRemoveAggregate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IKsAggregateControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"implement\"`*"]
pub trait IKsControl_Impl: Sized {
    fn KsProperty(&self, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows_core::Result<()>;
    fn KsMethod(&self, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows_core::Result<()>;
    fn KsEvent(&self, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IKsControl {}
impl IKsControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: isize>() -> IKsControl_Vtbl {
        unsafe extern "system" fn KsProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.KsProperty(::core::mem::transmute_copy(&property), ::core::mem::transmute_copy(&propertylength), ::core::mem::transmute_copy(&propertydata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn KsMethod<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.KsMethod(::core::mem::transmute_copy(&method), ::core::mem::transmute_copy(&methodlength), ::core::mem::transmute_copy(&methoddata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn KsEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.KsEvent(::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&eventlength), ::core::mem::transmute_copy(&eventdata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsProperty: KsProperty::<Identity, Impl, OFFSET>,
            KsMethod: KsMethod::<Identity, Impl, OFFSET>,
            KsEvent: KsEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IKsControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IKsFormatSupport_Impl: Sized {
    fn IsFormatSupported(&self, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetDevicePreferredFormat(&self) -> ::windows_core::Result<*mut KSDATAFORMAT>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IKsFormatSupport {}
#[cfg(feature = "Win32_Foundation")]
impl IKsFormatSupport_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsFormatSupport_Impl, const OFFSET: isize>() -> IKsFormatSupport_Vtbl {
        unsafe extern "system" fn IsFormatSupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsFormatSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsFormatSupported(::core::mem::transmute_copy(&pksformat), ::core::mem::transmute_copy(&cbformat), ::core::mem::transmute_copy(&pbsupported)).into()
        }
        unsafe extern "system" fn GetDevicePreferredFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsFormatSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppksformat: *mut *mut KSDATAFORMAT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevicePreferredFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppksformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsFormatSupported: IsFormatSupported::<Identity, Impl, OFFSET>,
            GetDevicePreferredFormat: GetDevicePreferredFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IKsFormatSupport as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"implement\"`*"]
pub trait IKsJackContainerId_Impl: Sized {
    fn GetJackContainerId(&self) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::RuntimeName for IKsJackContainerId {}
impl IKsJackContainerId_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackContainerId_Impl, const OFFSET: isize>() -> IKsJackContainerId_Vtbl {
        unsafe extern "system" fn GetJackContainerId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackContainerId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjackcontainerid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJackContainerId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pjackcontainerid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetJackContainerId: GetJackContainerId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IKsJackContainerId as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IKsJackDescription_Impl: Sized {
    fn GetJackCount(&self) -> ::windows_core::Result<u32>;
    fn GetJackDescription(&self, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IKsJackDescription {}
#[cfg(feature = "Win32_Foundation")]
impl IKsJackDescription_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription_Impl, const OFFSET: isize>() -> IKsJackDescription_Vtbl {
        unsafe extern "system" fn GetJackCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJackCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcjacks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetJackDescription(::core::mem::transmute_copy(&njack), ::core::mem::transmute_copy(&pdescription)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetJackCount: GetJackCount::<Identity, Impl, OFFSET>,
            GetJackDescription: GetJackDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IKsJackDescription as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"implement\"`*"]
pub trait IKsJackDescription2_Impl: Sized {
    fn GetJackCount(&self) -> ::windows_core::Result<u32>;
    fn GetJackDescription2(&self, njack: u32) -> ::windows_core::Result<KSJACK_DESCRIPTION2>;
}
impl ::windows_core::RuntimeName for IKsJackDescription2 {}
impl IKsJackDescription2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription2_Impl, const OFFSET: isize>() -> IKsJackDescription2_Vtbl {
        unsafe extern "system" fn GetJackCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJackCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcjacks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, njack: u32, pdescription2: *mut KSJACK_DESCRIPTION2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJackDescription2(::core::mem::transmute_copy(&njack)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdescription2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetJackCount: GetJackCount::<Identity, Impl, OFFSET>,
            GetJackDescription2: GetJackDescription2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IKsJackDescription2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IKsJackSinkInformation_Impl: Sized {
    fn GetJackSinkInformation(&self, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IKsJackSinkInformation {}
#[cfg(feature = "Win32_Foundation")]
impl IKsJackSinkInformation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackSinkInformation_Impl, const OFFSET: isize>() -> IKsJackSinkInformation_Vtbl {
        unsafe extern "system" fn GetJackSinkInformation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackSinkInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetJackSinkInformation(::core::mem::transmute_copy(&pjacksinkinformation)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetJackSinkInformation: GetJackSinkInformation::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IKsJackSinkInformation as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"implement\"`*"]
pub trait IKsPropertySet_Impl: Sized {
    fn Set(&self, propset: *const ::windows_core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *const ::core::ffi::c_void, datalength: u32) -> ::windows_core::Result<()>;
    fn Get(&self, propset: *const ::windows_core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows_core::Result<()>;
    fn QuerySupported(&self, propset: *const ::windows_core::GUID, id: u32) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for IKsPropertySet {}
impl IKsPropertySet_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: isize>() -> IKsPropertySet_Vtbl {
        unsafe extern "system" fn Set<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: *const ::windows_core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *const ::core::ffi::c_void, datalength: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set(::core::mem::transmute_copy(&propset), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&instancedata), ::core::mem::transmute_copy(&instancelength), ::core::mem::transmute_copy(&propertydata), ::core::mem::transmute_copy(&datalength)).into()
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: *const ::windows_core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Get(::core::mem::transmute_copy(&propset), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&instancedata), ::core::mem::transmute_copy(&instancelength), ::core::mem::transmute_copy(&propertydata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn QuerySupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: *const ::windows_core::GUID, id: u32, typesupport: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuerySupported(::core::mem::transmute_copy(&propset), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(typesupport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Set: Set::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            QuerySupported: QuerySupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IKsPropertySet as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"implement\"`*"]
pub trait IKsTopology_Impl: Sized {
    fn CreateNodeInstance(&self, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: ::core::option::Option<&::windows_core::IUnknown>, interfaceid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IKsTopology {}
impl IKsTopology_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsTopology_Impl, const OFFSET: isize>() -> IKsTopology_Vtbl {
        unsafe extern "system" fn CreateNodeInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: *mut ::core::ffi::c_void, interfaceid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateNodeInstance(::core::mem::transmute_copy(&nodeid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&desiredaccess), ::windows_core::from_raw_borrowed(&unkouter), ::core::mem::transmute_copy(&interfaceid), ::core::mem::transmute_copy(&interface)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateNodeInstance: CreateNodeInstance::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IKsTopology as ::windows_core::ComInterface>::IID
    }
}
