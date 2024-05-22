#[doc = "Register `CFG_GPMC_IRQSTATUS` reader"]
pub type R = crate::R<CfgGpmcIrqstatusSpec>;
#[doc = "Register `CFG_GPMC_IRQSTATUS` writer"]
pub type W = crate::W<CfgGpmcIrqstatusSpec>;
#[doc = "Field `FIFOEVENTSTATUS` reader - 0:0\\]
Status of the FIFOEvent interrupt"]
pub type FifoeventstatusR = crate::BitReader;
#[doc = "Field `FIFOEVENTSTATUS` writer - 0:0\\]
Status of the FIFOEvent interrupt"]
pub type FifoeventstatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERMINALCOUNTSTATUS` reader - 1:1\\]
Status of the TerminalCountEvent interrupt"]
pub type TerminalcountstatusR = crate::BitReader;
#[doc = "Field `TERMINALCOUNTSTATUS` writer - 1:1\\]
Status of the TerminalCountEvent interrupt"]
pub type TerminalcountstatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT0EDGEDETECTIONSTATUS` reader - 8:8\\]
Status of the Wait0 Edge Detection interrupt"]
pub type Wait0edgedetectionstatusR = crate::BitReader;
#[doc = "Field `WAIT0EDGEDETECTIONSTATUS` writer - 8:8\\]
Status of the Wait0 Edge Detection interrupt"]
pub type Wait0edgedetectionstatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT1EDGEDETECTIONSTATUS` reader - 9:9\\]
Status of the Wait1 Edge Detection interrupt"]
pub type Wait1edgedetectionstatusR = crate::BitReader;
#[doc = "Field `WAIT1EDGEDETECTIONSTATUS` writer - 9:9\\]
Status of the Wait1 Edge Detection interrupt"]
pub type Wait1edgedetectionstatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT2EDGEDETECTIONSTATUS` reader - 10:10\\]
Status of the Wait2 Edge Detection interrupt"]
pub type Wait2edgedetectionstatusR = crate::BitReader;
#[doc = "Field `WAIT2EDGEDETECTIONSTATUS` writer - 10:10\\]
Status of the Wait2 Edge Detection interrupt"]
pub type Wait2edgedetectionstatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT3EDGEDETECTIONSTATUS` reader - 11:11\\]
Status of the Wait3 Edge Detection interrupt"]
pub type Wait3edgedetectionstatusR = crate::BitReader;
#[doc = "Field `WAIT3EDGEDETECTIONSTATUS` writer - 11:11\\]
Status of the Wait3 Edge Detection interrupt"]
pub type Wait3edgedetectionstatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status of the FIFOEvent interrupt"]
    #[inline(always)]
    pub fn fifoeventstatus(&self) -> FifoeventstatusR {
        FifoeventstatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of the TerminalCountEvent interrupt"]
    #[inline(always)]
    pub fn terminalcountstatus(&self) -> TerminalcountstatusR {
        TerminalcountstatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Status of the Wait0 Edge Detection interrupt"]
    #[inline(always)]
    pub fn wait0edgedetectionstatus(&self) -> Wait0edgedetectionstatusR {
        Wait0edgedetectionstatusR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Status of the Wait1 Edge Detection interrupt"]
    #[inline(always)]
    pub fn wait1edgedetectionstatus(&self) -> Wait1edgedetectionstatusR {
        Wait1edgedetectionstatusR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Status of the Wait2 Edge Detection interrupt"]
    #[inline(always)]
    pub fn wait2edgedetectionstatus(&self) -> Wait2edgedetectionstatusR {
        Wait2edgedetectionstatusR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Status of the Wait3 Edge Detection interrupt"]
    #[inline(always)]
    pub fn wait3edgedetectionstatus(&self) -> Wait3edgedetectionstatusR {
        Wait3edgedetectionstatusR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status of the FIFOEvent interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fifoeventstatus(&mut self) -> FifoeventstatusW<CfgGpmcIrqstatusSpec> {
        FifoeventstatusW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of the TerminalCountEvent interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn terminalcountstatus(&mut self) -> TerminalcountstatusW<CfgGpmcIrqstatusSpec> {
        TerminalcountstatusW::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Status of the Wait0 Edge Detection interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wait0edgedetectionstatus(&mut self) -> Wait0edgedetectionstatusW<CfgGpmcIrqstatusSpec> {
        Wait0edgedetectionstatusW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Status of the Wait1 Edge Detection interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wait1edgedetectionstatus(&mut self) -> Wait1edgedetectionstatusW<CfgGpmcIrqstatusSpec> {
        Wait1edgedetectionstatusW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Status of the Wait2 Edge Detection interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wait2edgedetectionstatus(&mut self) -> Wait2edgedetectionstatusW<CfgGpmcIrqstatusSpec> {
        Wait2edgedetectionstatusW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Status of the Wait3 Edge Detection interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wait3edgedetectionstatus(&mut self) -> Wait3edgedetectionstatusW<CfgGpmcIrqstatusSpec> {
        Wait3edgedetectionstatusW::new(self, 11)
    }
}
#[doc = "This interrupt status register regroups all the status of the module internal events that can generate an interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_irqstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_irqstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcIrqstatusSpec;
impl crate::RegisterSpec for CfgGpmcIrqstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_irqstatus::R`](R) reader structure"]
impl crate::Readable for CfgGpmcIrqstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_irqstatus::W`](W) writer structure"]
impl crate::Writable for CfgGpmcIrqstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_IRQSTATUS to value 0"]
impl crate::Resettable for CfgGpmcIrqstatusSpec {
    const RESET_VALUE: u32 = 0;
}
