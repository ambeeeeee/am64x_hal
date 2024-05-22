#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_516` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy516Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_516` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy516Spec>;
#[doc = "Field `PHY_ADR_SLAVE_LOOP_CNT_UPDATE_0` reader - 2:0\\]
Reserved for address slice 0."]
pub type PhyAdrSlaveLoopCntUpdate0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SLAVE_LOOP_CNT_UPDATE_0` writer - 2:0\\]
Reserved for address slice 0."]
pub type PhyAdrSlaveLoopCntUpdate0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_ADR_SLV_DLY_ENC_OBS_SELECT_0` reader - 10:8\\]
Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 0."]
pub type PhyAdrSlvDlyEncObsSelect0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SLV_DLY_ENC_OBS_SELECT_0` writer - 10:8\\]
Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 0."]
pub type PhyAdrSlvDlyEncObsSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SC_PHY_ADR_SNAP_OBS_REGS_0` reader - 16:16\\]
Initiates a snapshot of the internal observation registers for address slice 0. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyAdrSnapObsRegs0R = crate::BitReader;
#[doc = "Field `SC_PHY_ADR_SNAP_OBS_REGS_0` writer - 16:16\\]
Initiates a snapshot of the internal observation registers for address slice 0. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyAdrSnapObsRegs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ADR_TSEL_ENABLE_0` reader - 24:24\\]
Enables tsel_en for address slice 0."]
pub type PhyAdrTselEnable0R = crate::BitReader;
#[doc = "Field `PHY_ADR_TSEL_ENABLE_0` writer - 24:24\\]
Enables tsel_en for address slice 0."]
pub type PhyAdrTselEnable0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Reserved for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_slave_loop_cnt_update_0(&self) -> PhyAdrSlaveLoopCntUpdate0R {
        PhyAdrSlaveLoopCntUpdate0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_slv_dly_enc_obs_select_0(&self) -> PhyAdrSlvDlyEncObsSelect0R {
        PhyAdrSlvDlyEncObsSelect0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Initiates a snapshot of the internal observation registers for address slice 0. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_adr_snap_obs_regs_0(&self) -> ScPhyAdrSnapObsRegs0R {
        ScPhyAdrSnapObsRegs0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables tsel_en for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_tsel_enable_0(&self) -> PhyAdrTselEnable0R {
        PhyAdrTselEnable0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Reserved for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_slave_loop_cnt_update_0(
        &mut self,
    ) -> PhyAdrSlaveLoopCntUpdate0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy516Spec> {
        PhyAdrSlaveLoopCntUpdate0W::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_slv_dly_enc_obs_select_0(
        &mut self,
    ) -> PhyAdrSlvDlyEncObsSelect0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy516Spec> {
        PhyAdrSlvDlyEncObsSelect0W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Initiates a snapshot of the internal observation registers for address slice 0. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_snap_obs_regs_0(
        &mut self,
    ) -> ScPhyAdrSnapObsRegs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy516Spec> {
        ScPhyAdrSnapObsRegs0W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables tsel_en for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_tsel_enable_0(
        &mut self,
    ) -> PhyAdrTselEnable0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy516Spec> {
        PhyAdrTselEnable0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_516\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_516::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_516::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy516Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy516Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_516::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy516Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_516::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy516Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_516 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy516Spec {
    const RESET_VALUE: u32 = 0;
}
