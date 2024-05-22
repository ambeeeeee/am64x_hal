#[doc = "Register `MEM_TCR` reader"]
pub type R = crate::R<MemTcrSpec>;
#[doc = "Register `MEM_TCR` writer"]
pub type W = crate::W<MemTcrSpec>;
#[doc = "Field `RX_FIFO_TRIG_HALT` reader - 3:0\\]
RX FIFO trigger level to HALT transmission (0 - 60)"]
pub type RxFifoTrigHaltR = crate::FieldReader;
#[doc = "Field `RX_FIFO_TRIG_HALT` writer - 3:0\\]
RX FIFO trigger level to HALT transmission (0 - 60)"]
pub type RxFifoTrigHaltW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_FIFO_TRIG_START` reader - 7:4\\]
RX FIFO trigger level to RESTORE transmission (0 - 60)"]
pub type RxFifoTrigStartR = crate::FieldReader;
#[doc = "Field `RX_FIFO_TRIG_START` writer - 7:4\\]
RX FIFO trigger level to RESTORE transmission (0 - 60)"]
pub type RxFifoTrigStartW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
RX FIFO trigger level to HALT transmission (0 - 60)"]
    #[inline(always)]
    pub fn rx_fifo_trig_halt(&self) -> RxFifoTrigHaltR {
        RxFifoTrigHaltR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
RX FIFO trigger level to RESTORE transmission (0 - 60)"]
    #[inline(always)]
    pub fn rx_fifo_trig_start(&self) -> RxFifoTrigStartR {
        RxFifoTrigStartR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
RX FIFO trigger level to HALT transmission (0 - 60)"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_trig_halt(&mut self) -> RxFifoTrigHaltW<MemTcrSpec> {
        RxFifoTrigHaltW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
RX FIFO trigger level to RESTORE transmission (0 - 60)"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_trig_start(&mut self) -> RxFifoTrigStartW<MemTcrSpec> {
        RxFifoTrigStartW::new(self, 4)
    }
}
#[doc = "Transmission Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemTcrSpec;
impl crate::RegisterSpec for MemTcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_tcr::R`](R) reader structure"]
impl crate::Readable for MemTcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_tcr::W`](W) writer structure"]
impl crate::Writable for MemTcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_TCR to value 0x15"]
impl crate::Resettable for MemTcrSpec {
    const RESET_VALUE: u32 = 0x15;
}
