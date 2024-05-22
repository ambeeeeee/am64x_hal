#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_69` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi69Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_69` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi69Spec>;
#[doc = "Field `PI_TDFI_WDQLVL_RESP` reader - 31:0\\]
DFI timing param tWDQLVL_RESP. Maximum number of DFI clocks that may occur between a write DQ training request and the associated mode enable."]
pub type PiTdfiWdqlvlRespR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_WDQLVL_RESP` writer - 31:0\\]
DFI timing param tWDQLVL_RESP. Maximum number of DFI clocks that may occur between a write DQ training request and the associated mode enable."]
pub type PiTdfiWdqlvlRespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
DFI timing param tWDQLVL_RESP. Maximum number of DFI clocks that may occur between a write DQ training request and the associated mode enable."]
    #[inline(always)]
    pub fn pi_tdfi_wdqlvl_resp(&self) -> PiTdfiWdqlvlRespR {
        PiTdfiWdqlvlRespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
DFI timing param tWDQLVL_RESP. Maximum number of DFI clocks that may occur between a write DQ training request and the associated mode enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wdqlvl_resp(
        &mut self,
    ) -> PiTdfiWdqlvlRespW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi69Spec> {
        PiTdfiWdqlvlRespW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_69\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_69::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_69::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi69Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi69Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_69::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi69Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_69::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi69Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_69 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi69Spec {
    const RESET_VALUE: u32 = 0;
}
