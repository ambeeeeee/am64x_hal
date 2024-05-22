#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_512` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy512Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_512` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy512Spec>;
#[doc = "Field `PHY_ADR_CLK_WR_BYPASS_SLAVE_DELAY_0` reader - 10:0\\]
Command/Address clock bypass mode slave delay setting for address slice 0."]
pub type PhyAdrClkWrBypassSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CLK_WR_BYPASS_SLAVE_DELAY_0` writer - 10:0\\]
Command/Address clock bypass mode slave delay setting for address slice 0."]
pub type PhyAdrClkWrBypassSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR_CLK_BYPASS_OVERRIDE_0` reader - 16:16\\]
Bypass mode override setting for address slice 0. Set to 1 to enable."]
pub type PhyAdrClkBypassOverride0R = crate::BitReader;
#[doc = "Field `PHY_ADR_CLK_BYPASS_OVERRIDE_0` writer - 16:16\\]
Bypass mode override setting for address slice 0. Set to 1 to enable."]
pub type PhyAdrClkBypassOverride0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_ADR_MANUAL_CLEAR_0` reader - 26:24\\]
Manual reset/clear of internal logic for address slice 0. Bit \\[0\\]
is reset of master delay min/max lock values. Bit \\[1\\]
is manual reset of master delay unlock counter. Bit \\[2\\]
clears the loopback error/results registers. Set each bit to 1 to reset. WRITE-ONLY"]
pub type ScPhyAdrManualClear0R = crate::FieldReader;
#[doc = "Field `SC_PHY_ADR_MANUAL_CLEAR_0` writer - 26:24\\]
Manual reset/clear of internal logic for address slice 0. Bit \\[0\\]
is reset of master delay min/max lock values. Bit \\[1\\]
is manual reset of master delay unlock counter. Bit \\[2\\]
clears the loopback error/results registers. Set each bit to 1 to reset. WRITE-ONLY"]
pub type ScPhyAdrManualClear0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Command/Address clock bypass mode slave delay setting for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_clk_wr_bypass_slave_delay_0(&self) -> PhyAdrClkWrBypassSlaveDelay0R {
        PhyAdrClkWrBypassSlaveDelay0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Bypass mode override setting for address slice 0. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_adr_clk_bypass_override_0(&self) -> PhyAdrClkBypassOverride0R {
        PhyAdrClkBypassOverride0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Manual reset/clear of internal logic for address slice 0. Bit \\[0\\]
is reset of master delay min/max lock values. Bit \\[1\\]
is manual reset of master delay unlock counter. Bit \\[2\\]
clears the loopback error/results registers. Set each bit to 1 to reset. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_adr_manual_clear_0(&self) -> ScPhyAdrManualClear0R {
        ScPhyAdrManualClear0R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Command/Address clock bypass mode slave delay setting for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_clk_wr_bypass_slave_delay_0(
        &mut self,
    ) -> PhyAdrClkWrBypassSlaveDelay0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy512Spec> {
        PhyAdrClkWrBypassSlaveDelay0W::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Bypass mode override setting for address slice 0. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_clk_bypass_override_0(
        &mut self,
    ) -> PhyAdrClkBypassOverride0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy512Spec> {
        PhyAdrClkBypassOverride0W::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Manual reset/clear of internal logic for address slice 0. Bit \\[0\\]
is reset of master delay min/max lock values. Bit \\[1\\]
is manual reset of master delay unlock counter. Bit \\[2\\]
clears the loopback error/results registers. Set each bit to 1 to reset. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_manual_clear_0(
        &mut self,
    ) -> ScPhyAdrManualClear0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy512Spec> {
        ScPhyAdrManualClear0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_512\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_512::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_512::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy512Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy512Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_512::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy512Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_512::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy512Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_512 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy512Spec {
    const RESET_VALUE: u32 = 0;
}
