#[doc = "Register `CFG_RX_EVT_CLR_ALT1_` reader"]
pub type R = crate::R<CfgRxEvtClrAlt1_Spec>;
#[doc = "Register `CFG_RX_EVT_CLR_ALT1_` writer"]
pub type W = crate::W<CfgRxEvtClrAlt1_Spec>;
#[doc = "Field `PING_WD_TO` reader - 0:0\\]
Ping Watchdog Timeout Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type PingWdToR = crate::BitReader;
#[doc = "Field `PING_WD_TO` writer - 0:0\\]
Ping Watchdog Timeout Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type PingWdToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_WD_TO` reader - 1:1\\]
Frame Watchdog Timeout Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type FrameWdToR = crate::BitReader;
#[doc = "Field `FRAME_WD_TO` writer - 1:1\\]
Frame Watchdog Timeout Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type FrameWdToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_ERR` reader - 2:2\\]
CRC Error Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type CrcErrR = crate::BitReader;
#[doc = "Field `CRC_ERR` writer - 2:2\\]
CRC Error Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type CrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE_ERR` reader - 3:3\\]
Frame Type Error Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type TypeErrR = crate::BitReader;
#[doc = "Field `TYPE_ERR` writer - 3:3\\]
Frame Type Error Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type TypeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOF_ERR` reader - 4:4\\]
End-of-Frame Error Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type EofErrR = crate::BitReader;
#[doc = "Field `EOF_ERR` writer - 4:4\\]
End-of-Frame Error Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type EofErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_OVERRUN` reader - 5:5\\]
Receive Buffer Overrun Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type BufOverrunR = crate::BitReader;
#[doc = "Field `BUF_OVERRUN` writer - 5:5\\]
Receive Buffer Overrun Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type BufOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_DONE` reader - 6:6\\]
Frame Done Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type FrameDoneR = crate::BitReader;
#[doc = "Field `FRAME_DONE` writer - 6:6\\]
Frame Done Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type FrameDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_UNDERRUN` reader - 7:7\\]
Receive Buffer Underrun Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[R/W\\]
= Writing a 0 to this bit will have no effect.1h \\[R/W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type BufUnderrunR = crate::BitReader;
#[doc = "Field `BUF_UNDERRUN` writer - 7:7\\]
Receive Buffer Underrun Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[R/W\\]
= Writing a 0 to this bit will have no effect.1h \\[R/W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type BufUnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_FRAME` reader - 8:8\\]
Error Frame Received Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type ErrFrameR = crate::BitReader;
#[doc = "Field `ERR_FRAME` writer - 8:8\\]
Error Frame Received Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type ErrFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PING_FRAME` reader - 9:9\\]
Ping Frame Received Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type PingFrameR = crate::BitReader;
#[doc = "Field `PING_FRAME` writer - 9:9\\]
Ping Frame Received Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type PingFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_OVERRUN` reader - 10:10\\]
Frame Overrun Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type FrameOverrunR = crate::BitReader;
#[doc = "Field `FRAME_OVERRUN` writer - 10:10\\]
Frame Overrun Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type FrameOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_FRAME` reader - 11:11\\]
Data Frame Received Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type DataFrameR = crate::BitReader;
#[doc = "Field `DATA_FRAME` writer - 11:11\\]
Data Frame Received Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type DataFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PING_TAG_MATCH` reader - 12:12\\]
Ping Tag Match Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type PingTagMatchR = crate::BitReader;
#[doc = "Field `PING_TAG_MATCH` writer - 12:12\\]
Ping Tag Match Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type PingTagMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TAG_MATCH` reader - 13:13\\]
Data Tag Match Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type DataTagMatchR = crate::BitReader;
#[doc = "Field `DATA_TAG_MATCH` writer - 13:13\\]
Data Tag Match Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type DataTagMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_TAG_MATCH` reader - 14:14\\]
Error Tag Match Glag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type ErrorTagMatchR = crate::BitReader;
#[doc = "Field `ERROR_TAG_MATCH` writer - 14:14\\]
Error Tag Match Glag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
pub type ErrorTagMatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Ping Watchdog Timeout Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn ping_wd_to(&self) -> PingWdToR {
        PingWdToR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Frame Watchdog Timeout Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn frame_wd_to(&self) -> FrameWdToR {
        FrameWdToR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CRC Error Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn crc_err(&self) -> CrcErrR {
        CrcErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Frame Type Error Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn type_err(&self) -> TypeErrR {
        TypeErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
End-of-Frame Error Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn eof_err(&self) -> EofErrR {
        EofErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Receive Buffer Overrun Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn buf_overrun(&self) -> BufOverrunR {
        BufOverrunR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Frame Done Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn frame_done(&self) -> FrameDoneR {
        FrameDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Receive Buffer Underrun Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[R/W\\]
= Writing a 0 to this bit will have no effect.1h \\[R/W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn buf_underrun(&self) -> BufUnderrunR {
        BufUnderrunR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Error Frame Received Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn err_frame(&self) -> ErrFrameR {
        ErrFrameR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Ping Frame Received Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn ping_frame(&self) -> PingFrameR {
        PingFrameR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Frame Overrun Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn frame_overrun(&self) -> FrameOverrunR {
        FrameOverrunR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Data Frame Received Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn data_frame(&self) -> DataFrameR {
        DataFrameR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Ping Tag Match Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn ping_tag_match(&self) -> PingTagMatchR {
        PingTagMatchR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Data Tag Match Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn data_tag_match(&self) -> DataTagMatchR {
        DataTagMatchR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Error Tag Match Glag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    pub fn error_tag_match(&self) -> ErrorTagMatchR {
        ErrorTagMatchR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Ping Watchdog Timeout Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn ping_wd_to(&mut self) -> PingWdToW<CfgRxEvtClrAlt1_Spec> {
        PingWdToW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Frame Watchdog Timeout Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn frame_wd_to(&mut self) -> FrameWdToW<CfgRxEvtClrAlt1_Spec> {
        FrameWdToW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CRC Error Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn crc_err(&mut self) -> CrcErrW<CfgRxEvtClrAlt1_Spec> {
        CrcErrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Frame Type Error Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn type_err(&mut self) -> TypeErrW<CfgRxEvtClrAlt1_Spec> {
        TypeErrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
End-of-Frame Error Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn eof_err(&mut self) -> EofErrW<CfgRxEvtClrAlt1_Spec> {
        EofErrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Receive Buffer Overrun Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn buf_overrun(&mut self) -> BufOverrunW<CfgRxEvtClrAlt1_Spec> {
        BufOverrunW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Frame Done Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn frame_done(&mut self) -> FrameDoneW<CfgRxEvtClrAlt1_Spec> {
        FrameDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Receive Buffer Underrun Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[R/W\\]
= Writing a 0 to this bit will have no effect.1h \\[R/W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn buf_underrun(&mut self) -> BufUnderrunW<CfgRxEvtClrAlt1_Spec> {
        BufUnderrunW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Error Frame Received Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn err_frame(&mut self) -> ErrFrameW<CfgRxEvtClrAlt1_Spec> {
        ErrFrameW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Ping Frame Received Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn ping_frame(&mut self) -> PingFrameW<CfgRxEvtClrAlt1_Spec> {
        PingFrameW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Frame Overrun Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn frame_overrun(&mut self) -> FrameOverrunW<CfgRxEvtClrAlt1_Spec> {
        FrameOverrunW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Data Frame Received Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn data_frame(&mut self) -> DataFrameW<CfgRxEvtClrAlt1_Spec> {
        DataFrameW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Ping Tag Match Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn ping_tag_match(&mut self) -> PingTagMatchW<CfgRxEvtClrAlt1_Spec> {
        PingTagMatchW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Data Tag Match Flag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn data_tag_match(&mut self) -> DataTagMatchW<CfgRxEvtClrAlt1_Spec> {
        DataTagMatchW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Error Tag Match Glag Clear bitThis bit clears the corresponding bit in the RX_EVT_STS register. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Writing a 1 to this bit will clear the corresponding bit in the RX_EVT_STS register to 0."]
    #[inline(always)]
    #[must_use]
    pub fn error_tag_match(&mut self) -> ErrorTagMatchW<CfgRxEvtClrAlt1_Spec> {
        ErrorTagMatchW::new(self, 14)
    }
}
#[doc = "Receive event and error clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_evt_clr_alt1_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_evt_clr_alt1_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxEvtClrAlt1_Spec;
impl crate::RegisterSpec for CfgRxEvtClrAlt1_Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_evt_clr_alt1_::R`](R) reader structure"]
impl crate::Readable for CfgRxEvtClrAlt1_Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_evt_clr_alt1_::W`](W) writer structure"]
impl crate::Writable for CfgRxEvtClrAlt1_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_EVT_CLR_ALT1_ to value 0"]
impl crate::Resettable for CfgRxEvtClrAlt1_Spec {
    const RESET_VALUE: u16 = 0;
}
