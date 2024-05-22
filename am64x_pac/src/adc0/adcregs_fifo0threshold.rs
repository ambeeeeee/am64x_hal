#[doc = "Register `ADCREGS_FIFO0THRESHOLD` reader"]
pub type R = crate::R<AdcregsFifo0thresholdSpec>;
#[doc = "Register `ADCREGS_FIFO0THRESHOLD` writer"]
pub type W = crate::W<AdcregsFifo0thresholdSpec>;
#[doc = "Field `THRESHOLD` reader - 7:0\\]
Program the desired FIFO0 data sample level minus 1 to reach before generating interrupt to CPU"]
pub type ThresholdR = crate::FieldReader;
#[doc = "Field `THRESHOLD` writer - 7:0\\]
Program the desired FIFO0 data sample level minus 1 to reach before generating interrupt to CPU"]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Program the desired FIFO0 data sample level minus 1 to reach before generating interrupt to CPU"]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Program the desired FIFO0 data sample level minus 1 to reach before generating interrupt to CPU"]
    #[inline(always)]
    #[must_use]
    pub fn threshold(&mut self) -> ThresholdW<AdcregsFifo0thresholdSpec> {
        ThresholdW::new(self, 0)
    }
}
#[doc = "FIFO threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_fifo0threshold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_fifo0threshold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsFifo0thresholdSpec;
impl crate::RegisterSpec for AdcregsFifo0thresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_fifo0threshold::R`](R) reader structure"]
impl crate::Readable for AdcregsFifo0thresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_fifo0threshold::W`](W) writer structure"]
impl crate::Writable for AdcregsFifo0thresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_FIFO0THRESHOLD to value 0"]
impl crate::Resettable for AdcregsFifo0thresholdSpec {
    const RESET_VALUE: u32 = 0;
}
