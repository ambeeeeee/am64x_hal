#[doc = "Register `ADCREGS_FIFO1WC` reader"]
pub type R = crate::R<AdcregsFifo1wcSpec>;
#[doc = "Register `ADCREGS_FIFO1WC` writer"]
pub type W = crate::W<AdcregsFifo1wcSpec>;
#[doc = "Field `NUMWDS` reader - 8:0\\]
number of words in the FIFO"]
pub type NumwdsR = crate::FieldReader<u16>;
#[doc = "Field `NUMWDS` writer - 8:0\\]
number of words in the FIFO"]
pub type NumwdsW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
number of words in the FIFO"]
    #[inline(always)]
    pub fn numwds(&self) -> NumwdsR {
        NumwdsR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
number of words in the FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn numwds(&mut self) -> NumwdsW<AdcregsFifo1wcSpec> {
        NumwdsW::new(self, 0)
    }
}
#[doc = "FIFO word count status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_fifo1wc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_fifo1wc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsFifo1wcSpec;
impl crate::RegisterSpec for AdcregsFifo1wcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_fifo1wc::R`](R) reader structure"]
impl crate::Readable for AdcregsFifo1wcSpec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_fifo1wc::W`](W) writer structure"]
impl crate::Writable for AdcregsFifo1wcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_FIFO1WC to value 0"]
impl crate::Resettable for AdcregsFifo1wcSpec {
    const RESET_VALUE: u32 = 0;
}
