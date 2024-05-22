#[doc = "Register `CPSW_NUSS_VBUSP_status_reg_out_pulse_0` reader"]
pub type R = crate::R<CpswNussVbuspStatusRegOutPulse0Spec>;
#[doc = "Register `CPSW_NUSS_VBUSP_status_reg_out_pulse_0` writer"]
pub type W = crate::W<CpswNussVbuspStatusRegOutPulse0Spec>;
#[doc = "Field `STATUS_OUT_PULSE_EVNT_PENDA` reader - 0:0\\]
Status for out_pulse_en_evnt_penda"]
pub type StatusOutPulseEvntPendaR = crate::BitReader;
#[doc = "Field `STATUS_OUT_PULSE_EVNT_PENDA` writer - 0:0\\]
Status for out_pulse_en_evnt_penda"]
pub type StatusOutPulseEvntPendaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_OUT_PULSE_MDIO_PENDA` reader - 1:1\\]
Status for out_pulse_en_mdio_penda"]
pub type StatusOutPulseMdioPendaR = crate::BitReader;
#[doc = "Field `STATUS_OUT_PULSE_MDIO_PENDA` writer - 1:1\\]
Status for out_pulse_en_mdio_penda"]
pub type StatusOutPulseMdioPendaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_OUT_PULSE_STAT_PENDA` reader - 2:2\\]
Status for out_pulse_en_stat_penda"]
pub type StatusOutPulseStatPendaR = crate::BitReader;
#[doc = "Field `STATUS_OUT_PULSE_STAT_PENDA` writer - 2:2\\]
Status for out_pulse_en_stat_penda"]
pub type StatusOutPulseStatPendaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status for out_pulse_en_evnt_penda"]
    #[inline(always)]
    pub fn status_out_pulse_evnt_penda(&self) -> StatusOutPulseEvntPendaR {
        StatusOutPulseEvntPendaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status for out_pulse_en_mdio_penda"]
    #[inline(always)]
    pub fn status_out_pulse_mdio_penda(&self) -> StatusOutPulseMdioPendaR {
        StatusOutPulseMdioPendaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Status for out_pulse_en_stat_penda"]
    #[inline(always)]
    pub fn status_out_pulse_stat_penda(&self) -> StatusOutPulseStatPendaR {
        StatusOutPulseStatPendaR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status for out_pulse_en_evnt_penda"]
    #[inline(always)]
    #[must_use]
    pub fn status_out_pulse_evnt_penda(
        &mut self,
    ) -> StatusOutPulseEvntPendaW<CpswNussVbuspStatusRegOutPulse0Spec> {
        StatusOutPulseEvntPendaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status for out_pulse_en_mdio_penda"]
    #[inline(always)]
    #[must_use]
    pub fn status_out_pulse_mdio_penda(
        &mut self,
    ) -> StatusOutPulseMdioPendaW<CpswNussVbuspStatusRegOutPulse0Spec> {
        StatusOutPulseMdioPendaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Status for out_pulse_en_stat_penda"]
    #[inline(always)]
    #[must_use]
    pub fn status_out_pulse_stat_penda(
        &mut self,
    ) -> StatusOutPulseStatPendaW<CpswNussVbuspStatusRegOutPulse0Spec> {
        StatusOutPulseStatPendaW::new(self, 2)
    }
}
#[doc = "Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_status_reg_out_pulse_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_status_reg_out_pulse_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspStatusRegOutPulse0Spec;
impl crate::RegisterSpec for CpswNussVbuspStatusRegOutPulse0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_status_reg_out_pulse_0::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspStatusRegOutPulse0Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_status_reg_out_pulse_0::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspStatusRegOutPulse0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_status_reg_out_pulse_0 to value 0"]
impl crate::Resettable for CpswNussVbuspStatusRegOutPulse0Spec {
    const RESET_VALUE: u32 = 0;
}
