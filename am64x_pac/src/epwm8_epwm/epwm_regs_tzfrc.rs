#[doc = "Register `EPWM_REGS_TZFRC` reader"]
pub type R = crate::R<EpwmRegsTzfrcSpec>;
#[doc = "Register `EPWM_REGS_TZFRC` writer"]
pub type W = crate::W<EpwmRegsTzfrcSpec>;
#[doc = "Field `CBC` reader - 1:1\\]
Force a Cycle-by-Cycle Trip Event via Software"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - 1:1\\]
Force a Cycle-by-Cycle Trip Event via Software"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` reader - 2:2\\]
Force a One-Shot Trip Event via Software"]
pub type OstR = crate::BitReader;
#[doc = "Field `OST` writer - 2:2\\]
Force a One-Shot Trip Event via Software"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
Force a Cycle-by-Cycle Trip Event via Software"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Force a One-Shot Trip Event via Software"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Force a Cycle-by-Cycle Trip Event via Software"]
    #[inline(always)]
    #[must_use]
    pub fn cbc(&mut self) -> CbcW<EpwmRegsTzfrcSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Force a One-Shot Trip Event via Software"]
    #[inline(always)]
    #[must_use]
    pub fn ost(&mut self) -> OstW<EpwmRegsTzfrcSpec> {
        OstW::new(self, 2)
    }
}
#[doc = "Trip Zone Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tzfrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tzfrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsTzfrcSpec;
impl crate::RegisterSpec for EpwmRegsTzfrcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_tzfrc::R`](R) reader structure"]
impl crate::Readable for EpwmRegsTzfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_tzfrc::W`](W) writer structure"]
impl crate::Writable for EpwmRegsTzfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_TZFRC to value 0"]
impl crate::Resettable for EpwmRegsTzfrcSpec {
    const RESET_VALUE: u16 = 0;
}
