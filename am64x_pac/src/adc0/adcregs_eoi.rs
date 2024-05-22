#[doc = "Register `ADCREGS_EOI` reader"]
pub type R = crate::R<AdcregsEoiSpec>;
#[doc = "Register `ADCREGS_EOI` writer"]
pub type W = crate::W<AdcregsEoiSpec>;
#[doc = "Field `LINENUMEOI` reader - 0:0\\]
Write 0 to flag End Of Interrupt."]
pub type LinenumeoiR = crate::BitReader;
#[doc = "Field `LINENUMEOI` writer - 0:0\\]
Write 0 to flag End Of Interrupt."]
pub type LinenumeoiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write 0 to flag End Of Interrupt."]
    #[inline(always)]
    pub fn linenumeoi(&self) -> LinenumeoiR {
        LinenumeoiR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write 0 to flag End Of Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn linenumeoi(&mut self) -> LinenumeoiW<AdcregsEoiSpec> {
        LinenumeoiW::new(self, 0)
    }
}
#[doc = "The End of Interrupt (EOI) MISC Register allows the CPU to acknowledge completion of an interrupt by writing to the EOI for MISC interrupt sources. An eoi_write signal will be generated and another interrupt will be triggered if interrupt sources remain. This register will be reset one cycle after it has been written to.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_eoi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_eoi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsEoiSpec;
impl crate::RegisterSpec for AdcregsEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_eoi::R`](R) reader structure"]
impl crate::Readable for AdcregsEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_eoi::W`](W) writer structure"]
impl crate::Writable for AdcregsEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_EOI to value 0"]
impl crate::Resettable for AdcregsEoiSpec {
    const RESET_VALUE: u32 = 0;
}
