#[doc = "Register `ADC12_FIFO_DMA_FIFO1DMADATA` reader"]
pub type R = crate::R<Adc12FifoDmaFifo1dmadataSpec>;
#[doc = "Register `ADC12_FIFO_DMA_FIFO1DMADATA` writer"]
pub type W = crate::W<Adc12FifoDmaFifo1dmadataSpec>;
#[doc = "Field `ADCDATA` reader - 11:0\\]
sampled ADC converted data value stored in FIFO"]
pub type AdcdataR = crate::FieldReader<u16>;
#[doc = "Field `ADCDATA` writer - 11:0\\]
sampled ADC converted data value stored in FIFO"]
pub type AdcdataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ADCCHANLID` reader - 19:16\\]
Optional ID tag of channel that captured the data. If tag option is disabled, these bits will be 0"]
pub type AdcchanlidR = crate::FieldReader;
#[doc = "Field `ADCCHANLID` writer - 19:16\\]
Optional ID tag of channel that captured the data. If tag option is disabled, these bits will be 0"]
pub type AdcchanlidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
sampled ADC converted data value stored in FIFO"]
    #[inline(always)]
    pub fn adcdata(&self) -> AdcdataR {
        AdcdataR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Optional ID tag of channel that captured the data. If tag option is disabled, these bits will be 0"]
    #[inline(always)]
    pub fn adcchanlid(&self) -> AdcchanlidR {
        AdcchanlidR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
sampled ADC converted data value stored in FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn adcdata(&mut self) -> AdcdataW<Adc12FifoDmaFifo1dmadataSpec> {
        AdcdataW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Optional ID tag of channel that captured the data. If tag option is disabled, these bits will be 0"]
    #[inline(always)]
    #[must_use]
    pub fn adcchanlid(&mut self) -> AdcchanlidW<Adc12FifoDmaFifo1dmadataSpec> {
        AdcchanlidW::new(self, 16)
    }
}
#[doc = "DMA sample FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12_fifo_dma_fifo1dmadata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12_fifo_dma_fifo1dmadata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12FifoDmaFifo1dmadataSpec;
impl crate::RegisterSpec for Adc12FifoDmaFifo1dmadataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc12_fifo_dma_fifo1dmadata::R`](R) reader structure"]
impl crate::Readable for Adc12FifoDmaFifo1dmadataSpec {}
#[doc = "`write(|w| ..)` method takes [`adc12_fifo_dma_fifo1dmadata::W`](W) writer structure"]
impl crate::Writable for Adc12FifoDmaFifo1dmadataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC12_FIFO_DMA_FIFO1DMADATA to value 0"]
impl crate::Resettable for Adc12FifoDmaFifo1dmadataSpec {
    const RESET_VALUE: u32 = 0;
}
