#[doc = "Register `MMR__VBUSP__CFG1_TH2` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg1Th2Spec>;
#[doc = "Register `MMR__VBUSP__CFG1_TH2` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg1Th2Spec>;
#[doc = "Field `TH2_VAL` reader - 9:0\\]
Threshold point-2, thpt2, temp-value. Is a 10-bit temperature reference value. This is the 10-bit value that represents a high temperature point, as per sensor spec, with which to compare the present temperature reading, same 10-bit format. If the current temperature reading is > th2_val\\[9:0\\]
and gt_th2_en=1, that will cause the VTM to set the gt_th2_alert output. Once the current temp reading =&lt; th2_val\\[9:0\\]
then gt_th2_alert output will go to 0. Reset value is at POR only."]
pub type Th2ValR = crate::FieldReader<u16>;
#[doc = "Field `TH2_VAL` writer - 9:0\\]
Threshold point-2, thpt2, temp-value. Is a 10-bit temperature reference value. This is the 10-bit value that represents a high temperature point, as per sensor spec, with which to compare the present temperature reading, same 10-bit format. If the current temperature reading is > th2_val\\[9:0\\]
and gt_th2_en=1, that will cause the VTM to set the gt_th2_alert output. Once the current temp reading =&lt; th2_val\\[9:0\\]
then gt_th2_alert output will go to 0. Reset value is at POR only."]
pub type Th2ValW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Threshold point-2, thpt2, temp-value. Is a 10-bit temperature reference value. This is the 10-bit value that represents a high temperature point, as per sensor spec, with which to compare the present temperature reading, same 10-bit format. If the current temperature reading is > th2_val\\[9:0\\]
and gt_th2_en=1, that will cause the VTM to set the gt_th2_alert output. Once the current temp reading =&lt; th2_val\\[9:0\\]
then gt_th2_alert output will go to 0. Reset value is at POR only."]
    #[inline(always)]
    pub fn th2_val(&self) -> Th2ValR {
        Th2ValR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Threshold point-2, thpt2, temp-value. Is a 10-bit temperature reference value. This is the 10-bit value that represents a high temperature point, as per sensor spec, with which to compare the present temperature reading, same 10-bit format. If the current temperature reading is > th2_val\\[9:0\\]
and gt_th2_en=1, that will cause the VTM to set the gt_th2_alert output. Once the current temp reading =&lt; th2_val\\[9:0\\]
then gt_th2_alert output will go to 0. Reset value is at POR only."]
    #[inline(always)]
    #[must_use]
    pub fn th2_val(&mut self) -> Th2ValW<Mmr_Vbusp_Cfg1Th2Spec> {
        Th2ValW::new(self, 0)
    }
}
#[doc = "Temperature Sensor Band-gap Threshold register 2 for sensor a.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_th2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_th2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg1Th2Spec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg1Th2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg1_th2::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg1Th2Spec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg1_th2::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg1Th2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG1_TH2 to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg1Th2Spec {
    const RESET_VALUE: u32 = 0;
}
