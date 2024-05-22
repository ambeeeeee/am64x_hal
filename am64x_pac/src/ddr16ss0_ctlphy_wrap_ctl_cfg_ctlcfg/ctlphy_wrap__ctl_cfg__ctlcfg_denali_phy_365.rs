#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_365` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy365Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_365` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy365Spec>;
#[doc = "Field `PHY_CLK_WRDM_SLAVE_DELAY_1` reader - 10:0\\]
Write clock slave delay setting for DM for slice 1."]
pub type PhyClkWrdmSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDM_SLAVE_DELAY_1` writer - 10:0\\]
Write clock slave delay setting for DM for slice 1."]
pub type PhyClkWrdmSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQS_SLAVE_DELAY_1` reader - 25:16\\]
Write clock slave delay setting for DQS for slice 1."]
pub type PhyClkWrdqsSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQS_SLAVE_DELAY_1` writer - 25:16\\]
Write clock slave delay setting for DQS for slice 1."]
pub type PhyClkWrdqsSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Write clock slave delay setting for DM for slice 1."]
    #[inline(always)]
    pub fn phy_clk_wrdm_slave_delay_1(&self) -> PhyClkWrdmSlaveDelay1R {
        PhyClkWrdmSlaveDelay1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Write clock slave delay setting for DQS for slice 1."]
    #[inline(always)]
    pub fn phy_clk_wrdqs_slave_delay_1(&self) -> PhyClkWrdqsSlaveDelay1R {
        PhyClkWrdqsSlaveDelay1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Write clock slave delay setting for DM for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdm_slave_delay_1(
        &mut self,
    ) -> PhyClkWrdmSlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy365Spec> {
        PhyClkWrdmSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Write clock slave delay setting for DQS for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdqs_slave_delay_1(
        &mut self,
    ) -> PhyClkWrdqsSlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy365Spec> {
        PhyClkWrdqsSlaveDelay1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_365\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_365::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_365::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy365Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy365Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_365::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy365Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_365::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy365Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_365 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy365Spec {
    const RESET_VALUE: u32 = 0;
}
