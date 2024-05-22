#[doc = "Register `MMR__VBUSP__CFG1_TH` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg1ThSpec>;
#[doc = "Register `MMR__VBUSP__CFG1_TH` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg1ThSpec>;
#[doc = "Field `TH0_VAL` reader - 9:0\\]
Threshold point-0, thpt0, temp-value. Is a 10-bit temperature reference value. This is the 10-bit value that represents a high temperature point, as per sensor spec, with which to compare the present temperature reading, same 10-bit format. If the current temperature reading is &lt; th0_val\\[9:0\\]
and lt_th0_en=1, that will cause the VTM to set the lt_th0_alert output. Once the current temp reading => th0_val\\[9:0\\]
then lt_th0_alert output will go to 0. Reset value is at POR only."]
pub type Th0ValR = crate::FieldReader<u16>;
#[doc = "Field `TH0_VAL` writer - 9:0\\]
Threshold point-0, thpt0, temp-value. Is a 10-bit temperature reference value. This is the 10-bit value that represents a high temperature point, as per sensor spec, with which to compare the present temperature reading, same 10-bit format. If the current temperature reading is &lt; th0_val\\[9:0\\]
and lt_th0_en=1, that will cause the VTM to set the lt_th0_alert output. Once the current temp reading => th0_val\\[9:0\\]
then lt_th0_alert output will go to 0. Reset value is at POR only."]
pub type Th0ValW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TH1_VAL` reader - 25:16\\]
Threshold point-1, thpt1, temp-value. Is a 10-bit temperature reference value. This is the 10-bit value that represents a high temperature point, as per sensor spec, with which to compare the present temperature reading, same 10-bit format. If the current temperature reading is > th1_val\\[9:0\\]
and gt_th1_en=1, that will cause the VTM to set the gt_th1_alert output. Once the current temp reading =&lt; th1_val\\[9:0\\]
then gt_th1_alert output will go to 0. Reset value is at POR only."]
pub type Th1ValR = crate::FieldReader<u16>;
#[doc = "Field `TH1_VAL` writer - 25:16\\]
Threshold point-1, thpt1, temp-value. Is a 10-bit temperature reference value. This is the 10-bit value that represents a high temperature point, as per sensor spec, with which to compare the present temperature reading, same 10-bit format. If the current temperature reading is > th1_val\\[9:0\\]
and gt_th1_en=1, that will cause the VTM to set the gt_th1_alert output. Once the current temp reading =&lt; th1_val\\[9:0\\]
then gt_th1_alert output will go to 0. Reset value is at POR only."]
pub type Th1ValW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Threshold point-0, thpt0, temp-value. Is a 10-bit temperature reference value. This is the 10-bit value that represents a high temperature point, as per sensor spec, with which to compare the present temperature reading, same 10-bit format. If the current temperature reading is &lt; th0_val\\[9:0\\]
and lt_th0_en=1, that will cause the VTM to set the lt_th0_alert output. Once the current temp reading => th0_val\\[9:0\\]
then lt_th0_alert output will go to 0. Reset value is at POR only."]
    #[inline(always)]
    pub fn th0_val(&self) -> Th0ValR {
        Th0ValR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Threshold point-1, thpt1, temp-value. Is a 10-bit temperature reference value. This is the 10-bit value that represents a high temperature point, as per sensor spec, with which to compare the present temperature reading, same 10-bit format. If the current temperature reading is > th1_val\\[9:0\\]
and gt_th1_en=1, that will cause the VTM to set the gt_th1_alert output. Once the current temp reading =&lt; th1_val\\[9:0\\]
then gt_th1_alert output will go to 0. Reset value is at POR only."]
    #[inline(always)]
    pub fn th1_val(&self) -> Th1ValR {
        Th1ValR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Threshold point-0, thpt0, temp-value. Is a 10-bit temperature reference value. This is the 10-bit value that represents a high temperature point, as per sensor spec, with which to compare the present temperature reading, same 10-bit format. If the current temperature reading is &lt; th0_val\\[9:0\\]
and lt_th0_en=1, that will cause the VTM to set the lt_th0_alert output. Once the current temp reading => th0_val\\[9:0\\]
then lt_th0_alert output will go to 0. Reset value is at POR only."]
    #[inline(always)]
    #[must_use]
    pub fn th0_val(&mut self) -> Th0ValW<Mmr_Vbusp_Cfg1ThSpec> {
        Th0ValW::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Threshold point-1, thpt1, temp-value. Is a 10-bit temperature reference value. This is the 10-bit value that represents a high temperature point, as per sensor spec, with which to compare the present temperature reading, same 10-bit format. If the current temperature reading is > th1_val\\[9:0\\]
and gt_th1_en=1, that will cause the VTM to set the gt_th1_alert output. Once the current temp reading =&lt; th1_val\\[9:0\\]
then gt_th1_alert output will go to 0. Reset value is at POR only."]
    #[inline(always)]
    #[must_use]
    pub fn th1_val(&mut self) -> Th1ValW<Mmr_Vbusp_Cfg1ThSpec> {
        Th1ValW::new(self, 16)
    }
}
#[doc = "Temperature Sensor Band-gap Threshold register for sensor a.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_th::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_th::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg1ThSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg1ThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg1_th::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg1ThSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg1_th::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg1ThSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG1_TH to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg1ThSpec {
    const RESET_VALUE: u32 = 0;
}
