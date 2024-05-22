#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1024` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1024Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1024` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1024Spec>;
#[doc = "Field `PHY_ADR_CLK_WR_BYPASS_SLAVE_DELAY_2` reader - 10:0\\]
Command/Address clock bypass mode slave delay setting for address slice 2."]
pub type PhyAdrClkWrBypassSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CLK_WR_BYPASS_SLAVE_DELAY_2` writer - 10:0\\]
Command/Address clock bypass mode slave delay setting for address slice 2."]
pub type PhyAdrClkWrBypassSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR_CLK_BYPASS_OVERRIDE_2` reader - 16:16\\]
Bypass mode override setting for address slice 2. Set to 1 to enable."]
pub type PhyAdrClkBypassOverride2R = crate::BitReader;
#[doc = "Field `PHY_ADR_CLK_BYPASS_OVERRIDE_2` writer - 16:16\\]
Bypass mode override setting for address slice 2. Set to 1 to enable."]
pub type PhyAdrClkBypassOverride2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_ADR_MANUAL_CLEAR_2` reader - 26:24\\]
Manual reset/clear of internal logic for address slice 2. Bit \\[0\\]
is reset of master delay min/max lock values. Bit \\[1\\]
is manual reset of master delay unlock counter. Bit \\[2\\]
clears the loopback error/results registers. Set each bit to 1 to reset. WRITE-ONLY"]
pub type ScPhyAdrManualClear2R = crate::FieldReader;
#[doc = "Field `SC_PHY_ADR_MANUAL_CLEAR_2` writer - 26:24\\]
Manual reset/clear of internal logic for address slice 2. Bit \\[0\\]
is reset of master delay min/max lock values. Bit \\[1\\]
is manual reset of master delay unlock counter. Bit \\[2\\]
clears the loopback error/results registers. Set each bit to 1 to reset. WRITE-ONLY"]
pub type ScPhyAdrManualClear2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Command/Address clock bypass mode slave delay setting for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_clk_wr_bypass_slave_delay_2(&self) -> PhyAdrClkWrBypassSlaveDelay2R {
        PhyAdrClkWrBypassSlaveDelay2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Bypass mode override setting for address slice 2. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_adr_clk_bypass_override_2(&self) -> PhyAdrClkBypassOverride2R {
        PhyAdrClkBypassOverride2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Manual reset/clear of internal logic for address slice 2. Bit \\[0\\]
is reset of master delay min/max lock values. Bit \\[1\\]
is manual reset of master delay unlock counter. Bit \\[2\\]
clears the loopback error/results registers. Set each bit to 1 to reset. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_adr_manual_clear_2(&self) -> ScPhyAdrManualClear2R {
        ScPhyAdrManualClear2R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Command/Address clock bypass mode slave delay setting for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_clk_wr_bypass_slave_delay_2(
        &mut self,
    ) -> PhyAdrClkWrBypassSlaveDelay2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1024Spec> {
        PhyAdrClkWrBypassSlaveDelay2W::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Bypass mode override setting for address slice 2. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_clk_bypass_override_2(
        &mut self,
    ) -> PhyAdrClkBypassOverride2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1024Spec> {
        PhyAdrClkBypassOverride2W::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Manual reset/clear of internal logic for address slice 2. Bit \\[0\\]
is reset of master delay min/max lock values. Bit \\[1\\]
is manual reset of master delay unlock counter. Bit \\[2\\]
clears the loopback error/results registers. Set each bit to 1 to reset. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_manual_clear_2(
        &mut self,
    ) -> ScPhyAdrManualClear2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1024Spec> {
        ScPhyAdrManualClear2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1024\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1024::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1024::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1024Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1024::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1024Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1024::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1024Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1024 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1024Spec {
    const RESET_VALUE: u32 = 0;
}
