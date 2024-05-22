#[doc = "Register `EPWM_REGS_PCCTL` reader"]
pub type R = crate::R<EpwmRegsPcctlSpec>;
#[doc = "Register `EPWM_REGS_PCCTL` writer"]
pub type W = crate::W<EpwmRegsPcctlSpec>;
#[doc = "Field `CHPEN` reader - 0:0\\]
PWM-chopping Enable"]
pub type ChpenR = crate::BitReader;
#[doc = "Field `CHPEN` writer - 0:0\\]
PWM-chopping Enable"]
pub type ChpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSHTWTH` reader - 4:1\\]
One-Shot Pulse Width"]
pub type OshtwthR = crate::FieldReader;
#[doc = "Field `OSHTWTH` writer - 4:1\\]
One-Shot Pulse Width"]
pub type OshtwthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHPFREQ` reader - 7:5\\]
Chopping Clock Frequency"]
pub type ChpfreqR = crate::FieldReader;
#[doc = "Field `CHPFREQ` writer - 7:5\\]
Chopping Clock Frequency"]
pub type ChpfreqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CHPDUTY` reader - 10:8\\]
Chopping Clock Duty Cycle"]
pub type ChpdutyR = crate::FieldReader;
#[doc = "Field `CHPDUTY` writer - 10:8\\]
Chopping Clock Duty Cycle"]
pub type ChpdutyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
PWM-chopping Enable"]
    #[inline(always)]
    pub fn chpen(&self) -> ChpenR {
        ChpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
One-Shot Pulse Width"]
    #[inline(always)]
    pub fn oshtwth(&self) -> OshtwthR {
        OshtwthR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Chopping Clock Frequency"]
    #[inline(always)]
    pub fn chpfreq(&self) -> ChpfreqR {
        ChpfreqR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Chopping Clock Duty Cycle"]
    #[inline(always)]
    pub fn chpduty(&self) -> ChpdutyR {
        ChpdutyR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
PWM-chopping Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chpen(&mut self) -> ChpenW<EpwmRegsPcctlSpec> {
        ChpenW::new(self, 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
One-Shot Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn oshtwth(&mut self) -> OshtwthW<EpwmRegsPcctlSpec> {
        OshtwthW::new(self, 1)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Chopping Clock Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn chpfreq(&mut self) -> ChpfreqW<EpwmRegsPcctlSpec> {
        ChpfreqW::new(self, 5)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Chopping Clock Duty Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn chpduty(&mut self) -> ChpdutyW<EpwmRegsPcctlSpec> {
        ChpdutyW::new(self, 8)
    }
}
#[doc = "PWM Chopper Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_pcctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_pcctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsPcctlSpec;
impl crate::RegisterSpec for EpwmRegsPcctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_pcctl::R`](R) reader structure"]
impl crate::Readable for EpwmRegsPcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_pcctl::W`](W) writer structure"]
impl crate::Writable for EpwmRegsPcctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_PCCTL to value 0"]
impl crate::Resettable for EpwmRegsPcctlSpec {
    const RESET_VALUE: u16 = 0;
}
