#[doc = "Register `CFG0_VDD_CORE_GLDTC_CTRL` reader"]
pub type R = crate::R<Cfg0VddCoreGldtcCtrlSpec>;
#[doc = "Register `CFG0_VDD_CORE_GLDTC_CTRL` writer"]
pub type W = crate::W<Cfg0VddCoreGldtcCtrlSpec>;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_THRESH_LO_SEL` reader - 5:0\\]
Selects the low voltage glitch threshold as a percentage of the monitored voltage"]
pub type VddCoreGldtcCtrlThreshLoSelR = crate::FieldReader;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_THRESH_LO_SEL` writer - 5:0\\]
Selects the low voltage glitch threshold as a percentage of the monitored voltage"]
pub type VddCoreGldtcCtrlThreshLoSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_THRESH_HI_SEL` reader - 13:8\\]
Selects the high voltage glitch threshold as a percentage of the monitored voltage"]
pub type VddCoreGldtcCtrlThreshHiSelR = crate::FieldReader;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_THRESH_HI_SEL` writer - 13:8\\]
Selects the high voltage glitch threshold as a percentage of the monitored voltage"]
pub type VddCoreGldtcCtrlThreshHiSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_LP_FILTER_SEL` reader - 18:16\\]
Selects the glitch detect low-pass filter bandwidth"]
pub type VddCoreGldtcCtrlLpFilterSelR = crate::FieldReader;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_LP_FILTER_SEL` writer - 18:16\\]
Selects the glitch detect low-pass filter bandwidth"]
pub type VddCoreGldtcCtrlLpFilterSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_RSTB` reader - 30:30\\]
Reset - active low. To ensure proper operation, rstb must be not be de-asserted for at least 100 ns after power-up (pwdb de-asserted). Additionally, rstb must be toggled low at least 200 ns after any change in threshold or low-pass filter settings to prevent abnormal trigger events."]
pub type VddCoreGldtcCtrlRstbR = crate::BitReader;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_RSTB` writer - 30:30\\]
Reset - active low. To ensure proper operation, rstb must be not be de-asserted for at least 100 ns after power-up (pwdb de-asserted). Additionally, rstb must be toggled low at least 200 ns after any change in threshold or low-pass filter settings to prevent abnormal trigger events."]
pub type VddCoreGldtcCtrlRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_PWDB` reader - 31:31\\]
Power down - active low."]
pub type VddCoreGldtcCtrlPwdbR = crate::BitReader;
#[doc = "Field `VDD_CORE_GLDTC_CTRL_PWDB` writer - 31:31\\]
Power down - active low."]
pub type VddCoreGldtcCtrlPwdbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Selects the low voltage glitch threshold as a percentage of the monitored voltage"]
    #[inline(always)]
    pub fn vdd_core_gldtc_ctrl_thresh_lo_sel(&self) -> VddCoreGldtcCtrlThreshLoSelR {
        VddCoreGldtcCtrlThreshLoSelR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Selects the high voltage glitch threshold as a percentage of the monitored voltage"]
    #[inline(always)]
    pub fn vdd_core_gldtc_ctrl_thresh_hi_sel(&self) -> VddCoreGldtcCtrlThreshHiSelR {
        VddCoreGldtcCtrlThreshHiSelR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Selects the glitch detect low-pass filter bandwidth"]
    #[inline(always)]
    pub fn vdd_core_gldtc_ctrl_lp_filter_sel(&self) -> VddCoreGldtcCtrlLpFilterSelR {
        VddCoreGldtcCtrlLpFilterSelR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Reset - active low. To ensure proper operation, rstb must be not be de-asserted for at least 100 ns after power-up (pwdb de-asserted). Additionally, rstb must be toggled low at least 200 ns after any change in threshold or low-pass filter settings to prevent abnormal trigger events."]
    #[inline(always)]
    pub fn vdd_core_gldtc_ctrl_rstb(&self) -> VddCoreGldtcCtrlRstbR {
        VddCoreGldtcCtrlRstbR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Power down - active low."]
    #[inline(always)]
    pub fn vdd_core_gldtc_ctrl_pwdb(&self) -> VddCoreGldtcCtrlPwdbR {
        VddCoreGldtcCtrlPwdbR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Selects the low voltage glitch threshold as a percentage of the monitored voltage"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_core_gldtc_ctrl_thresh_lo_sel(
        &mut self,
    ) -> VddCoreGldtcCtrlThreshLoSelW<Cfg0VddCoreGldtcCtrlSpec> {
        VddCoreGldtcCtrlThreshLoSelW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Selects the high voltage glitch threshold as a percentage of the monitored voltage"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_core_gldtc_ctrl_thresh_hi_sel(
        &mut self,
    ) -> VddCoreGldtcCtrlThreshHiSelW<Cfg0VddCoreGldtcCtrlSpec> {
        VddCoreGldtcCtrlThreshHiSelW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Selects the glitch detect low-pass filter bandwidth"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_core_gldtc_ctrl_lp_filter_sel(
        &mut self,
    ) -> VddCoreGldtcCtrlLpFilterSelW<Cfg0VddCoreGldtcCtrlSpec> {
        VddCoreGldtcCtrlLpFilterSelW::new(self, 16)
    }
    #[doc = "Bit 30 - 30:30\\]
Reset - active low. To ensure proper operation, rstb must be not be de-asserted for at least 100 ns after power-up (pwdb de-asserted). Additionally, rstb must be toggled low at least 200 ns after any change in threshold or low-pass filter settings to prevent abnormal trigger events."]
    #[inline(always)]
    #[must_use]
    pub fn vdd_core_gldtc_ctrl_rstb(&mut self) -> VddCoreGldtcCtrlRstbW<Cfg0VddCoreGldtcCtrlSpec> {
        VddCoreGldtcCtrlRstbW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Power down - active low."]
    #[inline(always)]
    #[must_use]
    pub fn vdd_core_gldtc_ctrl_pwdb(&mut self) -> VddCoreGldtcCtrlPwdbW<Cfg0VddCoreGldtcCtrlSpec> {
        VddCoreGldtcCtrlPwdbW::new(self, 31)
    }
}
#[doc = "CFG0_VDD_CORE_GLDTC_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_vdd_core_gldtc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_vdd_core_gldtc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0VddCoreGldtcCtrlSpec;
impl crate::RegisterSpec for Cfg0VddCoreGldtcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_vdd_core_gldtc_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0VddCoreGldtcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_vdd_core_gldtc_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0VddCoreGldtcCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_VDD_CORE_GLDTC_CTRL to value 0"]
impl crate::Resettable for Cfg0VddCoreGldtcCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
