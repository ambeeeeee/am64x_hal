#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1318` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1318Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1318` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1318Spec>;
#[doc = "Field `PHY_PLL_DESKEWCALIN_1` reader - 11:0\\]
PHY TOP level PLL_1 deskewcal in values."]
pub type PhyPllDeskewcalin1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PLL_DESKEWCALIN_1` writer - 11:0\\]
PHY TOP level PLL_1 deskewcal in values."]
pub type PhyPllDeskewcalin1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_LP4_BOOT_PLL_DESKEWCALIN_1` reader - 27:16\\]
PHY TOP level PLL_1 lpddr4 boot deskewcal in values."]
pub type PhyLp4BootPllDeskewcalin1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_LP4_BOOT_PLL_DESKEWCALIN_1` writer - 27:16\\]
PHY TOP level PLL_1 lpddr4 boot deskewcal in values."]
pub type PhyLp4BootPllDeskewcalin1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
PHY TOP level PLL_1 deskewcal in values."]
    #[inline(always)]
    pub fn phy_pll_deskewcalin_1(&self) -> PhyPllDeskewcalin1R {
        PhyPllDeskewcalin1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
PHY TOP level PLL_1 lpddr4 boot deskewcal in values."]
    #[inline(always)]
    pub fn phy_lp4_boot_pll_deskewcalin_1(&self) -> PhyLp4BootPllDeskewcalin1R {
        PhyLp4BootPllDeskewcalin1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
PHY TOP level PLL_1 deskewcal in values."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pll_deskewcalin_1(
        &mut self,
    ) -> PhyPllDeskewcalin1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1318Spec> {
        PhyPllDeskewcalin1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
PHY TOP level PLL_1 lpddr4 boot deskewcal in values."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_pll_deskewcalin_1(
        &mut self,
    ) -> PhyLp4BootPllDeskewcalin1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1318Spec> {
        PhyLp4BootPllDeskewcalin1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1318\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1318::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1318::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1318Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1318Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1318::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1318Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1318::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1318Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1318 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1318Spec {
    const RESET_VALUE: u32 = 0;
}
