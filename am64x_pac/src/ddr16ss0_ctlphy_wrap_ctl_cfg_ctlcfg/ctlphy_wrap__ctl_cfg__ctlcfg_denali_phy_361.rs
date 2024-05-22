#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_361` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy361Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_361` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy361Spec>;
#[doc = "Field `PHY_CLK_WRDQ0_SLAVE_DELAY_1` reader - 10:0\\]
Write clock slave delay setting for DQ0 for slice 1."]
pub type PhyClkWrdq0SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ0_SLAVE_DELAY_1` writer - 10:0\\]
Write clock slave delay setting for DQ0 for slice 1."]
pub type PhyClkWrdq0SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQ1_SLAVE_DELAY_1` reader - 26:16\\]
Write clock slave delay setting for DQ1 for slice 1."]
pub type PhyClkWrdq1SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ1_SLAVE_DELAY_1` writer - 26:16\\]
Write clock slave delay setting for DQ1 for slice 1."]
pub type PhyClkWrdq1SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Write clock slave delay setting for DQ0 for slice 1."]
    #[inline(always)]
    pub fn phy_clk_wrdq0_slave_delay_1(&self) -> PhyClkWrdq0SlaveDelay1R {
        PhyClkWrdq0SlaveDelay1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Write clock slave delay setting for DQ1 for slice 1."]
    #[inline(always)]
    pub fn phy_clk_wrdq1_slave_delay_1(&self) -> PhyClkWrdq1SlaveDelay1R {
        PhyClkWrdq1SlaveDelay1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Write clock slave delay setting for DQ0 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq0_slave_delay_1(
        &mut self,
    ) -> PhyClkWrdq0SlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy361Spec> {
        PhyClkWrdq0SlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Write clock slave delay setting for DQ1 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq1_slave_delay_1(
        &mut self,
    ) -> PhyClkWrdq1SlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy361Spec> {
        PhyClkWrdq1SlaveDelay1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_361\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_361::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_361::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy361Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy361Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_361::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy361Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_361::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy361Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_361 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy361Spec {
    const RESET_VALUE: u32 = 0;
}
