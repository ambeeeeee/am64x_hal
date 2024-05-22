#[doc = "Register `ADCREGS_ENABLE_SET` reader"]
pub type R = crate::R<AdcregsEnableSetSpec>;
#[doc = "Register `ADCREGS_ENABLE_SET` writer"]
pub type W = crate::W<AdcregsEnableSetSpec>;
#[doc = "Field `AFE_EOC_MISSING` reader - 0:0\\]
eoc from the analog front end missing at expected time after soc"]
pub type AfeEocMissingR = crate::BitReader;
#[doc = "Field `AFE_EOC_MISSING` writer - 0:0\\]
eoc from the analog front end missing at expected time after soc"]
pub type AfeEocMissingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDOFEQUENCE` reader - 1:1\\]
end of sequence"]
pub type EndofequenceR = crate::BitReader;
#[doc = "Field `ENDOFEQUENCE` writer - 1:1\\]
end of sequence"]
pub type EndofequenceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO0THRS` reader - 2:2\\]
fifo thresholds met"]
pub type Fifo0thrsR = crate::BitReader;
#[doc = "Field `FIFO0THRS` writer - 2:2\\]
fifo thresholds met"]
pub type Fifo0thrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO0OVFL` reader - 3:3\\]
fifo over flow"]
pub type Fifo0ovflR = crate::BitReader;
#[doc = "Field `FIFO0OVFL` writer - 3:3\\]
fifo over flow"]
pub type Fifo0ovflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO0UNFL` reader - 4:4\\]
fifo under flow"]
pub type Fifo0unflR = crate::BitReader;
#[doc = "Field `FIFO0UNFL` writer - 4:4\\]
fifo under flow"]
pub type Fifo0unflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO1THRS` reader - 5:5\\]
fifo thresholds met"]
pub type Fifo1thrsR = crate::BitReader;
#[doc = "Field `FIFO1THRS` writer - 5:5\\]
fifo thresholds met"]
pub type Fifo1thrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO1OVFL` reader - 6:6\\]
fifo over flow"]
pub type Fifo1ovflR = crate::BitReader;
#[doc = "Field `FIFO1OVFL` writer - 6:6\\]
fifo over flow"]
pub type Fifo1ovflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO1UNFL` reader - 7:7\\]
fifo under flow"]
pub type Fifo1unflR = crate::BitReader;
#[doc = "Field `FIFO1UNFL` writer - 7:7\\]
fifo under flow"]
pub type Fifo1unflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTOFRANGE` reader - 8:8\\]
sample out of range"]
pub type OutofrangeR = crate::BitReader;
#[doc = "Field `OUTOFRANGE` writer - 8:8\\]
sample out of range"]
pub type OutofrangeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
eoc from the analog front end missing at expected time after soc"]
    #[inline(always)]
    pub fn afe_eoc_missing(&self) -> AfeEocMissingR {
        AfeEocMissingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
end of sequence"]
    #[inline(always)]
    pub fn endofequence(&self) -> EndofequenceR {
        EndofequenceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
fifo thresholds met"]
    #[inline(always)]
    pub fn fifo0thrs(&self) -> Fifo0thrsR {
        Fifo0thrsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
fifo over flow"]
    #[inline(always)]
    pub fn fifo0ovfl(&self) -> Fifo0ovflR {
        Fifo0ovflR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
fifo under flow"]
    #[inline(always)]
    pub fn fifo0unfl(&self) -> Fifo0unflR {
        Fifo0unflR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
fifo thresholds met"]
    #[inline(always)]
    pub fn fifo1thrs(&self) -> Fifo1thrsR {
        Fifo1thrsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
fifo over flow"]
    #[inline(always)]
    pub fn fifo1ovfl(&self) -> Fifo1ovflR {
        Fifo1ovflR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
fifo under flow"]
    #[inline(always)]
    pub fn fifo1unfl(&self) -> Fifo1unflR {
        Fifo1unflR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
sample out of range"]
    #[inline(always)]
    pub fn outofrange(&self) -> OutofrangeR {
        OutofrangeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
eoc from the analog front end missing at expected time after soc"]
    #[inline(always)]
    #[must_use]
    pub fn afe_eoc_missing(&mut self) -> AfeEocMissingW<AdcregsEnableSetSpec> {
        AfeEocMissingW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
end of sequence"]
    #[inline(always)]
    #[must_use]
    pub fn endofequence(&mut self) -> EndofequenceW<AdcregsEnableSetSpec> {
        EndofequenceW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
fifo thresholds met"]
    #[inline(always)]
    #[must_use]
    pub fn fifo0thrs(&mut self) -> Fifo0thrsW<AdcregsEnableSetSpec> {
        Fifo0thrsW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
fifo over flow"]
    #[inline(always)]
    #[must_use]
    pub fn fifo0ovfl(&mut self) -> Fifo0ovflW<AdcregsEnableSetSpec> {
        Fifo0ovflW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
fifo under flow"]
    #[inline(always)]
    #[must_use]
    pub fn fifo0unfl(&mut self) -> Fifo0unflW<AdcregsEnableSetSpec> {
        Fifo0unflW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
fifo thresholds met"]
    #[inline(always)]
    #[must_use]
    pub fn fifo1thrs(&mut self) -> Fifo1thrsW<AdcregsEnableSetSpec> {
        Fifo1thrsW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
fifo over flow"]
    #[inline(always)]
    #[must_use]
    pub fn fifo1ovfl(&mut self) -> Fifo1ovflW<AdcregsEnableSetSpec> {
        Fifo1ovflW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
fifo under flow"]
    #[inline(always)]
    #[must_use]
    pub fn fifo1unfl(&mut self) -> Fifo1unflW<AdcregsEnableSetSpec> {
        Fifo1unflW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
sample out of range"]
    #[inline(always)]
    #[must_use]
    pub fn outofrange(&mut self) -> OutofrangeW<AdcregsEnableSetSpec> {
        OutofrangeW::new(self, 8)
    }
}
#[doc = "The IRQ_ENABLE_SET register allows the adc12 interrupt sources to be manually enabled when writing a 1 to a specific bit. Write 0: No action Write 1: Enable event Read 0: Event is disabled Read 1: Event is enabled\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_enable_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_enable_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsEnableSetSpec;
impl crate::RegisterSpec for AdcregsEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_enable_set::R`](R) reader structure"]
impl crate::Readable for AdcregsEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_enable_set::W`](W) writer structure"]
impl crate::Writable for AdcregsEnableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_ENABLE_SET to value 0"]
impl crate::Resettable for AdcregsEnableSetSpec {
    const RESET_VALUE: u32 = 0;
}
