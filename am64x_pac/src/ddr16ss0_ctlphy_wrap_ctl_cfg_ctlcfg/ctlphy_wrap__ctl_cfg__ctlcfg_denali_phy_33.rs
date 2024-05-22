#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_33` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy33Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_33` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy33Spec>;
#[doc = "Field `PHY_WDQLVL_CLK_JITTER_TOLERANCE_0` reader - 7:0\\]
Defines the minimum gap requirment for the LE and TE window for slice 0."]
pub type PhyWdqlvlClkJitterTolerance0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_CLK_JITTER_TOLERANCE_0` writer - 7:0\\]
Defines the minimum gap requirment for the LE and TE window for slice 0."]
pub type PhyWdqlvlClkJitterTolerance0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WDQLVL_BURST_CNT_0` reader - 13:8\\]
Defines the write/read burst length in bytes during the write data leveling sequence for slice 0."]
pub type PhyWdqlvlBurstCnt0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_BURST_CNT_0` writer - 13:8\\]
Defines the write/read burst length in bytes during the write data leveling sequence for slice 0."]
pub type PhyWdqlvlBurstCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_WDQLVL_PATT_0` reader - 18:16\\]
Defines the training patterns to be used during the write data leveling sequence for slice 0. Bit \\[0\\]
corresponds to the LFSR data training pattern. Bit \\[1\\]
corresponds to the CLK data training pattern. Bit \\[2\\]
corresponds to user-defined data pattern training. If multiple bits are set, the training for each of the chosen patterns will be executed and the settings that give the smallest data valid window eye will be chosen."]
pub type PhyWdqlvlPatt0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_PATT_0` writer - 18:16\\]
Defines the training patterns to be used during the write data leveling sequence for slice 0. Bit \\[0\\]
corresponds to the LFSR data training pattern. Bit \\[1\\]
corresponds to the CLK data training pattern. Bit \\[2\\]
corresponds to user-defined data pattern training. If multiple bits are set, the training for each of the chosen patterns will be executed and the settings that give the smallest data valid window eye will be chosen."]
pub type PhyWdqlvlPatt0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the minimum gap requirment for the LE and TE window for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_clk_jitter_tolerance_0(&self) -> PhyWdqlvlClkJitterTolerance0R {
        PhyWdqlvlClkJitterTolerance0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Defines the write/read burst length in bytes during the write data leveling sequence for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_burst_cnt_0(&self) -> PhyWdqlvlBurstCnt0R {
        PhyWdqlvlBurstCnt0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Defines the training patterns to be used during the write data leveling sequence for slice 0. Bit \\[0\\]
corresponds to the LFSR data training pattern. Bit \\[1\\]
corresponds to the CLK data training pattern. Bit \\[2\\]
corresponds to user-defined data pattern training. If multiple bits are set, the training for each of the chosen patterns will be executed and the settings that give the smallest data valid window eye will be chosen."]
    #[inline(always)]
    pub fn phy_wdqlvl_patt_0(&self) -> PhyWdqlvlPatt0R {
        PhyWdqlvlPatt0R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the minimum gap requirment for the LE and TE window for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_clk_jitter_tolerance_0(
        &mut self,
    ) -> PhyWdqlvlClkJitterTolerance0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy33Spec> {
        PhyWdqlvlClkJitterTolerance0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Defines the write/read burst length in bytes during the write data leveling sequence for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_burst_cnt_0(
        &mut self,
    ) -> PhyWdqlvlBurstCnt0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy33Spec> {
        PhyWdqlvlBurstCnt0W::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Defines the training patterns to be used during the write data leveling sequence for slice 0. Bit \\[0\\]
corresponds to the LFSR data training pattern. Bit \\[1\\]
corresponds to the CLK data training pattern. Bit \\[2\\]
corresponds to user-defined data pattern training. If multiple bits are set, the training for each of the chosen patterns will be executed and the settings that give the smallest data valid window eye will be chosen."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_patt_0(
        &mut self,
    ) -> PhyWdqlvlPatt0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy33Spec> {
        PhyWdqlvlPatt0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy33Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_33::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy33Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_33::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_33 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy33Spec {
    const RESET_VALUE: u32 = 0;
}
