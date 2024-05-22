#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1332` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1332Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1332` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1332Spec>;
#[doc = "Field `PHY_ADRCTL_RX_CAL` reader - 9:0\\]
PHY address/control RX calibration controls."]
pub type PhyAdrctlRxCalR = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADRCTL_RX_CAL` writer - 9:0\\]
PHY address/control RX calibration controls."]
pub type PhyAdrctlRxCalW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_ADRCTL_LP3_RX_CAL` reader - 28:16\\]
PHY CKE/RESET_N RX calibration controls."]
pub type PhyAdrctlLp3RxCalR = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADRCTL_LP3_RX_CAL` writer - 28:16\\]
PHY CKE/RESET_N RX calibration controls."]
pub type PhyAdrctlLp3RxCalW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
PHY address/control RX calibration controls."]
    #[inline(always)]
    pub fn phy_adrctl_rx_cal(&self) -> PhyAdrctlRxCalR {
        PhyAdrctlRxCalR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:28 - 28:16\\]
PHY CKE/RESET_N RX calibration controls."]
    #[inline(always)]
    pub fn phy_adrctl_lp3_rx_cal(&self) -> PhyAdrctlLp3RxCalR {
        PhyAdrctlLp3RxCalR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
PHY address/control RX calibration controls."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_rx_cal(
        &mut self,
    ) -> PhyAdrctlRxCalW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1332Spec> {
        PhyAdrctlRxCalW::new(self, 0)
    }
    #[doc = "Bits 16:28 - 28:16\\]
PHY CKE/RESET_N RX calibration controls."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_lp3_rx_cal(
        &mut self,
    ) -> PhyAdrctlLp3RxCalW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1332Spec> {
        PhyAdrctlLp3RxCalW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1332\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1332::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1332::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1332Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1332Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1332::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1332Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1332::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1332Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1332 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1332Spec {
    const RESET_VALUE: u32 = 0;
}
