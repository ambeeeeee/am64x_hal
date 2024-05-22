#[doc = "Register `EPWM_REGS_CMPCTL` reader"]
pub type R = crate::R<EpwmRegsCmpctlSpec>;
#[doc = "Register `EPWM_REGS_CMPCTL` writer"]
pub type W = crate::W<EpwmRegsCmpctlSpec>;
#[doc = "Field `LOADAMODE` reader - 1:0\\]
Active Counter-Compare A \\[CMPA\\]
Load From Shadow Select Mode This bit has no effect in immediate mode \\[CMPCTL\\[SHDWAMODE\\]
= 1\\]"]
pub type LoadamodeR = crate::FieldReader;
#[doc = "Field `LOADAMODE` writer - 1:0\\]
Active Counter-Compare A \\[CMPA\\]
Load From Shadow Select Mode This bit has no effect in immediate mode \\[CMPCTL\\[SHDWAMODE\\]
= 1\\]"]
pub type LoadamodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LOADBMODE` reader - 3:2\\]
Active Counter-Compare B \\[CMPB\\]
Load From Shadow Select Mode This bit has no effect in immediate mode \\[CMPCTL\\[SHDWBMODE\\]
= 1\\]"]
pub type LoadbmodeR = crate::FieldReader;
#[doc = "Field `LOADBMODE` writer - 3:2\\]
Active Counter-Compare B \\[CMPB\\]
Load From Shadow Select Mode This bit has no effect in immediate mode \\[CMPCTL\\[SHDWBMODE\\]
= 1\\]"]
pub type LoadbmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SHDWAMODE` reader - 4:4\\]
Counter-compare A \\[CMPA\\]
Register Operating Mode"]
pub type ShdwamodeR = crate::BitReader;
#[doc = "Field `SHDWAMODE` writer - 4:4\\]
Counter-compare A \\[CMPA\\]
Register Operating Mode"]
pub type ShdwamodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHDWBMODE` reader - 6:6\\]
Counter-compare B \\[CMPB\\]
Register Operating Mode"]
pub type ShdwbmodeR = crate::BitReader;
#[doc = "Field `SHDWBMODE` writer - 6:6\\]
Counter-compare B \\[CMPB\\]
Register Operating Mode"]
pub type ShdwbmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHDWAFULL` reader - 8:8\\]
Counter-compare A \\[CMPA\\]
Shadow Register Full Status Flag The flag bit is set when a 32 bit write to CMPA:CMPAHR register or a 16 bit write to CMPA register is made A 16 bit write to CMPAHR register will not affect the flag This bit self clears once a load-strobe occurs"]
pub type ShdwafullR = crate::BitReader;
#[doc = "Field `SHDWAFULL` writer - 8:8\\]
Counter-compare A \\[CMPA\\]
Shadow Register Full Status Flag The flag bit is set when a 32 bit write to CMPA:CMPAHR register or a 16 bit write to CMPA register is made A 16 bit write to CMPAHR register will not affect the flag This bit self clears once a load-strobe occurs"]
pub type ShdwafullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHDWBFULL` reader - 9:9\\]
Counter-compare B \\[CMPB\\]
Shadow Register Full Status Flag This bit self clears once a load-strobe occurs"]
pub type ShdwbfullR = crate::BitReader;
#[doc = "Field `SHDWBFULL` writer - 9:9\\]
Counter-compare B \\[CMPB\\]
Shadow Register Full Status Flag This bit self clears once a load-strobe occurs"]
pub type ShdwbfullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Active Counter-Compare A \\[CMPA\\]
Load From Shadow Select Mode This bit has no effect in immediate mode \\[CMPCTL\\[SHDWAMODE\\]
= 1\\]"]
    #[inline(always)]
    pub fn loadamode(&self) -> LoadamodeR {
        LoadamodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Active Counter-Compare B \\[CMPB\\]
Load From Shadow Select Mode This bit has no effect in immediate mode \\[CMPCTL\\[SHDWBMODE\\]
= 1\\]"]
    #[inline(always)]
    pub fn loadbmode(&self) -> LoadbmodeR {
        LoadbmodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Counter-compare A \\[CMPA\\]
Register Operating Mode"]
    #[inline(always)]
    pub fn shdwamode(&self) -> ShdwamodeR {
        ShdwamodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Counter-compare B \\[CMPB\\]
Register Operating Mode"]
    #[inline(always)]
    pub fn shdwbmode(&self) -> ShdwbmodeR {
        ShdwbmodeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Counter-compare A \\[CMPA\\]
Shadow Register Full Status Flag The flag bit is set when a 32 bit write to CMPA:CMPAHR register or a 16 bit write to CMPA register is made A 16 bit write to CMPAHR register will not affect the flag This bit self clears once a load-strobe occurs"]
    #[inline(always)]
    pub fn shdwafull(&self) -> ShdwafullR {
        ShdwafullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Counter-compare B \\[CMPB\\]
Shadow Register Full Status Flag This bit self clears once a load-strobe occurs"]
    #[inline(always)]
    pub fn shdwbfull(&self) -> ShdwbfullR {
        ShdwbfullR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Active Counter-Compare A \\[CMPA\\]
Load From Shadow Select Mode This bit has no effect in immediate mode \\[CMPCTL\\[SHDWAMODE\\]
= 1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn loadamode(&mut self) -> LoadamodeW<EpwmRegsCmpctlSpec> {
        LoadamodeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Active Counter-Compare B \\[CMPB\\]
Load From Shadow Select Mode This bit has no effect in immediate mode \\[CMPCTL\\[SHDWBMODE\\]
= 1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn loadbmode(&mut self) -> LoadbmodeW<EpwmRegsCmpctlSpec> {
        LoadbmodeW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
Counter-compare A \\[CMPA\\]
Register Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn shdwamode(&mut self) -> ShdwamodeW<EpwmRegsCmpctlSpec> {
        ShdwamodeW::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
Counter-compare B \\[CMPB\\]
Register Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn shdwbmode(&mut self) -> ShdwbmodeW<EpwmRegsCmpctlSpec> {
        ShdwbmodeW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Counter-compare A \\[CMPA\\]
Shadow Register Full Status Flag The flag bit is set when a 32 bit write to CMPA:CMPAHR register or a 16 bit write to CMPA register is made A 16 bit write to CMPAHR register will not affect the flag This bit self clears once a load-strobe occurs"]
    #[inline(always)]
    #[must_use]
    pub fn shdwafull(&mut self) -> ShdwafullW<EpwmRegsCmpctlSpec> {
        ShdwafullW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Counter-compare B \\[CMPB\\]
Shadow Register Full Status Flag This bit self clears once a load-strobe occurs"]
    #[inline(always)]
    #[must_use]
    pub fn shdwbfull(&mut self) -> ShdwbfullW<EpwmRegsCmpctlSpec> {
        ShdwbfullW::new(self, 9)
    }
}
#[doc = "Counter Compare Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_cmpctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_cmpctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsCmpctlSpec;
impl crate::RegisterSpec for EpwmRegsCmpctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_cmpctl::R`](R) reader structure"]
impl crate::Readable for EpwmRegsCmpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_cmpctl::W`](W) writer structure"]
impl crate::Writable for EpwmRegsCmpctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_CMPCTL to value 0"]
impl crate::Resettable for EpwmRegsCmpctlSpec {
    const RESET_VALUE: u16 = 0;
}
