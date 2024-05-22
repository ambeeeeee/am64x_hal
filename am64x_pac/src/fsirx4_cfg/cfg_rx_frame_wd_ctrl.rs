#[doc = "Register `CFG_RX_FRAME_WD_CTRL` reader"]
pub type R = crate::R<CfgRxFrameWdCtrlSpec>;
#[doc = "Register `CFG_RX_FRAME_WD_CTRL` writer"]
pub type W = crate::W<CfgRxFrameWdCtrlSpec>;
#[doc = "Field `FRAME_WD_CNT_RST` reader - 0:0\\]
Frame Watchdog Counter Reset bitThis bit will reset the frame watchdog counter to 0. This bit will always be read as 0. 0h \\[R/W\\]
= Writing a 0 to this bit has no effect.1h \\[W\\]
= The frame watchdog counter will be reset to 0."]
pub type FrameWdCntRstR = crate::BitReader;
#[doc = "Field `FRAME_WD_CNT_RST` writer - 0:0\\]
Frame Watchdog Counter Reset bitThis bit will reset the frame watchdog counter to 0. This bit will always be read as 0. 0h \\[R/W\\]
= Writing a 0 to this bit has no effect.1h \\[W\\]
= The frame watchdog counter will be reset to 0."]
pub type FrameWdCntRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_WD_EN` reader - 1:1\\]
Frame Watchdog Counter Enable bitThis bit will enable or disable the frame watchdog counter. The counter \\[RX_FRAME_WD_CNT\\]
will begin counting from 0 when a valid start-of-frame pattern is received. When the reference value \\[RX_FRAME_WD_REF\\]
is reached, it will generate a frame watchdog timeout event \\[RX_EVT_STS.FRAME_WD_TO\\]
and the counter value will reset to 0 and continue counting on the next valid start-of-frame. 0h \\[R/W\\]
= The frame watchdog counter is disabled and not running.1h \\[R/W\\]
= The frame watchdog counter logic is enabled and running."]
pub type FrameWdEnR = crate::BitReader;
#[doc = "Field `FRAME_WD_EN` writer - 1:1\\]
Frame Watchdog Counter Enable bitThis bit will enable or disable the frame watchdog counter. The counter \\[RX_FRAME_WD_CNT\\]
will begin counting from 0 when a valid start-of-frame pattern is received. When the reference value \\[RX_FRAME_WD_REF\\]
is reached, it will generate a frame watchdog timeout event \\[RX_EVT_STS.FRAME_WD_TO\\]
and the counter value will reset to 0 and continue counting on the next valid start-of-frame. 0h \\[R/W\\]
= The frame watchdog counter is disabled and not running.1h \\[R/W\\]
= The frame watchdog counter logic is enabled and running."]
pub type FrameWdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Frame Watchdog Counter Reset bitThis bit will reset the frame watchdog counter to 0. This bit will always be read as 0. 0h \\[R/W\\]
= Writing a 0 to this bit has no effect.1h \\[W\\]
= The frame watchdog counter will be reset to 0."]
    #[inline(always)]
    pub fn frame_wd_cnt_rst(&self) -> FrameWdCntRstR {
        FrameWdCntRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Frame Watchdog Counter Enable bitThis bit will enable or disable the frame watchdog counter. The counter \\[RX_FRAME_WD_CNT\\]
will begin counting from 0 when a valid start-of-frame pattern is received. When the reference value \\[RX_FRAME_WD_REF\\]
is reached, it will generate a frame watchdog timeout event \\[RX_EVT_STS.FRAME_WD_TO\\]
and the counter value will reset to 0 and continue counting on the next valid start-of-frame. 0h \\[R/W\\]
= The frame watchdog counter is disabled and not running.1h \\[R/W\\]
= The frame watchdog counter logic is enabled and running."]
    #[inline(always)]
    pub fn frame_wd_en(&self) -> FrameWdEnR {
        FrameWdEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Frame Watchdog Counter Reset bitThis bit will reset the frame watchdog counter to 0. This bit will always be read as 0. 0h \\[R/W\\]
= Writing a 0 to this bit has no effect.1h \\[W\\]
= The frame watchdog counter will be reset to 0."]
    #[inline(always)]
    #[must_use]
    pub fn frame_wd_cnt_rst(&mut self) -> FrameWdCntRstW<CfgRxFrameWdCtrlSpec> {
        FrameWdCntRstW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Frame Watchdog Counter Enable bitThis bit will enable or disable the frame watchdog counter. The counter \\[RX_FRAME_WD_CNT\\]
will begin counting from 0 when a valid start-of-frame pattern is received. When the reference value \\[RX_FRAME_WD_REF\\]
is reached, it will generate a frame watchdog timeout event \\[RX_EVT_STS.FRAME_WD_TO\\]
and the counter value will reset to 0 and continue counting on the next valid start-of-frame. 0h \\[R/W\\]
= The frame watchdog counter is disabled and not running.1h \\[R/W\\]
= The frame watchdog counter logic is enabled and running."]
    #[inline(always)]
    #[must_use]
    pub fn frame_wd_en(&mut self) -> FrameWdEnW<CfgRxFrameWdCtrlSpec> {
        FrameWdEnW::new(self, 1)
    }
}
#[doc = "Receive frame watchdog control register. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_frame_wd_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_frame_wd_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxFrameWdCtrlSpec;
impl crate::RegisterSpec for CfgRxFrameWdCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_frame_wd_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgRxFrameWdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_frame_wd_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgRxFrameWdCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_FRAME_WD_CTRL to value 0"]
impl crate::Resettable for CfgRxFrameWdCtrlSpec {
    const RESET_VALUE: u16 = 0;
}
