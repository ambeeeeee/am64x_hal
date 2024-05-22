#[doc = "Register `CFG_RX_INT2_CTRL_ALT1_` reader"]
pub type R = crate::R<CfgRxInt2CtrlAlt1_Spec>;
#[doc = "Register `CFG_RX_INT2_CTRL_ALT1_` writer"]
pub type W = crate::W<CfgRxInt2CtrlAlt1_Spec>;
#[doc = "Field `INT2_EN_PING_WD_TO` reader - 0:0\\]
Enable Ping Watchdog Timeout Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A ping watchdog timeout event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnPingWdToR = crate::BitReader;
#[doc = "Field `INT2_EN_PING_WD_TO` writer - 0:0\\]
Enable Ping Watchdog Timeout Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A ping watchdog timeout event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnPingWdToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_FRAME_WD_TO` reader - 1:1\\]
Enable Frame Watchdog Timeout Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame watchdog timeout event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnFrameWdToR = crate::BitReader;
#[doc = "Field `INT2_EN_FRAME_WD_TO` writer - 1:1\\]
Enable Frame Watchdog Timeout Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame watchdog timeout event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnFrameWdToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_CRC_ERR` reader - 2:2\\]
Enable CRC Error Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A CRC error will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnCrcErrR = crate::BitReader;
#[doc = "Field `INT2_EN_CRC_ERR` writer - 2:2\\]
Enable CRC Error Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A CRC error will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnCrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_TYPE_ERR` reader - 3:3\\]
Enable Frame Type Error Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame type error event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnTypeErrR = crate::BitReader;
#[doc = "Field `INT2_EN_TYPE_ERR` writer - 3:3\\]
Enable Frame Type Error Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame type error event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnTypeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_EOF_ERR` reader - 4:4\\]
Enable End-of-Frame Error Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= An end-of-frame error event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnEofErrR = crate::BitReader;
#[doc = "Field `INT2_EN_EOF_ERR` writer - 4:4\\]
Enable End-of-Frame Error Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= An end-of-frame error event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnEofErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_OVERRUN` reader - 5:5\\]
Enable Buffer Overrun Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A buffer overrun event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnOverrunR = crate::BitReader;
#[doc = "Field `INT2_EN_OVERRUN` writer - 5:5\\]
Enable Buffer Overrun Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A buffer overrun event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_FRAME_DONE` reader - 6:6\\]
Enable Frame Done Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame done event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnFrameDoneR = crate::BitReader;
#[doc = "Field `INT2_EN_FRAME_DONE` writer - 6:6\\]
Enable Frame Done Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame done event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnFrameDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_UNDERRUN` reader - 7:7\\]
Enable Buffer Underrun Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A buffer underrun event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnUnderrunR = crate::BitReader;
#[doc = "Field `INT2_EN_UNDERRUN` writer - 7:7\\]
Enable Buffer Underrun Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A buffer underrun event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnUnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_ERR_FRAME` reader - 8:8\\]
Enable Error Frame Received Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A error frame received event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnErrFrameR = crate::BitReader;
#[doc = "Field `INT2_EN_ERR_FRAME` writer - 8:8\\]
Enable Error Frame Received Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A error frame received event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnErrFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_PING_FRAME` reader - 9:9\\]
Enable Ping Frame Received Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A ping frame received event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnPingFrameR = crate::BitReader;
#[doc = "Field `INT2_EN_PING_FRAME` writer - 9:9\\]
Enable Ping Frame Received Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A ping frame received event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnPingFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_FRAME_OVERRUN` reader - 10:10\\]
Enable Frame Overrun Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame overrun event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnFrameOverrunR = crate::BitReader;
#[doc = "Field `INT2_EN_FRAME_OVERRUN` writer - 10:10\\]
Enable Frame Overrun Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame overrun event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnFrameOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_DATA_FRAME` reader - 11:11\\]
Enable Data Frame Received Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A data frame received event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnDataFrameR = crate::BitReader;
#[doc = "Field `INT2_EN_DATA_FRAME` writer - 11:11\\]
Enable Data Frame Received Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A data frame received event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnDataFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_PING_TAG_MATCH` reader - 12:12\\]
Enable Ping Frame Received with Tag Match Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A ping frame received with matching tag will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnPingTagMatchR = crate::BitReader;
#[doc = "Field `INT2_EN_PING_TAG_MATCH` writer - 12:12\\]
Enable Ping Frame Received with Tag Match Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A ping frame received with matching tag will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnPingTagMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_DATA_TAG_MATCH` reader - 13:13\\]
Enable Data Frame Received with Tag Match Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A data frame received with matching tag will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnDataTagMatchR = crate::BitReader;
#[doc = "Field `INT2_EN_DATA_TAG_MATCH` writer - 13:13\\]
Enable Data Frame Received with Tag Match Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A data frame received with matching tag will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnDataTagMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2_EN_ERROR_TAG_MATCH` reader - 14:14\\]
Enable Error Frame Received with Tag Match Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= An error frame received with matching tag will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnErrorTagMatchR = crate::BitReader;
#[doc = "Field `INT2_EN_ERROR_TAG_MATCH` writer - 14:14\\]
Enable Error Frame Received with Tag Match Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= An error frame received with matching tag will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int2EnErrorTagMatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Ping Watchdog Timeout Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A ping watchdog timeout event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_ping_wd_to(&self) -> Int2EnPingWdToR {
        Int2EnPingWdToR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable Frame Watchdog Timeout Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame watchdog timeout event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_frame_wd_to(&self) -> Int2EnFrameWdToR {
        Int2EnFrameWdToR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable CRC Error Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A CRC error will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_crc_err(&self) -> Int2EnCrcErrR {
        Int2EnCrcErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable Frame Type Error Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame type error event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_type_err(&self) -> Int2EnTypeErrR {
        Int2EnTypeErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable End-of-Frame Error Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= An end-of-frame error event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_eof_err(&self) -> Int2EnEofErrR {
        Int2EnEofErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable Buffer Overrun Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A buffer overrun event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_overrun(&self) -> Int2EnOverrunR {
        Int2EnOverrunR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable Frame Done Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame done event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_frame_done(&self) -> Int2EnFrameDoneR {
        Int2EnFrameDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable Buffer Underrun Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A buffer underrun event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_underrun(&self) -> Int2EnUnderrunR {
        Int2EnUnderrunR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable Error Frame Received Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A error frame received event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_err_frame(&self) -> Int2EnErrFrameR {
        Int2EnErrFrameR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable Ping Frame Received Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A ping frame received event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_ping_frame(&self) -> Int2EnPingFrameR {
        Int2EnPingFrameR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable Frame Overrun Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame overrun event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_frame_overrun(&self) -> Int2EnFrameOverrunR {
        Int2EnFrameOverrunR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Enable Data Frame Received Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A data frame received event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_data_frame(&self) -> Int2EnDataFrameR {
        Int2EnDataFrameR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable Ping Frame Received with Tag Match Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A ping frame received with matching tag will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_ping_tag_match(&self) -> Int2EnPingTagMatchR {
        Int2EnPingTagMatchR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Enable Data Frame Received with Tag Match Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A data frame received with matching tag will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_data_tag_match(&self) -> Int2EnDataTagMatchR {
        Int2EnDataTagMatchR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Enable Error Frame Received with Tag Match Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= An error frame received with matching tag will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int2_en_error_tag_match(&self) -> Int2EnErrorTagMatchR {
        Int2EnErrorTagMatchR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Ping Watchdog Timeout Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A ping watchdog timeout event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_ping_wd_to(&mut self) -> Int2EnPingWdToW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnPingWdToW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable Frame Watchdog Timeout Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame watchdog timeout event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_frame_wd_to(&mut self) -> Int2EnFrameWdToW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnFrameWdToW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable CRC Error Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A CRC error will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_crc_err(&mut self) -> Int2EnCrcErrW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnCrcErrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable Frame Type Error Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame type error event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_type_err(&mut self) -> Int2EnTypeErrW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnTypeErrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable End-of-Frame Error Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= An end-of-frame error event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_eof_err(&mut self) -> Int2EnEofErrW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnEofErrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable Buffer Overrun Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A buffer overrun event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_overrun(&mut self) -> Int2EnOverrunW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnOverrunW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable Frame Done Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame done event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_frame_done(&mut self) -> Int2EnFrameDoneW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnFrameDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable Buffer Underrun Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A buffer underrun event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_underrun(&mut self) -> Int2EnUnderrunW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnUnderrunW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable Error Frame Received Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A error frame received event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_err_frame(&mut self) -> Int2EnErrFrameW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnErrFrameW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable Ping Frame Received Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A ping frame received event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_ping_frame(&mut self) -> Int2EnPingFrameW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnPingFrameW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable Frame Overrun Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A frame overrun event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_frame_overrun(&mut self) -> Int2EnFrameOverrunW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnFrameOverrunW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Enable Data Frame Received Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A data frame received event will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_data_frame(&mut self) -> Int2EnDataFrameW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnDataFrameW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable Ping Frame Received with Tag Match Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A ping frame received with matching tag will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_ping_tag_match(&mut self) -> Int2EnPingTagMatchW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnPingTagMatchW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Enable Data Frame Received with Tag Match Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= A data frame received with matching tag will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_data_tag_match(&mut self) -> Int2EnDataTagMatchW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnDataTagMatchW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Enable Error Frame Received with Tag Match Interrupt to INT2 bitThis is an enable register which decides whether an interrupt \\[RX_INT2\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT2.1h \\[R/W\\]
= An error frame received with matching tag will trigger an interrupt on RX_INT2. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int2_en_error_tag_match(&mut self) -> Int2EnErrorTagMatchW<CfgRxInt2CtrlAlt1_Spec> {
        Int2EnErrorTagMatchW::new(self, 14)
    }
}
#[doc = "Receive interrupt control register for RX_INT2. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_int2_ctrl_alt1_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_int2_ctrl_alt1_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxInt2CtrlAlt1_Spec;
impl crate::RegisterSpec for CfgRxInt2CtrlAlt1_Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_int2_ctrl_alt1_::R`](R) reader structure"]
impl crate::Readable for CfgRxInt2CtrlAlt1_Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_int2_ctrl_alt1_::W`](W) writer structure"]
impl crate::Writable for CfgRxInt2CtrlAlt1_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_INT2_CTRL_ALT1_ to value 0"]
impl crate::Resettable for CfgRxInt2CtrlAlt1_Spec {
    const RESET_VALUE: u16 = 0;
}
