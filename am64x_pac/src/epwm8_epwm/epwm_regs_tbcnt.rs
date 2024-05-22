#[doc = "Register `EPWM_REGS_TBCNT` reader"]
pub type R = crate::R<EpwmRegsTbcntSpec>;
#[doc = "Register `EPWM_REGS_TBCNT` writer"]
pub type W = crate::W<EpwmRegsTbcntSpec>;
#[doc = "Field `TBCNT` reader - 15:0\\]
Reading these bits gives the current time-base counter value Writing to these bits sets the current time-base counter value The update happens as soon as the write occurs The write is NOT synchronized to the time-base clock \\[TBCLK\\]
and the register is not shadowed"]
pub type TbcntR = crate::FieldReader<u16>;
#[doc = "Field `TBCNT` writer - 15:0\\]
Reading these bits gives the current time-base counter value Writing to these bits sets the current time-base counter value The update happens as soon as the write occurs The write is NOT synchronized to the time-base clock \\[TBCLK\\]
and the register is not shadowed"]
pub type TbcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Reading these bits gives the current time-base counter value Writing to these bits sets the current time-base counter value The update happens as soon as the write occurs The write is NOT synchronized to the time-base clock \\[TBCLK\\]
and the register is not shadowed"]
    #[inline(always)]
    pub fn tbcnt(&self) -> TbcntR {
        TbcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Reading these bits gives the current time-base counter value Writing to these bits sets the current time-base counter value The update happens as soon as the write occurs The write is NOT synchronized to the time-base clock \\[TBCLK\\]
and the register is not shadowed"]
    #[inline(always)]
    #[must_use]
    pub fn tbcnt(&mut self) -> TbcntW<EpwmRegsTbcntSpec> {
        TbcntW::new(self, 0)
    }
}
#[doc = "Time Base Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tbcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tbcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsTbcntSpec;
impl crate::RegisterSpec for EpwmRegsTbcntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_tbcnt::R`](R) reader structure"]
impl crate::Readable for EpwmRegsTbcntSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_tbcnt::W`](W) writer structure"]
impl crate::Writable for EpwmRegsTbcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_TBCNT to value 0"]
impl crate::Resettable for EpwmRegsTbcntSpec {
    const RESET_VALUE: u16 = 0;
}
