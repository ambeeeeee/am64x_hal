#[doc = "Register `CFG_RX_INT1_CTRL_ALT1_` reader"]
pub type R = crate::R<CfgRxInt1CtrlAlt1_Spec>;
#[doc = "Register `CFG_RX_INT1_CTRL_ALT1_` writer"]
pub type W = crate::W<CfgRxInt1CtrlAlt1_Spec>;
#[doc = "Field `INT1_EN_PING_WD_TO` reader - 0:0\\]
Enable Ping Watchdog Timeout Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A ping watchdog timeout event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnPingWdToR = crate::BitReader;
#[doc = "Field `INT1_EN_PING_WD_TO` writer - 0:0\\]
Enable Ping Watchdog Timeout Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A ping watchdog timeout event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnPingWdToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_FRAME_WD_TO` reader - 1:1\\]
Enable Frame Watchdog Timeout Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame watchdog timeout event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnFrameWdToR = crate::BitReader;
#[doc = "Field `INT1_EN_FRAME_WD_TO` writer - 1:1\\]
Enable Frame Watchdog Timeout Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame watchdog timeout event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnFrameWdToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_CRC_ERR` reader - 2:2\\]
Enable CRC Error Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A CRC error will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnCrcErrR = crate::BitReader;
#[doc = "Field `INT1_EN_CRC_ERR` writer - 2:2\\]
Enable CRC Error Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A CRC error will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnCrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_TYPE_ERR` reader - 3:3\\]
Enable Frame Type Error Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame type error event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnTypeErrR = crate::BitReader;
#[doc = "Field `INT1_EN_TYPE_ERR` writer - 3:3\\]
Enable Frame Type Error Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame type error event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnTypeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_EOF_ERR` reader - 4:4\\]
Enable End-of-Frame Error Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= An end-of-frame error event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnEofErrR = crate::BitReader;
#[doc = "Field `INT1_EN_EOF_ERR` writer - 4:4\\]
Enable End-of-Frame Error Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= An end-of-frame error event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnEofErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_OVERRUN` reader - 5:5\\]
Enable Receive Buffer Overrun Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A receive buffer overrun event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnOverrunR = crate::BitReader;
#[doc = "Field `INT1_EN_OVERRUN` writer - 5:5\\]
Enable Receive Buffer Overrun Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A receive buffer overrun event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_FRAME_DONE` reader - 6:6\\]
Enable Frame Done Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame done event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnFrameDoneR = crate::BitReader;
#[doc = "Field `INT1_EN_FRAME_DONE` writer - 6:6\\]
Enable Frame Done Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame done event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnFrameDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_UNDERRUN` reader - 7:7\\]
Enable Buffer Underrun Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A buffer underrun event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnUnderrunR = crate::BitReader;
#[doc = "Field `INT1_EN_UNDERRUN` writer - 7:7\\]
Enable Buffer Underrun Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A buffer underrun event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnUnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_ERR_FRAME` reader - 8:8\\]
Enable ERROR Frame Received Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A error frame received event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnErrFrameR = crate::BitReader;
#[doc = "Field `INT1_EN_ERR_FRAME` writer - 8:8\\]
Enable ERROR Frame Received Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A error frame received event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnErrFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_PING_FRAME` reader - 9:9\\]
Enable Ping Frame Received Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A ping frame received event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnPingFrameR = crate::BitReader;
#[doc = "Field `INT1_EN_PING_FRAME` writer - 9:9\\]
Enable Ping Frame Received Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A ping frame received event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnPingFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_FRAME_OVERRUN` reader - 10:10\\]
Enable Frame Overrun Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame overrun event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnFrameOverrunR = crate::BitReader;
#[doc = "Field `INT1_EN_FRAME_OVERRUN` writer - 10:10\\]
Enable Frame Overrun Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame overrun event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnFrameOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_DATA_FRAME` reader - 11:11\\]
Enable Data Frame Received Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A data frame received event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnDataFrameR = crate::BitReader;
#[doc = "Field `INT1_EN_DATA_FRAME` writer - 11:11\\]
Enable Data Frame Received Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A data frame received event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnDataFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_PING_TAG_MATCH` reader - 12:12\\]
Enable Ping Frame Received with Tag Match Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A ping frame received with matching tag will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnPingTagMatchR = crate::BitReader;
#[doc = "Field `INT1_EN_PING_TAG_MATCH` writer - 12:12\\]
Enable Ping Frame Received with Tag Match Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A ping frame received with matching tag will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnPingTagMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_DATA_TAG_MATCH` reader - 13:13\\]
Enable Data Frame Received with Tag Match Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A data frame received with matching tag will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnDataTagMatchR = crate::BitReader;
#[doc = "Field `INT1_EN_DATA_TAG_MATCH` writer - 13:13\\]
Enable Data Frame Received with Tag Match Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A data frame received with matching tag will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnDataTagMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1_EN_ERROR_TAG_MATCH` reader - 14:14\\]
Enable Error Frame Received with Tag Match Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= An error frame received with matching tag will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnErrorTagMatchR = crate::BitReader;
#[doc = "Field `INT1_EN_ERROR_TAG_MATCH` writer - 14:14\\]
Enable Error Frame Received with Tag Match Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= An error frame received with matching tag will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
pub type Int1EnErrorTagMatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Ping Watchdog Timeout Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A ping watchdog timeout event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_ping_wd_to(&self) -> Int1EnPingWdToR {
        Int1EnPingWdToR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable Frame Watchdog Timeout Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame watchdog timeout event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_frame_wd_to(&self) -> Int1EnFrameWdToR {
        Int1EnFrameWdToR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable CRC Error Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A CRC error will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_crc_err(&self) -> Int1EnCrcErrR {
        Int1EnCrcErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable Frame Type Error Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame type error event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_type_err(&self) -> Int1EnTypeErrR {
        Int1EnTypeErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable End-of-Frame Error Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= An end-of-frame error event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_eof_err(&self) -> Int1EnEofErrR {
        Int1EnEofErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable Receive Buffer Overrun Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A receive buffer overrun event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_overrun(&self) -> Int1EnOverrunR {
        Int1EnOverrunR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable Frame Done Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame done event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_frame_done(&self) -> Int1EnFrameDoneR {
        Int1EnFrameDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable Buffer Underrun Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A buffer underrun event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_underrun(&self) -> Int1EnUnderrunR {
        Int1EnUnderrunR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable ERROR Frame Received Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A error frame received event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_err_frame(&self) -> Int1EnErrFrameR {
        Int1EnErrFrameR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable Ping Frame Received Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A ping frame received event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_ping_frame(&self) -> Int1EnPingFrameR {
        Int1EnPingFrameR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable Frame Overrun Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame overrun event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_frame_overrun(&self) -> Int1EnFrameOverrunR {
        Int1EnFrameOverrunR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Enable Data Frame Received Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A data frame received event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_data_frame(&self) -> Int1EnDataFrameR {
        Int1EnDataFrameR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable Ping Frame Received with Tag Match Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A ping frame received with matching tag will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_ping_tag_match(&self) -> Int1EnPingTagMatchR {
        Int1EnPingTagMatchR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Enable Data Frame Received with Tag Match Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A data frame received with matching tag will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_data_tag_match(&self) -> Int1EnDataTagMatchR {
        Int1EnDataTagMatchR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Enable Error Frame Received with Tag Match Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= An error frame received with matching tag will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    pub fn int1_en_error_tag_match(&self) -> Int1EnErrorTagMatchR {
        Int1EnErrorTagMatchR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Ping Watchdog Timeout Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A ping watchdog timeout event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_ping_wd_to(&mut self) -> Int1EnPingWdToW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnPingWdToW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable Frame Watchdog Timeout Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame watchdog timeout event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_frame_wd_to(&mut self) -> Int1EnFrameWdToW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnFrameWdToW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable CRC Error Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A CRC error will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_crc_err(&mut self) -> Int1EnCrcErrW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnCrcErrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable Frame Type Error Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame type error event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_type_err(&mut self) -> Int1EnTypeErrW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnTypeErrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable End-of-Frame Error Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= An end-of-frame error event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_eof_err(&mut self) -> Int1EnEofErrW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnEofErrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable Receive Buffer Overrun Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A receive buffer overrun event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_overrun(&mut self) -> Int1EnOverrunW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnOverrunW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable Frame Done Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame done event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_frame_done(&mut self) -> Int1EnFrameDoneW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnFrameDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable Buffer Underrun Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A buffer underrun event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_underrun(&mut self) -> Int1EnUnderrunW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnUnderrunW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable ERROR Frame Received Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A error frame received event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_err_frame(&mut self) -> Int1EnErrFrameW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnErrFrameW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable Ping Frame Received Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A ping frame received event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_ping_frame(&mut self) -> Int1EnPingFrameW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnPingFrameW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable Frame Overrun Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A frame overrun event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_frame_overrun(&mut self) -> Int1EnFrameOverrunW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnFrameOverrunW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Enable Data Frame Received Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A data frame received event will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_data_frame(&mut self) -> Int1EnDataFrameW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnDataFrameW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable Ping Frame Received with Tag Match Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A ping frame received with matching tag will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_ping_tag_match(&mut self) -> Int1EnPingTagMatchW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnPingTagMatchW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Enable Data Frame Received with Tag Match Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= A data frame received with matching tag will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_data_tag_match(&mut self) -> Int1EnDataTagMatchW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnDataTagMatchW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Enable Error Frame Received with Tag Match Interrupt to INT1 bitThis is an enable register which decides whether an interrupt \\[RX_INT1\\]
will be generated on the enabled event. 0h \\[R/W\\]
= This event will not trigger an interrupt on RX_INT1.1h \\[R/W\\]
= An error frame received with matching tag will trigger an interrupt on RX_INT1. The event itself will be latched in the corresponding bit in the RX_EVT_STS Register"]
    #[inline(always)]
    #[must_use]
    pub fn int1_en_error_tag_match(&mut self) -> Int1EnErrorTagMatchW<CfgRxInt1CtrlAlt1_Spec> {
        Int1EnErrorTagMatchW::new(self, 14)
    }
}
#[doc = "Receive interrupt control register for RX_INT1. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_int1_ctrl_alt1_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_int1_ctrl_alt1_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxInt1CtrlAlt1_Spec;
impl crate::RegisterSpec for CfgRxInt1CtrlAlt1_Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_int1_ctrl_alt1_::R`](R) reader structure"]
impl crate::Readable for CfgRxInt1CtrlAlt1_Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_int1_ctrl_alt1_::W`](W) writer structure"]
impl crate::Writable for CfgRxInt1CtrlAlt1_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_INT1_CTRL_ALT1_ to value 0"]
impl crate::Resettable for CfgRxInt1CtrlAlt1_Spec {
    const RESET_VALUE: u16 = 0;
}
