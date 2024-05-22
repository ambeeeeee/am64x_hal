#[doc = "Register `CFG0_VDD_CORE_GLDTC_STAT_PROXY` reader"]
pub type R = crate::R<Cfg0VddCoreGldtcStatProxySpec>;
#[doc = "Register `CFG0_VDD_CORE_GLDTC_STAT_PROXY` writer"]
pub type W = crate::W<Cfg0VddCoreGldtcStatProxySpec>;
#[doc = "Field `VDD_CORE_GLDTC_STAT_THRESH_LOW_FLAG_PROXY` reader - 0:0\\]
Low voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
pub type VddCoreGldtcStatThreshLowFlagProxyR = crate::BitReader;
#[doc = "Field `VDD_CORE_GLDTC_STAT_THRESH_LOW_FLAG_PROXY` writer - 0:0\\]
Low voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
pub type VddCoreGldtcStatThreshLowFlagProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDD_CORE_GLDTC_STAT_THRESH_HI_FLAG_PROXY` reader - 8:8\\]
High voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
pub type VddCoreGldtcStatThreshHiFlagProxyR = crate::BitReader;
#[doc = "Field `VDD_CORE_GLDTC_STAT_THRESH_HI_FLAG_PROXY` writer - 8:8\\]
High voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
pub type VddCoreGldtcStatThreshHiFlagProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Low voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
    #[inline(always)]
    pub fn vdd_core_gldtc_stat_thresh_low_flag_proxy(&self) -> VddCoreGldtcStatThreshLowFlagProxyR {
        VddCoreGldtcStatThreshLowFlagProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
High voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
    #[inline(always)]
    pub fn vdd_core_gldtc_stat_thresh_hi_flag_proxy(&self) -> VddCoreGldtcStatThreshHiFlagProxyR {
        VddCoreGldtcStatThreshHiFlagProxyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Low voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
    #[inline(always)]
    #[must_use]
    pub fn vdd_core_gldtc_stat_thresh_low_flag_proxy(
        &mut self,
    ) -> VddCoreGldtcStatThreshLowFlagProxyW<Cfg0VddCoreGldtcStatProxySpec> {
        VddCoreGldtcStatThreshLowFlagProxyW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
High voltage flag. This flag is cleared by clearing the VDD_CORE_GLDTC_CTRL_rstb bit."]
    #[inline(always)]
    #[must_use]
    pub fn vdd_core_gldtc_stat_thresh_hi_flag_proxy(
        &mut self,
    ) -> VddCoreGldtcStatThreshHiFlagProxyW<Cfg0VddCoreGldtcStatProxySpec> {
        VddCoreGldtcStatThreshHiFlagProxyW::new(self, 8)
    }
}
#[doc = "CFG0_VDD_CORE_GLDTC_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_vdd_core_gldtc_stat_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_vdd_core_gldtc_stat_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0VddCoreGldtcStatProxySpec;
impl crate::RegisterSpec for Cfg0VddCoreGldtcStatProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_vdd_core_gldtc_stat_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0VddCoreGldtcStatProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_vdd_core_gldtc_stat_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0VddCoreGldtcStatProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_VDD_CORE_GLDTC_STAT_PROXY to value 0"]
impl crate::Resettable for Cfg0VddCoreGldtcStatProxySpec {
    const RESET_VALUE: u32 = 0;
}
