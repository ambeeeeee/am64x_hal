#[doc = "Register `CFG_TX_EVT_STS` reader"]
pub type R = crate::R<CfgTxEvtStsSpec>;
#[doc = "Register `CFG_TX_EVT_STS` writer"]
pub type W = crate::W<CfgTxEvtStsSpec>;
#[doc = "Field `FRAME_DONE` reader - 0:0\\]
Frame Done Flag BitThis bit inditcates a Frame Done condition. This bit is set by hardware when a frame transmission has been completed. Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= Frame Done condition has not occured.1h \\[R\\]
= Frame Done condition has occured. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
pub type FrameDoneR = crate::BitReader;
#[doc = "Field `FRAME_DONE` writer - 0:0\\]
Frame Done Flag BitThis bit inditcates a Frame Done condition. This bit is set by hardware when a frame transmission has been completed. Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= Frame Done condition has not occured.1h \\[R\\]
= Frame Done condition has occured. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
pub type FrameDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_UNDERRUN` reader - 1:1\\]
Buffer Underrun Flag BitThis bit inditcates that buffer underrun has occured.Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= Buffer Underrun has not occured.1h \\[R\\]
= Buffer Underrun has occured. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
pub type BufUnderrunR = crate::BitReader;
#[doc = "Field `BUF_UNDERRUN` writer - 1:1\\]
Buffer Underrun Flag BitThis bit inditcates that buffer underrun has occured.Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= Buffer Underrun has not occured.1h \\[R\\]
= Buffer Underrun has occured. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
pub type BufUnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_OVERRUN` reader - 2:2\\]
Buffer Overrun Flag BitThis bit inditcates that buffer overrun has occured.Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= Buffer Overrun has not occured.1h \\[R\\]
= Buffer Overrun has occured. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
pub type BufOverrunR = crate::BitReader;
#[doc = "Field `BUF_OVERRUN` writer - 2:2\\]
Buffer Overrun Flag BitThis bit inditcates that buffer overrun has occured.Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= Buffer Overrun has not occured.1h \\[R\\]
= Buffer Overrun has occured. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
pub type BufOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PING_TRIGGERED` reader - 3:3\\]
Ping Frame Triggered Flag BitThis bit indicates that a ping frame has been triggered. This bit is set by hardware when either the ping timer or an external trigger event have occured. Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= A ping frame has not been triggered.1h \\[R\\]
= A ping frame has been triggered by either the ping timer or external trigger. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
pub type PingTriggeredR = crate::BitReader;
#[doc = "Field `PING_TRIGGERED` writer - 3:3\\]
Ping Frame Triggered Flag BitThis bit indicates that a ping frame has been triggered. This bit is set by hardware when either the ping timer or an external trigger event have occured. Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= A ping frame has not been triggered.1h \\[R\\]
= A ping frame has been triggered by either the ping timer or external trigger. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
pub type PingTriggeredW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Frame Done Flag BitThis bit inditcates a Frame Done condition. This bit is set by hardware when a frame transmission has been completed. Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= Frame Done condition has not occured.1h \\[R\\]
= Frame Done condition has occured. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
    #[inline(always)]
    pub fn frame_done(&self) -> FrameDoneR {
        FrameDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Buffer Underrun Flag BitThis bit inditcates that buffer underrun has occured.Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= Buffer Underrun has not occured.1h \\[R\\]
= Buffer Underrun has occured. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
    #[inline(always)]
    pub fn buf_underrun(&self) -> BufUnderrunR {
        BufUnderrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Buffer Overrun Flag BitThis bit inditcates that buffer overrun has occured.Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= Buffer Overrun has not occured.1h \\[R\\]
= Buffer Overrun has occured. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
    #[inline(always)]
    pub fn buf_overrun(&self) -> BufOverrunR {
        BufOverrunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Ping Frame Triggered Flag BitThis bit indicates that a ping frame has been triggered. This bit is set by hardware when either the ping timer or an external trigger event have occured. Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= A ping frame has not been triggered.1h \\[R\\]
= A ping frame has been triggered by either the ping timer or external trigger. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
    #[inline(always)]
    pub fn ping_triggered(&self) -> PingTriggeredR {
        PingTriggeredR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Frame Done Flag BitThis bit inditcates a Frame Done condition. This bit is set by hardware when a frame transmission has been completed. Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= Frame Done condition has not occured.1h \\[R\\]
= Frame Done condition has occured. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
    #[inline(always)]
    #[must_use]
    pub fn frame_done(&mut self) -> FrameDoneW<CfgTxEvtStsSpec> {
        FrameDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Buffer Underrun Flag BitThis bit inditcates that buffer underrun has occured.Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= Buffer Underrun has not occured.1h \\[R\\]
= Buffer Underrun has occured. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
    #[inline(always)]
    #[must_use]
    pub fn buf_underrun(&mut self) -> BufUnderrunW<CfgTxEvtStsSpec> {
        BufUnderrunW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Buffer Overrun Flag BitThis bit inditcates that buffer overrun has occured.Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= Buffer Overrun has not occured.1h \\[R\\]
= Buffer Overrun has occured. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
    #[inline(always)]
    #[must_use]
    pub fn buf_overrun(&mut self) -> BufOverrunW<CfgTxEvtStsSpec> {
        BufOverrunW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Ping Frame Triggered Flag BitThis bit indicates that a ping frame has been triggered. This bit is set by hardware when either the ping timer or an external trigger event have occured. Software can also force this bit to get set by writing to the TX_EVT_FRC register. 0h \\[R\\]
= A ping frame has not been triggered.1h \\[R\\]
= A ping frame has been triggered by either the ping timer or external trigger. To clear this bit, write to the corresponding bit in the TX_EVT_CLR register."]
    #[inline(always)]
    #[must_use]
    pub fn ping_triggered(&mut self) -> PingTriggeredW<CfgTxEvtStsSpec> {
        PingTriggeredW::new(self, 3)
    }
}
#[doc = "Transmit event and error status flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_evt_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_evt_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxEvtStsSpec;
impl crate::RegisterSpec for CfgTxEvtStsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_evt_sts::R`](R) reader structure"]
impl crate::Readable for CfgTxEvtStsSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_evt_sts::W`](W) writer structure"]
impl crate::Writable for CfgTxEvtStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_EVT_STS to value 0"]
impl crate::Resettable for CfgTxEvtStsSpec {
    const RESET_VALUE: u16 = 0;
}
