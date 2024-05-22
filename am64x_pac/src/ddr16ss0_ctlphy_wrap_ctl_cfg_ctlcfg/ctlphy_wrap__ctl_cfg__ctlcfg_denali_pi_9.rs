#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_9` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi9Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_9` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi9Spec>;
#[doc = "Field `PI_TDFI_PHYUPD_RESP` reader - 19:0\\]
Indicates the maximum number of DFI clock cycles registered between a dfi_phyupd_req signal assertion and a dfi_phyupd_ack signal assertion. READ-ONLY."]
pub type PiTdfiPhyupdRespR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_PHYUPD_RESP` writer - 19:0\\]
Indicates the maximum number of DFI clock cycles registered between a dfi_phyupd_req signal assertion and a dfi_phyupd_ack signal assertion. READ-ONLY."]
pub type PiTdfiPhyupdRespW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Indicates the maximum number of DFI clock cycles registered between a dfi_phyupd_req signal assertion and a dfi_phyupd_ack signal assertion. READ-ONLY."]
    #[inline(always)]
    pub fn pi_tdfi_phyupd_resp(&self) -> PiTdfiPhyupdRespR {
        PiTdfiPhyupdRespR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Indicates the maximum number of DFI clock cycles registered between a dfi_phyupd_req signal assertion and a dfi_phyupd_ack signal assertion. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phyupd_resp(
        &mut self,
    ) -> PiTdfiPhyupdRespW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi9Spec> {
        PiTdfiPhyupdRespW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi9Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_9::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi9Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_9::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_9 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi9Spec {
    const RESET_VALUE: u32 = 0;
}
