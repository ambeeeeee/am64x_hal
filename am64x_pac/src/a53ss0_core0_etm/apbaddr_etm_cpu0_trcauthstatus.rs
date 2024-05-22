#[doc = "Register `APBADDR_ETM_CPU0_TRCAUTHSTATUS` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrcauthstatusSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCAUTHSTATUS` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrcauthstatusSpec>;
#[doc = "Field `NSID` reader - 1:0\\]
Indicates whether the trace unit supports Non-secure invasive debug: 00 The trace unit does not support Non-secure invasive debug. All other values are reserved."]
pub type NsidR = crate::FieldReader;
#[doc = "Field `NSID` writer - 1:0\\]
Indicates whether the trace unit supports Non-secure invasive debug: 00 The trace unit does not support Non-secure invasive debug. All other values are reserved."]
pub type NsidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NSNID` reader - 3:2\\]
Indicates whether the system enables the trace unit to support Non-secure non-invasive debug: 00 The trace unit does not implement support for Non-secure non-invasive debug. 01 Reserved. 10 Non-secure non-invasive debug is disabled. 11 Non-secure non-invasive debug is enabled."]
pub type NsnidR = crate::FieldReader;
#[doc = "Field `NSNID` writer - 3:2\\]
Indicates whether the system enables the trace unit to support Non-secure non-invasive debug: 00 The trace unit does not implement support for Non-secure non-invasive debug. 01 Reserved. 10 Non-secure non-invasive debug is disabled. 11 Non-secure non-invasive debug is enabled."]
pub type NsnidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SID` reader - 5:4\\]
Indicates whether the trace unit supports Secure invasive debug: 00 The trace unit does not support Secure invasive debug. All other values are reserved."]
pub type SidR = crate::FieldReader;
#[doc = "Field `SID` writer - 5:4\\]
Indicates whether the trace unit supports Secure invasive debug: 00 The trace unit does not support Secure invasive debug. All other values are reserved."]
pub type SidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SNID` reader - 7:6\\]
Indicates whether the system enables the trace unit to support Secure non-invasive debug: 00 The trace unit does not implement support for Secure non-invasive debug. 01 Reserved. 10 Secure non-invasive debug is disabled. 11 Secure non-invasive debug is enabled."]
pub type SnidR = crate::FieldReader;
#[doc = "Field `SNID` writer - 7:6\\]
Indicates whether the system enables the trace unit to support Secure non-invasive debug: 00 The trace unit does not implement support for Secure non-invasive debug. 01 Reserved. 10 Secure non-invasive debug is disabled. 11 Secure non-invasive debug is enabled."]
pub type SnidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES0_TRCAUTHSTATUS_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Trcauthstatus31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCAUTHSTATUS_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Trcauthstatus31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Indicates whether the trace unit supports Non-secure invasive debug: 00 The trace unit does not support Non-secure invasive debug. All other values are reserved."]
    #[inline(always)]
    pub fn nsid(&self) -> NsidR {
        NsidR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Indicates whether the system enables the trace unit to support Non-secure non-invasive debug: 00 The trace unit does not implement support for Non-secure non-invasive debug. 01 Reserved. 10 Non-secure non-invasive debug is disabled. 11 Non-secure non-invasive debug is enabled."]
    #[inline(always)]
    pub fn nsnid(&self) -> NsnidR {
        NsnidR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Indicates whether the trace unit supports Secure invasive debug: 00 The trace unit does not support Secure invasive debug. All other values are reserved."]
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Indicates whether the system enables the trace unit to support Secure non-invasive debug: 00 The trace unit does not implement support for Secure non-invasive debug. 01 Reserved. 10 Secure non-invasive debug is disabled. 11 Secure non-invasive debug is enabled."]
    #[inline(always)]
    pub fn snid(&self) -> SnidR {
        SnidR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcauthstatus_31_8(&self) -> Res0Trcauthstatus31_8R {
        Res0Trcauthstatus31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Indicates whether the trace unit supports Non-secure invasive debug: 00 The trace unit does not support Non-secure invasive debug. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn nsid(&mut self) -> NsidW<ApbaddrEtmCpu0TrcauthstatusSpec> {
        NsidW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Indicates whether the system enables the trace unit to support Non-secure non-invasive debug: 00 The trace unit does not implement support for Non-secure non-invasive debug. 01 Reserved. 10 Non-secure non-invasive debug is disabled. 11 Non-secure non-invasive debug is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn nsnid(&mut self) -> NsnidW<ApbaddrEtmCpu0TrcauthstatusSpec> {
        NsnidW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Indicates whether the trace unit supports Secure invasive debug: 00 The trace unit does not support Secure invasive debug. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn sid(&mut self) -> SidW<ApbaddrEtmCpu0TrcauthstatusSpec> {
        SidW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Indicates whether the system enables the trace unit to support Secure non-invasive debug: 00 The trace unit does not implement support for Secure non-invasive debug. 01 Reserved. 10 Secure non-invasive debug is disabled. 11 Secure non-invasive debug is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn snid(&mut self) -> SnidW<ApbaddrEtmCpu0TrcauthstatusSpec> {
        SnidW::new(self, 6)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcauthstatus_31_8(
        &mut self,
    ) -> Res0Trcauthstatus31_8W<ApbaddrEtmCpu0TrcauthstatusSpec> {
        Res0Trcauthstatus31_8W::new(self, 8)
    }
}
#[doc = "Authentication Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcauthstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcauthstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrcauthstatusSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrcauthstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcauthstatus::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrcauthstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcauthstatus::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrcauthstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCAUTHSTATUS to value 0x88"]
impl crate::Resettable for ApbaddrEtmCpu0TrcauthstatusSpec {
    const RESET_VALUE: u32 = 0x88;
}
