#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_8` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy8Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_8` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy8Spec>;
#[doc = "Field `PHY_LPBK_DFX_TIMEOUT_EN_0` reader - 0:0\\]
Loopback read only test timeout mechanism enable for slice 0."]
pub type PhyLpbkDfxTimeoutEn0R = crate::BitReader;
#[doc = "Field `PHY_LPBK_DFX_TIMEOUT_EN_0` writer - 0:0\\]
Loopback read only test timeout mechanism enable for slice 0."]
pub type PhyLpbkDfxTimeoutEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_GATE_DELAY_COMP_DISABLE_0` reader - 8:8\\]
use the control whether to compensate half_cycle when gate_slave_delay is larger than half_cycle for the gate close for slice 0."]
pub type PhyGateDelayCompDisable0R = crate::BitReader;
#[doc = "Field `PHY_GATE_DELAY_COMP_DISABLE_0` writer - 8:8\\]
use the control whether to compensate half_cycle when gate_slave_delay is larger than half_cycle for the gate close for slice 0."]
pub type PhyGateDelayCompDisable0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Loopback read only test timeout mechanism enable for slice 0."]
    #[inline(always)]
    pub fn phy_lpbk_dfx_timeout_en_0(&self) -> PhyLpbkDfxTimeoutEn0R {
        PhyLpbkDfxTimeoutEn0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
use the control whether to compensate half_cycle when gate_slave_delay is larger than half_cycle for the gate close for slice 0."]
    #[inline(always)]
    pub fn phy_gate_delay_comp_disable_0(&self) -> PhyGateDelayCompDisable0R {
        PhyGateDelayCompDisable0R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Loopback read only test timeout mechanism enable for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpbk_dfx_timeout_en_0(
        &mut self,
    ) -> PhyLpbkDfxTimeoutEn0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy8Spec> {
        PhyLpbkDfxTimeoutEn0W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
use the control whether to compensate half_cycle when gate_slave_delay is larger than half_cycle for the gate close for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gate_delay_comp_disable_0(
        &mut self,
    ) -> PhyGateDelayCompDisable0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy8Spec> {
        PhyGateDelayCompDisable0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy8Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_8::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy8Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_8::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_8 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy8Spec {
    const RESET_VALUE: u32 = 0;
}
