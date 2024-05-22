#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_2` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy2Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_2` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy2Spec>;
#[doc = "Field `PHY_RDDQS_GATE_BYPASS_SLAVE_DELAY_0` reader - 9:0\\]
Read DQS bypass mode slave delay setting for slice 0."]
pub type PhyRddqsGateBypassSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_GATE_BYPASS_SLAVE_DELAY_0` writer - 9:0\\]
Read DQS bypass mode slave delay setting for slice 0."]
pub type PhyRddqsGateBypassSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_BYPASS_TWO_CYC_PREAMBLE_0` reader - 17:16\\]
Two_cycle_preamble for bypass mode for slice 0."]
pub type PhyBypassTwoCycPreamble0R = crate::FieldReader;
#[doc = "Field `PHY_BYPASS_TWO_CYC_PREAMBLE_0` writer - 17:16\\]
Two_cycle_preamble for bypass mode for slice 0."]
pub type PhyBypassTwoCycPreamble0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CLK_BYPASS_OVERRIDE_0` reader - 24:24\\]
Bypass mode override setting for slice 0."]
pub type PhyClkBypassOverride0R = crate::BitReader;
#[doc = "Field `PHY_CLK_BYPASS_OVERRIDE_0` writer - 24:24\\]
Bypass mode override setting for slice 0."]
pub type PhyClkBypassOverride0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Read DQS bypass mode slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_gate_bypass_slave_delay_0(&self) -> PhyRddqsGateBypassSlaveDelay0R {
        PhyRddqsGateBypassSlaveDelay0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Two_cycle_preamble for bypass mode for slice 0."]
    #[inline(always)]
    pub fn phy_bypass_two_cyc_preamble_0(&self) -> PhyBypassTwoCycPreamble0R {
        PhyBypassTwoCycPreamble0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Bypass mode override setting for slice 0."]
    #[inline(always)]
    pub fn phy_clk_bypass_override_0(&self) -> PhyClkBypassOverride0R {
        PhyClkBypassOverride0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Read DQS bypass mode slave delay setting for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_gate_bypass_slave_delay_0(
        &mut self,
    ) -> PhyRddqsGateBypassSlaveDelay0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy2Spec> {
        PhyRddqsGateBypassSlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Two_cycle_preamble for bypass mode for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_bypass_two_cyc_preamble_0(
        &mut self,
    ) -> PhyBypassTwoCycPreamble0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy2Spec> {
        PhyBypassTwoCycPreamble0W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Bypass mode override setting for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_bypass_override_0(
        &mut self,
    ) -> PhyClkBypassOverride0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy2Spec> {
        PhyClkBypassOverride0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy2Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_2::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_2::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_2 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy2Spec {
    const RESET_VALUE: u32 = 0;
}
