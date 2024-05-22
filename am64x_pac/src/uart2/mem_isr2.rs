#[doc = "Register `MEM_ISR2` reader"]
pub type R = crate::R<MemIsr2Spec>;
#[doc = "Register `MEM_ISR2` writer"]
pub type W = crate::W<MemIsr2Spec>;
#[doc = "Field `RXFIFO_EMPTY_STS` reader - 0:0\\]
RXFIFO interrupt pending"]
pub type RxfifoEmptyStsR = crate::BitReader;
#[doc = "Field `RXFIFO_EMPTY_STS` writer - 0:0\\]
RXFIFO interrupt pending"]
pub type RxfifoEmptyStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_EMPTY_STS` reader - 1:1\\]
TXFIFO interrupt pending"]
pub type TxfifoEmptyStsR = crate::BitReader;
#[doc = "Field `TXFIFO_EMPTY_STS` writer - 1:1\\]
TXFIFO interrupt pending"]
pub type TxfifoEmptyStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - "]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - "]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
RXFIFO interrupt pending"]
    #[inline(always)]
    pub fn rxfifo_empty_sts(&self) -> RxfifoEmptyStsR {
        RxfifoEmptyStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TXFIFO interrupt pending"]
    #[inline(always)]
    pub fn txfifo_empty_sts(&self) -> TxfifoEmptyStsR {
        TxfifoEmptyStsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
RXFIFO interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_empty_sts(&mut self) -> RxfifoEmptyStsW<MemIsr2Spec> {
        RxfifoEmptyStsW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TXFIFO interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_empty_sts(&mut self) -> TxfifoEmptyStsW<MemIsr2Spec> {
        TxfifoEmptyStsW::new(self, 1)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<MemIsr2Spec> {
        Reserved1W::new(self, 8)
    }
}
#[doc = "Status of RX/TX FIFOs empty corresponding interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_isr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_isr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemIsr2Spec;
impl crate::RegisterSpec for MemIsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_isr2::R`](R) reader structure"]
impl crate::Readable for MemIsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_isr2::W`](W) writer structure"]
impl crate::Writable for MemIsr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ISR2 to value 0x03"]
impl crate::Resettable for MemIsr2Spec {
    const RESET_VALUE: u32 = 0x03;
}
