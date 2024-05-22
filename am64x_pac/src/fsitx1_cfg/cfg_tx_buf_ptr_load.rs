#[doc = "Register `CFG_TX_BUF_PTR_LOAD` reader"]
pub type R = crate::R<CfgTxBufPtrLoadSpec>;
#[doc = "Register `CFG_TX_BUF_PTR_LOAD` writer"]
pub type W = crate::W<CfgTxBufPtrLoadSpec>;
#[doc = "Field `BUF_PTR_LOAD` reader - 3:0\\]
Buffer Pointer Load bitsThese bits are used to force the transmit buffer pointer to a desired index within the transmit buffer. The next transmission will begin picking data from this index and increment appropriately. This value will be reflected in TX_BUF_PTR_STS only after a minimum 3 SYSCLK cycles + 3 TXCLK cycles. This value should not be written while there is an active transmission as it may corrupt the ongoing frame or other undefined behavior."]
pub type BufPtrLoadR = crate::FieldReader;
#[doc = "Field `BUF_PTR_LOAD` writer - 3:0\\]
Buffer Pointer Load bitsThese bits are used to force the transmit buffer pointer to a desired index within the transmit buffer. The next transmission will begin picking data from this index and increment appropriately. This value will be reflected in TX_BUF_PTR_STS only after a minimum 3 SYSCLK cycles + 3 TXCLK cycles. This value should not be written while there is an active transmission as it may corrupt the ongoing frame or other undefined behavior."]
pub type BufPtrLoadW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Buffer Pointer Load bitsThese bits are used to force the transmit buffer pointer to a desired index within the transmit buffer. The next transmission will begin picking data from this index and increment appropriately. This value will be reflected in TX_BUF_PTR_STS only after a minimum 3 SYSCLK cycles + 3 TXCLK cycles. This value should not be written while there is an active transmission as it may corrupt the ongoing frame or other undefined behavior."]
    #[inline(always)]
    pub fn buf_ptr_load(&self) -> BufPtrLoadR {
        BufPtrLoadR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Buffer Pointer Load bitsThese bits are used to force the transmit buffer pointer to a desired index within the transmit buffer. The next transmission will begin picking data from this index and increment appropriately. This value will be reflected in TX_BUF_PTR_STS only after a minimum 3 SYSCLK cycles + 3 TXCLK cycles. This value should not be written while there is an active transmission as it may corrupt the ongoing frame or other undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn buf_ptr_load(&mut self) -> BufPtrLoadW<CfgTxBufPtrLoadSpec> {
        BufPtrLoadW::new(self, 0)
    }
}
#[doc = "Transmit buffer pointer control load register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_buf_ptr_load::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_buf_ptr_load::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxBufPtrLoadSpec;
impl crate::RegisterSpec for CfgTxBufPtrLoadSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_buf_ptr_load::R`](R) reader structure"]
impl crate::Readable for CfgTxBufPtrLoadSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_buf_ptr_load::W`](W) writer structure"]
impl crate::Writable for CfgTxBufPtrLoadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_BUF_PTR_LOAD to value 0"]
impl crate::Resettable for CfgTxBufPtrLoadSpec {
    const RESET_VALUE: u16 = 0;
}
