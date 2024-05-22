#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1319` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1319Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1319` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1319Spec>;
#[doc = "Field `PHY_PLL_REFOUT_SEL` reader - 0:0\\]
PHY PLL refout select."]
pub type PhyPllRefoutSelR = crate::BitReader;
#[doc = "Field `PHY_PLL_REFOUT_SEL` writer - 0:0\\]
PHY PLL refout select."]
pub type PhyPllRefoutSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP4_BOOT_LOW_FREQ_SEL` reader - 8:8\\]
Control the PLL domain enter/exit from the negative clock edge for LPDDR4 boot frequency."]
pub type PhyLp4BootLowFreqSelR = crate::BitReader;
#[doc = "Field `PHY_LP4_BOOT_LOW_FREQ_SEL` writer - 8:8\\]
Control the PLL domain enter/exit from the negative clock edge for LPDDR4 boot frequency."]
pub type PhyLp4BootLowFreqSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TCKSRE_WAIT` reader - 19:16\\]
Specifies the number of cycles the PHY should wait before turning off the PLL for a deep sleep or DFS event."]
pub type PhyTcksreWaitR = crate::FieldReader;
#[doc = "Field `PHY_TCKSRE_WAIT` writer - 19:16\\]
Specifies the number of cycles the PHY should wait before turning off the PLL for a deep sleep or DFS event."]
pub type PhyTcksreWaitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP_WAKEUP` reader - 31:24\\]
Specifies the number of cycles the PHY takes to wakeup in low power mode."]
pub type PhyLpWakeupR = crate::FieldReader;
#[doc = "Field `PHY_LP_WAKEUP` writer - 31:24\\]
Specifies the number of cycles the PHY takes to wakeup in low power mode."]
pub type PhyLpWakeupW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
PHY PLL refout select."]
    #[inline(always)]
    pub fn phy_pll_refout_sel(&self) -> PhyPllRefoutSelR {
        PhyPllRefoutSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Control the PLL domain enter/exit from the negative clock edge for LPDDR4 boot frequency."]
    #[inline(always)]
    pub fn phy_lp4_boot_low_freq_sel(&self) -> PhyLp4BootLowFreqSelR {
        PhyLp4BootLowFreqSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Specifies the number of cycles the PHY should wait before turning off the PLL for a deep sleep or DFS event."]
    #[inline(always)]
    pub fn phy_tcksre_wait(&self) -> PhyTcksreWaitR {
        PhyTcksreWaitR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Specifies the number of cycles the PHY takes to wakeup in low power mode."]
    #[inline(always)]
    pub fn phy_lp_wakeup(&self) -> PhyLpWakeupR {
        PhyLpWakeupR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
PHY PLL refout select."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pll_refout_sel(
        &mut self,
    ) -> PhyPllRefoutSelW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1319Spec> {
        PhyPllRefoutSelW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Control the PLL domain enter/exit from the negative clock edge for LPDDR4 boot frequency."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_low_freq_sel(
        &mut self,
    ) -> PhyLp4BootLowFreqSelW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1319Spec> {
        PhyLp4BootLowFreqSelW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Specifies the number of cycles the PHY should wait before turning off the PLL for a deep sleep or DFS event."]
    #[inline(always)]
    #[must_use]
    pub fn phy_tcksre_wait(&mut self) -> PhyTcksreWaitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1319Spec> {
        PhyTcksreWaitW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Specifies the number of cycles the PHY takes to wakeup in low power mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_wakeup(&mut self) -> PhyLpWakeupW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1319Spec> {
        PhyLpWakeupW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1319\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1319::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1319::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1319Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1319Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1319::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1319Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1319::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1319Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1319 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1319Spec {
    const RESET_VALUE: u32 = 0;
}
