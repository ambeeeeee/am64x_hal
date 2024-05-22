#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_186` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl186Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_186` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl186Spec>;
#[doc = "Field `TDFI_INIT_START_F2` reader - 23:0\\]
Defines the DFI tINIT_START timing parameter \\[in DFI clocks\\], the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. FC=2"]
pub type TdfiInitStartF2R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_INIT_START_F2` writer - 23:0\\]
Defines the DFI tINIT_START timing parameter \\[in DFI clocks\\], the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. FC=2"]
pub type TdfiInitStartF2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Defines the DFI tINIT_START timing parameter \\[in DFI clocks\\], the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. FC=2"]
    #[inline(always)]
    pub fn tdfi_init_start_f2(&self) -> TdfiInitStartF2R {
        TdfiInitStartF2R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Defines the DFI tINIT_START timing parameter \\[in DFI clocks\\], the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_init_start_f2(
        &mut self,
    ) -> TdfiInitStartF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl186Spec> {
        TdfiInitStartF2W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_186\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_186::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_186::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl186Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl186Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_186::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl186Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_186::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl186Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_186 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl186Spec {
    const RESET_VALUE: u32 = 0;
}
