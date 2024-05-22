#[doc = "Register `CPSW_NUSS_VBUSP_enable_reg_out_pulse_0` reader"]
pub type R = crate::R<CpswNussVbuspEnableRegOutPulse0Spec>;
#[doc = "Register `CPSW_NUSS_VBUSP_enable_reg_out_pulse_0` writer"]
pub type W = crate::W<CpswNussVbuspEnableRegOutPulse0Spec>;
#[doc = "Field `ENABLE_OUT_PULSE_EN_EVNT_PENDA` reader - 0:0\\]
Enable Set for out_pulse_en_evnt_penda"]
pub type EnableOutPulseEnEvntPendaR = crate::BitReader;
#[doc = "Field `ENABLE_OUT_PULSE_EN_EVNT_PENDA` writer - 0:0\\]
Enable Set for out_pulse_en_evnt_penda"]
pub type EnableOutPulseEnEvntPendaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_OUT_PULSE_EN_MDIO_PENDA` reader - 1:1\\]
Enable Set for out_pulse_en_mdio_penda"]
pub type EnableOutPulseEnMdioPendaR = crate::BitReader;
#[doc = "Field `ENABLE_OUT_PULSE_EN_MDIO_PENDA` writer - 1:1\\]
Enable Set for out_pulse_en_mdio_penda"]
pub type EnableOutPulseEnMdioPendaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_OUT_PULSE_EN_STAT_PENDA` reader - 2:2\\]
Enable Set for out_pulse_en_stat_penda"]
pub type EnableOutPulseEnStatPendaR = crate::BitReader;
#[doc = "Field `ENABLE_OUT_PULSE_EN_STAT_PENDA` writer - 2:2\\]
Enable Set for out_pulse_en_stat_penda"]
pub type EnableOutPulseEnStatPendaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Set for out_pulse_en_evnt_penda"]
    #[inline(always)]
    pub fn enable_out_pulse_en_evnt_penda(&self) -> EnableOutPulseEnEvntPendaR {
        EnableOutPulseEnEvntPendaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable Set for out_pulse_en_mdio_penda"]
    #[inline(always)]
    pub fn enable_out_pulse_en_mdio_penda(&self) -> EnableOutPulseEnMdioPendaR {
        EnableOutPulseEnMdioPendaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable Set for out_pulse_en_stat_penda"]
    #[inline(always)]
    pub fn enable_out_pulse_en_stat_penda(&self) -> EnableOutPulseEnStatPendaR {
        EnableOutPulseEnStatPendaR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Set for out_pulse_en_evnt_penda"]
    #[inline(always)]
    #[must_use]
    pub fn enable_out_pulse_en_evnt_penda(
        &mut self,
    ) -> EnableOutPulseEnEvntPendaW<CpswNussVbuspEnableRegOutPulse0Spec> {
        EnableOutPulseEnEvntPendaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable Set for out_pulse_en_mdio_penda"]
    #[inline(always)]
    #[must_use]
    pub fn enable_out_pulse_en_mdio_penda(
        &mut self,
    ) -> EnableOutPulseEnMdioPendaW<CpswNussVbuspEnableRegOutPulse0Spec> {
        EnableOutPulseEnMdioPendaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable Set for out_pulse_en_stat_penda"]
    #[inline(always)]
    #[must_use]
    pub fn enable_out_pulse_en_stat_penda(
        &mut self,
    ) -> EnableOutPulseEnStatPendaW<CpswNussVbuspEnableRegOutPulse0Spec> {
        EnableOutPulseEnStatPendaW::new(self, 2)
    }
}
#[doc = "Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_enable_reg_out_pulse_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_enable_reg_out_pulse_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspEnableRegOutPulse0Spec;
impl crate::RegisterSpec for CpswNussVbuspEnableRegOutPulse0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_enable_reg_out_pulse_0::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspEnableRegOutPulse0Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_enable_reg_out_pulse_0::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspEnableRegOutPulse0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_enable_reg_out_pulse_0 to value 0"]
impl crate::Resettable for CpswNussVbuspEnableRegOutPulse0Spec {
    const RESET_VALUE: u32 = 0;
}
