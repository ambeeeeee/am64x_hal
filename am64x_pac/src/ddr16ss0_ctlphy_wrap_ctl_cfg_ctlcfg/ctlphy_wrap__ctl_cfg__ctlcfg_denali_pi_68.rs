#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_68` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi68Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_68` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi68Spec>;
#[doc = "Field `PI_TDFI_WDQLVL_EN` reader - 7:0\\]
DFI timing param tWDQLVL_EN. Minimum number of DFI clocks required after the write DQ training enable signal is asserted until the first write command may be asserted."]
pub type PiTdfiWdqlvlEnR = crate::FieldReader;
#[doc = "Field `PI_TDFI_WDQLVL_EN` writer - 7:0\\]
DFI timing param tWDQLVL_EN. Minimum number of DFI clocks required after the write DQ training enable signal is asserted until the first write command may be asserted."]
pub type PiTdfiWdqlvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DFI timing param tWDQLVL_EN. Minimum number of DFI clocks required after the write DQ training enable signal is asserted until the first write command may be asserted."]
    #[inline(always)]
    pub fn pi_tdfi_wdqlvl_en(&self) -> PiTdfiWdqlvlEnR {
        PiTdfiWdqlvlEnR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DFI timing param tWDQLVL_EN. Minimum number of DFI clocks required after the write DQ training enable signal is asserted until the first write command may be asserted."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wdqlvl_en(&mut self) -> PiTdfiWdqlvlEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi68Spec> {
        PiTdfiWdqlvlEnW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_68::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_68::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi68Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi68Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_68::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi68Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_68::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi68Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_68 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi68Spec {
    const RESET_VALUE: u32 = 0;
}
