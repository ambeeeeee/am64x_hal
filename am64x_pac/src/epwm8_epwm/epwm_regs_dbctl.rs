#[doc = "Register `EPWM_REGS_DBCTL` reader"]
pub type R = crate::R<EpwmRegsDbctlSpec>;
#[doc = "Register `EPWM_REGS_DBCTL` writer"]
pub type W = crate::W<EpwmRegsDbctlSpec>;
#[doc = "Field `OUT_MODE` reader - 1:0\\]
Dead-band Output Mode Control Bit 1 controls the S1 switch and bit 0 controls the S0 switch This allows you to selectively enable or bypass the dead-band generation for the falling-edge and rising-edge delay"]
pub type OutModeR = crate::FieldReader;
#[doc = "Field `OUT_MODE` writer - 1:0\\]
Dead-band Output Mode Control Bit 1 controls the S1 switch and bit 0 controls the S0 switch This allows you to selectively enable or bypass the dead-band generation for the falling-edge and rising-edge delay"]
pub type OutModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `POLSEL` reader - 3:2\\]
Polarity Select Control Bit 3 controls the S3 switch and bit 2 controls the S2 switch This allows you to selectively invert one of the delayed signals before it is sent out of the dead-band submodule The following descriptions correspond to classical upper/lower switch control as found in one leg of a digital motor control inverter These assume that DBCTL\\[OUT_MODE\\]
= 1,1 and DBCTL\\[IN_MODE\\]
= 0,0 Other enhanced modes are also possible, but not regarded as typical usage modes"]
pub type PolselR = crate::FieldReader;
#[doc = "Field `POLSEL` writer - 3:2\\]
Polarity Select Control Bit 3 controls the S3 switch and bit 2 controls the S2 switch This allows you to selectively invert one of the delayed signals before it is sent out of the dead-band submodule The following descriptions correspond to classical upper/lower switch control as found in one leg of a digital motor control inverter These assume that DBCTL\\[OUT_MODE\\]
= 1,1 and DBCTL\\[IN_MODE\\]
= 0,0 Other enhanced modes are also possible, but not regarded as typical usage modes"]
pub type PolselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IN_MODE` reader - 5:4\\]
Dead Band Input Mode Control Bit 5 controls the S5 switch and bit 4 controls the S4 switch This allows you to select the input source to the falling-edge and rising-edge delay To produce classical dead-band waveforms, the default is EPWMxA In is the source for both falling and rising-edge delays"]
pub type InModeR = crate::FieldReader;
#[doc = "Field `IN_MODE` writer - 5:4\\]
Dead Band Input Mode Control Bit 5 controls the S5 switch and bit 4 controls the S4 switch This allows you to select the input source to the falling-edge and rising-edge delay To produce classical dead-band waveforms, the default is EPWMxA In is the source for both falling and rising-edge delays"]
pub type InModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Dead-band Output Mode Control Bit 1 controls the S1 switch and bit 0 controls the S0 switch This allows you to selectively enable or bypass the dead-band generation for the falling-edge and rising-edge delay"]
    #[inline(always)]
    pub fn out_mode(&self) -> OutModeR {
        OutModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Polarity Select Control Bit 3 controls the S3 switch and bit 2 controls the S2 switch This allows you to selectively invert one of the delayed signals before it is sent out of the dead-band submodule The following descriptions correspond to classical upper/lower switch control as found in one leg of a digital motor control inverter These assume that DBCTL\\[OUT_MODE\\]
= 1,1 and DBCTL\\[IN_MODE\\]
= 0,0 Other enhanced modes are also possible, but not regarded as typical usage modes"]
    #[inline(always)]
    pub fn polsel(&self) -> PolselR {
        PolselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Dead Band Input Mode Control Bit 5 controls the S5 switch and bit 4 controls the S4 switch This allows you to select the input source to the falling-edge and rising-edge delay To produce classical dead-band waveforms, the default is EPWMxA In is the source for both falling and rising-edge delays"]
    #[inline(always)]
    pub fn in_mode(&self) -> InModeR {
        InModeR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Dead-band Output Mode Control Bit 1 controls the S1 switch and bit 0 controls the S0 switch This allows you to selectively enable or bypass the dead-band generation for the falling-edge and rising-edge delay"]
    #[inline(always)]
    #[must_use]
    pub fn out_mode(&mut self) -> OutModeW<EpwmRegsDbctlSpec> {
        OutModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Polarity Select Control Bit 3 controls the S3 switch and bit 2 controls the S2 switch This allows you to selectively invert one of the delayed signals before it is sent out of the dead-band submodule The following descriptions correspond to classical upper/lower switch control as found in one leg of a digital motor control inverter These assume that DBCTL\\[OUT_MODE\\]
= 1,1 and DBCTL\\[IN_MODE\\]
= 0,0 Other enhanced modes are also possible, but not regarded as typical usage modes"]
    #[inline(always)]
    #[must_use]
    pub fn polsel(&mut self) -> PolselW<EpwmRegsDbctlSpec> {
        PolselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Dead Band Input Mode Control Bit 5 controls the S5 switch and bit 4 controls the S4 switch This allows you to select the input source to the falling-edge and rising-edge delay To produce classical dead-band waveforms, the default is EPWMxA In is the source for both falling and rising-edge delays"]
    #[inline(always)]
    #[must_use]
    pub fn in_mode(&mut self) -> InModeW<EpwmRegsDbctlSpec> {
        InModeW::new(self, 4)
    }
}
#[doc = "Dead-Band Generator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_dbctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_dbctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsDbctlSpec;
impl crate::RegisterSpec for EpwmRegsDbctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_dbctl::R`](R) reader structure"]
impl crate::Readable for EpwmRegsDbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_dbctl::W`](W) writer structure"]
impl crate::Writable for EpwmRegsDbctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_DBCTL to value 0"]
impl crate::Resettable for EpwmRegsDbctlSpec {
    const RESET_VALUE: u16 = 0;
}
