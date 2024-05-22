#[doc = "Register `MMR__VBUSP__CFG2_VTM_MISC_CTRL2` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg2VtmMiscCtrl2Spec>;
#[doc = "Register `MMR__VBUSP__CFG2_VTM_MISC_CTRL2` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg2VtmMiscCtrl2Spec>;
#[doc = "Field `MAXT_OUTRG_ALERT_THR` reader - 9:0\\]
This defines the global max temperature out of range sample value. If the alert is enabled globally and for the sensor and the sensor reads a value >= this value then the alert is triggered. Reset is at POR only."]
pub type MaxtOutrgAlertThrR = crate::FieldReader<u16>;
#[doc = "Field `MAXT_OUTRG_ALERT_THR` writer - 9:0\\]
This defines the global max temperature out of range sample value. If the alert is enabled globally and for the sensor and the sensor reads a value >= this value then the alert is triggered. Reset is at POR only."]
pub type MaxtOutrgAlertThrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `MAXT_OUTRG_ALERT_THR0` reader - 25:16\\]
This defines the global max temperature out of range safe sample value. If the alert is enabled globally and for the sensor and the sensor reads a value &lt;= this value then the alert is cleared after being triggered. This safe threshold defines at what temp the alert can be cleared. Reset is at POR only."]
pub type MaxtOutrgAlertThr0R = crate::FieldReader<u16>;
#[doc = "Field `MAXT_OUTRG_ALERT_THR0` writer - 25:16\\]
This defines the global max temperature out of range safe sample value. If the alert is enabled globally and for the sensor and the sensor reads a value &lt;= this value then the alert is cleared after being triggered. This safe threshold defines at what temp the alert can be cleared. Reset is at POR only."]
pub type MaxtOutrgAlertThr0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
This defines the global max temperature out of range sample value. If the alert is enabled globally and for the sensor and the sensor reads a value >= this value then the alert is triggered. Reset is at POR only."]
    #[inline(always)]
    pub fn maxt_outrg_alert_thr(&self) -> MaxtOutrgAlertThrR {
        MaxtOutrgAlertThrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
This defines the global max temperature out of range safe sample value. If the alert is enabled globally and for the sensor and the sensor reads a value &lt;= this value then the alert is cleared after being triggered. This safe threshold defines at what temp the alert can be cleared. Reset is at POR only."]
    #[inline(always)]
    pub fn maxt_outrg_alert_thr0(&self) -> MaxtOutrgAlertThr0R {
        MaxtOutrgAlertThr0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
This defines the global max temperature out of range sample value. If the alert is enabled globally and for the sensor and the sensor reads a value >= this value then the alert is triggered. Reset is at POR only."]
    #[inline(always)]
    #[must_use]
    pub fn maxt_outrg_alert_thr(&mut self) -> MaxtOutrgAlertThrW<Mmr_Vbusp_Cfg2VtmMiscCtrl2Spec> {
        MaxtOutrgAlertThrW::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
This defines the global max temperature out of range safe sample value. If the alert is enabled globally and for the sensor and the sensor reads a value &lt;= this value then the alert is cleared after being triggered. This safe threshold defines at what temp the alert can be cleared. Reset is at POR only."]
    #[inline(always)]
    #[must_use]
    pub fn maxt_outrg_alert_thr0(&mut self) -> MaxtOutrgAlertThr0W<Mmr_Vbusp_Cfg2VtmMiscCtrl2Spec> {
        MaxtOutrgAlertThr0W::new(self, 16)
    }
}
#[doc = "VTM miscellaneous control bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg2_vtm_misc_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg2_vtm_misc_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg2VtmMiscCtrl2Spec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg2VtmMiscCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg2_vtm_misc_ctrl2::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg2VtmMiscCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg2_vtm_misc_ctrl2::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg2VtmMiscCtrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG2_VTM_MISC_CTRL2 to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg2VtmMiscCtrl2Spec {
    const RESET_VALUE: u32 = 0;
}
