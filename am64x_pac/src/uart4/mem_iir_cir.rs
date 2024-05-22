#[doc = "Register `MEM_IIR_CIR` reader"]
pub type R = crate::R<MemIirCirSpec>;
#[doc = "Register `MEM_IIR_CIR` writer"]
pub type W = crate::W<MemIirCirSpec>;
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
#[doc = "Field `RX_OE_IT` reader - "]
pub type RxOeItR = crate::BitReader;
#[doc = "Field `RX_OE_IT` writer - "]
pub type RxOeItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_STATUS_IT` reader - "]
pub type TxStatusItR = crate::BitReader;
#[doc = "Field `TX_STATUS_IT` writer - "]
pub type TxStatusItW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn rx_oe_it(&self) -> RxOeItR {
        RxOeItR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_status_it(&self) -> TxStatusItR {
        TxStatusItR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rhr_it(&mut self) -> RhrItW<MemIirCirSpec> {
        RhrItW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn thr_it(&mut self) -> ThrItW<MemIirCirSpec> {
        ThrItW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rx_stop_it(&mut self) -> RxStopItW<MemIirCirSpec> {
        RxStopItW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rx_oe_it(&mut self) -> RxOeItW<MemIirCirSpec> {
        RxOeItW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tx_status_it(&mut self) -> TxStatusItW<MemIirCirSpec> {
        TxStatusItW::new(self, 5)
    }
}
#[doc = "The IIR is a read-only register, which provides the source of the interrupt in a prioritized manner.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_iir_cir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_iir_cir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemIirCirSpec;
impl crate::RegisterSpec for MemIirCirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_iir_cir::R`](R) reader structure"]
impl crate::Readable for MemIirCirSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_iir_cir::W`](W) writer structure"]
impl crate::Writable for MemIirCirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_IIR_CIR to value 0"]
impl crate::Resettable for MemIirCirSpec {
    const RESET_VALUE: u32 = 0;
}
