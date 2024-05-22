#[doc = "Register `CFG_RX_FRAME_WD_REF` reader"]
pub type R = crate::R<CfgRxFrameWdRefSpec>;
#[doc = "Register `CFG_RX_FRAME_WD_REF` writer"]
pub type W = crate::W<CfgRxFrameWdRefSpec>;
#[doc = "Field `FRAME_WD_REF` reader - 31:0\\]
Frame Watchdog Counter Reference ValueThis is the 32-bit reference value for the frame watchdog timeout counter. The counter will count up starting from 0 at a valid start-of-frame pattern and continue counting until this value is reached."]
pub type FrameWdRefR = crate::FieldReader<u32>;
#[doc = "Field `FRAME_WD_REF` writer - 31:0\\]
Frame Watchdog Counter Reference ValueThis is the 32-bit reference value for the frame watchdog timeout counter. The counter will count up starting from 0 at a valid start-of-frame pattern and continue counting until this value is reached."]
pub type FrameWdRefW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Frame Watchdog Counter Reference ValueThis is the 32-bit reference value for the frame watchdog timeout counter. The counter will count up starting from 0 at a valid start-of-frame pattern and continue counting until this value is reached."]
    #[inline(always)]
    pub fn frame_wd_ref(&self) -> FrameWdRefR {
        FrameWdRefR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Frame Watchdog Counter Reference ValueThis is the 32-bit reference value for the frame watchdog timeout counter. The counter will count up starting from 0 at a valid start-of-frame pattern and continue counting until this value is reached."]
    #[inline(always)]
    #[must_use]
    pub fn frame_wd_ref(&mut self) -> FrameWdRefW<CfgRxFrameWdRefSpec> {
        FrameWdRefW::new(self, 0)
    }
}
#[doc = "Receive frame watchdog counter reference. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_frame_wd_ref::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_frame_wd_ref::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxFrameWdRefSpec;
impl crate::RegisterSpec for CfgRxFrameWdRefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_rx_frame_wd_ref::R`](R) reader structure"]
impl crate::Readable for CfgRxFrameWdRefSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_frame_wd_ref::W`](W) writer structure"]
impl crate::Writable for CfgRxFrameWdRefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_RX_FRAME_WD_REF to value 0"]
impl crate::Resettable for CfgRxFrameWdRefSpec {
    const RESET_VALUE: u32 = 0;
}
