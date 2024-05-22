#[doc = "Register `EPWM_REGS_TZFLG` reader"]
pub type R = crate::R<EpwmRegsTzflgSpec>;
#[doc = "Register `EPWM_REGS_TZFLG` writer"]
pub type W = crate::W<EpwmRegsTzflgSpec>;
#[doc = "Field `INT` reader - 0:0\\]
Latched Trip Interrupt Status Flag"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - 0:0\\]
Latched Trip Interrupt Status Flag"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC` reader - 1:1\\]
Latched Status Flag for Cycle-By-Cycle Trip Event"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - 1:1\\]
Latched Status Flag for Cycle-By-Cycle Trip Event"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` reader - 2:2\\]
Latched Status Flag for A One-Shot Trip Event"]
pub type OstR = crate::BitReader;
#[doc = "Field `OST` writer - 2:2\\]
Latched Status Flag for A One-Shot Trip Event"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Latched Trip Interrupt Status Flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Latched Status Flag for Cycle-By-Cycle Trip Event"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Latched Status Flag for A One-Shot Trip Event"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Latched Trip Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<EpwmRegsTzflgSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Latched Status Flag for Cycle-By-Cycle Trip Event"]
    #[inline(always)]
    #[must_use]
    pub fn cbc(&mut self) -> CbcW<EpwmRegsTzflgSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Latched Status Flag for A One-Shot Trip Event"]
    #[inline(always)]
    #[must_use]
    pub fn ost(&mut self) -> OstW<EpwmRegsTzflgSpec> {
        OstW::new(self, 2)
    }
}
#[doc = "Trip Zone Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tzflg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tzflg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsTzflgSpec;
impl crate::RegisterSpec for EpwmRegsTzflgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_tzflg::R`](R) reader structure"]
impl crate::Readable for EpwmRegsTzflgSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_tzflg::W`](W) writer structure"]
impl crate::Writable for EpwmRegsTzflgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_TZFLG to value 0"]
impl crate::Resettable for EpwmRegsTzflgSpec {
    const RESET_VALUE: u16 = 0;
}
