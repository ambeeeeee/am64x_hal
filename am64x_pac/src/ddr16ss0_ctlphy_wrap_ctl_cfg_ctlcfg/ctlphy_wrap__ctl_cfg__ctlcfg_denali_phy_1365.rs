#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1365` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1365Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1365` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1365Spec>;
#[doc = "Field `PHY_CA_PARITY_ERR_PULSE_MIN` reader - 15:0\\]
PHY alert_n pulse width minimux value for CA parity error."]
pub type PhyCaParityErrPulseMinR = crate::FieldReader<u16>;
#[doc = "Field `PHY_CA_PARITY_ERR_PULSE_MIN` writer - 15:0\\]
PHY alert_n pulse width minimux value for CA parity error."]
pub type PhyCaParityErrPulseMinW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_ERR_MASK_EN` reader - 18:16\\]
PHY ERROR information report mask enable."]
pub type PhyErrMaskEnR = crate::FieldReader;
#[doc = "Field `PHY_ERR_MASK_EN` writer - 18:16\\]
PHY ERROR information report mask enable."]
pub type PhyErrMaskEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_ERR_STATUS` reader - 26:24\\]
PHY ERROR information."]
pub type PhyErrStatusR = crate::FieldReader;
#[doc = "Field `PHY_ERR_STATUS` writer - 26:24\\]
PHY ERROR information."]
pub type PhyErrStatusW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
PHY alert_n pulse width minimux value for CA parity error."]
    #[inline(always)]
    pub fn phy_ca_parity_err_pulse_min(&self) -> PhyCaParityErrPulseMinR {
        PhyCaParityErrPulseMinR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - 18:16\\]
PHY ERROR information report mask enable."]
    #[inline(always)]
    pub fn phy_err_mask_en(&self) -> PhyErrMaskEnR {
        PhyErrMaskEnR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
PHY ERROR information."]
    #[inline(always)]
    pub fn phy_err_status(&self) -> PhyErrStatusR {
        PhyErrStatusR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
PHY alert_n pulse width minimux value for CA parity error."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ca_parity_err_pulse_min(
        &mut self,
    ) -> PhyCaParityErrPulseMinW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1365Spec> {
        PhyCaParityErrPulseMinW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
PHY ERROR information report mask enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_err_mask_en(&mut self) -> PhyErrMaskEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1365Spec> {
        PhyErrMaskEnW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
PHY ERROR information."]
    #[inline(always)]
    #[must_use]
    pub fn phy_err_status(&mut self) -> PhyErrStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1365Spec> {
        PhyErrStatusW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1365\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1365::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1365::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1365Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1365Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1365::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1365Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1365::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1365Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1365 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1365Spec {
    const RESET_VALUE: u32 = 0;
}
