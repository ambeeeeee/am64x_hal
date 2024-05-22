#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_284` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy284Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_284` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy284Spec>;
#[doc = "Field `PHY_RDDQS_DQ_ENC_OBS_SELECT_1` reader - 3:0\\]
Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 1."]
pub type PhyRddqsDqEncObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_ENC_OBS_SELECT_1` writer - 3:0\\]
Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 1."]
pub type PhyRddqsDqEncObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WR_ENC_OBS_SELECT_1` reader - 11:8\\]
Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 1."]
pub type PhyWrEncObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_WR_ENC_OBS_SELECT_1` writer - 11:8\\]
Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 1."]
pub type PhyWrEncObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WR_SHIFT_OBS_SELECT_1` reader - 19:16\\]
Select value to map the internal write DQ/DQS automatic cycle/half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 1."]
pub type PhyWrShiftObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_WR_SHIFT_OBS_SELECT_1` writer - 19:16\\]
Select value to map the internal write DQ/DQS automatic cycle/half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 1."]
pub type PhyWrShiftObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_FIFO_PTR_OBS_SELECT_1` reader - 27:24\\]
Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 1."]
pub type PhyFifoPtrObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_FIFO_PTR_OBS_SELECT_1` writer - 27:24\\]
Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 1."]
pub type PhyFifoPtrObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_dq_enc_obs_select_1(&self) -> PhyRddqsDqEncObsSelect1R {
        PhyRddqsDqEncObsSelect1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 1."]
    #[inline(always)]
    pub fn phy_wr_enc_obs_select_1(&self) -> PhyWrEncObsSelect1R {
        PhyWrEncObsSelect1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Select value to map the internal write DQ/DQS automatic cycle/half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 1."]
    #[inline(always)]
    pub fn phy_wr_shift_obs_select_1(&self) -> PhyWrShiftObsSelect1R {
        PhyWrShiftObsSelect1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 1."]
    #[inline(always)]
    pub fn phy_fifo_ptr_obs_select_1(&self) -> PhyFifoPtrObsSelect1R {
        PhyFifoPtrObsSelect1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq_enc_obs_select_1(
        &mut self,
    ) -> PhyRddqsDqEncObsSelect1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy284Spec> {
        PhyRddqsDqEncObsSelect1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wr_enc_obs_select_1(
        &mut self,
    ) -> PhyWrEncObsSelect1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy284Spec> {
        PhyWrEncObsSelect1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Select value to map the internal write DQ/DQS automatic cycle/half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wr_shift_obs_select_1(
        &mut self,
    ) -> PhyWrShiftObsSelect1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy284Spec> {
        PhyWrShiftObsSelect1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_fifo_ptr_obs_select_1(
        &mut self,
    ) -> PhyFifoPtrObsSelect1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy284Spec> {
        PhyFifoPtrObsSelect1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_284\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_284::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_284::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy284Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy284Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_284::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy284Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_284::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy284Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_284 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy284Spec {
    const RESET_VALUE: u32 = 0;
}
