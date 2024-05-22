#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_258` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy258Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_258` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy258Spec>;
#[doc = "Field `PHY_RDDQS_GATE_BYPASS_SLAVE_DELAY_1` reader - 9:0\\]
Read DQS bypass mode slave delay setting for slice 1."]
pub type PhyRddqsGateBypassSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_GATE_BYPASS_SLAVE_DELAY_1` writer - 9:0\\]
Read DQS bypass mode slave delay setting for slice 1."]
pub type PhyRddqsGateBypassSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_BYPASS_TWO_CYC_PREAMBLE_1` reader - 17:16\\]
Two_cycle_preamble for bypass mode for slice 1."]
pub type PhyBypassTwoCycPreamble1R = crate::FieldReader;
#[doc = "Field `PHY_BYPASS_TWO_CYC_PREAMBLE_1` writer - 17:16\\]
Two_cycle_preamble for bypass mode for slice 1."]
pub type PhyBypassTwoCycPreamble1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CLK_BYPASS_OVERRIDE_1` reader - 24:24\\]
Bypass mode override setting for slice 1."]
pub type PhyClkBypassOverride1R = crate::BitReader;
#[doc = "Field `PHY_CLK_BYPASS_OVERRIDE_1` writer - 24:24\\]
Bypass mode override setting for slice 1."]
pub type PhyClkBypassOverride1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Read DQS bypass mode slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_gate_bypass_slave_delay_1(&self) -> PhyRddqsGateBypassSlaveDelay1R {
        PhyRddqsGateBypassSlaveDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Two_cycle_preamble for bypass mode for slice 1."]
    #[inline(always)]
    pub fn phy_bypass_two_cyc_preamble_1(&self) -> PhyBypassTwoCycPreamble1R {
        PhyBypassTwoCycPreamble1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Bypass mode override setting for slice 1."]
    #[inline(always)]
    pub fn phy_clk_bypass_override_1(&self) -> PhyClkBypassOverride1R {
        PhyClkBypassOverride1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Read DQS bypass mode slave delay setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_gate_bypass_slave_delay_1(
        &mut self,
    ) -> PhyRddqsGateBypassSlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy258Spec> {
        PhyRddqsGateBypassSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Two_cycle_preamble for bypass mode for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_bypass_two_cyc_preamble_1(
        &mut self,
    ) -> PhyBypassTwoCycPreamble1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy258Spec> {
        PhyBypassTwoCycPreamble1W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Bypass mode override setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_bypass_override_1(
        &mut self,
    ) -> PhyClkBypassOverride1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy258Spec> {
        PhyClkBypassOverride1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_258\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_258::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_258::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy258Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy258Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_258::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy258Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_258::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy258Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_258 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy258Spec {
    const RESET_VALUE: u32 = 0;
}
