#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_8` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi8Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_8` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi8Spec>;
#[doc = "Field `PI_TDFI_PHYMSTR_RESP` reader - 19:0\\]
Indicates the maximum number of DFI clock cycles registered between a dfi_phymstr_req signal assertion and a dfi_phymstr_ack signal assertion. READ-ONLY"]
pub type PiTdfiPhymstrRespR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_PHYMSTR_RESP` writer - 19:0\\]
Indicates the maximum number of DFI clock cycles registered between a dfi_phymstr_req signal assertion and a dfi_phymstr_ack signal assertion. READ-ONLY"]
pub type PiTdfiPhymstrRespW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Indicates the maximum number of DFI clock cycles registered between a dfi_phymstr_req signal assertion and a dfi_phymstr_ack signal assertion. READ-ONLY"]
    #[inline(always)]
    pub fn pi_tdfi_phymstr_resp(&self) -> PiTdfiPhymstrRespR {
        PiTdfiPhymstrRespR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Indicates the maximum number of DFI clock cycles registered between a dfi_phymstr_req signal assertion and a dfi_phymstr_ack signal assertion. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phymstr_resp(
        &mut self,
    ) -> PiTdfiPhymstrRespW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi8Spec> {
        PiTdfiPhymstrRespW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi8Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_8::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi8Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_8::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_8 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi8Spec {
    const RESET_VALUE: u32 = 0;
}
