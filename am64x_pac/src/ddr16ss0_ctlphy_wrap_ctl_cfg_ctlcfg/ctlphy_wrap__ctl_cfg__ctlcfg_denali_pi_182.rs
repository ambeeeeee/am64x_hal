#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_182` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi182Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_182` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi182Spec>;
#[doc = "Field `PI_WRLVL_EN_F2` reader - 1:0\\]
Enable the PI write leveling module for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiWrlvlEnF2R = crate::FieldReader;
#[doc = "Field `PI_WRLVL_EN_F2` writer - 1:0\\]
Enable the PI write leveling module for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiWrlvlEnF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_TDFI_WRLVL_WW_F0` reader - 17:8\\]
Defines the DFI tWRLVL_WW timing parameter \\[in DFI clocks\\]
for frequency set 0, the minimum cycles between dfi_wrlvl_strobe assertions."]
pub type PiTdfiWrlvlWwF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_WRLVL_WW_F0` writer - 17:8\\]
Defines the DFI tWRLVL_WW timing parameter \\[in DFI clocks\\]
for frequency set 0, the minimum cycles between dfi_wrlvl_strobe assertions."]
pub type PiTdfiWrlvlWwF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Enable the PI write leveling module for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_wrlvl_en_f2(&self) -> PiWrlvlEnF2R {
        PiWrlvlEnF2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Defines the DFI tWRLVL_WW timing parameter \\[in DFI clocks\\]
for frequency set 0, the minimum cycles between dfi_wrlvl_strobe assertions."]
    #[inline(always)]
    pub fn pi_tdfi_wrlvl_ww_f0(&self) -> PiTdfiWrlvlWwF0R {
        PiTdfiWrlvlWwF0R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Enable the PI write leveling module for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_en_f2(&mut self) -> PiWrlvlEnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi182Spec> {
        PiWrlvlEnF2W::new(self, 0)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Defines the DFI tWRLVL_WW timing parameter \\[in DFI clocks\\]
for frequency set 0, the minimum cycles between dfi_wrlvl_strobe assertions."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wrlvl_ww_f0(
        &mut self,
    ) -> PiTdfiWrlvlWwF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi182Spec> {
        PiTdfiWrlvlWwF0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_182\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_182::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_182::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi182Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi182Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_182::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi182Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_182::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi182Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_182 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi182Spec {
    const RESET_VALUE: u32 = 0;
}
