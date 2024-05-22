#[doc = "Register `CFG_GPMC_STATUS` reader"]
pub type R = crate::R<CfgGpmcStatusSpec>;
#[doc = "Register `CFG_GPMC_STATUS` writer"]
pub type W = crate::W<CfgGpmcStatusSpec>;
#[doc = "Field `EMPTYWRITEBUFFERSTATUS` reader - 0:0\\]
Stores the empty status of the write buffer"]
pub type EmptywritebufferstatusR = crate::BitReader;
#[doc = "Field `EMPTYWRITEBUFFERSTATUS` writer - 0:0\\]
Stores the empty status of the write buffer"]
pub type EmptywritebufferstatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT0STATUS` reader - 8:8\\]
Is a copy of input pin WAIT0. \\[Reset value is WAIT0 input pin sampled at IC reset\\]"]
pub type Wait0statusR = crate::BitReader;
#[doc = "Field `WAIT0STATUS` writer - 8:8\\]
Is a copy of input pin WAIT0. \\[Reset value is WAIT0 input pin sampled at IC reset\\]"]
pub type Wait0statusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT1STATUS` reader - 9:9\\]
Is a copy of input pin WAIT1. \\[Reset value is WAIT1 input pin sampled at IC reset\\]"]
pub type Wait1statusR = crate::BitReader;
#[doc = "Field `WAIT1STATUS` writer - 9:9\\]
Is a copy of input pin WAIT1. \\[Reset value is WAIT1 input pin sampled at IC reset\\]"]
pub type Wait1statusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT2STATUS` reader - 10:10\\]
Is a copy of input pin WAIT2. \\[Reset value is WAIT2 input pin sampled at IC reset\\]"]
pub type Wait2statusR = crate::BitReader;
#[doc = "Field `WAIT2STATUS` writer - 10:10\\]
Is a copy of input pin WAIT2. \\[Reset value is WAIT2 input pin sampled at IC reset\\]"]
pub type Wait2statusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT3STATUS` reader - 11:11\\]
Is a copy of input pin WAIT3. \\[Reset value is WAIT3 input pin sampled at IC reset\\]"]
pub type Wait3statusR = crate::BitReader;
#[doc = "Field `WAIT3STATUS` writer - 11:11\\]
Is a copy of input pin WAIT3. \\[Reset value is WAIT3 input pin sampled at IC reset\\]"]
pub type Wait3statusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Stores the empty status of the write buffer"]
    #[inline(always)]
    pub fn emptywritebufferstatus(&self) -> EmptywritebufferstatusR {
        EmptywritebufferstatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Is a copy of input pin WAIT0. \\[Reset value is WAIT0 input pin sampled at IC reset\\]"]
    #[inline(always)]
    pub fn wait0status(&self) -> Wait0statusR {
        Wait0statusR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Is a copy of input pin WAIT1. \\[Reset value is WAIT1 input pin sampled at IC reset\\]"]
    #[inline(always)]
    pub fn wait1status(&self) -> Wait1statusR {
        Wait1statusR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Is a copy of input pin WAIT2. \\[Reset value is WAIT2 input pin sampled at IC reset\\]"]
    #[inline(always)]
    pub fn wait2status(&self) -> Wait2statusR {
        Wait2statusR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Is a copy of input pin WAIT3. \\[Reset value is WAIT3 input pin sampled at IC reset\\]"]
    #[inline(always)]
    pub fn wait3status(&self) -> Wait3statusR {
        Wait3statusR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Stores the empty status of the write buffer"]
    #[inline(always)]
    #[must_use]
    pub fn emptywritebufferstatus(&mut self) -> EmptywritebufferstatusW<CfgGpmcStatusSpec> {
        EmptywritebufferstatusW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Is a copy of input pin WAIT0. \\[Reset value is WAIT0 input pin sampled at IC reset\\]"]
    #[inline(always)]
    #[must_use]
    pub fn wait0status(&mut self) -> Wait0statusW<CfgGpmcStatusSpec> {
        Wait0statusW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Is a copy of input pin WAIT1. \\[Reset value is WAIT1 input pin sampled at IC reset\\]"]
    #[inline(always)]
    #[must_use]
    pub fn wait1status(&mut self) -> Wait1statusW<CfgGpmcStatusSpec> {
        Wait1statusW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Is a copy of input pin WAIT2. \\[Reset value is WAIT2 input pin sampled at IC reset\\]"]
    #[inline(always)]
    #[must_use]
    pub fn wait2status(&mut self) -> Wait2statusW<CfgGpmcStatusSpec> {
        Wait2statusW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Is a copy of input pin WAIT3. \\[Reset value is WAIT3 input pin sampled at IC reset\\]"]
    #[inline(always)]
    #[must_use]
    pub fn wait3status(&mut self) -> Wait3statusW<CfgGpmcStatusSpec> {
        Wait3statusW::new(self, 11)
    }
}
#[doc = "The status register provides global status bits of the GPMC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcStatusSpec;
impl crate::RegisterSpec for CfgGpmcStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_status::R`](R) reader structure"]
impl crate::Readable for CfgGpmcStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_status::W`](W) writer structure"]
impl crate::Writable for CfgGpmcStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_STATUS to value 0x01"]
impl crate::Resettable for CfgGpmcStatusSpec {
    const RESET_VALUE: u32 = 0x01;
}
