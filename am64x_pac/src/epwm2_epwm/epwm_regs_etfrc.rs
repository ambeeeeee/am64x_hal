#[doc = "Register `EPWM_REGS_ETFRC` reader"]
pub type R = crate::R<EpwmRegsEtfrcSpec>;
#[doc = "Register `EPWM_REGS_ETFRC` writer"]
pub type W = crate::W<EpwmRegsEtfrcSpec>;
#[doc = "Field `INT` reader - 0:0\\]
INT Force Bit The interrupt will only be generated if the event is enabled in the ETSEL register The INT flag bit will be set regardless"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - 0:0\\]
INT Force Bit The interrupt will only be generated if the event is enabled in the ETSEL register The INT flag bit will be set regardless"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
INT Force Bit The interrupt will only be generated if the event is enabled in the ETSEL register The INT flag bit will be set regardless"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
INT Force Bit The interrupt will only be generated if the event is enabled in the ETSEL register The INT flag bit will be set regardless"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<EpwmRegsEtfrcSpec> {
        IntW::new(self, 0)
    }
}
#[doc = "Event Trigger Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_etfrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_etfrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsEtfrcSpec;
impl crate::RegisterSpec for EpwmRegsEtfrcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_etfrc::R`](R) reader structure"]
impl crate::Readable for EpwmRegsEtfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_etfrc::W`](W) writer structure"]
impl crate::Writable for EpwmRegsEtfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_ETFRC to value 0"]
impl crate::Resettable for EpwmRegsEtfrcSpec {
    const RESET_VALUE: u16 = 0;
}
