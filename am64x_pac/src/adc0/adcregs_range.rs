#[doc = "Register `ADCREGS_RANGE` reader"]
pub type R = crate::R<AdcregsRangeSpec>;
#[doc = "Register `ADCREGS_RANGE` writer"]
pub type W = crate::W<AdcregsRangeSpec>;
#[doc = "Field `LOWRANGE` reader - 11:0\\]
If the sampled data is &lt; value, then interrupt is generated"]
pub type LowrangeR = crate::FieldReader<u16>;
#[doc = "Field `LOWRANGE` writer - 11:0\\]
If the sampled data is &lt; value, then interrupt is generated"]
pub type LowrangeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HIRANGE` reader - 27:16\\]
If the sampled data is > value, then interrupt is generated"]
pub type HirangeR = crate::FieldReader<u16>;
#[doc = "Field `HIRANGE` writer - 27:16\\]
If the sampled data is > value, then interrupt is generated"]
pub type HirangeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
If the sampled data is &lt; value, then interrupt is generated"]
    #[inline(always)]
    pub fn lowrange(&self) -> LowrangeR {
        LowrangeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
If the sampled data is > value, then interrupt is generated"]
    #[inline(always)]
    pub fn hirange(&self) -> HirangeR {
        HirangeR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
If the sampled data is &lt; value, then interrupt is generated"]
    #[inline(always)]
    #[must_use]
    pub fn lowrange(&mut self) -> LowrangeW<AdcregsRangeSpec> {
        LowrangeW::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
If the sampled data is > value, then interrupt is generated"]
    #[inline(always)]
    #[must_use]
    pub fn hirange(&mut self) -> HirangeW<AdcregsRangeSpec> {
        HirangeW::new(self, 16)
    }
}
#[doc = "This feature requires the range check interrupt bit to be enabled first. The user can decide which channel input is compared by programming the RangeCheck bit of the StepConfig Registers. It is up to software to sort through FIFO data to determine which channel data was out of range. Software should only use LSB 10 bits for comparison and make sure bits 11,12 are 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_range::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_range::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsRangeSpec;
impl crate::RegisterSpec for AdcregsRangeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_range::R`](R) reader structure"]
impl crate::Readable for AdcregsRangeSpec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_range::W`](W) writer structure"]
impl crate::Writable for AdcregsRangeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_RANGE to value 0"]
impl crate::Resettable for AdcregsRangeSpec {
    const RESET_VALUE: u32 = 0;
}
