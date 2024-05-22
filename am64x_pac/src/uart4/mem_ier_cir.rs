#[doc = "Register `MEM_IER_CIR` reader"]
pub type R = crate::R<MemIerCirSpec>;
#[doc = "Register `MEM_IER_CIR` writer"]
pub type W = crate::W<MemIerCirSpec>;
#[doc = "Field `RHR_IT` reader - "]
pub type RhrItR = crate::BitReader;
#[doc = "Field `RHR_IT` writer - "]
pub type RhrItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_IT` reader - "]
pub type ThrItR = crate::BitReader;
#[doc = "Field `THR_IT` writer - "]
pub type ThrItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_STOP_IT` reader - "]
pub type RxStopItR = crate::BitReader;
#[doc = "Field `RX_STOP_IT` writer - "]
pub type RxStopItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVERRUN_IT` reader - "]
pub type RxOverrunItR = crate::BitReader;
#[doc = "Field `RX_OVERRUN_IT` writer - "]
pub type RxOverrunItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOT_USED1` reader - "]
pub type NotUsed1R = crate::BitReader;
#[doc = "Field `NOT_USED1` writer - "]
pub type NotUsed1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_STATUS_IT` reader - "]
pub type TxStatusItR = crate::BitReader;
#[doc = "Field `TX_STATUS_IT` writer - "]
pub type TxStatusItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOT_USED2` reader - "]
pub type NotUsed2R = crate::FieldReader;
#[doc = "Field `NOT_USED2` writer - "]
pub type NotUsed2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rhr_it(&self) -> RhrItR {
        RhrItR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn thr_it(&self) -> ThrItR {
        ThrItR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_stop_it(&self) -> RxStopItR {
        RxStopItR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_overrun_it(&self) -> RxOverrunItR {
        RxOverrunItR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn not_used1(&self) -> NotUsed1R {
        NotUsed1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_status_it(&self) -> TxStatusItR {
        TxStatusItR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn not_used2(&self) -> NotUsed2R {
        NotUsed2R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rhr_it(&mut self) -> RhrItW<MemIerCirSpec> {
        RhrItW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn thr_it(&mut self) -> ThrItW<MemIerCirSpec> {
        ThrItW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rx_stop_it(&mut self) -> RxStopItW<MemIerCirSpec> {
        RxStopItW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rx_overrun_it(&mut self) -> RxOverrunItW<MemIerCirSpec> {
        RxOverrunItW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn not_used1(&mut self) -> NotUsed1W<MemIerCirSpec> {
        NotUsed1W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tx_status_it(&mut self) -> TxStatusItW<MemIerCirSpec> {
        TxStatusItW::new(self, 5)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn not_used2(&mut self) -> NotUsed2W<MemIerCirSpec> {
        NotUsed2W::new(self, 6)
    }
}
#[doc = "The interrupt enable register (IER) can be programmed to enable/disable any interrupt. There are 6 types of interrupt in these modes, TX status, status FIFO interrupt, RX overrun, last byte in RX FIFO, THR interrupt and RHR interrupt and they can be enabled/disabled individually.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ier_cir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ier_cir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemIerCirSpec;
impl crate::RegisterSpec for MemIerCirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ier_cir::R`](R) reader structure"]
impl crate::Readable for MemIerCirSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_ier_cir::W`](W) writer structure"]
impl crate::Writable for MemIerCirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_IER_CIR to value 0"]
impl crate::Resettable for MemIerCirSpec {
    const RESET_VALUE: u32 = 0;
}
