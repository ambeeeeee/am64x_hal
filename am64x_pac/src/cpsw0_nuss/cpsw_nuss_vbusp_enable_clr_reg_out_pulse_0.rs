#[doc = "Register `CPSW_NUSS_VBUSP_enable_clr_reg_out_pulse_0` reader"]
pub type R = crate::R<CpswNussVbuspEnableClrRegOutPulse0Spec>;
#[doc = "Register `CPSW_NUSS_VBUSP_enable_clr_reg_out_pulse_0` writer"]
pub type W = crate::W<CpswNussVbuspEnableClrRegOutPulse0Spec>;
#[doc = "Field `ENABLE_OUT_PULSE_EN_EVNT_PENDA_CLR` reader - 0:0\\]
Enable Clear for out_pulse_en_evnt_penda"]
pub type EnableOutPulseEnEvntPendaClrR = crate::BitReader;
#[doc = "Field `ENABLE_OUT_PULSE_EN_EVNT_PENDA_CLR` writer - 0:0\\]
Enable Clear for out_pulse_en_evnt_penda"]
pub type EnableOutPulseEnEvntPendaClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_OUT_PULSE_EN_MDIO_PENDA_CLR` reader - 1:1\\]
Enable Clear for out_pulse_en_mdio_penda"]
pub type EnableOutPulseEnMdioPendaClrR = crate::BitReader;
#[doc = "Field `ENABLE_OUT_PULSE_EN_MDIO_PENDA_CLR` writer - 1:1\\]
Enable Clear for out_pulse_en_mdio_penda"]
pub type EnableOutPulseEnMdioPendaClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_OUT_PULSE_EN_STAT_PENDA_CLR` reader - 2:2\\]
Enable Clear for out_pulse_en_stat_penda"]
pub type EnableOutPulseEnStatPendaClrR = crate::BitReader;
#[doc = "Field `ENABLE_OUT_PULSE_EN_STAT_PENDA_CLR` writer - 2:2\\]
Enable Clear for out_pulse_en_stat_penda"]
pub type EnableOutPulseEnStatPendaClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Clear for out_pulse_en_evnt_penda"]
    #[inline(always)]
    pub fn enable_out_pulse_en_evnt_penda_clr(&self) -> EnableOutPulseEnEvntPendaClrR {
        EnableOutPulseEnEvntPendaClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable Clear for out_pulse_en_mdio_penda"]
    #[inline(always)]
    pub fn enable_out_pulse_en_mdio_penda_clr(&self) -> EnableOutPulseEnMdioPendaClrR {
        EnableOutPulseEnMdioPendaClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable Clear for out_pulse_en_stat_penda"]
    #[inline(always)]
    pub fn enable_out_pulse_en_stat_penda_clr(&self) -> EnableOutPulseEnStatPendaClrR {
        EnableOutPulseEnStatPendaClrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Clear for out_pulse_en_evnt_penda"]
    #[inline(always)]
    #[must_use]
    pub fn enable_out_pulse_en_evnt_penda_clr(
        &mut self,
    ) -> EnableOutPulseEnEvntPendaClrW<CpswNussVbuspEnableClrRegOutPulse0Spec> {
        EnableOutPulseEnEvntPendaClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable Clear for out_pulse_en_mdio_penda"]
    #[inline(always)]
    #[must_use]
    pub fn enable_out_pulse_en_mdio_penda_clr(
        &mut self,
    ) -> EnableOutPulseEnMdioPendaClrW<CpswNussVbuspEnableClrRegOutPulse0Spec> {
        EnableOutPulseEnMdioPendaClrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable Clear for out_pulse_en_stat_penda"]
    #[inline(always)]
    #[must_use]
    pub fn enable_out_pulse_en_stat_penda_clr(
        &mut self,
    ) -> EnableOutPulseEnStatPendaClrW<CpswNussVbuspEnableClrRegOutPulse0Spec> {
        EnableOutPulseEnStatPendaClrW::new(self, 2)
    }
}
#[doc = "Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_enable_clr_reg_out_pulse_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_enable_clr_reg_out_pulse_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspEnableClrRegOutPulse0Spec;
impl crate::RegisterSpec for CpswNussVbuspEnableClrRegOutPulse0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_enable_clr_reg_out_pulse_0::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspEnableClrRegOutPulse0Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_enable_clr_reg_out_pulse_0::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspEnableClrRegOutPulse0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_enable_clr_reg_out_pulse_0 to value 0"]
impl crate::Resettable for CpswNussVbuspEnableClrRegOutPulse0Spec {
    const RESET_VALUE: u32 = 0;
}
