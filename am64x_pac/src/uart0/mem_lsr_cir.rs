#[doc = "Register `MEM_LSR_CIR` reader"]
pub type R = crate::R<MemLsrCirSpec>;
#[doc = "Register `MEM_LSR_CIR` writer"]
pub type W = crate::W<MemLsrCirSpec>;
#[doc = "Field `RX_FIFO_E` reader - "]
pub type RxFifoER = crate::BitReader;
#[doc = "Field `RX_FIFO_E` writer - "]
pub type RxFifoEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_STOP` reader - 5:5\\]
The RX_STOP is generated based on the value set in the BOF Length register (EBLR). It is cleared on a single read of the LSR register"]
pub type RxStopR = crate::BitReader;
#[doc = "Field `RX_STOP` writer - 5:5\\]
The RX_STOP is generated based on the value set in the BOF Length register (EBLR). It is cleared on a single read of the LSR register"]
pub type RxStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_EMPTY` reader - "]
pub type ThrEmptyR = crate::BitReader;
#[doc = "Field `THR_EMPTY` writer - "]
pub type ThrEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_fifo_e(&self) -> RxFifoER {
        RxFifoER::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
The RX_STOP is generated based on the value set in the BOF Length register (EBLR). It is cleared on a single read of the LSR register"]
    #[inline(always)]
    pub fn rx_stop(&self) -> RxStopR {
        RxStopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn thr_empty(&self) -> ThrEmptyR {
        ThrEmptyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_e(&mut self) -> RxFifoEW<MemLsrCirSpec> {
        RxFifoEW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
The RX_STOP is generated based on the value set in the BOF Length register (EBLR). It is cleared on a single read of the LSR register"]
    #[inline(always)]
    #[must_use]
    pub fn rx_stop(&mut self) -> RxStopW<MemLsrCirSpec> {
        RxStopW::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn thr_empty(&mut self) -> ThrEmptyW<MemLsrCirSpec> {
        ThrEmptyW::new(self, 7)
    }
}
#[doc = "MEM_LSR_CIR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_lsr_cir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_lsr_cir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemLsrCirSpec;
impl crate::RegisterSpec for MemLsrCirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_lsr_cir::R`](R) reader structure"]
impl crate::Readable for MemLsrCirSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_lsr_cir::W`](W) writer structure"]
impl crate::Writable for MemLsrCirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_LSR_CIR to value 0x81"]
impl crate::Resettable for MemLsrCirSpec {
    const RESET_VALUE: u32 = 0x81;
}
