#[doc = "Register `CFG0_VDD_CORE_GLDTC_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0VddCoreGldtcCtrlProxySpec>;
#[doc = "Register `CFG0_VDD_CORE_GLDTC_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0VddCoreGldtcCtrlProxySpec>;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_THRESH_LO_SEL_PROXY` reader - 5:0\\]
Selects the low voltage glitch threshold as a percentage of the monitored voltage"]
pub type VddCoreGldtcCtrlThreshLoSelProxyR = crate::FieldReader;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_THRESH_LO_SEL_PROXY` writer - 5:0\\]
Selects the low voltage glitch threshold as a percentage of the monitored voltage"]
pub type VddCoreGldtcCtrlThreshLoSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_THRESH_HI_SEL_PROXY` reader - 13:8\\]
Selects the high voltage glitch threshold as a percentage of the monitored voltage"]
pub type VddCoreGldtcCtrlThreshHiSelProxyR = crate::FieldReader;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_THRESH_HI_SEL_PROXY` writer - 13:8\\]
Selects the high voltage glitch threshold as a percentage of the monitored voltage"]
pub type VddCoreGldtcCtrlThreshHiSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_LP_FILTER_SEL_PROXY` reader - 18:16\\]
Selects the glitch detect low-pass filter bandwidth"]
pub type VddCoreGldtcCtrlLpFilterSelProxyR = crate::FieldReader;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_LP_FILTER_SEL_PROXY` writer - 18:16\\]
Selects the glitch detect low-pass filter bandwidth"]
pub type VddCoreGldtcCtrlLpFilterSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_RSTB_PROXY` reader - 30:30\\]
Reset - active low. To ensure proper operation, rstb must be not be de-asserted for at least 100 ns after power-up (pwdb de-asserted). Additionally, rstb must be toggled low at least 200 ns after any change in threshold or low-pass filter settings to prevent abnormal trigger events."]
pub type VddCoreGldtcCtrlRstbProxyR = crate::BitReader;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_RSTB_PROXY` writer - 30:30\\]
Reset - active low. To ensure proper operation, rstb must be not be de-asserted for at least 100 ns after power-up (pwdb de-asserted). Additionally, rstb must be toggled low at least 200 ns after any change in threshold or low-pass filter settings to prevent abnormal trigger events."]
pub type VddCoreGldtcCtrlRstbProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_PWDB_PROXY` reader - 31:31\\]
Power down - active low."]
pub type VddCoreGldtcCtrlPwdbProxyR = crate::BitReader;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_PWDB_PROXY` writer - 31:31\\]
Power down - active low."]
pub type VddCoreGldtcCtrlPwdbProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Selects the low voltage glitch threshold as a percentage of the monitored voltage"]
    #[inline(always)]
    pub fn vdd_core_gldtc_ctrl_thresh_lo_sel_proxy(&self) -> VddCoreGldtcCtrlThreshLoSelProxyR {
        VddCoreGldtcCtrlThreshLoSelProxyR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Selects the high voltage glitch threshold as a percentage of the monitored voltage"]
    #[inline(always)]
    pub fn vdd_core_gldtc_ctrl_thresh_hi_sel_proxy(&self) -> VddCoreGldtcCtrlThreshHiSelProxyR {
        VddCoreGldtcCtrlThreshHiSelProxyR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Selects the glitch detect low-pass filter bandwidth"]
    #[inline(always)]
    pub fn vdd_core_gldtc_ctrl_lp_filter_sel_proxy(&self) -> VddCoreGldtcCtrlLpFilterSelProxyR {
        VddCoreGldtcCtrlLpFilterSelProxyR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Reset - active low. To ensure proper operation, rstb must be not be de-asserted for at least 100 ns after power-up (pwdb de-asserted). Additionally, rstb must be toggled low at least 200 ns after any change in threshold or low-pass filter settings to prevent abnormal trigger events."]
    #[inline(always)]
    pub fn vdd_core_gldtc_ctrl_rstb_proxy(&self) -> VddCoreGldtcCtrlRstbProxyR {
        VddCoreGldtcCtrlRstbProxyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Power down - active low."]
    #[inline(always)]
    pub fn vdd_core_gldtc_ctrl_pwdb_proxy(&self) -> VddCoreGldtcCtrlPwdbProxyR {
        VddCoreGldtcCtrlPwdbProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Selects the low voltage glitch threshold as a percentage of the monitored voltage"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_core_gldtc_ctrl_thresh_lo_sel_proxy(
        &mut self,
    ) -> VddCoreGldtcCtrlThreshLoSelProxyW<Cfg0VddCoreGldtcCtrlProxySpec> {
        VddCoreGldtcCtrlThreshLoSelProxyW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Selects the high voltage glitch threshold as a percentage of the monitored voltage"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_core_gldtc_ctrl_thresh_hi_sel_proxy(
        &mut self,
    ) -> VddCoreGldtcCtrlThreshHiSelProxyW<Cfg0VddCoreGldtcCtrlProxySpec> {
        VddCoreGldtcCtrlThreshHiSelProxyW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Selects the glitch detect low-pass filter bandwidth"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_core_gldtc_ctrl_lp_filter_sel_proxy(
        &mut self,
    ) -> VddCoreGldtcCtrlLpFilterSelProxyW<Cfg0VddCoreGldtcCtrlProxySpec> {
        VddCoreGldtcCtrlLpFilterSelProxyW::new(self, 16)
    }
    #[doc = "Bit 30 - 30:30\\]
Reset - active low. To ensure proper operation, rstb must be not be de-asserted for at least 100 ns after power-up (pwdb de-asserted). Additionally, rstb must be toggled low at least 200 ns after any change in threshold or low-pass filter settings to prevent abnormal trigger events."]
    #[inline(always)]
    #[must_use]
    pub fn vdd_core_gldtc_ctrl_rstb_proxy(
        &mut self,
    ) -> VddCoreGldtcCtrlRstbProxyW<Cfg0VddCoreGldtcCtrlProxySpec> {
        VddCoreGldtcCtrlRstbProxyW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Power down - active low."]
    #[inline(always)]
    #[must_use]
    pub fn vdd_core_gldtc_ctrl_pwdb_proxy(
        &mut self,
    ) -> VddCoreGldtcCtrlPwdbProxyW<Cfg0VddCoreGldtcCtrlProxySpec> {
        VddCoreGldtcCtrlPwdbProxyW::new(self, 31)
    }
}
#[doc = "CFG0_VDD_CORE_GLDTC_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_vdd_core_gldtc_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_vdd_core_gldtc_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0VddCoreGldtcCtrlProxySpec;
impl crate::RegisterSpec for Cfg0VddCoreGldtcCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_vdd_core_gldtc_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0VddCoreGldtcCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_vdd_core_gldtc_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0VddCoreGldtcCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_VDD_CORE_GLDTC_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0VddCoreGldtcCtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
