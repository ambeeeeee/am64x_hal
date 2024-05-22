#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_139` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi139Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_139` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi139Spec>;
#[doc = "Field `PI_DLL_RST_ADJ_DLY` reader - 7:0\\]
Minimum cycles after setting master delay in DLL until the DLL reset signal dll_rst_n may be asserted."]
pub type PiDllRstAdjDlyR = crate::FieldReader;
#[doc = "Field `PI_DLL_RST_ADJ_DLY` writer - 7:0\\]
Minimum cycles after setting master delay in DLL until the DLL reset signal dll_rst_n may be asserted."]
pub type PiDllRstAdjDlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Minimum cycles after setting master delay in DLL until the DLL reset signal dll_rst_n may be asserted."]
    #[inline(always)]
    pub fn pi_dll_rst_adj_dly(&self) -> PiDllRstAdjDlyR {
        PiDllRstAdjDlyR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Minimum cycles after setting master delay in DLL until the DLL reset signal dll_rst_n may be asserted."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dll_rst_adj_dly(
        &mut self,
    ) -> PiDllRstAdjDlyW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi139Spec> {
        PiDllRstAdjDlyW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_139\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_139::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_139::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi139Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi139Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_139::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi139Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_139::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi139Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_139 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi139Spec {
    const RESET_VALUE: u32 = 0;
}
