#[doc = "Register `CFG_TX_INT_CTRL` reader"]
pub type R = crate::R<CfgTxIntCtrlSpec>;
#[doc = "Register `CFG_TX_INT_CTRL` writer"]
pub type W = crate::W<CfgTxIntCtrlSpec>;
#[doc = "Field `INT1_EN_FRAME_DONE` reader - 0:0\\]
Enable Frame Done interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= A Frame Done event will trigger an interrupt on TX_INT1."]
pub type Int1EnFrameDoneR = crate::BitReader;
#[doc = "Field `INT1_EN_FRAME_DONE` writer - 0:0\\]
Enable Frame Done interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= A Frame Done event will trigger an interrupt on TX_INT1."]
pub type Int1EnFrameDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_BUF_UNDERRUN` reader - 1:1\\]
Enable Buffer Underrun Interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= A Buffer Underrun condition will trigger an interrupt on TX_INT1."]
pub type Int1EnBufUnderrunR = crate::BitReader;
#[doc = "Field `INT1_EN_BUF_UNDERRUN` writer - 1:1\\]
Enable Buffer Underrun Interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= A Buffer Underrun condition will trigger an interrupt on TX_INT1."]
pub type Int1EnBufUnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_BUF_OVERRUN` reader - 2:2\\]
Enable Buffer Overrun Interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= A Buffer Overrun condition will trigger an interrupt on TX_INT1."]
pub type Int1EnBufOverrunR = crate::BitReader;
#[doc = "Field `INT1_EN_BUF_OVERRUN` writer - 2:2\\]
Enable Buffer Overrun Interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= A Buffer Overrun condition will trigger an interrupt on TX_INT1."]
pub type Int1EnBufOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_PING_TO` reader - 3:3\\]
Enable Ping Timer Interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= The ping timer event will trigger an interrupt on TX_INT1."]
pub type Int1EnPingToR = crate::BitReader;
#[doc = "Field `INT1_EN_PING_TO` writer - 3:3\\]
Enable Ping Timer Interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= The ping timer event will trigger an interrupt on TX_INT1."]
pub type Int1EnPingToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_FRAME_DONE` reader - 8:8\\]
Enable Frame Done interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= A Frame Done event will trigger an interrupt on TX_INT2."]
pub type Int2EnFrameDoneR = crate::BitReader;
#[doc = "Field `INT2_EN_FRAME_DONE` writer - 8:8\\]
Enable Frame Done interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= A Frame Done event will trigger an interrupt on TX_INT2."]
pub type Int2EnFrameDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_BUF_UNDERRUN` reader - 9:9\\]
Enable Buffer Underrun Interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= A Buffer Underrun condition will trigger an interrupt on TX_INT2."]
pub type Int2EnBufUnderrunR = crate::BitReader;
#[doc = "Field `INT2_EN_BUF_UNDERRUN` writer - 9:9\\]
Enable Buffer Underrun Interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= A Buffer Underrun condition will trigger an interrupt on TX_INT2."]
pub type Int2EnBufUnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_BUF_OVERRUN` reader - 10:10\\]
Enable Buffer Overrun Interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= A Buffer Overrun condition will trigger an interrupt on TX_INT2."]
pub type Int2EnBufOverrunR = crate::BitReader;
#[doc = "Field `INT2_EN_BUF_OVERRUN` writer - 10:10\\]
Enable Buffer Overrun Interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= A Buffer Overrun condition will trigger an interrupt on TX_INT2."]
pub type Int2EnBufOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_PING_TO` reader - 11:11\\]
Enable PING Timer Interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= The ping timer event will trigger an interrupt on TX_INT2."]
pub type Int2EnPingToR = crate::BitReader;
#[doc = "Field `INT2_EN_PING_TO` writer - 11:11\\]
Enable PING Timer Interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= The ping timer event will trigger an interrupt on TX_INT2."]
pub type Int2EnPingToW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Frame Done interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= A Frame Done event will trigger an interrupt on TX_INT1."]
    #[inline(always)]
    pub fn int1_en_frame_done(&self) -> Int1EnFrameDoneR {
        Int1EnFrameDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable Buffer Underrun Interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= A Buffer Underrun condition will trigger an interrupt on TX_INT1."]
    #[inline(always)]
    pub fn int1_en_buf_underrun(&self) -> Int1EnBufUnderrunR {
        Int1EnBufUnderrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable Buffer Overrun Interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= A Buffer Overrun condition will trigger an interrupt on TX_INT1."]
    #[inline(always)]
    pub fn int1_en_buf_overrun(&self) -> Int1EnBufOverrunR {
        Int1EnBufOverrunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable Ping Timer Interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= The ping timer event will trigger an interrupt on TX_INT1."]
    #[inline(always)]
    pub fn int1_en_ping_to(&self) -> Int1EnPingToR {
        Int1EnPingToR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable Frame Done interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= A Frame Done event will trigger an interrupt on TX_INT2."]
    #[inline(always)]
    pub fn int2_en_frame_done(&self) -> Int2EnFrameDoneR {
        Int2EnFrameDoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable Buffer Underrun Interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= A Buffer Underrun condition will trigger an interrupt on TX_INT2."]
    #[inline(always)]
    pub fn int2_en_buf_underrun(&self) -> Int2EnBufUnderrunR {
        Int2EnBufUnderrunR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable Buffer Overrun Interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= A Buffer Overrun condition will trigger an interrupt on TX_INT2."]
    #[inline(always)]
    pub fn int2_en_buf_overrun(&self) -> Int2EnBufOverrunR {
        Int2EnBufOverrunR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Enable PING Timer Interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= The ping timer event will trigger an interrupt on TX_INT2."]
    #[inline(always)]
    pub fn int2_en_ping_to(&self) -> Int2EnPingToR {
        Int2EnPingToR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Frame Done interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= A Frame Done event will trigger an interrupt on TX_INT1."]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_frame_done(&mut self) -> Int1EnFrameDoneW<CfgTxIntCtrlSpec> {
        Int1EnFrameDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable Buffer Underrun Interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= A Buffer Underrun condition will trigger an interrupt on TX_INT1."]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_buf_underrun(&mut self) -> Int1EnBufUnderrunW<CfgTxIntCtrlSpec> {
        Int1EnBufUnderrunW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable Buffer Overrun Interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= A Buffer Overrun condition will trigger an interrupt on TX_INT1."]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_buf_overrun(&mut self) -> Int1EnBufOverrunW<CfgTxIntCtrlSpec> {
        Int1EnBufOverrunW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable Ping Timer Interrupt to INT1This bit allows the event to generate an interrupt on the INT1 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT1.1h \\[R/W\\]
= The ping timer event will trigger an interrupt on TX_INT1."]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_ping_to(&mut self) -> Int1EnPingToW<CfgTxIntCtrlSpec> {
        Int1EnPingToW::new(self, 3)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable Frame Done interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= A Frame Done event will trigger an interrupt on TX_INT2."]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_frame_done(&mut self) -> Int2EnFrameDoneW<CfgTxIntCtrlSpec> {
        Int2EnFrameDoneW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable Buffer Underrun Interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= A Buffer Underrun condition will trigger an interrupt on TX_INT2."]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_buf_underrun(&mut self) -> Int2EnBufUnderrunW<CfgTxIntCtrlSpec> {
        Int2EnBufUnderrunW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable Buffer Overrun Interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= A Buffer Overrun condition will trigger an interrupt on TX_INT2."]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_buf_overrun(&mut self) -> Int2EnBufOverrunW<CfgTxIntCtrlSpec> {
        Int2EnBufOverrunW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Enable PING Timer Interrupt to INT2This bit allows the event to generate an interrupt on the INT2 line. 0h \\[R/W\\]
= This event will not trigger an interrupt on TX_INT2.1h \\[R/W\\]
= The ping timer event will trigger an interrupt on TX_INT2."]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_ping_to(&mut self) -> Int2EnPingToW<CfgTxIntCtrlSpec> {
        Int2EnPingToW::new(self, 11)
    }
}
#[doc = "Transmit interrupt event control register. Protected by LOCK field in TX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_int_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_int_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxIntCtrlSpec;
impl crate::RegisterSpec for CfgTxIntCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_int_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgTxIntCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_int_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgTxIntCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_INT_CTRL to value 0"]
impl crate::Resettable for CfgTxIntCtrlSpec {
    const RESET_VALUE: u16 = 0;
}
