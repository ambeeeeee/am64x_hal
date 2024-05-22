#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_108` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy108Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_108` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy108Spec>;
#[doc = "Field `PHY_CLK_WRDQ6_SLAVE_DELAY_0` reader - 10:0\\]
Write clock slave delay setting for DQ6 for slice 0."]
pub type PhyClkWrdq6SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ6_SLAVE_DELAY_0` writer - 10:0\\]
Write clock slave delay setting for DQ6 for slice 0."]
pub type PhyClkWrdq6SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQ7_SLAVE_DELAY_0` reader - 26:16\\]
Write clock slave delay setting for DQ7 for slice 0."]
pub type PhyClkWrdq7SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ7_SLAVE_DELAY_0` writer - 26:16\\]
Write clock slave delay setting for DQ7 for slice 0."]
pub type PhyClkWrdq7SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Write clock slave delay setting for DQ6 for slice 0."]
    #[inline(always)]
    pub fn phy_clk_wrdq6_slave_delay_0(&self) -> PhyClkWrdq6SlaveDelay0R {
        PhyClkWrdq6SlaveDelay0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Write clock slave delay setting for DQ7 for slice 0."]
    #[inline(always)]
    pub fn phy_clk_wrdq7_slave_delay_0(&self) -> PhyClkWrdq7SlaveDelay0R {
        PhyClkWrdq7SlaveDelay0R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Write clock slave delay setting for DQ6 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq6_slave_delay_0(
        &mut self,
    ) -> PhyClkWrdq6SlaveDelay0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy108Spec> {
        PhyClkWrdq6SlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Write clock slave delay setting for DQ7 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq7_slave_delay_0(
        &mut self,
    ) -> PhyClkWrdq7SlaveDelay0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy108Spec> {
        PhyClkWrdq7SlaveDelay0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_108\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_108::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_108::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy108Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_108::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy108Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_108::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy108Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_108 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy108Spec {
    const RESET_VALUE: u32 = 0;
}
