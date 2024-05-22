#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1311` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1311Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1311` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1311Spec>;
#[doc = "Field `PHY_LP4_BOOT_PLL_CTRL` reader - 12:0\\]
PHY deskew PLL controls for LPDDR4 boot frequency."]
pub type PhyLp4BootPllCtrlR = crate::FieldReader<u16>;
#[doc = "Field `PHY_LP4_BOOT_PLL_CTRL` writer - 12:0\\]
PHY deskew PLL controls for LPDDR4 boot frequency."]
pub type PhyLp4BootPllCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `PHY_PLL_CTRL_OVERRIDE` reader - 31:16\\]
Individual PHY clock PLL control overrides."]
pub type PhyPllCtrlOverrideR = crate::FieldReader<u16>;
#[doc = "Field `PHY_PLL_CTRL_OVERRIDE` writer - 31:16\\]
Individual PHY clock PLL control overrides."]
pub type PhyPllCtrlOverrideW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:12 - 12:0\\]
PHY deskew PLL controls for LPDDR4 boot frequency."]
    #[inline(always)]
    pub fn phy_lp4_boot_pll_ctrl(&self) -> PhyLp4BootPllCtrlR {
        PhyLp4BootPllCtrlR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Individual PHY clock PLL control overrides."]
    #[inline(always)]
    pub fn phy_pll_ctrl_override(&self) -> PhyPllCtrlOverrideR {
        PhyPllCtrlOverrideR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 12:0\\]
PHY deskew PLL controls for LPDDR4 boot frequency."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_pll_ctrl(
        &mut self,
    ) -> PhyLp4BootPllCtrlW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1311Spec> {
        PhyLp4BootPllCtrlW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Individual PHY clock PLL control overrides."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pll_ctrl_override(
        &mut self,
    ) -> PhyPllCtrlOverrideW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1311Spec> {
        PhyPllCtrlOverrideW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1311\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1311::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1311::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1311Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1311Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1311::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1311Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1311::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1311Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1311 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1311Spec {
    const RESET_VALUE: u32 = 0;
}
