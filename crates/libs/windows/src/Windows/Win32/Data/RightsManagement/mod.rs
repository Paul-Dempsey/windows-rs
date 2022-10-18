#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAcquireAdvisories(hlicensestorage: u32, wszlicense: ::windows::core::PWSTR, wszurl: ::windows::core::PWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMAcquireAdvisories(hlicensestorage: u32, wszlicense: ::windows::core::PWSTR, wszurl: ::windows::core::PWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DRMAcquireAdvisories(hlicensestorage, ::core::mem::transmute(wszlicense), ::core::mem::transmute(wszurl), ::core::mem::transmute(pvcontext)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAcquireIssuanceLicenseTemplate(hclient: u32, uflags: u32, pvreserved: *mut ::core::ffi::c_void, pwsztemplateids: ::core::option::Option<&[::windows::core::PWSTR]>, wszurl: ::windows::core::PWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMAcquireIssuanceLicenseTemplate(hclient: u32, uflags: u32, pvreserved: *mut ::core::ffi::c_void, ctemplates: u32, pwsztemplateids: *mut ::windows::core::PWSTR, wszurl: ::windows::core::PWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DRMAcquireIssuanceLicenseTemplate(hclient, uflags, ::core::mem::transmute(pvreserved), pwsztemplateids.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pwsztemplateids.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(wszurl), ::core::mem::transmute(pvcontext)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAcquireLicense(hsession: u32, uflags: u32, wszgroupidentitycredential: ::windows::core::PWSTR, wszrequestedrights: ::windows::core::PWSTR, wszcustomdata: ::windows::core::PWSTR, wszurl: ::windows::core::PWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMAcquireLicense(hsession: u32, uflags: u32, wszgroupidentitycredential: ::windows::core::PWSTR, wszrequestedrights: ::windows::core::PWSTR, wszcustomdata: ::windows::core::PWSTR, wszurl: ::windows::core::PWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DRMAcquireLicense(hsession, uflags, ::core::mem::transmute(wszgroupidentitycredential), ::core::mem::transmute(wszrequestedrights), ::core::mem::transmute(wszcustomdata), ::core::mem::transmute(wszurl), ::core::mem::transmute(pvcontext)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMActivate<'a, P0>(hclient: u32, uflags: u32, ulangid: u32, pactservinfo: *mut DRM_ACTSERV_INFO, pvcontext: *mut ::core::ffi::c_void, hparentwnd: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMActivate(hclient: u32, uflags: u32, ulangid: u32, pactservinfo: *mut DRM_ACTSERV_INFO, pvcontext: *mut ::core::ffi::c_void, hparentwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT;
    }
    DRMActivate(hclient, uflags, ulangid, ::core::mem::transmute(pactservinfo), ::core::mem::transmute(pvcontext), hparentwnd.into()).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAddLicense(hlicensestorage: u32, uflags: u32, wszlicense: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMAddLicense(hlicensestorage: u32, uflags: u32, wszlicense: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMAddLicense(hlicensestorage, uflags, ::core::mem::transmute(wszlicense)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAddRightWithUser(hissuancelicense: u32, hright: u32, huser: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMAddRightWithUser(hissuancelicense: u32, hright: u32, huser: u32) -> ::windows::core::HRESULT;
    }
    DRMAddRightWithUser(hissuancelicense, hright, huser).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAttest(henablingprincipal: u32, wszdata: ::windows::core::PWSTR, etype: DRMATTESTTYPE, pcattestedblob: *mut u32, wszattestedblob: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMAttest(henablingprincipal: u32, wszdata: ::windows::core::PWSTR, etype: DRMATTESTTYPE, pcattestedblob: *mut u32, wszattestedblob: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMAttest(henablingprincipal, ::core::mem::transmute(wszdata), etype, ::core::mem::transmute(pcattestedblob), ::core::mem::transmute(wszattestedblob)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCheckSecurity(henv: u32, clevel: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCheckSecurity(henv: u32, clevel: u32) -> ::windows::core::HRESULT;
    }
    DRMCheckSecurity(henv, clevel).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMClearAllRights(hissuancelicense: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMClearAllRights(hissuancelicense: u32) -> ::windows::core::HRESULT;
    }
    DRMClearAllRights(hissuancelicense).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCloseEnvironmentHandle(henv: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCloseEnvironmentHandle(henv: u32) -> ::windows::core::HRESULT;
    }
    DRMCloseEnvironmentHandle(henv).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCloseHandle(handle: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCloseHandle(handle: u32) -> ::windows::core::HRESULT;
    }
    DRMCloseHandle(handle).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMClosePubHandle(hpub: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMClosePubHandle(hpub: u32) -> ::windows::core::HRESULT;
    }
    DRMClosePubHandle(hpub).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCloseQueryHandle(hquery: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCloseQueryHandle(hquery: u32) -> ::windows::core::HRESULT;
    }
    DRMCloseQueryHandle(hquery).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCloseSession(hsession: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCloseSession(hsession: u32) -> ::windows::core::HRESULT;
    }
    DRMCloseSession(hsession).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMConstructCertificateChain(rgwszcertificates: &[::windows::core::PWSTR], pcchain: *mut u32, wszchain: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMConstructCertificateChain(ccertificates: u32, rgwszcertificates: *mut ::windows::core::PWSTR, pcchain: *mut u32, wszchain: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMConstructCertificateChain(rgwszcertificates.len() as _, ::core::mem::transmute(rgwszcertificates.as_ptr()), ::core::mem::transmute(pcchain), ::core::mem::transmute(wszchain)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateBoundLicense(henv: u32, pparams: *mut DRMBOUNDLICENSEPARAMS, wszlicensechain: ::windows::core::PWSTR, phboundlicense: *mut u32, pherrorlog: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateBoundLicense(henv: u32, pparams: *mut DRMBOUNDLICENSEPARAMS, wszlicensechain: ::windows::core::PWSTR, phboundlicense: *mut u32, pherrorlog: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateBoundLicense(henv, ::core::mem::transmute(pparams), ::core::mem::transmute(wszlicensechain), ::core::mem::transmute(phboundlicense), ::core::mem::transmute(pherrorlog)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateClientSession(pfncallback: DRMCALLBACK, ucallbackversion: u32, wszgroupidprovidertype: ::windows::core::PWSTR, wszgroupid: ::windows::core::PWSTR, phclient: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateClientSession(pfncallback: *mut ::core::ffi::c_void, ucallbackversion: u32, wszgroupidprovidertype: ::windows::core::PWSTR, wszgroupid: ::windows::core::PWSTR, phclient: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateClientSession(::core::mem::transmute(pfncallback), ucallbackversion, ::core::mem::transmute(wszgroupidprovidertype), ::core::mem::transmute(wszgroupid), ::core::mem::transmute(phclient)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateEnablingBitsDecryptor(hboundlicense: u32, wszright: ::windows::core::PWSTR, hauxlib: u32, wszauxplug: ::windows::core::PWSTR, phdecryptor: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateEnablingBitsDecryptor(hboundlicense: u32, wszright: ::windows::core::PWSTR, hauxlib: u32, wszauxplug: ::windows::core::PWSTR, phdecryptor: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateEnablingBitsDecryptor(hboundlicense, ::core::mem::transmute(wszright), hauxlib, ::core::mem::transmute(wszauxplug), ::core::mem::transmute(phdecryptor)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateEnablingBitsEncryptor(hboundlicense: u32, wszright: ::windows::core::PWSTR, hauxlib: u32, wszauxplug: ::windows::core::PWSTR, phencryptor: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateEnablingBitsEncryptor(hboundlicense: u32, wszright: ::windows::core::PWSTR, hauxlib: u32, wszauxplug: ::windows::core::PWSTR, phencryptor: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateEnablingBitsEncryptor(hboundlicense, ::core::mem::transmute(wszright), hauxlib, ::core::mem::transmute(wszauxplug), ::core::mem::transmute(phencryptor)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateEnablingPrincipal(henv: u32, hlibrary: u32, wszobject: ::windows::core::PWSTR, pidprincipal: *mut DRMID, wszcredentials: ::windows::core::PWSTR, phenablingprincipal: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateEnablingPrincipal(henv: u32, hlibrary: u32, wszobject: ::windows::core::PWSTR, pidprincipal: *mut DRMID, wszcredentials: ::windows::core::PWSTR, phenablingprincipal: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateEnablingPrincipal(henv, hlibrary, ::core::mem::transmute(wszobject), ::core::mem::transmute(pidprincipal), ::core::mem::transmute(wszcredentials), ::core::mem::transmute(phenablingprincipal)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMCreateIssuanceLicense(psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, wszreferralinfoname: ::windows::core::PWSTR, wszreferralinfourl: ::windows::core::PWSTR, howner: u32, wszissuancelicense: ::windows::core::PWSTR, hboundlicense: u32, phissuancelicense: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateIssuanceLicense(psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, wszreferralinfoname: ::windows::core::PWSTR, wszreferralinfourl: ::windows::core::PWSTR, howner: u32, wszissuancelicense: ::windows::core::PWSTR, hboundlicense: u32, phissuancelicense: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateIssuanceLicense(::core::mem::transmute(psttimefrom), ::core::mem::transmute(psttimeuntil), ::core::mem::transmute(wszreferralinfoname), ::core::mem::transmute(wszreferralinfourl), howner, ::core::mem::transmute(wszissuancelicense), hboundlicense, ::core::mem::transmute(phissuancelicense)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateLicenseStorageSession(henv: u32, hdefaultlibrary: u32, hclient: u32, uflags: u32, wszissuancelicense: ::windows::core::PWSTR, phlicensestorage: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateLicenseStorageSession(henv: u32, hdefaultlibrary: u32, hclient: u32, uflags: u32, wszissuancelicense: ::windows::core::PWSTR, phlicensestorage: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateLicenseStorageSession(henv, hdefaultlibrary, hclient, uflags, ::core::mem::transmute(wszissuancelicense), ::core::mem::transmute(phlicensestorage)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMCreateRight(wszrightname: ::windows::core::PWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME, cextendedinfo: u32, pwszextendedinfoname: ::core::option::Option<*mut ::windows::core::PWSTR>, pwszextendedinfovalue: ::core::option::Option<*mut ::windows::core::PWSTR>, phright: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateRight(wszrightname: ::windows::core::PWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME, cextendedinfo: u32, pwszextendedinfoname: *mut ::windows::core::PWSTR, pwszextendedinfovalue: *mut ::windows::core::PWSTR, phright: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateRight(::core::mem::transmute(wszrightname), ::core::mem::transmute(pstfrom), ::core::mem::transmute(pstuntil), cextendedinfo, ::core::mem::transmute(pwszextendedinfoname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pwszextendedinfovalue.unwrap_or(::std::ptr::null())), ::core::mem::transmute(phright)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateUser(wszusername: ::windows::core::PWSTR, wszuserid: ::windows::core::PWSTR, wszuseridtype: ::windows::core::PWSTR, phuser: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateUser(wszusername: ::windows::core::PWSTR, wszuserid: ::windows::core::PWSTR, wszuseridtype: ::windows::core::PWSTR, phuser: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateUser(::core::mem::transmute(wszusername), ::core::mem::transmute(wszuserid), ::core::mem::transmute(wszuseridtype), ::core::mem::transmute(phuser)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDecode(wszalgid: ::windows::core::PWSTR, wszencodedstring: ::windows::core::PWSTR, pudecodeddatalen: *mut u32, pbdecodeddata: *mut u8) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDecode(wszalgid: ::windows::core::PWSTR, wszencodedstring: ::windows::core::PWSTR, pudecodeddatalen: *mut u32, pbdecodeddata: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMDecode(::core::mem::transmute(wszalgid), ::core::mem::transmute(wszencodedstring), ::core::mem::transmute(pudecodeddatalen), ::core::mem::transmute(pbdecodeddata)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDeconstructCertificateChain(wszchain: ::windows::core::PWSTR, iwhich: u32, pccert: *mut u32, wszcert: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDeconstructCertificateChain(wszchain: ::windows::core::PWSTR, iwhich: u32, pccert: *mut u32, wszcert: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMDeconstructCertificateChain(::core::mem::transmute(wszchain), iwhich, ::core::mem::transmute(pccert), ::core::mem::transmute(wszcert)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDecrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDecrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMDecrypt(hcryptoprovider, iposition, cnuminbytes, ::core::mem::transmute(pbindata), ::core::mem::transmute(pcnumoutbytes), ::core::mem::transmute(pboutdata)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDeleteLicense(hsession: u32, wszlicenseid: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDeleteLicense(hsession: u32, wszlicenseid: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMDeleteLicense(hsession, ::core::mem::transmute(wszlicenseid)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDuplicateEnvironmentHandle(htocopy: u32, phcopy: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDuplicateEnvironmentHandle(htocopy: u32, phcopy: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMDuplicateEnvironmentHandle(htocopy, ::core::mem::transmute(phcopy)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDuplicateHandle(htocopy: u32, phcopy: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDuplicateHandle(htocopy: u32, phcopy: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMDuplicateHandle(htocopy, ::core::mem::transmute(phcopy)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDuplicatePubHandle(hpubin: u32, phpubout: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDuplicatePubHandle(hpubin: u32, phpubout: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMDuplicatePubHandle(hpubin, ::core::mem::transmute(phpubout)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDuplicateSession(hsessionin: u32, phsessionout: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDuplicateSession(hsessionin: u32, phsessionout: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMDuplicateSession(hsessionin, ::core::mem::transmute(phsessionout)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMEncode(wszalgid: ::windows::core::PWSTR, udatalen: u32, pbdecodeddata: *mut u8, puencodedstringlen: *mut u32, wszencodedstring: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMEncode(wszalgid: ::windows::core::PWSTR, udatalen: u32, pbdecodeddata: *mut u8, puencodedstringlen: *mut u32, wszencodedstring: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMEncode(::core::mem::transmute(wszalgid), udatalen, ::core::mem::transmute(pbdecodeddata), ::core::mem::transmute(puencodedstringlen), ::core::mem::transmute(wszencodedstring)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMEncrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMEncrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMEncrypt(hcryptoprovider, iposition, cnuminbytes, ::core::mem::transmute(pbindata), ::core::mem::transmute(pcnumoutbytes), ::core::mem::transmute(pboutdata)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMEnumerateLicense(hsession: u32, uflags: u32, uindex: u32, pfsharedflag: *mut super::super::Foundation::BOOL, pucertificatedatalen: *mut u32, wszcertificatedata: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMEnumerateLicense(hsession: u32, uflags: u32, uindex: u32, pfsharedflag: *mut super::super::Foundation::BOOL, pucertificatedatalen: *mut u32, wszcertificatedata: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMEnumerateLicense(hsession, uflags, uindex, ::core::mem::transmute(pfsharedflag), ::core::mem::transmute(pucertificatedatalen), ::core::mem::transmute(wszcertificatedata)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetApplicationSpecificData(hissuancelicense: u32, uindex: u32, punamelength: *mut u32, wszname: ::windows::core::PWSTR, puvaluelength: *mut u32, wszvalue: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetApplicationSpecificData(hissuancelicense: u32, uindex: u32, punamelength: *mut u32, wszname: ::windows::core::PWSTR, puvaluelength: *mut u32, wszvalue: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetApplicationSpecificData(hissuancelicense, uindex, ::core::mem::transmute(punamelength), ::core::mem::transmute(wszname), ::core::mem::transmute(puvaluelength), ::core::mem::transmute(wszvalue)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetBoundLicenseAttribute(hqueryroot: u32, wszattribute: ::windows::core::PWSTR, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetBoundLicenseAttribute(hqueryroot: u32, wszattribute: ::windows::core::PWSTR, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMGetBoundLicenseAttribute(hqueryroot, ::core::mem::transmute(wszattribute), iwhich, ::core::mem::transmute(peencoding), ::core::mem::transmute(pcbuffer), ::core::mem::transmute(pbbuffer)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetBoundLicenseAttributeCount(hqueryroot: u32, wszattribute: ::windows::core::PWSTR, pcattributes: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetBoundLicenseAttributeCount(hqueryroot: u32, wszattribute: ::windows::core::PWSTR, pcattributes: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetBoundLicenseAttributeCount(hqueryroot, ::core::mem::transmute(wszattribute), ::core::mem::transmute(pcattributes)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetBoundLicenseObject(hqueryroot: u32, wszsubobjecttype: ::windows::core::PWSTR, iwhich: u32, phsubobject: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetBoundLicenseObject(hqueryroot: u32, wszsubobjecttype: ::windows::core::PWSTR, iwhich: u32, phsubobject: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetBoundLicenseObject(hqueryroot, ::core::mem::transmute(wszsubobjecttype), iwhich, ::core::mem::transmute(phsubobject)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetBoundLicenseObjectCount(hqueryroot: u32, wszsubobjecttype: ::windows::core::PWSTR, pcsubobjects: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetBoundLicenseObjectCount(hqueryroot: u32, wszsubobjecttype: ::windows::core::PWSTR, pcsubobjects: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetBoundLicenseObjectCount(hqueryroot, ::core::mem::transmute(wszsubobjecttype), ::core::mem::transmute(pcsubobjects)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetCertificateChainCount(wszchain: ::windows::core::PWSTR, pccertcount: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetCertificateChainCount(wszchain: ::windows::core::PWSTR, pccertcount: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetCertificateChainCount(::core::mem::transmute(wszchain), ::core::mem::transmute(pccertcount)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetClientVersion(pdrmclientversioninfo: *mut DRM_CLIENT_VERSION_INFO) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetClientVersion(pdrmclientversioninfo: *mut DRM_CLIENT_VERSION_INFO) -> ::windows::core::HRESULT;
    }
    DRMGetClientVersion(::core::mem::transmute(pdrmclientversioninfo)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetEnvironmentInfo(handle: u32, wszattribute: ::windows::core::PWSTR, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetEnvironmentInfo(handle: u32, wszattribute: ::windows::core::PWSTR, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMGetEnvironmentInfo(handle, ::core::mem::transmute(wszattribute), ::core::mem::transmute(peencoding), ::core::mem::transmute(pcbuffer), ::core::mem::transmute(pbbuffer)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetInfo(handle: u32, wszattribute: ::windows::core::PWSTR, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetInfo(handle: u32, wszattribute: ::windows::core::PWSTR, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMGetInfo(handle, ::core::mem::transmute(wszattribute), ::core::mem::transmute(peencoding), ::core::mem::transmute(pcbuffer), ::core::mem::transmute(pbbuffer)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetIntervalTime(hissuancelicense: u32, pcdays: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetIntervalTime(hissuancelicense: u32, pcdays: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetIntervalTime(hissuancelicense, ::core::mem::transmute(pcdays)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetIssuanceLicenseInfo(hissuancelicense: u32, psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, uflags: u32, pudistributionpointnamelength: *mut u32, wszdistributionpointname: ::windows::core::PWSTR, pudistributionpointurllength: *mut u32, wszdistributionpointurl: ::windows::core::PWSTR, phowner: *mut u32, pfofficial: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetIssuanceLicenseInfo(hissuancelicense: u32, psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, uflags: u32, pudistributionpointnamelength: *mut u32, wszdistributionpointname: ::windows::core::PWSTR, pudistributionpointurllength: *mut u32, wszdistributionpointurl: ::windows::core::PWSTR, phowner: *mut u32, pfofficial: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    DRMGetIssuanceLicenseInfo(hissuancelicense, ::core::mem::transmute(psttimefrom), ::core::mem::transmute(psttimeuntil), uflags, ::core::mem::transmute(pudistributionpointnamelength), ::core::mem::transmute(wszdistributionpointname), ::core::mem::transmute(pudistributionpointurllength), ::core::mem::transmute(wszdistributionpointurl), ::core::mem::transmute(phowner), ::core::mem::transmute(pfofficial)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetIssuanceLicenseTemplate(hissuancelicense: u32, puissuancelicensetemplatelength: *mut u32, wszissuancelicensetemplate: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetIssuanceLicenseTemplate(hissuancelicense: u32, puissuancelicensetemplatelength: *mut u32, wszissuancelicensetemplate: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetIssuanceLicenseTemplate(hissuancelicense, ::core::mem::transmute(puissuancelicensetemplatelength), ::core::mem::transmute(wszissuancelicensetemplate)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetMetaData(hissuancelicense: u32, pucontentidlength: *mut u32, wszcontentid: ::windows::core::PWSTR, pucontentidtypelength: *mut u32, wszcontentidtype: ::windows::core::PWSTR, puskuidlength: *mut u32, wszskuid: ::windows::core::PWSTR, puskuidtypelength: *mut u32, wszskuidtype: ::windows::core::PWSTR, pucontenttypelength: *mut u32, wszcontenttype: ::windows::core::PWSTR, pucontentnamelength: *mut u32, wszcontentname: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetMetaData(hissuancelicense: u32, pucontentidlength: *mut u32, wszcontentid: ::windows::core::PWSTR, pucontentidtypelength: *mut u32, wszcontentidtype: ::windows::core::PWSTR, puskuidlength: *mut u32, wszskuid: ::windows::core::PWSTR, puskuidtypelength: *mut u32, wszskuidtype: ::windows::core::PWSTR, pucontenttypelength: *mut u32, wszcontenttype: ::windows::core::PWSTR, pucontentnamelength: *mut u32, wszcontentname: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetMetaData(
        hissuancelicense,
        ::core::mem::transmute(pucontentidlength),
        ::core::mem::transmute(wszcontentid),
        ::core::mem::transmute(pucontentidtypelength),
        ::core::mem::transmute(wszcontentidtype),
        ::core::mem::transmute(puskuidlength),
        ::core::mem::transmute(wszskuid),
        ::core::mem::transmute(puskuidtypelength),
        ::core::mem::transmute(wszskuidtype),
        ::core::mem::transmute(pucontenttypelength),
        ::core::mem::transmute(wszcontenttype),
        ::core::mem::transmute(pucontentnamelength),
        ::core::mem::transmute(wszcontentname),
    )
    .ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetNameAndDescription(hissuancelicense: u32, uindex: u32, pulcid: *mut u32, punamelength: *mut u32, wszname: ::windows::core::PWSTR, pudescriptionlength: *mut u32, wszdescription: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetNameAndDescription(hissuancelicense: u32, uindex: u32, pulcid: *mut u32, punamelength: *mut u32, wszname: ::windows::core::PWSTR, pudescriptionlength: *mut u32, wszdescription: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetNameAndDescription(hissuancelicense, uindex, ::core::mem::transmute(pulcid), ::core::mem::transmute(punamelength), ::core::mem::transmute(wszname), ::core::mem::transmute(pudescriptionlength), ::core::mem::transmute(wszdescription)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetOwnerLicense(hissuancelicense: u32, puownerlicenselength: *mut u32, wszownerlicense: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetOwnerLicense(hissuancelicense: u32, puownerlicenselength: *mut u32, wszownerlicense: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetOwnerLicense(hissuancelicense, ::core::mem::transmute(puownerlicenselength), ::core::mem::transmute(wszownerlicense)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetProcAddress(hlibrary: u32, wszprocname: ::windows::core::PWSTR, ppfnprocaddress: *mut super::super::Foundation::FARPROC) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetProcAddress(hlibrary: u32, wszprocname: ::windows::core::PWSTR, ppfnprocaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DRMGetProcAddress(hlibrary, ::core::mem::transmute(wszprocname), ::core::mem::transmute(ppfnprocaddress)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetRevocationPoint(hissuancelicense: u32, puidlength: *mut u32, wszid: ::windows::core::PWSTR, puidtypelength: *mut u32, wszidtype: ::windows::core::PWSTR, puurllength: *mut u32, wszrl: ::windows::core::PWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, punamelength: *mut u32, wszname: ::windows::core::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetRevocationPoint(hissuancelicense: u32, puidlength: *mut u32, wszid: ::windows::core::PWSTR, puidtypelength: *mut u32, wszidtype: ::windows::core::PWSTR, puurllength: *mut u32, wszrl: ::windows::core::PWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, punamelength: *mut u32, wszname: ::windows::core::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetRevocationPoint(hissuancelicense, ::core::mem::transmute(puidlength), ::core::mem::transmute(wszid), ::core::mem::transmute(puidtypelength), ::core::mem::transmute(wszidtype), ::core::mem::transmute(puurllength), ::core::mem::transmute(wszrl), ::core::mem::transmute(pstfrequency), ::core::mem::transmute(punamelength), ::core::mem::transmute(wszname), ::core::mem::transmute(pupublickeylength), ::core::mem::transmute(wszpublickey)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetRightExtendedInfo(hright: u32, uindex: u32, puextendedinfonamelength: *mut u32, wszextendedinfoname: ::windows::core::PWSTR, puextendedinfovaluelength: *mut u32, wszextendedinfovalue: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetRightExtendedInfo(hright: u32, uindex: u32, puextendedinfonamelength: *mut u32, wszextendedinfoname: ::windows::core::PWSTR, puextendedinfovaluelength: *mut u32, wszextendedinfovalue: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetRightExtendedInfo(hright, uindex, ::core::mem::transmute(puextendedinfonamelength), ::core::mem::transmute(wszextendedinfoname), ::core::mem::transmute(puextendedinfovaluelength), ::core::mem::transmute(wszextendedinfovalue)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetRightInfo(hright: u32, purightnamelength: *mut u32, wszrightname: ::windows::core::PWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetRightInfo(hright: u32, purightnamelength: *mut u32, wszrightname: ::windows::core::PWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT;
    }
    DRMGetRightInfo(hright, ::core::mem::transmute(purightnamelength), ::core::mem::transmute(wszrightname), ::core::mem::transmute(pstfrom), ::core::mem::transmute(pstuntil)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetSecurityProvider(uflags: u32, putypelen: *mut u32, wsztype: ::windows::core::PWSTR, pupathlen: *mut u32, wszpath: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetSecurityProvider(uflags: u32, putypelen: *mut u32, wsztype: ::windows::core::PWSTR, pupathlen: *mut u32, wszpath: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetSecurityProvider(uflags, ::core::mem::transmute(putypelen), ::core::mem::transmute(wsztype), ::core::mem::transmute(pupathlen), ::core::mem::transmute(wszpath)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetServiceLocation(hclient: u32, uservicetype: u32, uservicelocation: u32, wszissuancelicense: ::windows::core::PWSTR, puserviceurllength: *mut u32, wszserviceurl: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetServiceLocation(hclient: u32, uservicetype: u32, uservicelocation: u32, wszissuancelicense: ::windows::core::PWSTR, puserviceurllength: *mut u32, wszserviceurl: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetServiceLocation(hclient, uservicetype, uservicelocation, ::core::mem::transmute(wszissuancelicense), ::core::mem::transmute(puserviceurllength), ::core::mem::transmute(wszserviceurl)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetSignedIssuanceLicense(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *mut u8, cbsymkey: u32, wszsymkeytype: ::windows::core::PWSTR, wszclientlicensorcertificate: ::windows::core::PWSTR, pfncallback: DRMCALLBACK, wszurl: ::windows::core::PWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetSignedIssuanceLicense(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *mut u8, cbsymkey: u32, wszsymkeytype: ::windows::core::PWSTR, wszclientlicensorcertificate: ::windows::core::PWSTR, pfncallback: *mut ::core::ffi::c_void, wszurl: ::windows::core::PWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DRMGetSignedIssuanceLicense(henv, hissuancelicense, uflags, ::core::mem::transmute(pbsymkey), cbsymkey, ::core::mem::transmute(wszsymkeytype), ::core::mem::transmute(wszclientlicensorcertificate), ::core::mem::transmute(pfncallback), ::core::mem::transmute(wszurl), ::core::mem::transmute(pvcontext)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetSignedIssuanceLicenseEx(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: ::core::option::Option<&[u8]>, wszsymkeytype: ::windows::core::PWSTR, pvreserved: ::core::option::Option<*mut ::core::ffi::c_void>, henablingprincipal: u32, hboundlicenseclc: u32, pfncallback: DRMCALLBACK, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetSignedIssuanceLicenseEx(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *mut u8, cbsymkey: u32, wszsymkeytype: ::windows::core::PWSTR, pvreserved: *mut ::core::ffi::c_void, henablingprincipal: u32, hboundlicenseclc: u32, pfncallback: *mut ::core::ffi::c_void, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DRMGetSignedIssuanceLicenseEx(henv, hissuancelicense, uflags, ::core::mem::transmute(pbsymkey.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pbsymkey.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(wszsymkeytype), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())), henablingprincipal, hboundlicenseclc, ::core::mem::transmute(pfncallback), ::core::mem::transmute(pvcontext)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetTime(henv: u32, etimeridtype: DRMTIMETYPE, potimeobject: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetTime(henv: u32, etimeridtype: DRMTIMETYPE, potimeobject: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT;
    }
    DRMGetTime(henv, etimeridtype, ::core::mem::transmute(potimeobject)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUnboundLicenseAttribute(hqueryroot: u32, wszattributetype: ::windows::core::PWSTR, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUnboundLicenseAttribute(hqueryroot: u32, wszattributetype: ::windows::core::PWSTR, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMGetUnboundLicenseAttribute(hqueryroot, ::core::mem::transmute(wszattributetype), iwhich, ::core::mem::transmute(peencoding), ::core::mem::transmute(pcbuffer), ::core::mem::transmute(pbbuffer)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUnboundLicenseAttributeCount(hqueryroot: u32, wszattributetype: ::windows::core::PWSTR, pcattributes: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUnboundLicenseAttributeCount(hqueryroot: u32, wszattributetype: ::windows::core::PWSTR, pcattributes: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetUnboundLicenseAttributeCount(hqueryroot, ::core::mem::transmute(wszattributetype), ::core::mem::transmute(pcattributes)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUnboundLicenseObject(hqueryroot: u32, wszsubobjecttype: ::windows::core::PWSTR, iindex: u32, phsubquery: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUnboundLicenseObject(hqueryroot: u32, wszsubobjecttype: ::windows::core::PWSTR, iindex: u32, phsubquery: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetUnboundLicenseObject(hqueryroot, ::core::mem::transmute(wszsubobjecttype), iindex, ::core::mem::transmute(phsubquery)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUnboundLicenseObjectCount(hqueryroot: u32, wszsubobjecttype: ::windows::core::PWSTR, pcsubobjects: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUnboundLicenseObjectCount(hqueryroot: u32, wszsubobjecttype: ::windows::core::PWSTR, pcsubobjects: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetUnboundLicenseObjectCount(hqueryroot, ::core::mem::transmute(wszsubobjecttype), ::core::mem::transmute(pcsubobjects)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetUsagePolicy(hissuancelicense: u32, uindex: u32, peusagepolicytype: *mut DRM_USAGEPOLICY_TYPE, pfexclusion: *mut super::super::Foundation::BOOL, punamelength: *mut u32, wszname: ::windows::core::PWSTR, puminversionlength: *mut u32, wszminversion: ::windows::core::PWSTR, pumaxversionlength: *mut u32, wszmaxversion: ::windows::core::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows::core::PWSTR, pudigestalgorithmlength: *mut u32, wszdigestalgorithm: ::windows::core::PWSTR, pcbdigest: *mut u32, pbdigest: *mut u8) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUsagePolicy(hissuancelicense: u32, uindex: u32, peusagepolicytype: *mut DRM_USAGEPOLICY_TYPE, pfexclusion: *mut super::super::Foundation::BOOL, punamelength: *mut u32, wszname: ::windows::core::PWSTR, puminversionlength: *mut u32, wszminversion: ::windows::core::PWSTR, pumaxversionlength: *mut u32, wszmaxversion: ::windows::core::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows::core::PWSTR, pudigestalgorithmlength: *mut u32, wszdigestalgorithm: ::windows::core::PWSTR, pcbdigest: *mut u32, pbdigest: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMGetUsagePolicy(
        hissuancelicense,
        uindex,
        ::core::mem::transmute(peusagepolicytype),
        ::core::mem::transmute(pfexclusion),
        ::core::mem::transmute(punamelength),
        ::core::mem::transmute(wszname),
        ::core::mem::transmute(puminversionlength),
        ::core::mem::transmute(wszminversion),
        ::core::mem::transmute(pumaxversionlength),
        ::core::mem::transmute(wszmaxversion),
        ::core::mem::transmute(pupublickeylength),
        ::core::mem::transmute(wszpublickey),
        ::core::mem::transmute(pudigestalgorithmlength),
        ::core::mem::transmute(wszdigestalgorithm),
        ::core::mem::transmute(pcbdigest),
        ::core::mem::transmute(pbdigest),
    )
    .ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUserInfo(huser: u32, puusernamelength: *mut u32, wszusername: ::windows::core::PWSTR, puuseridlength: *mut u32, wszuserid: ::windows::core::PWSTR, puuseridtypelength: *mut u32, wszuseridtype: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUserInfo(huser: u32, puusernamelength: *mut u32, wszusername: ::windows::core::PWSTR, puuseridlength: *mut u32, wszuserid: ::windows::core::PWSTR, puuseridtypelength: *mut u32, wszuseridtype: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetUserInfo(huser, ::core::mem::transmute(puusernamelength), ::core::mem::transmute(wszusername), ::core::mem::transmute(puuseridlength), ::core::mem::transmute(wszuserid), ::core::mem::transmute(puuseridtypelength), ::core::mem::transmute(wszuseridtype)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUserRights(hissuancelicense: u32, huser: u32, uindex: u32, phright: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUserRights(hissuancelicense: u32, huser: u32, uindex: u32, phright: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetUserRights(hissuancelicense, huser, uindex, ::core::mem::transmute(phright)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUsers(hissuancelicense: u32, uindex: u32, phuser: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUsers(hissuancelicense: u32, uindex: u32, phuser: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetUsers(hissuancelicense, uindex, ::core::mem::transmute(phuser)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMInitEnvironment(esecurityprovidertype: DRMSECURITYPROVIDERTYPE, especification: DRMSPECTYPE, wszsecurityprovider: ::windows::core::PWSTR, wszmanifestcredentials: ::windows::core::PWSTR, wszmachinecredentials: ::windows::core::PWSTR, phenv: *mut u32, phdefaultlibrary: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMInitEnvironment(esecurityprovidertype: DRMSECURITYPROVIDERTYPE, especification: DRMSPECTYPE, wszsecurityprovider: ::windows::core::PWSTR, wszmanifestcredentials: ::windows::core::PWSTR, wszmachinecredentials: ::windows::core::PWSTR, phenv: *mut u32, phdefaultlibrary: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMInitEnvironment(esecurityprovidertype, especification, ::core::mem::transmute(wszsecurityprovider), ::core::mem::transmute(wszmanifestcredentials), ::core::mem::transmute(wszmachinecredentials), ::core::mem::transmute(phenv), ::core::mem::transmute(phdefaultlibrary)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMIsActivated(hclient: u32, uflags: u32, pactservinfo: *mut DRM_ACTSERV_INFO) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMIsActivated(hclient: u32, uflags: u32, pactservinfo: *mut DRM_ACTSERV_INFO) -> ::windows::core::HRESULT;
    }
    DRMIsActivated(hclient, uflags, ::core::mem::transmute(pactservinfo)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMIsWindowProtected<'a, P0>(hwnd: P0, pfprotected: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMIsWindowProtected(hwnd: super::super::Foundation::HWND, pfprotected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    DRMIsWindowProtected(hwnd.into(), ::core::mem::transmute(pfprotected)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMLoadLibrary(henv: u32, especification: DRMSPECTYPE, wszlibraryprovider: ::windows::core::PWSTR, wszcredentials: ::windows::core::PWSTR, phlibrary: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMLoadLibrary(henv: u32, especification: DRMSPECTYPE, wszlibraryprovider: ::windows::core::PWSTR, wszcredentials: ::windows::core::PWSTR, phlibrary: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMLoadLibrary(henv, especification, ::core::mem::transmute(wszlibraryprovider), ::core::mem::transmute(wszcredentials), ::core::mem::transmute(phlibrary)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMParseUnboundLicense(wszcertificate: ::windows::core::PWSTR, phqueryroot: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMParseUnboundLicense(wszcertificate: ::windows::core::PWSTR, phqueryroot: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMParseUnboundLicense(::core::mem::transmute(wszcertificate), ::core::mem::transmute(phqueryroot)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMRegisterContent<'a, P0>(fregister: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMRegisterContent(fregister: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    DRMRegisterContent(fregister.into()).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMRegisterProtectedWindow<'a, P0>(henv: u32, hwnd: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMRegisterProtectedWindow(henv: u32, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT;
    }
    DRMRegisterProtectedWindow(henv, hwnd.into()).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMRegisterRevocationList(henv: u32, wszrevocationlist: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMRegisterRevocationList(henv: u32, wszrevocationlist: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMRegisterRevocationList(henv, ::core::mem::transmute(wszrevocationlist)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMRepair() -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMRepair() -> ::windows::core::HRESULT;
    }
    DRMRepair().ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMSetApplicationSpecificData<'a, P0>(hissuancelicense: u32, fdelete: P0, wszname: ::windows::core::PWSTR, wszvalue: ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMSetApplicationSpecificData(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, wszname: ::windows::core::PWSTR, wszvalue: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMSetApplicationSpecificData(hissuancelicense, fdelete.into(), ::core::mem::transmute(wszname), ::core::mem::transmute(wszvalue)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMSetGlobalOptions(eglobaloptions: DRMGLOBALOPTIONS, pvdata: *mut ::core::ffi::c_void, dwlen: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMSetGlobalOptions(eglobaloptions: DRMGLOBALOPTIONS, pvdata: *mut ::core::ffi::c_void, dwlen: u32) -> ::windows::core::HRESULT;
    }
    DRMSetGlobalOptions(eglobaloptions, ::core::mem::transmute(pvdata), dwlen).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMSetIntervalTime(hissuancelicense: u32, cdays: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMSetIntervalTime(hissuancelicense: u32, cdays: u32) -> ::windows::core::HRESULT;
    }
    DRMSetIntervalTime(hissuancelicense, cdays).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMSetMetaData(hissuancelicense: u32, wszcontentid: ::windows::core::PWSTR, wszcontentidtype: ::windows::core::PWSTR, wszskuid: ::windows::core::PWSTR, wszskuidtype: ::windows::core::PWSTR, wszcontenttype: ::windows::core::PWSTR, wszcontentname: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMSetMetaData(hissuancelicense: u32, wszcontentid: ::windows::core::PWSTR, wszcontentidtype: ::windows::core::PWSTR, wszskuid: ::windows::core::PWSTR, wszskuidtype: ::windows::core::PWSTR, wszcontenttype: ::windows::core::PWSTR, wszcontentname: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMSetMetaData(hissuancelicense, ::core::mem::transmute(wszcontentid), ::core::mem::transmute(wszcontentidtype), ::core::mem::transmute(wszskuid), ::core::mem::transmute(wszskuidtype), ::core::mem::transmute(wszcontenttype), ::core::mem::transmute(wszcontentname)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMSetNameAndDescription<'a, P0>(hissuancelicense: u32, fdelete: P0, lcid: u32, wszname: ::windows::core::PWSTR, wszdescription: ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMSetNameAndDescription(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, lcid: u32, wszname: ::windows::core::PWSTR, wszdescription: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMSetNameAndDescription(hissuancelicense, fdelete.into(), lcid, ::core::mem::transmute(wszname), ::core::mem::transmute(wszdescription)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMSetRevocationPoint<'a, P0>(hissuancelicense: u32, fdelete: P0, wszid: ::windows::core::PWSTR, wszidtype: ::windows::core::PWSTR, wszurl: ::windows::core::PWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, wszname: ::windows::core::PWSTR, wszpublickey: ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMSetRevocationPoint(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, wszid: ::windows::core::PWSTR, wszidtype: ::windows::core::PWSTR, wszurl: ::windows::core::PWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, wszname: ::windows::core::PWSTR, wszpublickey: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMSetRevocationPoint(hissuancelicense, fdelete.into(), ::core::mem::transmute(wszid), ::core::mem::transmute(wszidtype), ::core::mem::transmute(wszurl), ::core::mem::transmute(pstfrequency), ::core::mem::transmute(wszname), ::core::mem::transmute(wszpublickey)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMSetUsagePolicy<'a, P0, P1>(hissuancelicense: u32, eusagepolicytype: DRM_USAGEPOLICY_TYPE, fdelete: P0, fexclusion: P1, wszname: ::windows::core::PWSTR, wszminversion: ::windows::core::PWSTR, wszmaxversion: ::windows::core::PWSTR, wszpublickey: ::windows::core::PWSTR, wszdigestalgorithm: ::windows::core::PWSTR, pbdigest: *mut u8, cbdigest: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMSetUsagePolicy(hissuancelicense: u32, eusagepolicytype: DRM_USAGEPOLICY_TYPE, fdelete: super::super::Foundation::BOOL, fexclusion: super::super::Foundation::BOOL, wszname: ::windows::core::PWSTR, wszminversion: ::windows::core::PWSTR, wszmaxversion: ::windows::core::PWSTR, wszpublickey: ::windows::core::PWSTR, wszdigestalgorithm: ::windows::core::PWSTR, pbdigest: *mut u8, cbdigest: u32) -> ::windows::core::HRESULT;
    }
    DRMSetUsagePolicy(hissuancelicense, eusagepolicytype, fdelete.into(), fexclusion.into(), ::core::mem::transmute(wszname), ::core::mem::transmute(wszminversion), ::core::mem::transmute(wszmaxversion), ::core::mem::transmute(wszpublickey), ::core::mem::transmute(wszdigestalgorithm), ::core::mem::transmute(pbdigest), cbdigest).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMVerify(wszdata: ::windows::core::PWSTR, pcattesteddata: *mut u32, wszattesteddata: ::windows::core::PWSTR, petype: *mut DRMATTESTTYPE, pcprincipal: *mut u32, wszprincipal: ::windows::core::PWSTR, pcmanifest: *mut u32, wszmanifest: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMVerify(wszdata: ::windows::core::PWSTR, pcattesteddata: *mut u32, wszattesteddata: ::windows::core::PWSTR, petype: *mut DRMATTESTTYPE, pcprincipal: *mut u32, wszprincipal: ::windows::core::PWSTR, pcmanifest: *mut u32, wszmanifest: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMVerify(::core::mem::transmute(wszdata), ::core::mem::transmute(pcattesteddata), ::core::mem::transmute(wszattesteddata), ::core::mem::transmute(petype), ::core::mem::transmute(pcprincipal), ::core::mem::transmute(wszprincipal), ::core::mem::transmute(pcmanifest), ::core::mem::transmute(wszmanifest)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMACTSERVINFOVERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMBINDINGFLAGS_IGNORE_VALIDITY_INTERVALS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMBOUNDLICENSEPARAMSVERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMCALLBACKVERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMCLIENTSTRUCTVERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMENVHANDLE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMHANDLE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMHSESSION_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMIDVERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMLICENSEACQDATAVERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMPUBHANDLE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMQUERYHANDLE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ACTIVATE_CANCEL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ACTIVATE_DELAYED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ACTIVATE_GROUPIDENTITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ACTIVATE_MACHINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ACTIVATE_SHARED_GROUPIDENTITY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ACTIVATE_SILENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ACTIVATE_TEMPORARY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ADD_LICENSE_NOPERSIST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ADD_LICENSE_PERSIST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AILT_CANCEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AILT_NONSILENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AILT_OBTAIN_ALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AL_CANCEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AL_FETCHNOADVISORY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AL_NONSILENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AL_NOPERSIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AL_NOUI: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AUTO_GENERATE_KEY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_DEFAULTGROUPIDTYPE_PASSPORT: ::windows::core::PCWSTR = ::windows::w!("PassportAuthProvider");
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_DEFAULTGROUPIDTYPE_WINDOWSAUTH: ::windows::core::PCWSTR = ::windows::w!("WindowsAuthProvider");
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_CLIENTLICENSOR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_CLIENTLICENSOR_LID: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_EUL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_EUL_LID: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_EXPIRED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_GROUPIDENTITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_GROUPIDENTITY_LID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_GROUPIDENTITY_NAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_ISSUANCELICENSE_TEMPLATE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_ISSUANCELICENSE_TEMPLATE_LID: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_ISSUERNAME: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_MACHINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_REVOCATIONLIST: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_REVOCATIONLIST_LID: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_SPECIFIED_CLIENTLICENSOR: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_SPECIFIED_GROUPIDENTITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_LOCKBOXTYPE_BLACKBOX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_LOCKBOXTYPE_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_LOCKBOXTYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_LOCKBOXTYPE_WHITEBOX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_OWNER_LICENSE_NOPERSIST: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_REUSE_KEY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVER_ISSUANCELICENSE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVICE_LOCATION_ENTERPRISE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVICE_LOCATION_INTERNET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVICE_TYPE_ACTIVATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVICE_TYPE_CERTIFICATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVICE_TYPE_CLIENTLICENSOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVICE_TYPE_PUBLISHING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVICE_TYPE_SILENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SIGN_CANCEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SIGN_OFFLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SIGN_ONLINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const MSDRM_CLIENT_ZONE: u32 = 52992u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const MSDRM_POLICY_ZONE: u32 = 37632u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRMATTESTTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMATTESTTYPE_FULLENVIRONMENT: DRMATTESTTYPE = DRMATTESTTYPE(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMATTESTTYPE_HASHONLY: DRMATTESTTYPE = DRMATTESTTYPE(1i32);
impl ::core::marker::Copy for DRMATTESTTYPE {}
impl ::core::clone::Clone for DRMATTESTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRMATTESTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRMATTESTTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRMATTESTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMATTESTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRMENCODINGTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMENCODINGTYPE_BASE64: DRMENCODINGTYPE = DRMENCODINGTYPE(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMENCODINGTYPE_STRING: DRMENCODINGTYPE = DRMENCODINGTYPE(1i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMENCODINGTYPE_LONG: DRMENCODINGTYPE = DRMENCODINGTYPE(2i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMENCODINGTYPE_TIME: DRMENCODINGTYPE = DRMENCODINGTYPE(3i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMENCODINGTYPE_UINT: DRMENCODINGTYPE = DRMENCODINGTYPE(4i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMENCODINGTYPE_RAW: DRMENCODINGTYPE = DRMENCODINGTYPE(5i32);
impl ::core::marker::Copy for DRMENCODINGTYPE {}
impl ::core::clone::Clone for DRMENCODINGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRMENCODINGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRMENCODINGTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRMENCODINGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMENCODINGTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRMGLOBALOPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMGLOBALOPTIONS_USE_WINHTTP: DRMGLOBALOPTIONS = DRMGLOBALOPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMGLOBALOPTIONS_USE_SERVERSECURITYPROCESSOR: DRMGLOBALOPTIONS = DRMGLOBALOPTIONS(1i32);
impl ::core::marker::Copy for DRMGLOBALOPTIONS {}
impl ::core::clone::Clone for DRMGLOBALOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRMGLOBALOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRMGLOBALOPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRMGLOBALOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMGLOBALOPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRMSECURITYPROVIDERTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMSECURITYPROVIDERTYPE_SOFTWARESECREP: DRMSECURITYPROVIDERTYPE = DRMSECURITYPROVIDERTYPE(0i32);
impl ::core::marker::Copy for DRMSECURITYPROVIDERTYPE {}
impl ::core::clone::Clone for DRMSECURITYPROVIDERTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRMSECURITYPROVIDERTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRMSECURITYPROVIDERTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRMSECURITYPROVIDERTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMSECURITYPROVIDERTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRMSPECTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMSPECTYPE_UNKNOWN: DRMSPECTYPE = DRMSPECTYPE(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMSPECTYPE_FILENAME: DRMSPECTYPE = DRMSPECTYPE(1i32);
impl ::core::marker::Copy for DRMSPECTYPE {}
impl ::core::clone::Clone for DRMSPECTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRMSPECTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRMSPECTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRMSPECTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMSPECTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRMTIMETYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMTIMETYPE_SYSTEMUTC: DRMTIMETYPE = DRMTIMETYPE(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMTIMETYPE_SYSTEMLOCAL: DRMTIMETYPE = DRMTIMETYPE(1i32);
impl ::core::marker::Copy for DRMTIMETYPE {}
impl ::core::clone::Clone for DRMTIMETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRMTIMETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRMTIMETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRMTIMETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMTIMETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRM_DISTRIBUTION_POINT_INFO(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_DISTRIBUTION_POINT_LICENSE_ACQUISITION: DRM_DISTRIBUTION_POINT_INFO = DRM_DISTRIBUTION_POINT_INFO(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_DISTRIBUTION_POINT_PUBLISHING: DRM_DISTRIBUTION_POINT_INFO = DRM_DISTRIBUTION_POINT_INFO(1i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_DISTRIBUTION_POINT_REFERRAL_INFO: DRM_DISTRIBUTION_POINT_INFO = DRM_DISTRIBUTION_POINT_INFO(2i32);
impl ::core::marker::Copy for DRM_DISTRIBUTION_POINT_INFO {}
impl ::core::clone::Clone for DRM_DISTRIBUTION_POINT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRM_DISTRIBUTION_POINT_INFO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRM_DISTRIBUTION_POINT_INFO {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRM_DISTRIBUTION_POINT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRM_DISTRIBUTION_POINT_INFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRM_STATUS_MSG(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_MSG_ACTIVATE_MACHINE: DRM_STATUS_MSG = DRM_STATUS_MSG(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_MSG_ACTIVATE_GROUPIDENTITY: DRM_STATUS_MSG = DRM_STATUS_MSG(1i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_MSG_ACQUIRE_LICENSE: DRM_STATUS_MSG = DRM_STATUS_MSG(2i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_MSG_ACQUIRE_ADVISORY: DRM_STATUS_MSG = DRM_STATUS_MSG(3i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_MSG_SIGN_ISSUANCE_LICENSE: DRM_STATUS_MSG = DRM_STATUS_MSG(4i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_MSG_ACQUIRE_CLIENTLICENSOR: DRM_STATUS_MSG = DRM_STATUS_MSG(5i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_MSG_ACQUIRE_ISSUANCE_LICENSE_TEMPLATE: DRM_STATUS_MSG = DRM_STATUS_MSG(6i32);
impl ::core::marker::Copy for DRM_STATUS_MSG {}
impl ::core::clone::Clone for DRM_STATUS_MSG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRM_STATUS_MSG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRM_STATUS_MSG {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRM_STATUS_MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRM_STATUS_MSG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRM_USAGEPOLICY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_USAGEPOLICY_TYPE_BYNAME: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_USAGEPOLICY_TYPE_BYPUBLICKEY: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_USAGEPOLICY_TYPE_BYDIGEST: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_USAGEPOLICY_TYPE_OSEXCLUSION: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(3i32);
impl ::core::marker::Copy for DRM_USAGEPOLICY_TYPE {}
impl ::core::clone::Clone for DRM_USAGEPOLICY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRM_USAGEPOLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRM_USAGEPOLICY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRM_USAGEPOLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRM_USAGEPOLICY_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub struct DRMBOUNDLICENSEPARAMS {
    pub uVersion: u32,
    pub hEnablingPrincipal: u32,
    pub hSecureStore: u32,
    pub wszRightsRequested: ::windows::core::PWSTR,
    pub wszRightsGroup: ::windows::core::PWSTR,
    pub idResource: DRMID,
    pub cAuthenticatorCount: u32,
    pub rghAuthenticators: *mut u32,
    pub wszDefaultEnablingPrincipalCredentials: ::windows::core::PWSTR,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DRMBOUNDLICENSEPARAMS {}
impl ::core::clone::Clone for DRMBOUNDLICENSEPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRMBOUNDLICENSEPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRMBOUNDLICENSEPARAMS")
            .field("uVersion", &self.uVersion)
            .field("hEnablingPrincipal", &self.hEnablingPrincipal)
            .field("hSecureStore", &self.hSecureStore)
            .field("wszRightsRequested", &self.wszRightsRequested)
            .field("wszRightsGroup", &self.wszRightsGroup)
            .field("idResource", &self.idResource)
            .field("cAuthenticatorCount", &self.cAuthenticatorCount)
            .field("rghAuthenticators", &self.rghAuthenticators)
            .field("wszDefaultEnablingPrincipalCredentials", &self.wszDefaultEnablingPrincipalCredentials)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DRMBOUNDLICENSEPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRMBOUNDLICENSEPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRMBOUNDLICENSEPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRMBOUNDLICENSEPARAMS {}
impl ::core::default::Default for DRMBOUNDLICENSEPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub struct DRMID {
    pub uVersion: u32,
    pub wszIDType: ::windows::core::PWSTR,
    pub wszID: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DRMID {}
impl ::core::clone::Clone for DRMID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRMID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRMID").field("uVersion", &self.uVersion).field("wszIDType", &self.wszIDType).field("wszID", &self.wszID).finish()
    }
}
unsafe impl ::windows::core::Abi for DRMID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRMID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRMID>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRMID {}
impl ::core::default::Default for DRMID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub struct DRM_ACTSERV_INFO {
    pub uVersion: u32,
    pub wszPubKey: ::windows::core::PWSTR,
    pub wszURL: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DRM_ACTSERV_INFO {}
impl ::core::clone::Clone for DRM_ACTSERV_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_ACTSERV_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_ACTSERV_INFO").field("uVersion", &self.uVersion).field("wszPubKey", &self.wszPubKey).field("wszURL", &self.wszURL).finish()
    }
}
unsafe impl ::windows::core::Abi for DRM_ACTSERV_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_ACTSERV_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_ACTSERV_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_ACTSERV_INFO {}
impl ::core::default::Default for DRM_ACTSERV_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub struct DRM_CLIENT_VERSION_INFO {
    pub uStructVersion: u32,
    pub dwVersion: [u32; 4],
    pub wszHierarchy: [u16; 256],
    pub wszProductId: [u16; 256],
    pub wszProductDescription: [u16; 256],
}
impl ::core::marker::Copy for DRM_CLIENT_VERSION_INFO {}
impl ::core::clone::Clone for DRM_CLIENT_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_CLIENT_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_CLIENT_VERSION_INFO").field("uStructVersion", &self.uStructVersion).field("dwVersion", &self.dwVersion).field("wszHierarchy", &self.wszHierarchy).field("wszProductId", &self.wszProductId).field("wszProductDescription", &self.wszProductDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for DRM_CLIENT_VERSION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_CLIENT_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_CLIENT_VERSION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_CLIENT_VERSION_INFO {}
impl ::core::default::Default for DRM_CLIENT_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub struct DRM_LICENSE_ACQ_DATA {
    pub uVersion: u32,
    pub wszURL: ::windows::core::PWSTR,
    pub wszLocalFilename: ::windows::core::PWSTR,
    pub pbPostData: *mut u8,
    pub dwPostDataSize: u32,
    pub wszFriendlyName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DRM_LICENSE_ACQ_DATA {}
impl ::core::clone::Clone for DRM_LICENSE_ACQ_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_LICENSE_ACQ_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_LICENSE_ACQ_DATA").field("uVersion", &self.uVersion).field("wszURL", &self.wszURL).field("wszLocalFilename", &self.wszLocalFilename).field("pbPostData", &self.pbPostData).field("dwPostDataSize", &self.dwPostDataSize).field("wszFriendlyName", &self.wszFriendlyName).finish()
    }
}
unsafe impl ::windows::core::Abi for DRM_LICENSE_ACQ_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_LICENSE_ACQ_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_LICENSE_ACQ_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_LICENSE_ACQ_DATA {}
impl ::core::default::Default for DRM_LICENSE_ACQ_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub type DRMCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: DRM_STATUS_MSG, param1: ::windows::core::HRESULT, param2: *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
