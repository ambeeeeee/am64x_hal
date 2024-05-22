#[doc = "Register `MEM_IER2` reader"]
pub type R = crate::R<MemIer2Spec>;
#[doc = "Register `MEM_IER2` writer"]
pub type W = crate::W<MemIer2Spec>;
#[doc = "Field `EN_RXFIFO_EMPTY` reader - 0:0\\]
Enables\\[1\\]/disables\\[0\\]
EN_RXFIFO_EMPTY interrupt."]
pub type EnRxfifoEmptyR = crate::BitReader;
#[doc = "Field `EN_RXFIFO_EMPTY` writer - 0:0\\]
Enables\\[1\\]/disables\\[0\\]
EN_RXFIFO_EMPTY interrupt."]
pub type EnRxfifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_TXFIFO_EMPTY` reader - 1:1\\]
Enables\\[1\\]/DISABLES\\[00 EN_TXFIFO_EMPTY interrupt."]
pub type EnTxfifoEmptyR = crate::BitReader;
#[doc = "Field `EN_TXFIFO_EMPTY` writer - 1:1\\]
Enables\\[1\\]/DISABLES\\[00 EN_TXFIFO_EMPTY interrupt."]
pub type EnTxfifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RHR_IT_DIS` reader - "]
pub type RhrItDisR = crate::BitReader;
#[doc = "Field `RHR_IT_DIS` writer - "]
pub type RhrItDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - "]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - "]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables\\[1\\]/disables\\[0\\]
EN_RXFIFO_EMPTY interrupt."]
    #[inline(always)]
    pub fn en_rxfifo_empty(&self) -> EnRxfifoEmptyR {
        EnRxfifoEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables\\[1\\]/DISABLES\\[00 EN_TXFIFO_EMPTY interrupt."]
    #[inline(always)]
    pub fn en_txfifo_empty(&self) -> EnTxfifoEmptyR {
        EnTxfifoEmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rhr_it_dis(&self) -> RhrItDisR {
        RhrItDisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables\\[1\\]/disables\\[0\\]
EN_RXFIFO_EMPTY interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn en_rxfifo_empty(&mut self) -> EnRxfifoEmptyW<MemIer2Spec> {
        EnRxfifoEmptyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables\\[1\\]/DISABLES\\[00 EN_TXFIFO_EMPTY interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn en_txfifo_empty(&mut self) -> EnTxfifoEmptyW<MemIer2Spec> {
        EnTxfifoEmptyW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rhr_it_dis(&mut self) -> RhrItDisW<MemIer2Spec> {
        RhrItDisW::new(self, 2)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<MemIer2Spec> {
        Reserved1W::new(self, 8)
    }
}
#[doc = "Enables RX/TX FIFOs empty corresponding interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ier2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ier2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemIer2Spec;
impl crate::RegisterSpec for MemIer2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ier2::R`](R) reader structure"]
impl crate::Readable for MemIer2Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_ier2::W`](W) writer structure"]
impl crate::Writable for MemIer2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_IER2 to value 0"]
impl crate::Resettable for MemIer2Spec {
    const RESET_VALUE: u32 = 0;
}
