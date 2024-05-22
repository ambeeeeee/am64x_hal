#[doc = "Register `BCDMA_TCHAN_TFIFO_DEPTH` reader"]
pub type R = crate::R<BcdmaTchanTfifoDepthSpec>;
#[doc = "Register `BCDMA_TCHAN_TFIFO_DEPTH` writer"]
pub type W = crate::W<BcdmaTchanTfifoDepthSpec>;
#[doc = "Field `FDEPTH` reader - 7:0\\]
FIFO Depth: This field contains the number of Tx FIFO bytes which will be allowed to be stored for the channel. The minimum value is equal to the PSI-L interface data path width (tstrm_wdth), the maximum value varies by channel class (ultra-high capacity/high capacity/normal capacity) and is equal to the tubuf_size/thbuf_size/tbuf_size parameter respectively multiplied by the PSI-L data path width (tstrm_wdth). The fdepth must always be an integer multiple of tstrm_wdth. The reset value of this register varies by channel class (ultra-high capacity/high capacity/normal capacity) but will be equal to the tubuf_size/thbuf_size/tbuf_size parameter respectively multiplied by the PSI-L interface data width (tstrm_wdth)."]
pub type FdepthR = crate::FieldReader;
#[doc = "Field `FDEPTH` writer - 7:0\\]
FIFO Depth: This field contains the number of Tx FIFO bytes which will be allowed to be stored for the channel. The minimum value is equal to the PSI-L interface data path width (tstrm_wdth), the maximum value varies by channel class (ultra-high capacity/high capacity/normal capacity) and is equal to the tubuf_size/thbuf_size/tbuf_size parameter respectively multiplied by the PSI-L data path width (tstrm_wdth). The fdepth must always be an integer multiple of tstrm_wdth. The reset value of this register varies by channel class (ultra-high capacity/high capacity/normal capacity) but will be equal to the tubuf_size/thbuf_size/tbuf_size parameter respectively multiplied by the PSI-L interface data width (tstrm_wdth)."]
pub type FdepthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
FIFO Depth: This field contains the number of Tx FIFO bytes which will be allowed to be stored for the channel. The minimum value is equal to the PSI-L interface data path width (tstrm_wdth), the maximum value varies by channel class (ultra-high capacity/high capacity/normal capacity) and is equal to the tubuf_size/thbuf_size/tbuf_size parameter respectively multiplied by the PSI-L data path width (tstrm_wdth). The fdepth must always be an integer multiple of tstrm_wdth. The reset value of this register varies by channel class (ultra-high capacity/high capacity/normal capacity) but will be equal to the tubuf_size/thbuf_size/tbuf_size parameter respectively multiplied by the PSI-L interface data width (tstrm_wdth)."]
    #[inline(always)]
    pub fn fdepth(&self) -> FdepthR {
        FdepthR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
FIFO Depth: This field contains the number of Tx FIFO bytes which will be allowed to be stored for the channel. The minimum value is equal to the PSI-L interface data path width (tstrm_wdth), the maximum value varies by channel class (ultra-high capacity/high capacity/normal capacity) and is equal to the tubuf_size/thbuf_size/tbuf_size parameter respectively multiplied by the PSI-L data path width (tstrm_wdth). The fdepth must always be an integer multiple of tstrm_wdth. The reset value of this register varies by channel class (ultra-high capacity/high capacity/normal capacity) but will be equal to the tubuf_size/thbuf_size/tbuf_size parameter respectively multiplied by the PSI-L interface data width (tstrm_wdth)."]
    #[inline(always)]
    #[must_use]
    pub fn fdepth(&mut self) -> FdepthW<BcdmaTchanTfifoDepthSpec> {
        FdepthW::new(self, 0)
    }
}
#[doc = "The fifo depth register is used to specify how many FIFO data phases deep the Tx per channel FIFO will be for the channel. While the maximum depth of the Tx FIFO is set at design time, the FIFO depth can be artificially reduced in order to control the maximum latency which can be introduced due to buffering effects.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchan_tfifo_depth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchan_tfifo_depth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaTchanTfifoDepthSpec;
impl crate::RegisterSpec for BcdmaTchanTfifoDepthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_tchan_tfifo_depth::R`](R) reader structure"]
impl crate::Readable for BcdmaTchanTfifoDepthSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_tchan_tfifo_depth::W`](W) writer structure"]
impl crate::Writable for BcdmaTchanTfifoDepthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_TCHAN_TFIFO_DEPTH to value 0x0192"]
impl crate::Resettable for BcdmaTchanTfifoDepthSpec {
    const RESET_VALUE: u32 = 0x0192;
}
