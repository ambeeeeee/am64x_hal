#[doc = "Register `MMR__VBUSP__CFG1_EVT_STAT` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg1EvtStatSpec>;
#[doc = "Register `MMR__VBUSP__CFG1_EVT_STAT` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg1EvtStatSpec>;
#[doc = "Field `GT_TH1_ALERT` reader - 0:0\\]
This bit reflects the status of the merged temperature alert resulting from the OR of all the similar alerts produced by the temp-monitors selected as showed in field VTM_VD\\[a\\]_EVT_SEL_SET.tsens_evt_sel. This field shows the actual present value of the resulting OR function of the combined selected signals."]
pub type GtTh1AlertR = crate::BitReader;
#[doc = "Field `GT_TH1_ALERT` writer - 0:0\\]
This bit reflects the status of the merged temperature alert resulting from the OR of all the similar alerts produced by the temp-monitors selected as showed in field VTM_VD\\[a\\]_EVT_SEL_SET.tsens_evt_sel. This field shows the actual present value of the resulting OR function of the combined selected signals."]
pub type GtTh1AlertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GT_TH2_ALERT` reader - 1:1\\]
This bit reflects the status of the merged temperature alert resulting from the combination of all the similar alerts produced by the temp-monitors selected as showed in field VTM_VD\\[a\\]_EVT_SEL_SET.tsens_evt_sel. This field shows the actual present level of the resulting OR function of the combined selected signals."]
pub type GtTh2AlertR = crate::BitReader;
#[doc = "Field `GT_TH2_ALERT` writer - 1:1\\]
This bit reflects the status of the merged temperature alert resulting from the combination of all the similar alerts produced by the temp-monitors selected as showed in field VTM_VD\\[a\\]_EVT_SEL_SET.tsens_evt_sel. This field shows the actual present level of the resulting OR function of the combined selected signals."]
pub type GtTh2AlertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LT_TH0_ALERT` reader - 2:2\\]
This bit reflects the status of the TH0 undertemp alert resulting from the AND of all the similar alerts produced by the temp sensors selected by VTM_VD\\[a\\]_EVT_SEL_SET.tsens_evt_sel."]
pub type LtTh0AlertR = crate::BitReader;
#[doc = "Field `LT_TH0_ALERT` writer - 2:2\\]
This bit reflects the status of the TH0 undertemp alert resulting from the AND of all the similar alerts produced by the temp sensors selected by VTM_VD\\[a\\]_EVT_SEL_SET.tsens_evt_sel."]
pub type LtTh0AlertW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit reflects the status of the merged temperature alert resulting from the OR of all the similar alerts produced by the temp-monitors selected as showed in field VTM_VD\\[a\\]_EVT_SEL_SET.tsens_evt_sel. This field shows the actual present value of the resulting OR function of the combined selected signals."]
    #[inline(always)]
    pub fn gt_th1_alert(&self) -> GtTh1AlertR {
        GtTh1AlertR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit reflects the status of the merged temperature alert resulting from the combination of all the similar alerts produced by the temp-monitors selected as showed in field VTM_VD\\[a\\]_EVT_SEL_SET.tsens_evt_sel. This field shows the actual present level of the resulting OR function of the combined selected signals."]
    #[inline(always)]
    pub fn gt_th2_alert(&self) -> GtTh2AlertR {
        GtTh2AlertR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit reflects the status of the TH0 undertemp alert resulting from the AND of all the similar alerts produced by the temp sensors selected by VTM_VD\\[a\\]_EVT_SEL_SET.tsens_evt_sel."]
    #[inline(always)]
    pub fn lt_th0_alert(&self) -> LtTh0AlertR {
        LtTh0AlertR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit reflects the status of the merged temperature alert resulting from the OR of all the similar alerts produced by the temp-monitors selected as showed in field VTM_VD\\[a\\]_EVT_SEL_SET.tsens_evt_sel. This field shows the actual present value of the resulting OR function of the combined selected signals."]
    #[inline(always)]
    #[must_use]
    pub fn gt_th1_alert(&mut self) -> GtTh1AlertW<Mmr_Vbusp_Cfg1EvtStatSpec> {
        GtTh1AlertW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit reflects the status of the merged temperature alert resulting from the combination of all the similar alerts produced by the temp-monitors selected as showed in field VTM_VD\\[a\\]_EVT_SEL_SET.tsens_evt_sel. This field shows the actual present level of the resulting OR function of the combined selected signals."]
    #[inline(always)]
    #[must_use]
    pub fn gt_th2_alert(&mut self) -> GtTh2AlertW<Mmr_Vbusp_Cfg1EvtStatSpec> {
        GtTh2AlertW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit reflects the status of the TH0 undertemp alert resulting from the AND of all the similar alerts produced by the temp sensors selected by VTM_VD\\[a\\]_EVT_SEL_SET.tsens_evt_sel."]
    #[inline(always)]
    #[must_use]
    pub fn lt_th0_alert(&mut self) -> LtTh0AlertW<Mmr_Vbusp_Cfg1EvtStatSpec> {
        LtTh0AlertW::new(self, 2)
    }
}
#[doc = "Voltage domain a event and control status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_evt_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_evt_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg1EvtStatSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg1EvtStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg1_evt_stat::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg1EvtStatSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg1_evt_stat::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg1EvtStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG1_EVT_STAT to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg1EvtStatSpec {
    const RESET_VALUE: u32 = 0;
}
