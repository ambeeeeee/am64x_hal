#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_371` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy371Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_371` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy371Spec>;
#[doc = "Field `PHY_RDDQS_DQ4_FALL_SLAVE_DELAY_1` reader - 9:0\\]
Falling edge read DQS slave delay setting for DQ4 for slice 1."]
pub type PhyRddqsDq4FallSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ4_FALL_SLAVE_DELAY_1` writer - 9:0\\]
Falling edge read DQS slave delay setting for DQ4 for slice 1."]
pub type PhyRddqsDq4FallSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ5_RISE_SLAVE_DELAY_1` reader - 25:16\\]
Rising edge read DQS slave delay setting for DQ5 for slice 1."]
pub type PhyRddqsDq5RiseSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ5_RISE_SLAVE_DELAY_1` writer - 25:16\\]
Rising edge read DQS slave delay setting for DQ5 for slice 1."]
pub type PhyRddqsDq5RiseSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Falling edge read DQS slave delay setting for DQ4 for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_dq4_fall_slave_delay_1(&self) -> PhyRddqsDq4FallSlaveDelay1R {
        PhyRddqsDq4FallSlaveDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Rising edge read DQS slave delay setting for DQ5 for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_dq5_rise_slave_delay_1(&self) -> PhyRddqsDq5RiseSlaveDelay1R {
        PhyRddqsDq5RiseSlaveDelay1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Falling edge read DQS slave delay setting for DQ4 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq4_fall_slave_delay_1(
        &mut self,
    ) -> PhyRddqsDq4FallSlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy371Spec> {
        PhyRddqsDq4FallSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Rising edge read DQS slave delay setting for DQ5 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq5_rise_slave_delay_1(
        &mut self,
    ) -> PhyRddqsDq5RiseSlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy371Spec> {
        PhyRddqsDq5RiseSlaveDelay1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_371\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_371::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_371::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy371Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy371Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_371::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy371Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_371::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy371Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_371 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy371Spec {
    const RESET_VALUE: u32 = 0;
}
