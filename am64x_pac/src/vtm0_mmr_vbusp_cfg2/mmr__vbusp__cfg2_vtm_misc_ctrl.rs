#[doc = "Register `MMR__VBUSP__CFG2_VTM_MISC_CTRL` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg2VtmMiscCtrlSpec>;
#[doc = "Register `MMR__VBUSP__CFG2_VTM_MISC_CTRL` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg2VtmMiscCtrlSpec>;
#[doc = "Field `ANY_MAXT_OUTRG_ALERT_EN` reader - 0:0\\]
This bit when enabled will cause the VTM's output therm_maxtemp_outrange_alert to be driven high, if any of the sources for the maxt_outrg_alert, is set high. Whenever all the maxt_outrg_alert enabled sensor alerts, out of the 8 possible are back to 0 then the output, therm_maxtemp_outrange_alert, will also return to 0. Reset is at POR only."]
pub type AnyMaxtOutrgAlertEnR = crate::BitReader;
#[doc = "Field `ANY_MAXT_OUTRG_ALERT_EN` writer - 0:0\\]
This bit when enabled will cause the VTM's output therm_maxtemp_outrange_alert to be driven high, if any of the sources for the maxt_outrg_alert, is set high. Whenever all the maxt_outrg_alert enabled sensor alerts, out of the 8 possible are back to 0 then the output, therm_maxtemp_outrange_alert, will also return to 0. Reset is at POR only."]
pub type AnyMaxtOutrgAlertEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit when enabled will cause the VTM's output therm_maxtemp_outrange_alert to be driven high, if any of the sources for the maxt_outrg_alert, is set high. Whenever all the maxt_outrg_alert enabled sensor alerts, out of the 8 possible are back to 0 then the output, therm_maxtemp_outrange_alert, will also return to 0. Reset is at POR only."]
    #[inline(always)]
    pub fn any_maxt_outrg_alert_en(&self) -> AnyMaxtOutrgAlertEnR {
        AnyMaxtOutrgAlertEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit when enabled will cause the VTM's output therm_maxtemp_outrange_alert to be driven high, if any of the sources for the maxt_outrg_alert, is set high. Whenever all the maxt_outrg_alert enabled sensor alerts, out of the 8 possible are back to 0 then the output, therm_maxtemp_outrange_alert, will also return to 0. Reset is at POR only."]
    #[inline(always)]
    #[must_use]
    pub fn any_maxt_outrg_alert_en(
        &mut self,
    ) -> AnyMaxtOutrgAlertEnW<Mmr_Vbusp_Cfg2VtmMiscCtrlSpec> {
        AnyMaxtOutrgAlertEnW::new(self, 0)
    }
}
#[doc = "VTM miscellaneous control bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg2_vtm_misc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg2_vtm_misc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg2VtmMiscCtrlSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg2VtmMiscCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg2_vtm_misc_ctrl::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg2VtmMiscCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg2_vtm_misc_ctrl::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg2VtmMiscCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG2_VTM_MISC_CTRL to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg2VtmMiscCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
