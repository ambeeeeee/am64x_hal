#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_305` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy305Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_305` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy305Spec>;
#[doc = "Field `PHY_WRDQ_BASE_SLV_DLY_ENC_OBS_1` reader - 7:0\\]
Observation register containing write DQ base slave delay encoded value for slice 1. READ-ONLY"]
pub type PhyWrdqBaseSlvDlyEncObs1R = crate::FieldReader;
#[doc = "Field `PHY_WRDQ_BASE_SLV_DLY_ENC_OBS_1` writer - 7:0\\]
Observation register containing write DQ base slave delay encoded value for slice 1. READ-ONLY"]
pub type PhyWrdqBaseSlvDlyEncObs1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WR_ADDER_SLV_DLY_ENC_OBS_1` reader - 15:8\\]
Observation register containing write adder slave delay encoded value for slice 1. READ-ONLY"]
pub type PhyWrAdderSlvDlyEncObs1R = crate::FieldReader;
#[doc = "Field `PHY_WR_ADDER_SLV_DLY_ENC_OBS_1` writer - 15:8\\]
Observation register containing write adder slave delay encoded value for slice 1. READ-ONLY"]
pub type PhyWrAdderSlvDlyEncObs1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WR_SHIFT_OBS_1` reader - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for slice 1. READ-ONLY"]
pub type PhyWrShiftObs1R = crate::FieldReader;
#[doc = "Field `PHY_WR_SHIFT_OBS_1` writer - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for slice 1. READ-ONLY"]
pub type PhyWrShiftObs1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Observation register containing write DQ base slave delay encoded value for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wrdq_base_slv_dly_enc_obs_1(&self) -> PhyWrdqBaseSlvDlyEncObs1R {
        PhyWrdqBaseSlvDlyEncObs1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Observation register containing write adder slave delay encoded value for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wr_adder_slv_dly_enc_obs_1(&self) -> PhyWrAdderSlvDlyEncObs1R {
        PhyWrAdderSlvDlyEncObs1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wr_shift_obs_1(&self) -> PhyWrShiftObs1R {
        PhyWrShiftObs1R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Observation register containing write DQ base slave delay encoded value for slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrdq_base_slv_dly_enc_obs_1(
        &mut self,
    ) -> PhyWrdqBaseSlvDlyEncObs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy305Spec> {
        PhyWrdqBaseSlvDlyEncObs1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Observation register containing write adder slave delay encoded value for slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wr_adder_slv_dly_enc_obs_1(
        &mut self,
    ) -> PhyWrAdderSlvDlyEncObs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy305Spec> {
        PhyWrAdderSlvDlyEncObs1W::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wr_shift_obs_1(
        &mut self,
    ) -> PhyWrShiftObs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy305Spec> {
        PhyWrShiftObs1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_305\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_305::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_305::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy305Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy305Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_305::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy305Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_305::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy305Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_305 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy305Spec {
    const RESET_VALUE: u32 = 0;
}
