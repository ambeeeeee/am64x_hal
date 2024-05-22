#[doc = "Register `CFG_RX_FRAME_WD_CNT` reader"]
pub type R = crate::R<CfgRxFrameWdCntSpec>;
#[doc = "Register `CFG_RX_FRAME_WD_CNT` writer"]
pub type W = crate::W<CfgRxFrameWdCntSpec>;
#[doc = "Field `FRAME_WD_CNT` reader - 31:0\\]
Frame Watchdog Counter ValueThis is the 32-bit read-only register which shows the current value of the frame watchdog counter. This counter is reset to 0 in a variety of ways: A write to FRME_WD_CNT_RST, a match with FRAME_WD_REF, or the reception of a successful data frame."]
pub type FrameWdCntR = crate::FieldReader<u32>;
#[doc = "Field `FRAME_WD_CNT` writer - 31:0\\]
Frame Watchdog Counter ValueThis is the 32-bit read-only register which shows the current value of the frame watchdog counter. This counter is reset to 0 in a variety of ways: A write to FRME_WD_CNT_RST, a match with FRAME_WD_REF, or the reception of a successful data frame."]
pub type FrameWdCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Frame Watchdog Counter ValueThis is the 32-bit read-only register which shows the current value of the frame watchdog counter. This counter is reset to 0 in a variety of ways: A write to FRME_WD_CNT_RST, a match with FRAME_WD_REF, or the reception of a successful data frame."]
    #[inline(always)]
    pub fn frame_wd_cnt(&self) -> FrameWdCntR {
        FrameWdCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Frame Watchdog Counter ValueThis is the 32-bit read-only register which shows the current value of the frame watchdog counter. This counter is reset to 0 in a variety of ways: A write to FRME_WD_CNT_RST, a match with FRAME_WD_REF, or the reception of a successful data frame."]
    #[inline(always)]
    #[must_use]
    pub fn frame_wd_cnt(&mut self) -> FrameWdCntW<CfgRxFrameWdCntSpec> {
        FrameWdCntW::new(self, 0)
    }
}
#[doc = "Receive frame watchdog current count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_frame_wd_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_frame_wd_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxFrameWdCntSpec;
impl crate::RegisterSpec for CfgRxFrameWdCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_rx_frame_wd_cnt::R`](R) reader structure"]
impl crate::Readable for CfgRxFrameWdCntSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_frame_wd_cnt::W`](W) writer structure"]
impl crate::Writable for CfgRxFrameWdCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_RX_FRAME_WD_CNT to value 0"]
impl crate::Resettable for CfgRxFrameWdCntSpec {
    const RESET_VALUE: u32 = 0;
}
