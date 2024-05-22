#[doc = "Register `EPWM_REGS_ETFLG` reader"]
pub type R = crate::R<EpwmRegsEtflgSpec>;
#[doc = "Register `EPWM_REGS_ETFLG` writer"]
pub type W = crate::W<EpwmRegsEtflgSpec>;
#[doc = "Field `INT` reader - 0:0\\]
Latched ePWM Interrupt \\[EPWMx_INT\\]
Status Flag"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - 0:0\\]
Latched ePWM Interrupt \\[EPWMx_INT\\]
Status Flag"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Latched ePWM Interrupt \\[EPWMx_INT\\]
Status Flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Latched ePWM Interrupt \\[EPWMx_INT\\]
Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<EpwmRegsEtflgSpec> {
        IntW::new(self, 0)
    }
}
#[doc = "Event Trigger Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_etflg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_etflg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsEtflgSpec;
impl crate::RegisterSpec for EpwmRegsEtflgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_etflg::R`](R) reader structure"]
impl crate::Readable for EpwmRegsEtflgSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_etflg::W`](W) writer structure"]
impl crate::Writable for EpwmRegsEtflgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_ETFLG to value 0"]
impl crate::Resettable for EpwmRegsEtflgSpec {
    const RESET_VALUE: u16 = 0;
}
