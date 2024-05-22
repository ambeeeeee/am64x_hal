#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_332` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy332Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_332` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy332Spec>;
#[doc = "Field `PHY_VREF_INITIAL_STOP_POINT_1` reader - 6:0\\]
Data slice initial VREF training stop value for slice 1."]
pub type PhyVrefInitialStopPoint1R = crate::FieldReader;
#[doc = "Field `PHY_VREF_INITIAL_STOP_POINT_1` writer - 6:0\\]
Data slice initial VREF training stop value for slice 1."]
pub type PhyVrefInitialStopPoint1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_VREF_TRAINING_CTRL_1` reader - 9:8\\]
Data slice vref training enable control for slice 1."]
pub type PhyVrefTrainingCtrl1R = crate::FieldReader;
#[doc = "Field `PHY_VREF_TRAINING_CTRL_1` writer - 9:8\\]
Data slice vref training enable control for slice 1."]
pub type PhyVrefTrainingCtrl1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_NTP_TRAIN_EN_1` reader - 16:16\\]
Enable for No-Topology training for slice 1."]
pub type PhyNtpTrainEn1R = crate::BitReader;
#[doc = "Field `PHY_NTP_TRAIN_EN_1` writer - 16:16\\]
Enable for No-Topology training for slice 1."]
pub type PhyNtpTrainEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_NTP_WDQ_STEP_SIZE_1` reader - 31:24\\]
Step size of WR DQ slave delay during No-Topology training for slice 1."]
pub type PhyNtpWdqStepSize1R = crate::FieldReader;
#[doc = "Field `PHY_NTP_WDQ_STEP_SIZE_1` writer - 31:24\\]
Step size of WR DQ slave delay during No-Topology training for slice 1."]
pub type PhyNtpWdqStepSize1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Data slice initial VREF training stop value for slice 1."]
    #[inline(always)]
    pub fn phy_vref_initial_stop_point_1(&self) -> PhyVrefInitialStopPoint1R {
        PhyVrefInitialStopPoint1R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Data slice vref training enable control for slice 1."]
    #[inline(always)]
    pub fn phy_vref_training_ctrl_1(&self) -> PhyVrefTrainingCtrl1R {
        PhyVrefTrainingCtrl1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable for No-Topology training for slice 1."]
    #[inline(always)]
    pub fn phy_ntp_train_en_1(&self) -> PhyNtpTrainEn1R {
        PhyNtpTrainEn1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Step size of WR DQ slave delay during No-Topology training for slice 1."]
    #[inline(always)]
    pub fn phy_ntp_wdq_step_size_1(&self) -> PhyNtpWdqStepSize1R {
        PhyNtpWdqStepSize1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Data slice initial VREF training stop value for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_vref_initial_stop_point_1(
        &mut self,
    ) -> PhyVrefInitialStopPoint1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy332Spec> {
        PhyVrefInitialStopPoint1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Data slice vref training enable control for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_vref_training_ctrl_1(
        &mut self,
    ) -> PhyVrefTrainingCtrl1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy332Spec> {
        PhyVrefTrainingCtrl1W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable for No-Topology training for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_train_en_1(
        &mut self,
    ) -> PhyNtpTrainEn1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy332Spec> {
        PhyNtpTrainEn1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Step size of WR DQ slave delay during No-Topology training for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_wdq_step_size_1(
        &mut self,
    ) -> PhyNtpWdqStepSize1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy332Spec> {
        PhyNtpWdqStepSize1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_332\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_332::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_332::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy332Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy332Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_332::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy332Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_332::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy332Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_332 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy332Spec {
    const RESET_VALUE: u32 = 0;
}
