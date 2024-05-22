#[doc = "Register `MMR__VBUSP__CFG1_VTM_GT_TH2_INT_RAW_STAT_SET` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg1VtmGtTh2IntRawStatSetSpec>;
#[doc = "Register `MMR__VBUSP__CFG1_VTM_GT_TH2_INT_RAW_STAT_SET` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg1VtmGtTh2IntRawStatSetSpec>;
#[doc = "Field `INT_VD` reader - 7:0\\]
Interrupt pending bit set for gt_th2_int from VD\\[7:0\\]. Write-operation: 0: Nothing happens. 1: Causes the interrupt flag to be set. Used to manually force/drive an interrupt pending event. Reads return the pending status regardless of the enable setting."]
pub type IntVdR = crate::FieldReader;
#[doc = "Field `INT_VD` writer - 7:0\\]
Interrupt pending bit set for gt_th2_int from VD\\[7:0\\]. Write-operation: 0: Nothing happens. 1: Causes the interrupt flag to be set. Used to manually force/drive an interrupt pending event. Reads return the pending status regardless of the enable setting."]
pub type IntVdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt pending bit set for gt_th2_int from VD\\[7:0\\]. Write-operation: 0: Nothing happens. 1: Causes the interrupt flag to be set. Used to manually force/drive an interrupt pending event. Reads return the pending status regardless of the enable setting."]
    #[inline(always)]
    pub fn int_vd(&self) -> IntVdR {
        IntVdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt pending bit set for gt_th2_int from VD\\[7:0\\]. Write-operation: 0: Nothing happens. 1: Causes the interrupt flag to be set. Used to manually force/drive an interrupt pending event. Reads return the pending status regardless of the enable setting."]
    #[inline(always)]
    #[must_use]
    pub fn int_vd(&mut self) -> IntVdW<Mmr_Vbusp_Cfg1VtmGtTh2IntRawStatSetSpec> {
        IntVdW::new(self, 0)
    }
}
#[doc = "Interrupt RAW event status and set MMR for interrupt GT_TH2 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH2_INT_EN_STAT_CLR are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_gt_th2_int_raw_stat_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_gt_th2_int_raw_stat_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg1VtmGtTh2IntRawStatSetSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg1VtmGtTh2IntRawStatSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg1_vtm_gt_th2_int_raw_stat_set::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg1VtmGtTh2IntRawStatSetSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg1_vtm_gt_th2_int_raw_stat_set::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg1VtmGtTh2IntRawStatSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG1_VTM_GT_TH2_INT_RAW_STAT_SET to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg1VtmGtTh2IntRawStatSetSpec {
    const RESET_VALUE: u32 = 0;
}
