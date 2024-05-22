#[doc = "Register `EPWM_REGS_AQCTLB` reader"]
pub type R = crate::R<EpwmRegsAqctlbSpec>;
#[doc = "Register `EPWM_REGS_AQCTLB` writer"]
pub type W = crate::W<EpwmRegsAqctlbSpec>;
#[doc = "Field `ZRO` reader - 1:0\\]
Action when counter equals zero Note: By definition, in count up-down mode when the counter equals 0 the direction is defined as 1 or counting up"]
pub type ZroR = crate::FieldReader;
#[doc = "Field `ZRO` writer - 1:0\\]
Action when counter equals zero Note: By definition, in count up-down mode when the counter equals 0 the direction is defined as 1 or counting up"]
pub type ZroW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRD` reader - 3:2\\]
Action when the counter equals the period Note: By definition, in count up-down mode when the counter equals period the direction is defined as 0 or counting down"]
pub type PrdR = crate::FieldReader;
#[doc = "Field `PRD` writer - 3:2\\]
Action when the counter equals the period Note: By definition, in count up-down mode when the counter equals period the direction is defined as 0 or counting down"]
pub type PrdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CAU` reader - 5:4\\]
Action when the counter equals the active CMPA register and the counter is incrementing"]
pub type CauR = crate::FieldReader;
#[doc = "Field `CAU` writer - 5:4\\]
Action when the counter equals the active CMPA register and the counter is incrementing"]
pub type CauW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CAD` reader - 7:6\\]
Action when the counter equals the active CMPA register and the counter is decrementing"]
pub type CadR = crate::FieldReader;
#[doc = "Field `CAD` writer - 7:6\\]
Action when the counter equals the active CMPA register and the counter is decrementing"]
pub type CadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CBU` reader - 9:8\\]
Action when the counter equals the active CMPB register and the counter is incrementing"]
pub type CbuR = crate::FieldReader;
#[doc = "Field `CBU` writer - 9:8\\]
Action when the counter equals the active CMPB register and the counter is incrementing"]
pub type CbuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CBD` reader - 11:10\\]
Action when the counter equals the active CMPB register and the counter is decrementing"]
pub type CbdR = crate::FieldReader;
#[doc = "Field `CBD` writer - 11:10\\]
Action when the counter equals the active CMPB register and the counter is decrementing"]
pub type CbdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Action when counter equals zero Note: By definition, in count up-down mode when the counter equals 0 the direction is defined as 1 or counting up"]
    #[inline(always)]
    pub fn zro(&self) -> ZroR {
        ZroR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Action when the counter equals the period Note: By definition, in count up-down mode when the counter equals period the direction is defined as 0 or counting down"]
    #[inline(always)]
    pub fn prd(&self) -> PrdR {
        PrdR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Action when the counter equals the active CMPA register and the counter is incrementing"]
    #[inline(always)]
    pub fn cau(&self) -> CauR {
        CauR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Action when the counter equals the active CMPA register and the counter is decrementing"]
    #[inline(always)]
    pub fn cad(&self) -> CadR {
        CadR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Action when the counter equals the active CMPB register and the counter is incrementing"]
    #[inline(always)]
    pub fn cbu(&self) -> CbuR {
        CbuR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Action when the counter equals the active CMPB register and the counter is decrementing"]
    #[inline(always)]
    pub fn cbd(&self) -> CbdR {
        CbdR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Action when counter equals zero Note: By definition, in count up-down mode when the counter equals 0 the direction is defined as 1 or counting up"]
    #[inline(always)]
    #[must_use]
    pub fn zro(&mut self) -> ZroW<EpwmRegsAqctlbSpec> {
        ZroW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Action when the counter equals the period Note: By definition, in count up-down mode when the counter equals period the direction is defined as 0 or counting down"]
    #[inline(always)]
    #[must_use]
    pub fn prd(&mut self) -> PrdW<EpwmRegsAqctlbSpec> {
        PrdW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Action when the counter equals the active CMPA register and the counter is incrementing"]
    #[inline(always)]
    #[must_use]
    pub fn cau(&mut self) -> CauW<EpwmRegsAqctlbSpec> {
        CauW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Action when the counter equals the active CMPA register and the counter is decrementing"]
    #[inline(always)]
    #[must_use]
    pub fn cad(&mut self) -> CadW<EpwmRegsAqctlbSpec> {
        CadW::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Action when the counter equals the active CMPB register and the counter is incrementing"]
    #[inline(always)]
    #[must_use]
    pub fn cbu(&mut self) -> CbuW<EpwmRegsAqctlbSpec> {
        CbuW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Action when the counter equals the active CMPB register and the counter is decrementing"]
    #[inline(always)]
    #[must_use]
    pub fn cbd(&mut self) -> CbdW<EpwmRegsAqctlbSpec> {
        CbdW::new(self, 10)
    }
}
#[doc = "Action Qualifier Control Register For Output B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_aqctlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_aqctlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsAqctlbSpec;
impl crate::RegisterSpec for EpwmRegsAqctlbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_aqctlb::R`](R) reader structure"]
impl crate::Readable for EpwmRegsAqctlbSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_aqctlb::W`](W) writer structure"]
impl crate::Writable for EpwmRegsAqctlbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_AQCTLB to value 0"]
impl crate::Resettable for EpwmRegsAqctlbSpec {
    const RESET_VALUE: u16 = 0;
}
