#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_347` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy347Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_347` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy347Spec>;
#[doc = "Field `PHY_DQ_IE_TIMING_1` reader - 7:0\\]
Start/end timing values for DQ/DM input enable signals for slice 1."]
pub type PhyDqIeTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQ_IE_TIMING_1` writer - 7:0\\]
Start/end timing values for DQ/DM input enable signals for slice 1."]
pub type PhyDqIeTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQS_IE_TIMING_1` reader - 15:8\\]
Start/end timing values for DQS input enable signals for slice 1."]
pub type PhyDqsIeTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQS_IE_TIMING_1` writer - 15:8\\]
Start/end timing values for DQS input enable signals for slice 1."]
pub type PhyDqsIeTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RDDATA_EN_IE_DLY_1` reader - 17:16\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
pub type PhyRddataEnIeDly1R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_IE_DLY_1` writer - 17:16\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
pub type PhyRddataEnIeDly1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_IE_MODE_1` reader - 25:24\\]
Input enable mode bits for slice 1. Bit \\[0\\]
enables the mode where the input enables are always on; set to 1 to enable. Bit \\[1\\]
disables the input enable on the DM signal; set to 1 to disable."]
pub type PhyIeMode1R = crate::FieldReader;
#[doc = "Field `PHY_IE_MODE_1` writer - 25:24\\]
Input enable mode bits for slice 1. Bit \\[0\\]
enables the mode where the input enables are always on; set to 1 to enable. Bit \\[1\\]
disables the input enable on the DM signal; set to 1 to disable."]
pub type PhyIeMode1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Start/end timing values for DQ/DM input enable signals for slice 1."]
    #[inline(always)]
    pub fn phy_dq_ie_timing_1(&self) -> PhyDqIeTiming1R {
        PhyDqIeTiming1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Start/end timing values for DQS input enable signals for slice 1."]
    #[inline(always)]
    pub fn phy_dqs_ie_timing_1(&self) -> PhyDqsIeTiming1R {
        PhyDqsIeTiming1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
    #[inline(always)]
    pub fn phy_rddata_en_ie_dly_1(&self) -> PhyRddataEnIeDly1R {
        PhyRddataEnIeDly1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Input enable mode bits for slice 1. Bit \\[0\\]
enables the mode where the input enables are always on; set to 1 to enable. Bit \\[1\\]
disables the input enable on the DM signal; set to 1 to disable."]
    #[inline(always)]
    pub fn phy_ie_mode_1(&self) -> PhyIeMode1R {
        PhyIeMode1R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Start/end timing values for DQ/DM input enable signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_ie_timing_1(
        &mut self,
    ) -> PhyDqIeTiming1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy347Spec> {
        PhyDqIeTiming1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Start/end timing values for DQS input enable signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_ie_timing_1(
        &mut self,
    ) -> PhyDqsIeTiming1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy347Spec> {
        PhyDqsIeTiming1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_ie_dly_1(
        &mut self,
    ) -> PhyRddataEnIeDly1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy347Spec> {
        PhyRddataEnIeDly1W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Input enable mode bits for slice 1. Bit \\[0\\]
enables the mode where the input enables are always on; set to 1 to enable. Bit \\[1\\]
disables the input enable on the DM signal; set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ie_mode_1(&mut self) -> PhyIeMode1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy347Spec> {
        PhyIeMode1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_347\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_347::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_347::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy347Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy347Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_347::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy347Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_347::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy347Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_347 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy347Spec {
    const RESET_VALUE: u32 = 0;
}
