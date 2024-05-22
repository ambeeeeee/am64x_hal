#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_6` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi6Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_6` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi6Spec>;
#[doc = "Field `PI_DFI_PHYMSTR_CS_STATE_R` reader - 0:0\\]
This signal indicates the state of the DRAM when the PHY becomes the master. 'b0: The PHY specifies the required state, using the dfi_phymstr_state_sel signal. 'b1: is reserved."]
pub type PiDfiPhymstrCsStateRR = crate::BitReader;
#[doc = "Field `PI_DFI_PHYMSTR_CS_STATE_R` writer - 0:0\\]
This signal indicates the state of the DRAM when the PHY becomes the master. 'b0: The PHY specifies the required state, using the dfi_phymstr_state_sel signal. 'b1: is reserved."]
pub type PiDfiPhymstrCsStateRW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_DFI_PHYMSTR_STATE_SEL_R` reader - 8:8\\]
DFI PHY Master State Select: Indication from the PHY to the MC whether the requested memory state is IDLE or Self refresh. 'b0: indicates that the corresponding CS must be put into the IDLE state. 'b1: indicates that the corresponding CS must be put into the Self refresh state."]
pub type PiDfiPhymstrStateSelRR = crate::BitReader;
#[doc = "Field `PI_DFI_PHYMSTR_STATE_SEL_R` writer - 8:8\\]
DFI PHY Master State Select: Indication from the PHY to the MC whether the requested memory state is IDLE or Self refresh. 'b0: indicates that the corresponding CS must be put into the IDLE state. 'b1: indicates that the corresponding CS must be put into the Self refresh state."]
pub type PiDfiPhymstrStateSelRW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This signal indicates the state of the DRAM when the PHY becomes the master. 'b0: The PHY specifies the required state, using the dfi_phymstr_state_sel signal. 'b1: is reserved."]
    #[inline(always)]
    pub fn pi_dfi_phymstr_cs_state_r(&self) -> PiDfiPhymstrCsStateRR {
        PiDfiPhymstrCsStateRR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DFI PHY Master State Select: Indication from the PHY to the MC whether the requested memory state is IDLE or Self refresh. 'b0: indicates that the corresponding CS must be put into the IDLE state. 'b1: indicates that the corresponding CS must be put into the Self refresh state."]
    #[inline(always)]
    pub fn pi_dfi_phymstr_state_sel_r(&self) -> PiDfiPhymstrStateSelRR {
        PiDfiPhymstrStateSelRR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This signal indicates the state of the DRAM when the PHY becomes the master. 'b0: The PHY specifies the required state, using the dfi_phymstr_state_sel signal. 'b1: is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dfi_phymstr_cs_state_r(
        &mut self,
    ) -> PiDfiPhymstrCsStateRW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi6Spec> {
        PiDfiPhymstrCsStateRW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DFI PHY Master State Select: Indication from the PHY to the MC whether the requested memory state is IDLE or Self refresh. 'b0: indicates that the corresponding CS must be put into the IDLE state. 'b1: indicates that the corresponding CS must be put into the Self refresh state."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dfi_phymstr_state_sel_r(
        &mut self,
    ) -> PiDfiPhymstrStateSelRW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi6Spec> {
        PiDfiPhymstrStateSelRW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi6Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_6::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi6Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_6::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_6 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi6Spec {
    const RESET_VALUE: u32 = 0;
}
