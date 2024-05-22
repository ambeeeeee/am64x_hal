#[doc = "Register `CFG0_VDD_CORE_GLDTC_STAT` reader"]
pub type R = crate::R<Cfg0VddCoreGldtcStatSpec>;
#[doc = "Register `CFG0_VDD_CORE_GLDTC_STAT` writer"]
pub type W = crate::W<Cfg0VddCoreGldtcStatSpec>;
#[doc = "Field `VDD_CORE_GLDTC_STAT_THRESH_LOW_FLAG` reader - 0:0\\]
Low voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
pub type VddCoreGldtcStatThreshLowFlagR = crate::BitReader;
#[doc = "Field `VDD_CORE_GLDTC_STAT_THRESH_LOW_FLAG` writer - 0:0\\]
Low voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
pub type VddCoreGldtcStatThreshLowFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDD_CORE_GLDTC_STAT_THRESH_HI_FLAG` reader - 8:8\\]
High voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
pub type VddCoreGldtcStatThreshHiFlagR = crate::BitReader;
#[doc = "Field `VDD_CORE_GLDTC_STAT_THRESH_HI_FLAG` writer - 8:8\\]
High voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
pub type VddCoreGldtcStatThreshHiFlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Low voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
    #[inline(always)]
    pub fn vdd_core_gldtc_stat_thresh_low_flag(&self) -> VddCoreGldtcStatThreshLowFlagR {
        VddCoreGldtcStatThreshLowFlagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
High voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
    #[inline(always)]
    pub fn vdd_core_gldtc_stat_thresh_hi_flag(&self) -> VddCoreGldtcStatThreshHiFlagR {
        VddCoreGldtcStatThreshHiFlagR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Low voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
    #[inline(always)]
    #[must_use]
    pub fn vdd_core_gldtc_stat_thresh_low_flag(
        &mut self,
    ) -> VddCoreGldtcStatThreshLowFlagW<Cfg0VddCoreGldtcStatSpec> {
        VddCoreGldtcStatThreshLowFlagW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
High voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
    #[inline(always)]
    #[must_use]
    pub fn vdd_core_gldtc_stat_thresh_hi_flag(
        &mut self,
    ) -> VddCoreGldtcStatThreshHiFlagW<Cfg0VddCoreGldtcStatSpec> {
        VddCoreGldtcStatThreshHiFlagW::new(self, 8)
    }
}
#[doc = "CFG0_VDD_CORE_GLDTC_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_vdd_core_gldtc_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_vdd_core_gldtc_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0VddCoreGldtcStatSpec;
impl crate::RegisterSpec for Cfg0VddCoreGldtcStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_vdd_core_gldtc_stat::R`](R) reader structure"]
impl crate::Readable for Cfg0VddCoreGldtcStatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_vdd_core_gldtc_stat::W`](W) writer structure"]
impl crate::Writable for Cfg0VddCoreGldtcStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_VDD_CORE_GLDTC_STAT to value 0"]
impl crate::Resettable for Cfg0VddCoreGldtcStatSpec {
    const RESET_VALUE: u32 = 0;
}
