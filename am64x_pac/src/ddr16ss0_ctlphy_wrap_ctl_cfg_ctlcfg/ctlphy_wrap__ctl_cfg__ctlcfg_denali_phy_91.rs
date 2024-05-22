#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_91` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy91Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_91` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy91Spec>;
#[doc = "Field `PHY_DQ_IE_TIMING_0` reader - 7:0\\]
Start/end timing values for DQ/DM input enable signals for slice 0."]
pub type PhyDqIeTiming0R = crate::FieldReader;
#[doc = "Field `PHY_DQ_IE_TIMING_0` writer - 7:0\\]
Start/end timing values for DQ/DM input enable signals for slice 0."]
pub type PhyDqIeTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQS_IE_TIMING_0` reader - 15:8\\]
Start/end timing values for DQS input enable signals for slice 0."]
pub type PhyDqsIeTiming0R = crate::FieldReader;
#[doc = "Field `PHY_DQS_IE_TIMING_0` writer - 15:8\\]
Start/end timing values for DQS input enable signals for slice 0."]
pub type PhyDqsIeTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RDDATA_EN_IE_DLY_0` reader - 17:16\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
pub type PhyRddataEnIeDly0R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_IE_DLY_0` writer - 17:16\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
pub type PhyRddataEnIeDly0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_IE_MODE_0` reader - 25:24\\]
Input enable mode bits for slice 0. Bit \\[0\\]
enables the mode where the input enables are always on; set to 1 to enable. Bit \\[1\\]
disables the input enable on the DM signal; set to 1 to disable."]
pub type PhyIeMode0R = crate::FieldReader;
#[doc = "Field `PHY_IE_MODE_0` writer - 25:24\\]
Input enable mode bits for slice 0. Bit \\[0\\]
enables the mode where the input enables are always on; set to 1 to enable. Bit \\[1\\]
disables the input enable on the DM signal; set to 1 to disable."]
pub type PhyIeMode0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Start/end timing values for DQ/DM input enable signals for slice 0."]
    #[inline(always)]
    pub fn phy_dq_ie_timing_0(&self) -> PhyDqIeTiming0R {
        PhyDqIeTiming0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Start/end timing values for DQS input enable signals for slice 0."]
    #[inline(always)]
    pub fn phy_dqs_ie_timing_0(&self) -> PhyDqsIeTiming0R {
        PhyDqsIeTiming0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
    #[inline(always)]
    pub fn phy_rddata_en_ie_dly_0(&self) -> PhyRddataEnIeDly0R {
        PhyRddataEnIeDly0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Input enable mode bits for slice 0. Bit \\[0\\]
enables the mode where the input enables are always on; set to 1 to enable. Bit \\[1\\]
disables the input enable on the DM signal; set to 1 to disable."]
    #[inline(always)]
    pub fn phy_ie_mode_0(&self) -> PhyIeMode0R {
        PhyIeMode0R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Start/end timing values for DQ/DM input enable signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_ie_timing_0(
        &mut self,
    ) -> PhyDqIeTiming0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy91Spec> {
        PhyDqIeTiming0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Start/end timing values for DQS input enable signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_ie_timing_0(
        &mut self,
    ) -> PhyDqsIeTiming0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy91Spec> {
        PhyDqsIeTiming0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_ie_dly_0(
        &mut self,
    ) -> PhyRddataEnIeDly0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy91Spec> {
        PhyRddataEnIeDly0W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Input enable mode bits for slice 0. Bit \\[0\\]
enables the mode where the input enables are always on; set to 1 to enable. Bit \\[1\\]
disables the input enable on the DM signal; set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ie_mode_0(&mut self) -> PhyIeMode0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy91Spec> {
        PhyIeMode0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_91\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_91::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_91::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy91Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy91Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_91::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy91Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_91::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy91Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_91 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy91Spec {
    const RESET_VALUE: u32 = 0;
}
