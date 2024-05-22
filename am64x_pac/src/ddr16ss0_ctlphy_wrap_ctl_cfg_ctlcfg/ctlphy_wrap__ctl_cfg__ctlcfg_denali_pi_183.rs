#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_183` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi183Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_183` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi183Spec>;
#[doc = "Field `PI_TDFI_WRLVL_WW_F1` reader - 9:0\\]
Defines the DFI tWRLVL_WW timing parameter \\[in DFI clocks\\]
for frequency set 1, the minimum cycles between dfi_wrlvl_strobe assertions."]
pub type PiTdfiWrlvlWwF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_WRLVL_WW_F1` writer - 9:0\\]
Defines the DFI tWRLVL_WW timing parameter \\[in DFI clocks\\]
for frequency set 1, the minimum cycles between dfi_wrlvl_strobe assertions."]
pub type PiTdfiWrlvlWwF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TDFI_WRLVL_WW_F2` reader - 25:16\\]
Defines the DFI tWRLVL_WW timing parameter \\[in DFI clocks\\]
for frequency set 2, the minimum cycles between dfi_wrlvl_strobe assertions."]
pub type PiTdfiWrlvlWwF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_WRLVL_WW_F2` writer - 25:16\\]
Defines the DFI tWRLVL_WW timing parameter \\[in DFI clocks\\]
for frequency set 2, the minimum cycles between dfi_wrlvl_strobe assertions."]
pub type PiTdfiWrlvlWwF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Defines the DFI tWRLVL_WW timing parameter \\[in DFI clocks\\]
for frequency set 1, the minimum cycles between dfi_wrlvl_strobe assertions."]
    #[inline(always)]
    pub fn pi_tdfi_wrlvl_ww_f1(&self) -> PiTdfiWrlvlWwF1R {
        PiTdfiWrlvlWwF1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Defines the DFI tWRLVL_WW timing parameter \\[in DFI clocks\\]
for frequency set 2, the minimum cycles between dfi_wrlvl_strobe assertions."]
    #[inline(always)]
    pub fn pi_tdfi_wrlvl_ww_f2(&self) -> PiTdfiWrlvlWwF2R {
        PiTdfiWrlvlWwF2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Defines the DFI tWRLVL_WW timing parameter \\[in DFI clocks\\]
for frequency set 1, the minimum cycles between dfi_wrlvl_strobe assertions."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wrlvl_ww_f1(
        &mut self,
    ) -> PiTdfiWrlvlWwF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi183Spec> {
        PiTdfiWrlvlWwF1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Defines the DFI tWRLVL_WW timing parameter \\[in DFI clocks\\]
for frequency set 2, the minimum cycles between dfi_wrlvl_strobe assertions."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wrlvl_ww_f2(
        &mut self,
    ) -> PiTdfiWrlvlWwF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi183Spec> {
        PiTdfiWrlvlWwF2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_183\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_183::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_183::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi183Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi183Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_183::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi183Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_183::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi183Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_183 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi183Spec {
    const RESET_VALUE: u32 = 0;
}
