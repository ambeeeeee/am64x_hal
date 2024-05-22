#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_204` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi204Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_204` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi204Spec>;
#[doc = "Field `PI_TDFI_CACSCA_F1` reader - 4:0\\]
Defines the DFI tcalvl_cs_ca timing parameter, the number of PHY DFI clocks from the assertion of dfi_calvl_ca_sel to the assertion of dfi_cs for frequency set 1."]
pub type PiTdfiCacscaF1R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CACSCA_F1` writer - 4:0\\]
Defines the DFI tcalvl_cs_ca timing parameter, the number of PHY DFI clocks from the assertion of dfi_calvl_ca_sel to the assertion of dfi_cs for frequency set 1."]
pub type PiTdfiCacscaF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TDFI_CASEL_F1` reader - 12:8\\]
Defines the DFI tcalvl_ca_sel timing parameter, the width of dfi_calvl_ca_sel in PHY DFI clock cycles for frequency set 1."]
pub type PiTdfiCaselF1R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CASEL_F1` writer - 12:8\\]
Defines the DFI tcalvl_ca_sel timing parameter, the width of dfi_calvl_ca_sel in PHY DFI clock cycles for frequency set 1."]
pub type PiTdfiCaselF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TVREF_SHORT_F1` reader - 25:16\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter = 1 for frequency set 1."]
pub type PiTvrefShortF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TVREF_SHORT_F1` writer - 25:16\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter = 1 for frequency set 1."]
pub type PiTvrefShortF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Defines the DFI tcalvl_cs_ca timing parameter, the number of PHY DFI clocks from the assertion of dfi_calvl_ca_sel to the assertion of dfi_cs for frequency set 1."]
    #[inline(always)]
    pub fn pi_tdfi_cacsca_f1(&self) -> PiTdfiCacscaF1R {
        PiTdfiCacscaF1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Defines the DFI tcalvl_ca_sel timing parameter, the width of dfi_calvl_ca_sel in PHY DFI clock cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_tdfi_casel_f1(&self) -> PiTdfiCaselF1R {
        PiTdfiCaselF1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter = 1 for frequency set 1."]
    #[inline(always)]
    pub fn pi_tvref_short_f1(&self) -> PiTvrefShortF1R {
        PiTvrefShortF1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Defines the DFI tcalvl_cs_ca timing parameter, the number of PHY DFI clocks from the assertion of dfi_calvl_ca_sel to the assertion of dfi_cs for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_cacsca_f1(
        &mut self,
    ) -> PiTdfiCacscaF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi204Spec> {
        PiTdfiCacscaF1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Defines the DFI tcalvl_ca_sel timing parameter, the width of dfi_calvl_ca_sel in PHY DFI clock cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_casel_f1(&mut self) -> PiTdfiCaselF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi204Spec> {
        PiTdfiCaselF1W::new(self, 8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter = 1 for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tvref_short_f1(
        &mut self,
    ) -> PiTvrefShortF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi204Spec> {
        PiTvrefShortF1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_204\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_204::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_204::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi204Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_204::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi204Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_204::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi204Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_204 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi204Spec {
    const RESET_VALUE: u32 = 0;
}
