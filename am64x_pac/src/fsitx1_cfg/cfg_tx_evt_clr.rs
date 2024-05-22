#[doc = "Register `CFG_TX_EVT_CLR` reader"]
pub type R = crate::R<CfgTxEvtClrSpec>;
#[doc = "Register `CFG_TX_EVT_CLR` writer"]
pub type W = crate::W<CfgTxEvtClrSpec>;
#[doc = "Field `FRAME_DONE` reader - 0:0\\]
Frame Done Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0."]
pub type FrameDoneR = crate::BitReader;
#[doc = "Field `FRAME_DONE` writer - 0:0\\]
Frame Done Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0."]
pub type FrameDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_UNDERRUN` reader - 1:1\\]
Buffer Underrun Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0."]
pub type BufUnderrunR = crate::BitReader;
#[doc = "Field `BUF_UNDERRUN` writer - 1:1\\]
Buffer Underrun Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0."]
pub type BufUnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_OVERRUN` reader - 2:2\\]
Buffer Overrun Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0."]
pub type BufOverrunR = crate::BitReader;
#[doc = "Field `BUF_OVERRUN` writer - 2:2\\]
Buffer Overrun Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0."]
pub type BufOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PING_TRIGGERED` reader - 3:3\\]
Ping Frame Triggered Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0. Note: This bit may not always be cleared when writing to the corresponding TX_EVT_CLR bit. If PING_TIMEOUT MODE is configured to be 0, a hardware ping timeout may occur when another frame is actively being transmitted. In this case, if this bit still shows as 1 after the clear bit is written then the ping frame has been triggered but not serviced. This bit does not indicate that the ping frame has been completely sent, only that it has been triggered by the timeout event."]
pub type PingTriggeredR = crate::BitReader;
#[doc = "Field `PING_TRIGGERED` writer - 3:3\\]
Ping Frame Triggered Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0. Note: This bit may not always be cleared when writing to the corresponding TX_EVT_CLR bit. If PING_TIMEOUT MODE is configured to be 0, a hardware ping timeout may occur when another frame is actively being transmitted. In this case, if this bit still shows as 1 after the clear bit is written then the ping frame has been triggered but not serviced. This bit does not indicate that the ping frame has been completely sent, only that it has been triggered by the timeout event."]
pub type PingTriggeredW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Frame Done Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn frame_done(&self) -> FrameDoneR {
        FrameDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Buffer Underrun Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn buf_underrun(&self) -> BufUnderrunR {
        BufUnderrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Buffer Overrun Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn buf_overrun(&self) -> BufOverrunR {
        BufOverrunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Ping Frame Triggered Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0. Note: This bit may not always be cleared when writing to the corresponding TX_EVT_CLR bit. If PING_TIMEOUT MODE is configured to be 0, a hardware ping timeout may occur when another frame is actively being transmitted. In this case, if this bit still shows as 1 after the clear bit is written then the ping frame has been triggered but not serviced. This bit does not indicate that the ping frame has been completely sent, only that it has been triggered by the timeout event."]
    #[inline(always)]
    pub fn ping_triggered(&self) -> PingTriggeredR {
        PingTriggeredR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Frame Done Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn frame_done(&mut self) -> FrameDoneW<CfgTxEvtClrSpec> {
        FrameDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Buffer Underrun Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn buf_underrun(&mut self) -> BufUnderrunW<CfgTxEvtClrSpec> {
        BufUnderrunW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Buffer Overrun Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn buf_overrun(&mut self) -> BufOverrunW<CfgTxEvtClrSpec> {
        BufOverrunW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Ping Frame Triggered Flag Clear bitThis bit clears the corresponding bit in the TX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the TX_EVT_STS register to 0. Note: This bit may not always be cleared when writing to the corresponding TX_EVT_CLR bit. If PING_TIMEOUT MODE is configured to be 0, a hardware ping timeout may occur when another frame is actively being transmitted. In this case, if this bit still shows as 1 after the clear bit is written then the ping frame has been triggered but not serviced. This bit does not indicate that the ping frame has been completely sent, only that it has been triggered by the timeout event."]
    #[inline(always)]
    #[must_use]
    pub fn ping_triggered(&mut self) -> PingTriggeredW<CfgTxEvtClrSpec> {
        PingTriggeredW::new(self, 3)
    }
}
#[doc = "Transmit event and error clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_evt_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_evt_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxEvtClrSpec;
impl crate::RegisterSpec for CfgTxEvtClrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_evt_clr::R`](R) reader structure"]
impl crate::Readable for CfgTxEvtClrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_evt_clr::W`](W) writer structure"]
impl crate::Writable for CfgTxEvtClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_EVT_CLR to value 0"]
impl crate::Resettable for CfgTxEvtClrSpec {
    const RESET_VALUE: u16 = 0;
}
