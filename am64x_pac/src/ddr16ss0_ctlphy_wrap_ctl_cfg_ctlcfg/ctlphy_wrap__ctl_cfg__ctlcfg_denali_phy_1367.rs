#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1367` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1367Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1367` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1367Spec>;
#[doc = "Field `PHY_DS1_DQS_ERR_COUNTER` reader - 31:0\\]
PHY DATA SLICE 1 DQS ERROR counter."]
pub type PhyDs1DqsErrCounterR = crate::FieldReader<u32>;
#[doc = "Field `PHY_DS1_DQS_ERR_COUNTER` writer - 31:0\\]
PHY DATA SLICE 1 DQS ERROR counter."]
pub type PhyDs1DqsErrCounterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
PHY DATA SLICE 1 DQS ERROR counter."]
    #[inline(always)]
    pub fn phy_ds1_dqs_err_counter(&self) -> PhyDs1DqsErrCounterR {
        PhyDs1DqsErrCounterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
PHY DATA SLICE 1 DQS ERROR counter."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ds1_dqs_err_counter(
        &mut self,
    ) -> PhyDs1DqsErrCounterW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1367Spec> {
        PhyDs1DqsErrCounterW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1367\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1367::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1367::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1367Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1367Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1367::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1367Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1367::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1367Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1367 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1367Spec {
    const RESET_VALUE: u32 = 0;
}
