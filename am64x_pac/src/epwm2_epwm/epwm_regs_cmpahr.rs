#[doc = "Register `EPWM_REGS_CMPAHR` reader"]
pub type R = crate::R<EpwmRegsCmpahrSpec>;
#[doc = "Register `EPWM_REGS_CMPAHR` writer"]
pub type W = crate::W<EpwmRegsCmpahrSpec>;
#[doc = "Field `CMPAHR` reader - 15:8\\]
Compare A High-Resolution register bits for MEP step control A minimum value of 1h is needed to enable HRPWM capabilities Valid MEP range of operation 1-255h"]
pub type CmpahrR = crate::FieldReader;
#[doc = "Field `CMPAHR` writer - 15:8\\]
Compare A High-Resolution register bits for MEP step control A minimum value of 1h is needed to enable HRPWM capabilities Valid MEP range of operation 1-255h"]
pub type CmpahrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - 15:8\\]
Compare A High-Resolution register bits for MEP step control A minimum value of 1h is needed to enable HRPWM capabilities Valid MEP range of operation 1-255h"]
    #[inline(always)]
    pub fn cmpahr(&self) -> CmpahrR {
        CmpahrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15:8\\]
Compare A High-Resolution register bits for MEP step control A minimum value of 1h is needed to enable HRPWM capabilities Valid MEP range of operation 1-255h"]
    #[inline(always)]
    #[must_use]
    pub fn cmpahr(&mut self) -> CmpahrW<EpwmRegsCmpahrSpec> {
        CmpahrW::new(self, 8)
    }
}
#[doc = "Counter Compare A High Resolution Register. This register is only available on ePWM instances that include the high-resolution PWM (HRPWM) extension; otherwise, this location is reserved.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_cmpahr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_cmpahr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsCmpahrSpec;
impl crate::RegisterSpec for EpwmRegsCmpahrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_cmpahr::R`](R) reader structure"]
impl crate::Readable for EpwmRegsCmpahrSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_cmpahr::W`](W) writer structure"]
impl crate::Writable for EpwmRegsCmpahrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_CMPAHR to value 0"]
impl crate::Resettable for EpwmRegsCmpahrSpec {
    const RESET_VALUE: u16 = 0;
}
