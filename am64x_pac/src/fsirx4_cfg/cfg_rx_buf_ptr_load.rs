#[doc = "Register `CFG_RX_BUF_PTR_LOAD` reader"]
pub type R = crate::R<CfgRxBufPtrLoadSpec>;
#[doc = "Register `CFG_RX_BUF_PTR_LOAD` writer"]
pub type W = crate::W<CfgRxBufPtrLoadSpec>;
#[doc = "Field `BUF_PTR_LOAD` reader - 3:0\\]
Buffer Pointer Load.This is the value to be loaded into the receive word pointer when written. This is to allow software to force the receiver to start storing the received data starting at a specific location in the buffer. NOTE: The value of the CURR_BUF_PTR in the RX_BUF_PTR_STS will not get reflected immediately. This will take effect only when there is a valid receive operation with incoming clocks after \\[3 RXCLK + 3 SYCLK\\]
cycles."]
pub type BufPtrLoadR = crate::FieldReader;
#[doc = "Field `BUF_PTR_LOAD` writer - 3:0\\]
Buffer Pointer Load.This is the value to be loaded into the receive word pointer when written. This is to allow software to force the receiver to start storing the received data starting at a specific location in the buffer. NOTE: The value of the CURR_BUF_PTR in the RX_BUF_PTR_STS will not get reflected immediately. This will take effect only when there is a valid receive operation with incoming clocks after \\[3 RXCLK + 3 SYCLK\\]
cycles."]
pub type BufPtrLoadW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Buffer Pointer Load.This is the value to be loaded into the receive word pointer when written. This is to allow software to force the receiver to start storing the received data starting at a specific location in the buffer. NOTE: The value of the CURR_BUF_PTR in the RX_BUF_PTR_STS will not get reflected immediately. This will take effect only when there is a valid receive operation with incoming clocks after \\[3 RXCLK + 3 SYCLK\\]
cycles."]
    #[inline(always)]
    pub fn buf_ptr_load(&self) -> BufPtrLoadR {
        BufPtrLoadR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Buffer Pointer Load.This is the value to be loaded into the receive word pointer when written. This is to allow software to force the receiver to start storing the received data starting at a specific location in the buffer. NOTE: The value of the CURR_BUF_PTR in the RX_BUF_PTR_STS will not get reflected immediately. This will take effect only when there is a valid receive operation with incoming clocks after \\[3 RXCLK + 3 SYCLK\\]
cycles."]
    #[inline(always)]
    #[must_use]
    pub fn buf_ptr_load(&mut self) -> BufPtrLoadW<CfgRxBufPtrLoadSpec> {
        BufPtrLoadW::new(self, 0)
    }
}
#[doc = "Receive buffer pointer load register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_buf_ptr_load::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_buf_ptr_load::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxBufPtrLoadSpec;
impl crate::RegisterSpec for CfgRxBufPtrLoadSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_buf_ptr_load::R`](R) reader structure"]
impl crate::Readable for CfgRxBufPtrLoadSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_buf_ptr_load::W`](W) writer structure"]
impl crate::Writable for CfgRxBufPtrLoadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_BUF_PTR_LOAD to value 0"]
impl crate::Resettable for CfgRxBufPtrLoadSpec {
    const RESET_VALUE: u16 = 0;
}
