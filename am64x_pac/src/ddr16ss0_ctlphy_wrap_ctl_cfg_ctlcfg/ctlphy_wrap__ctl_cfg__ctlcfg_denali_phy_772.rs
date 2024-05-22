#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_772` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy772Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_772` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy772Spec>;
#[doc = "Field `PHY_ADR_SLAVE_LOOP_CNT_UPDATE_1` reader - 2:0\\]
Reserved for address slice 1."]
pub type PhyAdrSlaveLoopCntUpdate1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SLAVE_LOOP_CNT_UPDATE_1` writer - 2:0\\]
Reserved for address slice 1."]
pub type PhyAdrSlaveLoopCntUpdate1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_ADR_SLV_DLY_ENC_OBS_SELECT_1` reader - 10:8\\]
Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 1."]
pub type PhyAdrSlvDlyEncObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SLV_DLY_ENC_OBS_SELECT_1` writer - 10:8\\]
Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 1."]
pub type PhyAdrSlvDlyEncObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SC_PHY_ADR_SNAP_OBS_REGS_1` reader - 16:16\\]
Initiates a snapshot of the internal observation registers for address slice 1. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyAdrSnapObsRegs1R = crate::BitReader;
#[doc = "Field `SC_PHY_ADR_SNAP_OBS_REGS_1` writer - 16:16\\]
Initiates a snapshot of the internal observation registers for address slice 1. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyAdrSnapObsRegs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ADR_TSEL_ENABLE_1` reader - 24:24\\]
Enables tsel_en for address slice 1."]
pub type PhyAdrTselEnable1R = crate::BitReader;
#[doc = "Field `PHY_ADR_TSEL_ENABLE_1` writer - 24:24\\]
Enables tsel_en for address slice 1."]
pub type PhyAdrTselEnable1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Reserved for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_slave_loop_cnt_update_1(&self) -> PhyAdrSlaveLoopCntUpdate1R {
        PhyAdrSlaveLoopCntUpdate1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_slv_dly_enc_obs_select_1(&self) -> PhyAdrSlvDlyEncObsSelect1R {
        PhyAdrSlvDlyEncObsSelect1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Initiates a snapshot of the internal observation registers for address slice 1. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_adr_snap_obs_regs_1(&self) -> ScPhyAdrSnapObsRegs1R {
        ScPhyAdrSnapObsRegs1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables tsel_en for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_tsel_enable_1(&self) -> PhyAdrTselEnable1R {
        PhyAdrTselEnable1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Reserved for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_slave_loop_cnt_update_1(
        &mut self,
    ) -> PhyAdrSlaveLoopCntUpdate1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy772Spec> {
        PhyAdrSlaveLoopCntUpdate1W::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_slv_dly_enc_obs_select_1(
        &mut self,
    ) -> PhyAdrSlvDlyEncObsSelect1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy772Spec> {
        PhyAdrSlvDlyEncObsSelect1W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Initiates a snapshot of the internal observation registers for address slice 1. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_snap_obs_regs_1(
        &mut self,
    ) -> ScPhyAdrSnapObsRegs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy772Spec> {
        ScPhyAdrSnapObsRegs1W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables tsel_en for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_tsel_enable_1(
        &mut self,
    ) -> PhyAdrTselEnable1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy772Spec> {
        PhyAdrTselEnable1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_772\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_772::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_772::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy772Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy772Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_772::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy772Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_772::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy772Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_772 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy772Spec {
    const RESET_VALUE: u32 = 0;
}
