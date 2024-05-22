#[doc = "Register `ADCREGS_FIFO0DATA` reader"]
pub type R = crate::R<AdcregsFifo0dataSpec>;
#[doc = "Register `ADCREGS_FIFO0DATA` writer"]
pub type W = crate::W<AdcregsFifo0dataSpec>;
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
    pub fn adcdata(&mut self) -> AdcdataW<AdcregsFifo0dataSpec> {
        AdcdataW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Optional ID tag of channel that captured the data. If tag option is disabled, these bits will be 0"]
    #[inline(always)]
    #[must_use]
    pub fn adcchanlid(&mut self) -> AdcchanlidW<AdcregsFifo0dataSpec> {
        AdcchanlidW::new(self, 16)
    }
}
#[doc = "A read from this register will auto increment the FIFO read pointer. If you read when FIFO is empty,it will trigger an underflow interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_fifo0data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_fifo0data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsFifo0dataSpec;
impl crate::RegisterSpec for AdcregsFifo0dataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_fifo0data::R`](R) reader structure"]
impl crate::Readable for AdcregsFifo0dataSpec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_fifo0data::W`](W) writer structure"]
impl crate::Writable for AdcregsFifo0dataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_FIFO0DATA to value 0"]
impl crate::Resettable for AdcregsFifo0dataSpec {
    const RESET_VALUE: u32 = 0;
}
