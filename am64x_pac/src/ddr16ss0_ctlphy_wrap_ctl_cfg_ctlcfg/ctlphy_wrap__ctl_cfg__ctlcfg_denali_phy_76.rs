#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_76` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy76Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_76` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy76Spec>;
#[doc = "Field `PHY_VREF_INITIAL_STOP_POINT_0` reader - 6:0\\]
Data slice initial VREF training stop value for slice 0."]
pub type PhyVrefInitialStopPoint0R = crate::FieldReader;
#[doc = "Field `PHY_VREF_INITIAL_STOP_POINT_0` writer - 6:0\\]
Data slice initial VREF training stop value for slice 0."]
pub type PhyVrefInitialStopPoint0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_VREF_TRAINING_CTRL_0` reader - 9:8\\]
Data slice vref training enable control for slice 0."]
pub type PhyVrefTrainingCtrl0R = crate::FieldReader;
#[doc = "Field `PHY_VREF_TRAINING_CTRL_0` writer - 9:8\\]
Data slice vref training enable control for slice 0."]
pub type PhyVrefTrainingCtrl0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_NTP_TRAIN_EN_0` reader - 16:16\\]
Enable for No-Topology training for slice 0."]
pub type PhyNtpTrainEn0R = crate::BitReader;
#[doc = "Field `PHY_NTP_TRAIN_EN_0` writer - 16:16\\]
Enable for No-Topology training for slice 0."]
pub type PhyNtpTrainEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_NTP_WDQ_STEP_SIZE_0` reader - 31:24\\]
Step size of WR DQ slave delay during No-Topology training for slice 0."]
pub type PhyNtpWdqStepSize0R = crate::FieldReader;
#[doc = "Field `PHY_NTP_WDQ_STEP_SIZE_0` writer - 31:24\\]
Step size of WR DQ slave delay during No-Topology training for slice 0."]
pub type PhyNtpWdqStepSize0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Data slice initial VREF training stop value for slice 0."]
    #[inline(always)]
    pub fn phy_vref_initial_stop_point_0(&self) -> PhyVrefInitialStopPoint0R {
        PhyVrefInitialStopPoint0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Data slice vref training enable control for slice 0."]
    #[inline(always)]
    pub fn phy_vref_training_ctrl_0(&self) -> PhyVrefTrainingCtrl0R {
        PhyVrefTrainingCtrl0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable for No-Topology training for slice 0."]
    #[inline(always)]
    pub fn phy_ntp_train_en_0(&self) -> PhyNtpTrainEn0R {
        PhyNtpTrainEn0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Step size of WR DQ slave delay during No-Topology training for slice 0."]
    #[inline(always)]
    pub fn phy_ntp_wdq_step_size_0(&self) -> PhyNtpWdqStepSize0R {
        PhyNtpWdqStepSize0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Data slice initial VREF training stop value for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_vref_initial_stop_point_0(
        &mut self,
    ) -> PhyVrefInitialStopPoint0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy76Spec> {
        PhyVrefInitialStopPoint0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Data slice vref training enable control for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_vref_training_ctrl_0(
        &mut self,
    ) -> PhyVrefTrainingCtrl0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy76Spec> {
        PhyVrefTrainingCtrl0W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable for No-Topology training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_train_en_0(
        &mut self,
    ) -> PhyNtpTrainEn0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy76Spec> {
        PhyNtpTrainEn0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Step size of WR DQ slave delay during No-Topology training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_wdq_step_size_0(
        &mut self,
    ) -> PhyNtpWdqStepSize0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy76Spec> {
        PhyNtpWdqStepSize0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_76::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_76::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy76Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy76Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_76::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy76Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_76::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy76Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_76 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy76Spec {
    const RESET_VALUE: u32 = 0;
}
