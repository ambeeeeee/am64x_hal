#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_518` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy518Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_518` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy518Spec>;
#[doc = "Field `PHY_ADR_SLV_DLY_CTRL_GATE_DISABLE_0` reader - 0:0\\]
Power reduction slv_dly_control block gate disable for address slice 0."]
pub type PhyAdrSlvDlyCtrlGateDisable0R = crate::BitReader;
#[doc = "Field `PHY_ADR_SLV_DLY_CTRL_GATE_DISABLE_0` writer - 0:0\\]
Power reduction slv_dly_control block gate disable for address slice 0."]
pub type PhyAdrSlvDlyCtrlGateDisable0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ADR_TYPE_0` reader - 9:8\\]
DRAM type for address slice 0."]
pub type PhyAdrType0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_TYPE_0` writer - 9:8\\]
DRAM type for address slice 0."]
pub type PhyAdrType0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADR_WRADDR_SHIFT_OBS_0` reader - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for address slice 0. READ-ONLY"]
pub type PhyAdrWraddrShiftObs0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_WRADDR_SHIFT_OBS_0` writer - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for address slice 0. READ-ONLY"]
pub type PhyAdrWraddrShiftObs0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_ADR_IE_MODE_0` reader - 24:24\\]
Input enable control for address slice 0."]
pub type PhyAdrIeMode0R = crate::BitReader;
#[doc = "Field `PHY_ADR_IE_MODE_0` writer - 24:24\\]
Input enable control for address slice 0."]
pub type PhyAdrIeMode0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Power reduction slv_dly_control block gate disable for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_slv_dly_ctrl_gate_disable_0(&self) -> PhyAdrSlvDlyCtrlGateDisable0R {
        PhyAdrSlvDlyCtrlGateDisable0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
DRAM type for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_type_0(&self) -> PhyAdrType0R {
        PhyAdrType0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for address slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_wraddr_shift_obs_0(&self) -> PhyAdrWraddrShiftObs0R {
        PhyAdrWraddrShiftObs0R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Input enable control for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_ie_mode_0(&self) -> PhyAdrIeMode0R {
        PhyAdrIeMode0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power reduction slv_dly_control block gate disable for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_slv_dly_ctrl_gate_disable_0(
        &mut self,
    ) -> PhyAdrSlvDlyCtrlGateDisable0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy518Spec> {
        PhyAdrSlvDlyCtrlGateDisable0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
DRAM type for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_type_0(&mut self) -> PhyAdrType0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy518Spec> {
        PhyAdrType0W::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Observation register containing automatic half cycle and cycle shift values for address slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_wraddr_shift_obs_0(
        &mut self,
    ) -> PhyAdrWraddrShiftObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy518Spec> {
        PhyAdrWraddrShiftObs0W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Input enable control for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_ie_mode_0(
        &mut self,
    ) -> PhyAdrIeMode0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy518Spec> {
        PhyAdrIeMode0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_518\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_518::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_518::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy518Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy518Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_518::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy518Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_518::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy518Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_518 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy518Spec {
    const RESET_VALUE: u32 = 0;
}
