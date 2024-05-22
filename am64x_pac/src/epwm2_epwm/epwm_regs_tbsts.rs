#[doc = "Register `EPWM_REGS_TBSTS` reader"]
pub type R = crate::R<EpwmRegsTbstsSpec>;
#[doc = "Register `EPWM_REGS_TBSTS` writer"]
pub type W = crate::W<EpwmRegsTbstsSpec>;
#[doc = "Field `CTRDIR` reader - 0:0\\]
Time-Base Counter Direction Status Bit At reset, the counter is frozen, therefore, this bit has no meaning To make this bit meaningful, you must first set the appropriate mode via TBCTL\\[CTRMODE\\]"]
pub type CtrdirR = crate::BitReader;
#[doc = "Field `CTRDIR` writer - 0:0\\]
Time-Base Counter Direction Status Bit At reset, the counter is frozen, therefore, this bit has no meaning To make this bit meaningful, you must first set the appropriate mode via TBCTL\\[CTRMODE\\]"]
pub type CtrdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCI` reader - 1:1\\]
Input Synchronization Latched Status Bit"]
pub type SynciR = crate::BitReader;
#[doc = "Field `SYNCI` writer - 1:1\\]
Input Synchronization Latched Status Bit"]
pub type SynciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRMAX` reader - 2:2\\]
Time-Base Counter Max Latched Status Bit"]
pub type CtrmaxR = crate::BitReader;
#[doc = "Field `CTRMAX` writer - 2:2\\]
Time-Base Counter Max Latched Status Bit"]
pub type CtrmaxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Time-Base Counter Direction Status Bit At reset, the counter is frozen, therefore, this bit has no meaning To make this bit meaningful, you must first set the appropriate mode via TBCTL\\[CTRMODE\\]"]
    #[inline(always)]
    pub fn ctrdir(&self) -> CtrdirR {
        CtrdirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Input Synchronization Latched Status Bit"]
    #[inline(always)]
    pub fn synci(&self) -> SynciR {
        SynciR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Time-Base Counter Max Latched Status Bit"]
    #[inline(always)]
    pub fn ctrmax(&self) -> CtrmaxR {
        CtrmaxR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Time-Base Counter Direction Status Bit At reset, the counter is frozen, therefore, this bit has no meaning To make this bit meaningful, you must first set the appropriate mode via TBCTL\\[CTRMODE\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ctrdir(&mut self) -> CtrdirW<EpwmRegsTbstsSpec> {
        CtrdirW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Input Synchronization Latched Status Bit"]
    #[inline(always)]
    #[must_use]
    pub fn synci(&mut self) -> SynciW<EpwmRegsTbstsSpec> {
        SynciW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Time-Base Counter Max Latched Status Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ctrmax(&mut self) -> CtrmaxW<EpwmRegsTbstsSpec> {
        CtrmaxW::new(self, 2)
    }
}
#[doc = "Time-Base Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tbsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tbsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsTbstsSpec;
impl crate::RegisterSpec for EpwmRegsTbstsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_tbsts::R`](R) reader structure"]
impl crate::Readable for EpwmRegsTbstsSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_tbsts::W`](W) writer structure"]
impl crate::Writable for EpwmRegsTbstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_TBSTS to value 0"]
impl crate::Resettable for EpwmRegsTbstsSpec {
    const RESET_VALUE: u16 = 0;
}
