#[doc = "Register `APBADDR_ETM_CPU1_TRCDEVTYPE` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcdevtypeSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCDEVTYPE` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcdevtypeSpec>;
#[doc = "Field `MAIN` reader - 3:0\\]
Returns 0x3, to indicate that the ETM is a trace source.All other values are reserved."]
pub type MainR = crate::FieldReader;
#[doc = "Field `MAIN` writer - 3:0\\]
Returns 0x3, to indicate that the ETM is a trace source.All other values are reserved."]
pub type MainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SUB` reader - 7:4\\]
Returns 0x1, to indicate that the ETM generates processor trace.All other values are reserved."]
pub type SubR = crate::FieldReader;
#[doc = "Field `SUB` writer - 7:4\\]
Returns 0x1, to indicate that the ETM generates processor trace.All other values are reserved."]
pub type SubW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_TRCDEVTYPE_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Trcdevtype31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCDEVTYPE_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Trcdevtype31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Returns 0x3, to indicate that the ETM is a trace source.All other values are reserved."]
    #[inline(always)]
    pub fn main(&self) -> MainR {
        MainR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Returns 0x1, to indicate that the ETM generates processor trace.All other values are reserved."]
    #[inline(always)]
    pub fn sub(&self) -> SubR {
        SubR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcdevtype_31_8(&self) -> Res0Trcdevtype31_8R {
        Res0Trcdevtype31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Returns 0x3, to indicate that the ETM is a trace source.All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn main(&mut self) -> MainW<ApbaddrEtmCpu1TrcdevtypeSpec> {
        MainW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Returns 0x1, to indicate that the ETM generates processor trace.All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn sub(&mut self) -> SubW<ApbaddrEtmCpu1TrcdevtypeSpec> {
        SubW::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcdevtype_31_8(&mut self) -> Res0Trcdevtype31_8W<ApbaddrEtmCpu1TrcdevtypeSpec> {
        Res0Trcdevtype31_8W::new(self, 8)
    }
}
#[doc = "Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcdevtype::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcdevtype::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcdevtypeSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcdevtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcdevtype::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcdevtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcdevtype::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcdevtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCDEVTYPE to value 0x13"]
impl crate::Resettable for ApbaddrEtmCpu1TrcdevtypeSpec {
    const RESET_VALUE: u32 = 0x13;
}
