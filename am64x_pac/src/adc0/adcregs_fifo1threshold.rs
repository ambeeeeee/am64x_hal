#[doc = "Register `ADCREGS_FIFO1THRESHOLD` reader"]
pub type R = crate::R<AdcregsFifo1thresholdSpec>;
#[doc = "Register `ADCREGS_FIFO1THRESHOLD` writer"]
pub type W = crate::W<AdcregsFifo1thresholdSpec>;
#[doc = "Field `THRESHOLD` reader - 7:0\\]
Program the desired FIFO1 data sample level minus 1 to reach before generating interrupt to CPU"]
pub type ThresholdR = crate::FieldReader;
#[doc = "Field `THRESHOLD` writer - 7:0\\]
Program the desired FIFO1 data sample level minus 1 to reach before generating interrupt to CPU"]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Program the desired FIFO1 data sample level minus 1 to reach before generating interrupt to CPU"]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Program the desired FIFO1 data sample level minus 1 to reach before generating interrupt to CPU"]
    #[inline(always)]
    #[must_use]
    pub fn threshold(&mut self) -> ThresholdW<AdcregsFifo1thresholdSpec> {
        ThresholdW::new(self, 0)
    }
}
#[doc = "FIFO threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_fifo1threshold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_fifo1threshold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsFifo1thresholdSpec;
impl crate::RegisterSpec for AdcregsFifo1thresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_fifo1threshold::R`](R) reader structure"]
impl crate::Readable for AdcregsFifo1thresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_fifo1threshold::W`](W) writer structure"]
impl crate::Writable for AdcregsFifo1thresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_FIFO1THRESHOLD to value 0"]
impl crate::Resettable for AdcregsFifo1thresholdSpec {
    const RESET_VALUE: u32 = 0;
}
