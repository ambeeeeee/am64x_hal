#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_184` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl184Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_184` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl184Spec>;
#[doc = "Field `TDFI_INIT_START_F1` reader - 23:0\\]
Defines the DFI tINIT_START timing parameter \\[in DFI clocks\\], the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. FC=1"]
pub type TdfiInitStartF1R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_INIT_START_F1` writer - 23:0\\]
Defines the DFI tINIT_START timing parameter \\[in DFI clocks\\], the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. FC=1"]
pub type TdfiInitStartF1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Defines the DFI tINIT_START timing parameter \\[in DFI clocks\\], the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. FC=1"]
    #[inline(always)]
    pub fn tdfi_init_start_f1(&self) -> TdfiInitStartF1R {
        TdfiInitStartF1R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Defines the DFI tINIT_START timing parameter \\[in DFI clocks\\], the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_init_start_f1(
        &mut self,
    ) -> TdfiInitStartF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl184Spec> {
        TdfiInitStartF1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_184\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_184::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_184::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl184Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl184Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_184::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl184Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_184::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl184Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_184 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl184Spec {
    const RESET_VALUE: u32 = 0;
}
