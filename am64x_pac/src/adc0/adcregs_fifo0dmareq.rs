#[doc = "Register `ADCREGS_FIFO0DMAREQ` reader"]
pub type R = crate::R<AdcregsFifo0dmareqSpec>;
#[doc = "Register `ADCREGS_FIFO0DMAREQ` writer"]
pub type W = crate::W<AdcregsFifo0dmareqSpec>;
#[doc = "Field `DMAREQLEVEL` reader - 7:0\\]
Number of words minus 1 in FIFO0 before generating a DMA request"]
pub type DmareqlevelR = crate::FieldReader;
#[doc = "Field `DMAREQLEVEL` writer - 7:0\\]
Number of words minus 1 in FIFO0 before generating a DMA request"]
pub type DmareqlevelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Number of words minus 1 in FIFO0 before generating a DMA request"]
    #[inline(always)]
    pub fn dmareqlevel(&self) -> DmareqlevelR {
        DmareqlevelR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Number of words minus 1 in FIFO0 before generating a DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn dmareqlevel(&mut self) -> DmareqlevelW<AdcregsFifo0dmareqSpec> {
        DmareqlevelW::new(self, 0)
    }
}
#[doc = "dma request.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_fifo0dmareq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_fifo0dmareq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsFifo0dmareqSpec;
impl crate::RegisterSpec for AdcregsFifo0dmareqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_fifo0dmareq::R`](R) reader structure"]
impl crate::Readable for AdcregsFifo0dmareqSpec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_fifo0dmareq::W`](W) writer structure"]
impl crate::Writable for AdcregsFifo0dmareqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_FIFO0DMAREQ to value 0"]
impl crate::Resettable for AdcregsFifo0dmareqSpec {
    const RESET_VALUE: u32 = 0;
}
