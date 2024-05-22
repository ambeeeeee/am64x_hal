#[doc = "Register `APBADDR_DBG_CPU1_DBGAUTHSTATUS_EL1` reader"]
pub type R = crate::R<ApbaddrDbgCpu1DbgauthstatusEl1Spec>;
#[doc = "Register `APBADDR_DBG_CPU1_DBGAUTHSTATUS_EL1` writer"]
pub type W = crate::W<ApbaddrDbgCpu1DbgauthstatusEl1Spec>;
#[doc = "Field `NSID` reader - 1:0\\]
Non-secure invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Secure. 10 Implemented and disabled. ExternalInvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalInvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
pub type NsidR = crate::FieldReader;
#[doc = "Field `NSID` writer - 1:0\\]
Non-secure invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Secure. 10 Implemented and disabled. ExternalInvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalInvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
pub type NsidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NSNID` reader - 3:2\\]
Non-secure non-invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Secure. 10 Implemented and disabled. ExternalNoninvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalNoninvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
pub type NsnidR = crate::FieldReader;
#[doc = "Field `NSNID` writer - 3:2\\]
Non-secure non-invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Secure. 10 Implemented and disabled. ExternalNoninvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalNoninvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
pub type NsnidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SID` reader - 5:4\\]
Secure invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Non-secure. 10 Implemented and disabled. ExternalSecureInvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalSecureInvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
pub type SidR = crate::FieldReader;
#[doc = "Field `SID` writer - 5:4\\]
Secure invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Non-secure. 10 Implemented and disabled. ExternalSecureInvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalSecureInvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
pub type SidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SNID` reader - 7:6\\]
Secure non-invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Non-secure. 10 Implemented and disabled. ExternalSecureNoninvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalSecureNoninvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
pub type SnidR = crate::FieldReader;
#[doc = "Field `SNID` writer - 7:6\\]
Secure non-invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Non-secure. 10 Implemented and disabled. ExternalSecureNoninvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalSecureNoninvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
pub type SnidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES0_DBGAUTHSTATUS_EL1_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0DbgauthstatusEl1_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_DBGAUTHSTATUS_EL1_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0DbgauthstatusEl1_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Non-secure invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Secure. 10 Implemented and disabled. ExternalInvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalInvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
    #[inline(always)]
    pub fn nsid(&self) -> NsidR {
        NsidR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Non-secure non-invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Secure. 10 Implemented and disabled. ExternalNoninvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalNoninvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
    #[inline(always)]
    pub fn nsnid(&self) -> NsnidR {
        NsnidR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Secure invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Non-secure. 10 Implemented and disabled. ExternalSecureInvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalSecureInvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Secure non-invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Non-secure. 10 Implemented and disabled. ExternalSecureNoninvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalSecureNoninvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
    #[inline(always)]
    pub fn snid(&self) -> SnidR {
        SnidR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_dbgauthstatus_el1_31_8(&self) -> Res0DbgauthstatusEl1_31_8R {
        Res0DbgauthstatusEl1_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Non-secure invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Secure. 10 Implemented and disabled. ExternalInvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalInvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn nsid(&mut self) -> NsidW<ApbaddrDbgCpu1DbgauthstatusEl1Spec> {
        NsidW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Non-secure non-invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Secure. 10 Implemented and disabled. ExternalNoninvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalNoninvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn nsnid(&mut self) -> NsnidW<ApbaddrDbgCpu1DbgauthstatusEl1Spec> {
        NsnidW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Secure invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Non-secure. 10 Implemented and disabled. ExternalSecureInvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalSecureInvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn sid(&mut self) -> SidW<ApbaddrDbgCpu1DbgauthstatusEl1Spec> {
        SidW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Secure non-invasive debug. Possible values of this field are: 00 Not implemented. EL3 is not implemented and the processor is Non-secure. 10 Implemented and disabled. ExternalSecureNoninvasiveDebugEnabled\\[\\]
== FALSE. 11 Implemented and enabled. ExternalSecureNoninvasiveDebugEnabled\\[\\]
== TRUE. Other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn snid(&mut self) -> SnidW<ApbaddrDbgCpu1DbgauthstatusEl1Spec> {
        SnidW::new(self, 6)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_dbgauthstatus_el1_31_8(
        &mut self,
    ) -> Res0DbgauthstatusEl1_31_8W<ApbaddrDbgCpu1DbgauthstatusEl1Spec> {
        Res0DbgauthstatusEl1_31_8W::new(self, 8)
    }
}
#[doc = "Debug Authentication Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgauthstatus_el1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgauthstatus_el1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1DbgauthstatusEl1Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu1DbgauthstatusEl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_dbgauthstatus_el1::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1DbgauthstatusEl1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_dbgauthstatus_el1::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1DbgauthstatusEl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_DBGAUTHSTATUS_EL1 to value 0xaa"]
impl crate::Resettable for ApbaddrDbgCpu1DbgauthstatusEl1Spec {
    const RESET_VALUE: u32 = 0xaa;
}
