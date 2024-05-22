#[doc = "Register `MMR__VBUSP__CFG1_VTM_GT_TH2_INT_EN_SET` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg1VtmGtTh2IntEnSetSpec>;
#[doc = "Register `MMR__VBUSP__CFG1_VTM_GT_TH2_INT_EN_SET` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg1VtmGtTh2IntEnSetSpec>;
#[doc = "Field `INT_VD` reader - 7:0\\]
Interrupt enable bit for gt_th2_int from VD\\[7:0\\]. Write-operation: 0: Nothing happens. 1: Causes the bit to be set to 1. Reads return the enable settings."]
pub type IntVdR = crate::FieldReader;
#[doc = "Field `INT_VD` writer - 7:0\\]
Interrupt enable bit for gt_th2_int from VD\\[7:0\\]. Write-operation: 0: Nothing happens. 1: Causes the bit to be set to 1. Reads return the enable settings."]
pub type IntVdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt enable bit for gt_th2_int from VD\\[7:0\\]. Write-operation: 0: Nothing happens. 1: Causes the bit to be set to 1. Reads return the enable settings."]
    #[inline(always)]
    pub fn int_vd(&self) -> IntVdR {
        IntVdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt enable bit for gt_th2_int from VD\\[7:0\\]. Write-operation: 0: Nothing happens. 1: Causes the bit to be set to 1. Reads return the enable settings."]
    #[inline(always)]
    #[must_use]
    pub fn int_vd(&mut self) -> IntVdW<Mmr_Vbusp_Cfg1VtmGtTh2IntEnSetSpec> {
        IntVdW::new(self, 0)
    }
}
#[doc = "Enable set MMR for interrupt GT_TH2 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH2_INT_EN_CLR are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_gt_th2_int_en_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_gt_th2_int_en_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg1VtmGtTh2IntEnSetSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg1VtmGtTh2IntEnSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg1_vtm_gt_th2_int_en_set::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg1VtmGtTh2IntEnSetSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg1_vtm_gt_th2_int_en_set::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg1VtmGtTh2IntEnSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG1_VTM_GT_TH2_INT_EN_SET to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg1VtmGtTh2IntEnSetSpec {
    const RESET_VALUE: u32 = 0;
}
