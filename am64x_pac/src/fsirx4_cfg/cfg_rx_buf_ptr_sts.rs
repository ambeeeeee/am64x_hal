#[doc = "Register `CFG_RX_BUF_PTR_STS` reader"]
pub type R = crate::R<CfgRxBufPtrStsSpec>;
#[doc = "Register `CFG_RX_BUF_PTR_STS` writer"]
pub type W = crate::W<CfgRxBufPtrStsSpec>;
#[doc = "Field `CURR_BUF_PTR` reader - 3:0\\]
Current Buffer Pointer IndexThis bitfield will show the current index of the buffer pointer. This value is only valid when there is no active transmission."]
pub type CurrBufPtrR = crate::FieldReader;
#[doc = "Field `CURR_BUF_PTR` writer - 3:0\\]
Current Buffer Pointer IndexThis bitfield will show the current index of the buffer pointer. This value is only valid when there is no active transmission."]
pub type CurrBufPtrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CURR_WORD_CNT` reader - 12:8\\]
Words Available in the Receive BufferThis bitfield indicates the number of valid data words present in the receive buffer that have not been read by the application software. This bitfield is only valid when there is no active transfer. Note: This value will not be valid if there has been a buffer overrun or underrun condition."]
pub type CurrWordCntR = crate::FieldReader;
#[doc = "Field `CURR_WORD_CNT` writer - 12:8\\]
Words Available in the Receive BufferThis bitfield indicates the number of valid data words present in the receive buffer that have not been read by the application software. This bitfield is only valid when there is no active transfer. Note: This value will not be valid if there has been a buffer overrun or underrun condition."]
pub type CurrWordCntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Current Buffer Pointer IndexThis bitfield will show the current index of the buffer pointer. This value is only valid when there is no active transmission."]
    #[inline(always)]
    pub fn curr_buf_ptr(&self) -> CurrBufPtrR {
        CurrBufPtrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Words Available in the Receive BufferThis bitfield indicates the number of valid data words present in the receive buffer that have not been read by the application software. This bitfield is only valid when there is no active transfer. Note: This value will not be valid if there has been a buffer overrun or underrun condition."]
    #[inline(always)]
    pub fn curr_word_cnt(&self) -> CurrWordCntR {
        CurrWordCntR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Current Buffer Pointer IndexThis bitfield will show the current index of the buffer pointer. This value is only valid when there is no active transmission."]
    #[inline(always)]
    #[must_use]
    pub fn curr_buf_ptr(&mut self) -> CurrBufPtrW<CfgRxBufPtrStsSpec> {
        CurrBufPtrW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Words Available in the Receive BufferThis bitfield indicates the number of valid data words present in the receive buffer that have not been read by the application software. This bitfield is only valid when there is no active transfer. Note: This value will not be valid if there has been a buffer overrun or underrun condition."]
    #[inline(always)]
    #[must_use]
    pub fn curr_word_cnt(&mut self) -> CurrWordCntW<CfgRxBufPtrStsSpec> {
        CurrWordCntW::new(self, 8)
    }
}
#[doc = "Receive buffer pointer status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_buf_ptr_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_buf_ptr_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxBufPtrStsSpec;
impl crate::RegisterSpec for CfgRxBufPtrStsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_buf_ptr_sts::R`](R) reader structure"]
impl crate::Readable for CfgRxBufPtrStsSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_buf_ptr_sts::W`](W) writer structure"]
impl crate::Writable for CfgRxBufPtrStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_BUF_PTR_STS to value 0"]
impl crate::Resettable for CfgRxBufPtrStsSpec {
    const RESET_VALUE: u16 = 0;
}
