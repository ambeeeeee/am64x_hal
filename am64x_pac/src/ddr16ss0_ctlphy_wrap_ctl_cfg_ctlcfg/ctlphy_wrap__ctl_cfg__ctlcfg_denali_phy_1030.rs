#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1030` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1030Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1030` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1030Spec>;
#[doc = "Field `PHY_ADR_SLV_DLY_CTRL_GATE_DISABLE_2` reader - 0:0\\]
Power reduction slv_dly_control block gate disable for address slice 2."]
pub type PhyAdrSlvDlyCtrlGateDisable2R = crate::BitReader;
#[doc = "Field `PHY_ADR_SLV_DLY_CTRL_GATE_DISABLE_2` writer - 0:0\\]
Power reduction slv_dly_control block gate disable for address slice 2."]
pub type PhyAdrSlvDlyCtrlGateDisable2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ADR_TYPE_2` reader - 9:8\\]
DRAM type for address slice 2."]
pub type PhyAdrType2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_TYPE_2` writer - 9:8\\]
DRAM type for address slice 2."]
pub type PhyAdrType2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADR_WRADDR_SHIFT_OBS_2` reader - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for address slice 2. READ-ONLY"]
pub type PhyAdrWraddrShiftObs2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_WRADDR_SHIFT_OBS_2` writer - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for address slice 2. READ-ONLY"]
pub type PhyAdrWraddrShiftObs2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_ADR_IE_MODE_2` reader - 24:24\\]
Input enable control for address slice 2."]
pub type PhyAdrIeMode2R = crate::BitReader;
#[doc = "Field `PHY_ADR_IE_MODE_2` writer - 24:24\\]
Input enable control for address slice 2."]
pub type PhyAdrIeMode2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Power reduction slv_dly_control block gate disable for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_slv_dly_ctrl_gate_disable_2(&self) -> PhyAdrSlvDlyCtrlGateDisable2R {
        PhyAdrSlvDlyCtrlGateDisable2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
DRAM type for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_type_2(&self) -> PhyAdrType2R {
        PhyAdrType2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for address slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_wraddr_shift_obs_2(&self) -> PhyAdrWraddrShiftObs2R {
        PhyAdrWraddrShiftObs2R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Input enable control for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_ie_mode_2(&self) -> PhyAdrIeMode2R {
        PhyAdrIeMode2R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power reduction slv_dly_control block gate disable for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_slv_dly_ctrl_gate_disable_2(
        &mut self,
    ) -> PhyAdrSlvDlyCtrlGateDisable2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1030Spec> {
        PhyAdrSlvDlyCtrlGateDisable2W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
DRAM type for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_type_2(&mut self) -> PhyAdrType2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1030Spec> {
        PhyAdrType2W::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for address slice 2. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_wraddr_shift_obs_2(
        &mut self,
    ) -> PhyAdrWraddrShiftObs2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1030Spec> {
        PhyAdrWraddrShiftObs2W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Input enable control for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_ie_mode_2(
        &mut self,
    ) -> PhyAdrIeMode2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1030Spec> {
        PhyAdrIeMode2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1030\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1030::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1030::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1030Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1030::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1030Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1030::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1030Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1030 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1030Spec {
    const RESET_VALUE: u32 = 0;
}
