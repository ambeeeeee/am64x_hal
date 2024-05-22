#[doc = "Register `MMR__VBUSP__CFG2_VTM_SAMPLE_CTRL` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg2VtmSampleCtrlSpec>;
#[doc = "Register `MMR__VBUSP__CFG2_VTM_SAMPLE_CTRL` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg2VtmSampleCtrlSpec>;
#[doc = "Field `SAMPLE_PER_CNT` reader - 15:0\\]
Temperature sensor sample period count selector. Device specific. Default set by e-fuse or tie-off. This defines the sample period or number of sensor clocks between consecutive samples of the sensor allowed, from the start of the previous sample request to the start of the next sample request. After the sample is received the sensor status MMR fields will not be updated and the sensor will remain disabled. Reset value is from e-fuse at POR, efuse_sample_per_cnt."]
pub type SamplePerCntR = crate::FieldReader<u16>;
#[doc = "Field `SAMPLE_PER_CNT` writer - 15:0\\]
Temperature sensor sample period count selector. Device specific. Default set by e-fuse or tie-off. This defines the sample period or number of sensor clocks between consecutive samples of the sensor allowed, from the start of the previous sample request to the start of the next sample request. After the sample is received the sensor status MMR fields will not be updated and the sensor will remain disabled. Reset value is from e-fuse at POR, efuse_sample_per_cnt."]
pub type SamplePerCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Temperature sensor sample period count selector. Device specific. Default set by e-fuse or tie-off. This defines the sample period or number of sensor clocks between consecutive samples of the sensor allowed, from the start of the previous sample request to the start of the next sample request. After the sample is received the sensor status MMR fields will not be updated and the sensor will remain disabled. Reset value is from e-fuse at POR, efuse_sample_per_cnt."]
    #[inline(always)]
    pub fn sample_per_cnt(&self) -> SamplePerCntR {
        SamplePerCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Temperature sensor sample period count selector. Device specific. Default set by e-fuse or tie-off. This defines the sample period or number of sensor clocks between consecutive samples of the sensor allowed, from the start of the previous sample request to the start of the next sample request. After the sample is received the sensor status MMR fields will not be updated and the sensor will remain disabled. Reset value is from e-fuse at POR, efuse_sample_per_cnt."]
    #[inline(always)]
    #[must_use]
    pub fn sample_per_cnt(&mut self) -> SamplePerCntW<Mmr_Vbusp_Cfg2VtmSampleCtrlSpec> {
        SamplePerCntW::new(self, 0)
    }
}
#[doc = "VTM sample related control MMR. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary. The e-fuse value is sampled from input port efuse_sample_clk_cnt. The sample_clk_cnt field is Device specific.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg2_vtm_sample_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg2_vtm_sample_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg2VtmSampleCtrlSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg2VtmSampleCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg2_vtm_sample_ctrl::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg2VtmSampleCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg2_vtm_sample_ctrl::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg2VtmSampleCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG2_VTM_SAMPLE_CTRL to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg2VtmSampleCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
