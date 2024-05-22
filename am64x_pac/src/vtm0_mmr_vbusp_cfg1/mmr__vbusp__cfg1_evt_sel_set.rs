#[doc = "Register `MMR__VBUSP__CFG1_EVT_SEL_SET` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg1EvtSelSetSpec>;
#[doc = "Register `MMR__VBUSP__CFG1_EVT_SEL_SET` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg1EvtSelSetSpec>;
#[doc = "Field `TSENS_EVT_SEL` reader - 23:16\\]
In this field we select which of the event contributions of the 8-maximum possible temp-monitors controlled by this VTM will contribute to generate the merged event/alerts of this VD. Any combination of them could be selected and varies between SOCs and VDs. Eg: 0x00 : No temp-monitor event contributes to generate the temperature events of this VD. 0x06: Temp-monitors\\[2,1\\]
contribute to generate the temperature events of this VD. ... 0xFF: All 8 temp-monitors contribute to generate the temperature events of this VD. 0: Writing 0 to this field produces no effect. 1: Writing 1 to any of the bits in this field sets to 1 the corresponding bit in that field."]
pub type TsensEvtSelR = crate::FieldReader;
#[doc = "Field `TSENS_EVT_SEL` writer - 23:16\\]
In this field we select which of the event contributions of the 8-maximum possible temp-monitors controlled by this VTM will contribute to generate the merged event/alerts of this VD. Any combination of them could be selected and varies between SOCs and VDs. Eg: 0x00 : No temp-monitor event contributes to generate the temperature events of this VD. 0x06: Temp-monitors\\[2,1\\]
contribute to generate the temperature events of this VD. ... 0xFF: All 8 temp-monitors contribute to generate the temperature events of this VD. 0: Writing 0 to this field produces no effect. 1: Writing 1 to any of the bits in this field sets to 1 the corresponding bit in that field."]
pub type TsensEvtSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23 - 23:16\\]
In this field we select which of the event contributions of the 8-maximum possible temp-monitors controlled by this VTM will contribute to generate the merged event/alerts of this VD. Any combination of them could be selected and varies between SOCs and VDs. Eg: 0x00 : No temp-monitor event contributes to generate the temperature events of this VD. 0x06: Temp-monitors\\[2,1\\]
contribute to generate the temperature events of this VD. ... 0xFF: All 8 temp-monitors contribute to generate the temperature events of this VD. 0: Writing 0 to this field produces no effect. 1: Writing 1 to any of the bits in this field sets to 1 the corresponding bit in that field."]
    #[inline(always)]
    pub fn tsens_evt_sel(&self) -> TsensEvtSelR {
        TsensEvtSelR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - 23:16\\]
In this field we select which of the event contributions of the 8-maximum possible temp-monitors controlled by this VTM will contribute to generate the merged event/alerts of this VD. Any combination of them could be selected and varies between SOCs and VDs. Eg: 0x00 : No temp-monitor event contributes to generate the temperature events of this VD. 0x06: Temp-monitors\\[2,1\\]
contribute to generate the temperature events of this VD. ... 0xFF: All 8 temp-monitors contribute to generate the temperature events of this VD. 0: Writing 0 to this field produces no effect. 1: Writing 1 to any of the bits in this field sets to 1 the corresponding bit in that field."]
    #[inline(always)]
    #[must_use]
    pub fn tsens_evt_sel(&mut self) -> TsensEvtSelW<Mmr_Vbusp_Cfg1EvtSelSetSpec> {
        TsensEvtSelW::new(self, 16)
    }
}
#[doc = "Voltage domain a event select and control set register. NOTE: This MMR and the companion MMR VTM_VD\\[a\\]_EVT_SEL_CLR are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_evt_sel_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_evt_sel_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg1EvtSelSetSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg1EvtSelSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg1_evt_sel_set::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg1EvtSelSetSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg1_evt_sel_set::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg1EvtSelSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG1_EVT_SEL_SET to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg1EvtSelSetSpec {
    const RESET_VALUE: u32 = 0;
}
