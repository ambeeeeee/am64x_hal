#[doc = "Register `MEM_TXFLL` reader"]
pub type R = crate::R<MemTxfllSpec>;
#[doc = "Register `MEM_TXFLL` writer"]
pub type W = crate::W<MemTxfllSpec>;
#[doc = "Field `TXFLL` reader - 7:0\\]
LSB register used to specify the frame length"]
pub type TxfllR = crate::FieldReader;
#[doc = "Field `TXFLL` writer - 7:0\\]
LSB register used to specify the frame length"]
pub type TxfllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
LSB register used to specify the frame length"]
    #[inline(always)]
    pub fn txfll(&self) -> TxfllR {
        TxfllR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
LSB register used to specify the frame length"]
    #[inline(always)]
    #[must_use]
    pub fn txfll(&mut self) -> TxfllW<MemTxfllSpec> {
        TxfllW::new(self, 0)
    }
}
#[doc = "IrDA modes only. The registers TXFLL and TXFLH hold the 13-bit transmit frame length (expressed in bytes). TXFLL holds the least significant bits and TXFLH holds the most significant bits. The frame length value is used if the frame length method of frame closing is used.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_txfll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_txfll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemTxfllSpec;
impl crate::RegisterSpec for MemTxfllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_txfll::R`](R) reader structure"]
impl crate::Readable for MemTxfllSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_txfll::W`](W) writer structure"]
impl crate::Writable for MemTxfllSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_TXFLL to value 0"]
impl crate::Resettable for MemTxfllSpec {
    const RESET_VALUE: u32 = 0;
}
