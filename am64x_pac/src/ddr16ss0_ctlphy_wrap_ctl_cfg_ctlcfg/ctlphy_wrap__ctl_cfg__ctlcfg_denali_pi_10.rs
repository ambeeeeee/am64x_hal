#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_10` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi10Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_10` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi10Spec>;
#[doc = "Field `PI_TDFI_PHYUPD_MAX` reader - 31:0\\]
Indicates the maximum number of DFI clock cycles registered while the dfi_phyupd_req signal is asserted and the dfi_phy_ack signal is asserted. READ-ONLY."]
pub type PiTdfiPhyupdMaxR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_PHYUPD_MAX` writer - 31:0\\]
Indicates the maximum number of DFI clock cycles registered while the dfi_phyupd_req signal is asserted and the dfi_phy_ack signal is asserted. READ-ONLY."]
pub type PiTdfiPhyupdMaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the maximum number of DFI clock cycles registered while the dfi_phyupd_req signal is asserted and the dfi_phy_ack signal is asserted. READ-ONLY."]
    #[inline(always)]
    pub fn pi_tdfi_phyupd_max(&self) -> PiTdfiPhyupdMaxR {
        PiTdfiPhyupdMaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the maximum number of DFI clock cycles registered while the dfi_phyupd_req signal is asserted and the dfi_phy_ack signal is asserted. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phyupd_max(
        &mut self,
    ) -> PiTdfiPhyupdMaxW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi10Spec> {
        PiTdfiPhyupdMaxW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi10Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_10::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi10Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_10::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_10 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi10Spec {
    const RESET_VALUE: u32 = 0;
}
