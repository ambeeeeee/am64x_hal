#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_49` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy49Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_49` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy49Spec>;
#[doc = "Field `PHY_WRDQ_BASE_SLV_DLY_ENC_OBS_0` reader - 7:0\\]
Observation register containing write DQ base slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyWrdqBaseSlvDlyEncObs0R = crate::FieldReader;
#[doc = "Field `PHY_WRDQ_BASE_SLV_DLY_ENC_OBS_0` writer - 7:0\\]
Observation register containing write DQ base slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyWrdqBaseSlvDlyEncObs0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WR_ADDER_SLV_DLY_ENC_OBS_0` reader - 15:8\\]
Observation register containing write adder slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyWrAdderSlvDlyEncObs0R = crate::FieldReader;
#[doc = "Field `PHY_WR_ADDER_SLV_DLY_ENC_OBS_0` writer - 15:8\\]
Observation register containing write adder slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyWrAdderSlvDlyEncObs0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WR_SHIFT_OBS_0` reader - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for slice 0. READ-ONLY"]
pub type PhyWrShiftObs0R = crate::FieldReader;
#[doc = "Field `PHY_WR_SHIFT_OBS_0` writer - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for slice 0. READ-ONLY"]
pub type PhyWrShiftObs0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Observation register containing write DQ base slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wrdq_base_slv_dly_enc_obs_0(&self) -> PhyWrdqBaseSlvDlyEncObs0R {
        PhyWrdqBaseSlvDlyEncObs0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Observation register containing write adder slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wr_adder_slv_dly_enc_obs_0(&self) -> PhyWrAdderSlvDlyEncObs0R {
        PhyWrAdderSlvDlyEncObs0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wr_shift_obs_0(&self) -> PhyWrShiftObs0R {
        PhyWrShiftObs0R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Observation register containing write DQ base slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrdq_base_slv_dly_enc_obs_0(
        &mut self,
    ) -> PhyWrdqBaseSlvDlyEncObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy49Spec> {
        PhyWrdqBaseSlvDlyEncObs0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Observation register containing write adder slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wr_adder_slv_dly_enc_obs_0(
        &mut self,
    ) -> PhyWrAdderSlvDlyEncObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy49Spec> {
        PhyWrAdderSlvDlyEncObs0W::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wr_shift_obs_0(
        &mut self,
    ) -> PhyWrShiftObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy49Spec> {
        PhyWrShiftObs0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_49::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_49::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy49Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy49Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_49::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy49Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_49::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy49Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_49 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy49Spec {
    const RESET_VALUE: u32 = 0;
}
