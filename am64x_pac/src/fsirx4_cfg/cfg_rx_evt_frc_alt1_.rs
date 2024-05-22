#[doc = "Register `CFG_RX_EVT_FRC_ALT1_` reader"]
pub type R = crate::R<CfgRxEvtFrcAlt1_Spec>;
#[doc = "Register `CFG_RX_EVT_FRC_ALT1_` writer"]
pub type W = crate::W<CfgRxEvtFrcAlt1_Spec>;
#[doc = "Field `PING_WD_TO` reader - 0:0\\]
Ping Watchdog Timeout Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type PingWdToR = crate::BitReader;
#[doc = "Field `PING_WD_TO` writer - 0:0\\]
Ping Watchdog Timeout Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type PingWdToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_WD_TO` reader - 1:1\\]
Frame Watchdog Timeout Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type FrameWdToR = crate::BitReader;
#[doc = "Field `FRAME_WD_TO` writer - 1:1\\]
Frame Watchdog Timeout Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type FrameWdToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_ERR` reader - 2:2\\]
CRC Error Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type CrcErrR = crate::BitReader;
#[doc = "Field `CRC_ERR` writer - 2:2\\]
CRC Error Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type CrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE_ERR` reader - 3:3\\]
Frame Type Error Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type TypeErrR = crate::BitReader;
#[doc = "Field `TYPE_ERR` writer - 3:3\\]
Frame Type Error Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type TypeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOF_ERR` reader - 4:4\\]
End-of-Frame Error Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type EofErrR = crate::BitReader;
#[doc = "Field `EOF_ERR` writer - 4:4\\]
End-of-Frame Error Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type EofErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_OVERRUN` reader - 5:5\\]
Receive Buffer Overrun Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type BufOverrunR = crate::BitReader;
#[doc = "Field `BUF_OVERRUN` writer - 5:5\\]
Receive Buffer Overrun Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type BufOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_DONE` reader - 6:6\\]
Frame Done Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type FrameDoneR = crate::BitReader;
#[doc = "Field `FRAME_DONE` writer - 6:6\\]
Frame Done Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type FrameDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_UNDERRUN` reader - 7:7\\]
Receive Buffer Underrun Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type BufUnderrunR = crate::BitReader;
#[doc = "Field `BUF_UNDERRUN` writer - 7:7\\]
Receive Buffer Underrun Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type BufUnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_FRAME` reader - 8:8\\]
Error Frame Received Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type ErrFrameR = crate::BitReader;
#[doc = "Field `ERR_FRAME` writer - 8:8\\]
Error Frame Received Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type ErrFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PING_FRAME` reader - 9:9\\]
Ping Frame Received Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type PingFrameR = crate::BitReader;
#[doc = "Field `PING_FRAME` writer - 9:9\\]
Ping Frame Received Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type PingFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_OVERRUN` reader - 10:10\\]
Frame Overrun Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type FrameOverrunR = crate::BitReader;
#[doc = "Field `FRAME_OVERRUN` writer - 10:10\\]
Frame Overrun Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type FrameOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_FRAME` reader - 11:11\\]
Data Frame Received Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type DataFrameR = crate::BitReader;
#[doc = "Field `DATA_FRAME` writer - 11:11\\]
Data Frame Received Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type DataFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PING_TAG_MATCH` reader - 12:12\\]
Ping Tag Match Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type PingTagMatchR = crate::BitReader;
#[doc = "Field `PING_TAG_MATCH` writer - 12:12\\]
Ping Tag Match Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type PingTagMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TAG_MATCH` reader - 13:13\\]
Data Tag Match Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type DataTagMatchR = crate::BitReader;
#[doc = "Field `DATA_TAG_MATCH` writer - 13:13\\]
Data Tag Match Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type DataTagMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_TAG_MATCH` reader - 14:14\\]
Error Tag Match Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type ErrorTagMatchR = crate::BitReader;
#[doc = "Field `ERROR_TAG_MATCH` writer - 14:14\\]
Error Tag Match Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
pub type ErrorTagMatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Ping Watchdog Timeout Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn ping_wd_to(&self) -> PingWdToR {
        PingWdToR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Frame Watchdog Timeout Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn frame_wd_to(&self) -> FrameWdToR {
        FrameWdToR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CRC Error Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn crc_err(&self) -> CrcErrR {
        CrcErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Frame Type Error Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn type_err(&self) -> TypeErrR {
        TypeErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
End-of-Frame Error Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn eof_err(&self) -> EofErrR {
        EofErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Receive Buffer Overrun Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn buf_overrun(&self) -> BufOverrunR {
        BufOverrunR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Frame Done Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn frame_done(&self) -> FrameDoneR {
        FrameDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Receive Buffer Underrun Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn buf_underrun(&self) -> BufUnderrunR {
        BufUnderrunR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Error Frame Received Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn err_frame(&self) -> ErrFrameR {
        ErrFrameR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Ping Frame Received Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn ping_frame(&self) -> PingFrameR {
        PingFrameR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Frame Overrun Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn frame_overrun(&self) -> FrameOverrunR {
        FrameOverrunR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Data Frame Received Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn data_frame(&self) -> DataFrameR {
        DataFrameR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Ping Tag Match Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn ping_tag_match(&self) -> PingTagMatchR {
        PingTagMatchR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Data Tag Match Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn data_tag_match(&self) -> DataTagMatchR {
        DataTagMatchR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Error Tag Match Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    pub fn error_tag_match(&self) -> ErrorTagMatchR {
        ErrorTagMatchR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Ping Watchdog Timeout Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn ping_wd_to(&mut self) -> PingWdToW<CfgRxEvtFrcAlt1_Spec> {
        PingWdToW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Frame Watchdog Timeout Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn frame_wd_to(&mut self) -> FrameWdToW<CfgRxEvtFrcAlt1_Spec> {
        FrameWdToW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CRC Error Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn crc_err(&mut self) -> CrcErrW<CfgRxEvtFrcAlt1_Spec> {
        CrcErrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Frame Type Error Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn type_err(&mut self) -> TypeErrW<CfgRxEvtFrcAlt1_Spec> {
        TypeErrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
End-of-Frame Error Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn eof_err(&mut self) -> EofErrW<CfgRxEvtFrcAlt1_Spec> {
        EofErrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Receive Buffer Overrun Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn buf_overrun(&mut self) -> BufOverrunW<CfgRxEvtFrcAlt1_Spec> {
        BufOverrunW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Frame Done Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn frame_done(&mut self) -> FrameDoneW<CfgRxEvtFrcAlt1_Spec> {
        FrameDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Receive Buffer Underrun Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn buf_underrun(&mut self) -> BufUnderrunW<CfgRxEvtFrcAlt1_Spec> {
        BufUnderrunW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Error Frame Received Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn err_frame(&mut self) -> ErrFrameW<CfgRxEvtFrcAlt1_Spec> {
        ErrFrameW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Ping Frame Received Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn ping_frame(&mut self) -> PingFrameW<CfgRxEvtFrcAlt1_Spec> {
        PingFrameW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Frame Overrun Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn frame_overrun(&mut self) -> FrameOverrunW<CfgRxEvtFrcAlt1_Spec> {
        FrameOverrunW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Data Frame Received Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn data_frame(&mut self) -> DataFrameW<CfgRxEvtFrcAlt1_Spec> {
        DataFrameW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Ping Tag Match Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn ping_tag_match(&mut self) -> PingTagMatchW<CfgRxEvtFrcAlt1_Spec> {
        PingTagMatchW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Data Tag Match Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn data_tag_match(&mut self) -> DataTagMatchW<CfgRxEvtFrcAlt1_Spec> {
        DataTagMatchW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Error Tag Match Flag Force bitThis bit will cause the corresponding bit in the RX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding bit in the RX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn error_tag_match(&mut self) -> ErrorTagMatchW<CfgRxEvtFrcAlt1_Spec> {
        ErrorTagMatchW::new(self, 14)
    }
}
#[doc = "Receive event and error flag force register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_evt_frc_alt1_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_evt_frc_alt1_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxEvtFrcAlt1_Spec;
impl crate::RegisterSpec for CfgRxEvtFrcAlt1_Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_evt_frc_alt1_::R`](R) reader structure"]
impl crate::Readable for CfgRxEvtFrcAlt1_Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_evt_frc_alt1_::W`](W) writer structure"]
impl crate::Writable for CfgRxEvtFrcAlt1_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_EVT_FRC_ALT1_ to value 0"]
impl crate::Resettable for CfgRxEvtFrcAlt1_Spec {
    const RESET_VALUE: u16 = 0;
}
