#[doc = "Register `CFG_TSICR` reader"]
pub type R = crate::R<CfgTsicrSpec>;
#[doc = "Register `CFG_TSICR` writer"]
pub type W = crate::W<CfgTsicrSpec>;
#[doc = "Field `SFT` reader - 1:1\\]
This bit reset all the functional part of teh module"]
pub type SftR = crate::BitReader;
#[doc = "Field `SFT` writer - 1:1\\]
This bit reset all the functional part of teh module"]
pub type SftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSTED` reader - 2:2\\]
Reset value of POSTED depends on hardware integration module at design time. Software must read POSTED field to get the hardwar module configuration"]
pub type PostedR = crate::BitReader;
#[doc = "Field `POSTED` writer - 2:2\\]
Reset value of POSTED depends on hardware integration module at design time. Software must read POSTED field to get the hardwar module configuration"]
pub type PostedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_MODE` reader - 3:3\\]
Select posted/non-posted mode for read operation. This bit is not used when configured in posted mode"]
pub type ReadModeR = crate::BitReader;
#[doc = "Field `READ_MODE` writer - 3:3\\]
Select posted/non-posted mode for read operation. This bit is not used when configured in posted mode"]
pub type ReadModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_AFTER_IDLE` reader - 4:4\\]
Select if the synchronization mechanism used for first TCRR read operation after IDLE state is active"]
pub type ReadAfterIdleR = crate::BitReader;
#[doc = "Field `READ_AFTER_IDLE` writer - 4:4\\]
Select if the synchronization mechanism used for first TCRR read operation after IDLE state is active"]
pub type ReadAfterIdleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
This bit reset all the functional part of teh module"]
    #[inline(always)]
    pub fn sft(&self) -> SftR {
        SftR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Reset value of POSTED depends on hardware integration module at design time. Software must read POSTED field to get the hardwar module configuration"]
    #[inline(always)]
    pub fn posted(&self) -> PostedR {
        PostedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Select posted/non-posted mode for read operation. This bit is not used when configured in posted mode"]
    #[inline(always)]
    pub fn read_mode(&self) -> ReadModeR {
        ReadModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select if the synchronization mechanism used for first TCRR read operation after IDLE state is active"]
    #[inline(always)]
    pub fn read_after_idle(&self) -> ReadAfterIdleR {
        ReadAfterIdleR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
This bit reset all the functional part of teh module"]
    #[inline(always)]
    #[must_use]
    pub fn sft(&mut self) -> SftW<CfgTsicrSpec> {
        SftW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Reset value of POSTED depends on hardware integration module at design time. Software must read POSTED field to get the hardwar module configuration"]
    #[inline(always)]
    #[must_use]
    pub fn posted(&mut self) -> PostedW<CfgTsicrSpec> {
        PostedW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Select posted/non-posted mode for read operation. This bit is not used when configured in posted mode"]
    #[inline(always)]
    #[must_use]
    pub fn read_mode(&mut self) -> ReadModeW<CfgTsicrSpec> {
        ReadModeW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Select if the synchronization mechanism used for first TCRR read operation after IDLE state is active"]
    #[inline(always)]
    #[must_use]
    pub fn read_after_idle(&mut self) -> ReadAfterIdleW<CfgTsicrSpec> {
        ReadAfterIdleW::new(self, 4)
    }
}
#[doc = "Timer Synchronous Interface Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tsicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tsicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTsicrSpec;
impl crate::RegisterSpec for CfgTsicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tsicr::R`](R) reader structure"]
impl crate::Readable for CfgTsicrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tsicr::W`](W) writer structure"]
impl crate::Writable for CfgTsicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TSICR to value 0"]
impl crate::Resettable for CfgTsicrSpec {
    const RESET_VALUE: u32 = 0;
}
