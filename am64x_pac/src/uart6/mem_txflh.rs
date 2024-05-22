#[doc = "Register `MEM_TXFLH` reader"]
pub type R = crate::R<MemTxflhSpec>;
#[doc = "Register `MEM_TXFLH` writer"]
pub type W = crate::W<MemTxflhSpec>;
#[doc = "Field `TXFLH` reader - 4:0\\]
MSB register used to specify the frame length"]
pub type TxflhR = crate::FieldReader;
#[doc = "Field `TXFLH` writer - 4:0\\]
MSB register used to specify the frame length"]
pub type TxflhW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
MSB register used to specify the frame length"]
    #[inline(always)]
    pub fn txflh(&self) -> TxflhR {
        TxflhR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
MSB register used to specify the frame length"]
    #[inline(always)]
    #[must_use]
    pub fn txflh(&mut self) -> TxflhW<MemTxflhSpec> {
        TxflhW::new(self, 0)
    }
}
#[doc = "IrDA modes only. The registers TXFLL and TXFLH hold the 13-bit transmit frame length (expressed in bytes). TXFLL holds the least significant bits and TXFLH holds the most significant bits. The frame length value is used if the frame length method of frame closing is used.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_txflh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_txflh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemTxflhSpec;
impl crate::RegisterSpec for MemTxflhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_txflh::R`](R) reader structure"]
impl crate::Readable for MemTxflhSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_txflh::W`](W) writer structure"]
impl crate::Writable for MemTxflhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_TXFLH to value 0"]
impl crate::Resettable for MemTxflhSpec {
    const RESET_VALUE: u32 = 0;
}
