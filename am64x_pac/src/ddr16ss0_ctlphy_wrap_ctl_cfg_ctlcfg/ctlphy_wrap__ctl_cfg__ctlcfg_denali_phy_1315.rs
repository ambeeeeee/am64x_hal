#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1315` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1315Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1315` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1315Spec>;
#[doc = "Field `PHY_PLL_DESKEWCALIN_0` reader - 11:0\\]
PHY TOP level PLL_0 deskewcal in values."]
pub type PhyPllDeskewcalin0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PLL_DESKEWCALIN_0` writer - 11:0\\]
PHY TOP level PLL_0 deskewcal in values."]
pub type PhyPllDeskewcalin0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_LP4_BOOT_PLL_DESKEWCALIN_0` reader - 27:16\\]
PHY TOP level PLL_0 lpddr4 boot deskewcal in values."]
pub type PhyLp4BootPllDeskewcalin0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_LP4_BOOT_PLL_DESKEWCALIN_0` writer - 27:16\\]
PHY TOP level PLL_0 lpddr4 boot deskewcal in values."]
pub type PhyLp4BootPllDeskewcalin0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
PHY TOP level PLL_0 deskewcal in values."]
    #[inline(always)]
    pub fn phy_pll_deskewcalin_0(&self) -> PhyPllDeskewcalin0R {
        PhyPllDeskewcalin0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
PHY TOP level PLL_0 lpddr4 boot deskewcal in values."]
    #[inline(always)]
    pub fn phy_lp4_boot_pll_deskewcalin_0(&self) -> PhyLp4BootPllDeskewcalin0R {
        PhyLp4BootPllDeskewcalin0R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
PHY TOP level PLL_0 deskewcal in values."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pll_deskewcalin_0(
        &mut self,
    ) -> PhyPllDeskewcalin0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1315Spec> {
        PhyPllDeskewcalin0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
PHY TOP level PLL_0 lpddr4 boot deskewcal in values."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_pll_deskewcalin_0(
        &mut self,
    ) -> PhyLp4BootPllDeskewcalin0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1315Spec> {
        PhyLp4BootPllDeskewcalin0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1315\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1315::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1315::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1315Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1315Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1315::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1315Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1315::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1315Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1315 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1315Spec {
    const RESET_VALUE: u32 = 0;
}
