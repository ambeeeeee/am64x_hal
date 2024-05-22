#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_205` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi205Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_205` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi205Spec>;
#[doc = "Field `PI_TVREF_LONG_F1` reader - 9:0\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter gt 1 for frequency set 1."]
pub type PiTvrefLongF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TVREF_LONG_F1` writer - 9:0\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter gt 1 for frequency set 1."]
pub type PiTvrefLongF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TDFI_CACSCA_F2` reader - 20:16\\]
Defines the DFI tcalvl_cs_ca timing parameter, the number of PHY DFI clocks from the assertion of dfi_calvl_ca_sel to the assertion of dfi_cs for frequency set 2."]
pub type PiTdfiCacscaF2R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CACSCA_F2` writer - 20:16\\]
Defines the DFI tcalvl_cs_ca timing parameter, the number of PHY DFI clocks from the assertion of dfi_calvl_ca_sel to the assertion of dfi_cs for frequency set 2."]
pub type PiTdfiCacscaF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TDFI_CASEL_F2` reader - 28:24\\]
Defines the DFI tcalvl_ca_sel timing parameter, the width of dfi_calvl_ca_sel in PHY DFI clock cycles for frequency set 2."]
pub type PiTdfiCaselF2R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CASEL_F2` writer - 28:24\\]
Defines the DFI tcalvl_ca_sel timing parameter, the width of dfi_calvl_ca_sel in PHY DFI clock cycles for frequency set 2."]
pub type PiTdfiCaselF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter gt 1 for frequency set 1."]
    #[inline(always)]
    pub fn pi_tvref_long_f1(&self) -> PiTvrefLongF1R {
        PiTvrefLongF1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Defines the DFI tcalvl_cs_ca timing parameter, the number of PHY DFI clocks from the assertion of dfi_calvl_ca_sel to the assertion of dfi_cs for frequency set 2."]
    #[inline(always)]
    pub fn pi_tdfi_cacsca_f2(&self) -> PiTdfiCacscaF2R {
        PiTdfiCacscaF2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Defines the DFI tcalvl_ca_sel timing parameter, the width of dfi_calvl_ca_sel in PHY DFI clock cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_tdfi_casel_f2(&self) -> PiTdfiCaselF2R {
        PiTdfiCaselF2R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter gt 1 for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tvref_long_f1(&mut self) -> PiTvrefLongF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi205Spec> {
        PiTvrefLongF1W::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Defines the DFI tcalvl_cs_ca timing parameter, the number of PHY DFI clocks from the assertion of dfi_calvl_ca_sel to the assertion of dfi_cs for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_cacsca_f2(
        &mut self,
    ) -> PiTdfiCacscaF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi205Spec> {
        PiTdfiCacscaF2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Defines the DFI tcalvl_ca_sel timing parameter, the width of dfi_calvl_ca_sel in PHY DFI clock cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_casel_f2(&mut self) -> PiTdfiCaselF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi205Spec> {
        PiTdfiCaselF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_205\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_205::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_205::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi205Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi205Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_205::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi205Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_205::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi205Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_205 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi205Spec {
    const RESET_VALUE: u32 = 0;
}
