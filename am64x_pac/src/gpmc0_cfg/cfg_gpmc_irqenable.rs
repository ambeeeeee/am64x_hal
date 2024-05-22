#[doc = "Register `CFG_GPMC_IRQENABLE` reader"]
pub type R = crate::R<CfgGpmcIrqenableSpec>;
#[doc = "Register `CFG_GPMC_IRQENABLE` writer"]
pub type W = crate::W<CfgGpmcIrqenableSpec>;
#[doc = "Field `FIFOEVENTENABLE` reader - 0:0\\]
Enables the FIFOEvent interrupt"]
pub type FifoeventenableR = crate::BitReader;
#[doc = "Field `FIFOEVENTENABLE` writer - 0:0\\]
Enables the FIFOEvent interrupt"]
pub type FifoeventenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERMINALCOUNTEVENTENABLE` reader - 1:1\\]
Enables TerminalCountEvent interrupt issuing in pre-fetch or write posting mode"]
pub type TerminalcounteventenableR = crate::BitReader;
#[doc = "Field `TERMINALCOUNTEVENTENABLE` writer - 1:1\\]
Enables TerminalCountEvent interrupt issuing in pre-fetch or write posting mode"]
pub type TerminalcounteventenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT0EDGEDETECTIONENABLE` reader - 8:8\\]
Enables the Wait0 Edge Detection interrupt"]
pub type Wait0edgedetectionenableR = crate::BitReader;
#[doc = "Field `WAIT0EDGEDETECTIONENABLE` writer - 8:8\\]
Enables the Wait0 Edge Detection interrupt"]
pub type Wait0edgedetectionenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT1EDGEDETECTIONENABLE` reader - 9:9\\]
Enables the Wait1 Edge Detection interrupt"]
pub type Wait1edgedetectionenableR = crate::BitReader;
#[doc = "Field `WAIT1EDGEDETECTIONENABLE` writer - 9:9\\]
Enables the Wait1 Edge Detection interrupt"]
pub type Wait1edgedetectionenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT2EDGEDETECTIONENABLE` reader - 10:10\\]
Enables the Wait2 Edge Detection interrupt"]
pub type Wait2edgedetectionenableR = crate::BitReader;
#[doc = "Field `WAIT2EDGEDETECTIONENABLE` writer - 10:10\\]
Enables the Wait2 Edge Detection interrupt"]
pub type Wait2edgedetectionenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT3EDGEDETECTIONENABLE` reader - 11:11\\]
Enables the Wait3 Edge Detection interrupt"]
pub type Wait3edgedetectionenableR = crate::BitReader;
#[doc = "Field `WAIT3EDGEDETECTIONENABLE` writer - 11:11\\]
Enables the Wait3 Edge Detection interrupt"]
pub type Wait3edgedetectionenableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the FIFOEvent interrupt"]
    #[inline(always)]
    pub fn fifoeventenable(&self) -> FifoeventenableR {
        FifoeventenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables TerminalCountEvent interrupt issuing in pre-fetch or write posting mode"]
    #[inline(always)]
    pub fn terminalcounteventenable(&self) -> TerminalcounteventenableR {
        TerminalcounteventenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables the Wait0 Edge Detection interrupt"]
    #[inline(always)]
    pub fn wait0edgedetectionenable(&self) -> Wait0edgedetectionenableR {
        Wait0edgedetectionenableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enables the Wait1 Edge Detection interrupt"]
    #[inline(always)]
    pub fn wait1edgedetectionenable(&self) -> Wait1edgedetectionenableR {
        Wait1edgedetectionenableR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enables the Wait2 Edge Detection interrupt"]
    #[inline(always)]
    pub fn wait2edgedetectionenable(&self) -> Wait2edgedetectionenableR {
        Wait2edgedetectionenableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Enables the Wait3 Edge Detection interrupt"]
    #[inline(always)]
    pub fn wait3edgedetectionenable(&self) -> Wait3edgedetectionenableR {
        Wait3edgedetectionenableR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the FIFOEvent interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fifoeventenable(&mut self) -> FifoeventenableW<CfgGpmcIrqenableSpec> {
        FifoeventenableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables TerminalCountEvent interrupt issuing in pre-fetch or write posting mode"]
    #[inline(always)]
    #[must_use]
    pub fn terminalcounteventenable(&mut self) -> TerminalcounteventenableW<CfgGpmcIrqenableSpec> {
        TerminalcounteventenableW::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables the Wait0 Edge Detection interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wait0edgedetectionenable(&mut self) -> Wait0edgedetectionenableW<CfgGpmcIrqenableSpec> {
        Wait0edgedetectionenableW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enables the Wait1 Edge Detection interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wait1edgedetectionenable(&mut self) -> Wait1edgedetectionenableW<CfgGpmcIrqenableSpec> {
        Wait1edgedetectionenableW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Enables the Wait2 Edge Detection interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wait2edgedetectionenable(&mut self) -> Wait2edgedetectionenableW<CfgGpmcIrqenableSpec> {
        Wait2edgedetectionenableW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Enables the Wait3 Edge Detection interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wait3edgedetectionenable(&mut self) -> Wait3edgedetectionenableW<CfgGpmcIrqenableSpec> {
        Wait3edgedetectionenableW::new(self, 11)
    }
}
#[doc = "The interrupt enable register allows to mask/unmask the module internal sources of interrupt, on a event-by-event basis.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_irqenable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_irqenable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcIrqenableSpec;
impl crate::RegisterSpec for CfgGpmcIrqenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_irqenable::R`](R) reader structure"]
impl crate::Readable for CfgGpmcIrqenableSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_irqenable::W`](W) writer structure"]
impl crate::Writable for CfgGpmcIrqenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_IRQENABLE to value 0"]
impl crate::Resettable for CfgGpmcIrqenableSpec {
    const RESET_VALUE: u32 = 0;
}
