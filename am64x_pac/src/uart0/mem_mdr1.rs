#[doc = "Register `MEM_MDR1` reader"]
pub type R = crate::R<MemMdr1Spec>;
#[doc = "Register `MEM_MDR1` writer"]
pub type W = crate::W<MemMdr1Spec>;
#[doc = "Field `MODE_SELECT` reader - "]
pub type ModeSelectR = crate::FieldReader;
#[doc = "Field `MODE_SELECT` writer - "]
pub type ModeSelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IR_SLEEP` reader - "]
pub type IrSleepR = crate::BitReader;
#[doc = "Field `IR_SLEEP` writer - "]
pub type IrSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_TXIR` reader - 4:4\\]
Used to configure the infrared transceiver."]
pub type SetTxirR = crate::BitReader;
#[doc = "Field `SET_TXIR` writer - 4:4\\]
Used to configure the infrared transceiver."]
pub type SetTxirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCT` reader - 5:5\\]
Store and control the transmission"]
pub type SctR = crate::BitReader;
#[doc = "Field `SCT` writer - 5:5\\]
Store and control the transmission"]
pub type SctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIP_MODE` reader - 6:6\\]
MIR/FIR modes only."]
pub type SipModeR = crate::BitReader;
#[doc = "Field `SIP_MODE` writer - 6:6\\]
MIR/FIR modes only."]
pub type SipModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_END_MODE` reader - 7:7\\]
IrDA mode only."]
pub type FrameEndModeR = crate::BitReader;
#[doc = "Field `FRAME_END_MODE` writer - 7:7\\]
IrDA mode only."]
pub type FrameEndModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn mode_select(&self) -> ModeSelectR {
        ModeSelectR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ir_sleep(&self) -> IrSleepR {
        IrSleepR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Used to configure the infrared transceiver."]
    #[inline(always)]
    pub fn set_txir(&self) -> SetTxirR {
        SetTxirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Store and control the transmission"]
    #[inline(always)]
    pub fn sct(&self) -> SctR {
        SctR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
MIR/FIR modes only."]
    #[inline(always)]
    pub fn sip_mode(&self) -> SipModeR {
        SipModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
IrDA mode only."]
    #[inline(always)]
    pub fn frame_end_mode(&self) -> FrameEndModeR {
        FrameEndModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn mode_select(&mut self) -> ModeSelectW<MemMdr1Spec> {
        ModeSelectW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ir_sleep(&mut self) -> IrSleepW<MemMdr1Spec> {
        IrSleepW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Used to configure the infrared transceiver."]
    #[inline(always)]
    #[must_use]
    pub fn set_txir(&mut self) -> SetTxirW<MemMdr1Spec> {
        SetTxirW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Store and control the transmission"]
    #[inline(always)]
    #[must_use]
    pub fn sct(&mut self) -> SctW<MemMdr1Spec> {
        SctW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
MIR/FIR modes only."]
    #[inline(always)]
    #[must_use]
    pub fn sip_mode(&mut self) -> SipModeW<MemMdr1Spec> {
        SipModeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
IrDA mode only."]
    #[inline(always)]
    #[must_use]
    pub fn frame_end_mode(&mut self) -> FrameEndModeW<MemMdr1Spec> {
        FrameEndModeW::new(self, 7)
    }
}
#[doc = "The mode of operation can be programmed by writing to MDR1\\[2:0\\]
and therefore the MDR1 must be programmed on start-up after configuration of the configuration registers (DLL, DLH, LCR). The value of MDR1\\[2:0\\]
must not be changed again during normal operation. Note: If the module is disabled by setting the MODE_SELECT field to\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mdr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mdr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemMdr1Spec;
impl crate::RegisterSpec for MemMdr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_mdr1::R`](R) reader structure"]
impl crate::Readable for MemMdr1Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_mdr1::W`](W) writer structure"]
impl crate::Writable for MemMdr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_MDR1 to value 0x07"]
impl crate::Resettable for MemMdr1Spec {
    const RESET_VALUE: u32 = 0x07;
}
