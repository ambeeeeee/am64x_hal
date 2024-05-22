#[doc = "Register `EPWM_REGS_ETCLR` reader"]
pub type R = crate::R<EpwmRegsEtclrSpec>;
#[doc = "Register `EPWM_REGS_ETCLR` writer"]
pub type W = crate::W<EpwmRegsEtclrSpec>;
#[doc = "Field `INT` reader - 0:0\\]
ePWM Interrupt \\[EPWMx_INT\\]
Flag Clear Bit"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - 0:0\\]
ePWM Interrupt \\[EPWMx_INT\\]
Flag Clear Bit"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ePWM Interrupt \\[EPWMx_INT\\]
Flag Clear Bit"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ePWM Interrupt \\[EPWMx_INT\\]
Flag Clear Bit"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<EpwmRegsEtclrSpec> {
        IntW::new(self, 0)
    }
}
#[doc = "Event Trigger Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_etclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_etclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsEtclrSpec;
impl crate::RegisterSpec for EpwmRegsEtclrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_etclr::R`](R) reader structure"]
impl crate::Readable for EpwmRegsEtclrSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_etclr::W`](W) writer structure"]
impl crate::Writable for EpwmRegsEtclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_ETCLR to value 0"]
impl crate::Resettable for EpwmRegsEtclrSpec {
    const RESET_VALUE: u16 = 0;
}
