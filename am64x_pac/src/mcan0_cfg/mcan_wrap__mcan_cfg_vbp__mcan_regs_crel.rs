#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_CREL` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsCrelSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_CREL` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsCrelSpec>;
#[doc = "Field `DAY` reader - 7:0\\]
Time Stamp Day"]
pub type DayR = crate::FieldReader;
#[doc = "Field `DAY` writer - 7:0\\]
Time Stamp Day"]
pub type DayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MON` reader - 15:8\\]
Time Stamp Month"]
pub type MonR = crate::FieldReader;
#[doc = "Field `MON` writer - 15:8\\]
Time Stamp Month"]
pub type MonW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `YEAR` reader - 19:16\\]
Time Stamp Year"]
pub type YearR = crate::FieldReader;
#[doc = "Field `YEAR` writer - 19:16\\]
Time Stamp Year"]
pub type YearW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SUBSTEP` reader - 23:20\\]
Sub-Step of Core Release"]
pub type SubstepR = crate::FieldReader;
#[doc = "Field `SUBSTEP` writer - 23:20\\]
Sub-Step of Core Release"]
pub type SubstepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STEP` reader - 27:24\\]
Step of Core Release"]
pub type StepR = crate::FieldReader;
#[doc = "Field `STEP` writer - 27:24\\]
Step of Core Release"]
pub type StepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REL` reader - 31:28\\]
Core Release"]
pub type RelR = crate::FieldReader;
#[doc = "Field `REL` writer - 31:28\\]
Core Release"]
pub type RelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Time Stamp Day"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Time Stamp Month"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Time Stamp Year"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Sub-Step of Core Release"]
    #[inline(always)]
    pub fn substep(&self) -> SubstepR {
        SubstepR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Step of Core Release"]
    #[inline(always)]
    pub fn step(&self) -> StepR {
        StepR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Core Release"]
    #[inline(always)]
    pub fn rel(&self) -> RelR {
        RelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Time Stamp Day"]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DayW<McanWrap_McanCfgVbp_McanRegsCrelSpec> {
        DayW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Time Stamp Month"]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MonW<McanWrap_McanCfgVbp_McanRegsCrelSpec> {
        MonW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Time Stamp Year"]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YearW<McanWrap_McanCfgVbp_McanRegsCrelSpec> {
        YearW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Sub-Step of Core Release"]
    #[inline(always)]
    #[must_use]
    pub fn substep(&mut self) -> SubstepW<McanWrap_McanCfgVbp_McanRegsCrelSpec> {
        SubstepW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Step of Core Release"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> StepW<McanWrap_McanCfgVbp_McanRegsCrelSpec> {
        StepW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Core Release"]
    #[inline(always)]
    #[must_use]
    pub fn rel(&mut self) -> RelW<McanWrap_McanCfgVbp_McanRegsCrelSpec> {
        RelW::new(self, 28)
    }
}
#[doc = "Release dependent constant (version + date)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_crel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_crel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsCrelSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsCrelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_crel::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsCrelSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_crel::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsCrelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_CREL to value 0x3238_0608"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsCrelSpec {
    const RESET_VALUE: u32 = 0x3238_0608;
}
