#[doc = "Register `APBADDR_ETM_CPU1_TRCPDCR` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcpdcrSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCPDCR` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcpdcrSpec>;
#[doc = "Field `RES0_TRCPDCR_2_0` reader - 2:0\\]
Reserved, RES0."]
pub type Res0Trcpdcr2_0R = crate::FieldReader;
#[doc = "Field `RES0_TRCPDCR_2_0` writer - 2:0\\]
Reserved, RES0."]
pub type Res0Trcpdcr2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PU` reader - 3:3\\]
Powerup request bit: 0 The system can remove power from the trace unit. The TRCPDSR indicates if the trace unit is powered down. 1 The system must provide power to the trace unit. Typically, a trace unit drives a signal representing the value of this bit to a power controller to request that the trace unit core power domain is powered up. However, if the trace unit and the processor are in the same power domain then the implementation might combine the PU status with a signal from the processor."]
pub type PuR = crate::BitReader;
#[doc = "Field `PU` writer - 3:3\\]
Powerup request bit: 0 The system can remove power from the trace unit. The TRCPDSR indicates if the trace unit is powered down. 1 The system must provide power to the trace unit. Typically, a trace unit drives a signal representing the value of this bit to a power controller to request that the trace unit core power domain is powered up. However, if the trace unit and the processor are in the same power domain then the implementation might combine the PU status with a signal from the processor."]
pub type PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCPDCR_31_4` reader - 31:4\\]
Reserved, RES0."]
pub type Res0Trcpdcr31_4R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCPDCR_31_4` writer - 31:4\\]
Reserved, RES0."]
pub type Res0Trcpdcr31_4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcpdcr_2_0(&self) -> Res0Trcpdcr2_0R {
        Res0Trcpdcr2_0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Powerup request bit: 0 The system can remove power from the trace unit. The TRCPDSR indicates if the trace unit is powered down. 1 The system must provide power to the trace unit. Typically, a trace unit drives a signal representing the value of this bit to a power controller to request that the trace unit core power domain is powered up. However, if the trace unit and the processor are in the same power domain then the implementation might combine the PU status with a signal from the processor."]
    #[inline(always)]
    pub fn pu(&self) -> PuR {
        PuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcpdcr_31_4(&self) -> Res0Trcpdcr31_4R {
        Res0Trcpdcr31_4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcpdcr_2_0(&mut self) -> Res0Trcpdcr2_0W<ApbaddrEtmCpu1TrcpdcrSpec> {
        Res0Trcpdcr2_0W::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Powerup request bit: 0 The system can remove power from the trace unit. The TRCPDSR indicates if the trace unit is powered down. 1 The system must provide power to the trace unit. Typically, a trace unit drives a signal representing the value of this bit to a power controller to request that the trace unit core power domain is powered up. However, if the trace unit and the processor are in the same power domain then the implementation might combine the PU status with a signal from the processor."]
    #[inline(always)]
    #[must_use]
    pub fn pu(&mut self) -> PuW<ApbaddrEtmCpu1TrcpdcrSpec> {
        PuW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcpdcr_31_4(&mut self) -> Res0Trcpdcr31_4W<ApbaddrEtmCpu1TrcpdcrSpec> {
        Res0Trcpdcr31_4W::new(self, 4)
    }
}
#[doc = "Power Down Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcpdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcpdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcpdcrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcpdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcpdcr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcpdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcpdcr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcpdcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCPDCR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1TrcpdcrSpec {
    const RESET_VALUE: u32 = 0;
}
