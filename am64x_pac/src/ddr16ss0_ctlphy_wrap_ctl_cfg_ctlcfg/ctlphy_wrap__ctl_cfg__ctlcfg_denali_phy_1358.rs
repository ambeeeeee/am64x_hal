#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1358` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1358Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1358` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1358Spec>;
#[doc = "Field `PHY_AC_SLV_DLY_CTRL_GATE_DISABLE` reader - 0:0\\]
ac slice slv_dly_control block power reduction disable."]
pub type PhyAcSlvDlyCtrlGateDisableR = crate::BitReader;
#[doc = "Field `PHY_AC_SLV_DLY_CTRL_GATE_DISABLE` writer - 0:0\\]
ac slice slv_dly_control block power reduction disable."]
pub type PhyAcSlvDlyCtrlGateDisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ac slice slv_dly_control block power reduction disable."]
    #[inline(always)]
    pub fn phy_ac_slv_dly_ctrl_gate_disable(&self) -> PhyAcSlvDlyCtrlGateDisableR {
        PhyAcSlvDlyCtrlGateDisableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ac slice slv_dly_control block power reduction disable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_slv_dly_ctrl_gate_disable(
        &mut self,
    ) -> PhyAcSlvDlyCtrlGateDisableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1358Spec> {
        PhyAcSlvDlyCtrlGateDisableW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1358\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1358::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1358::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1358Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1358Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1358::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1358Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1358::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1358Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1358 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1358Spec {
    const RESET_VALUE: u32 = 0;
}
