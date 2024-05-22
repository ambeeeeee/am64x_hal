#[doc = "Register `MCRC64_REGS_CRC_STATUS` reader"]
pub type R = crate::R<Mcrc64RegsCrcStatusSpec>;
#[doc = "Register `MCRC64_REGS_CRC_STATUS` writer"]
pub type W = crate::W<Mcrc64RegsCrcStatusSpec>;
#[doc = "Field `CH1_CCIT` reader - 0:0\\]
Channel 1 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
pub type Ch1CcitR = crate::BitReader;
#[doc = "Field `CH1_CCIT` writer - 0:0\\]
Channel 1 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
pub type Ch1CcitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CRC_FAIL` reader - 1:1\\]
Channel 1 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
pub type Ch1CrcFailR = crate::BitReader;
#[doc = "Field `CH1_CRC_FAIL` writer - 1:1\\]
Channel 1 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
pub type Ch1CrcFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_OVER` reader - 2:2\\]
Channel 1 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
pub type Ch1OverR = crate::BitReader;
#[doc = "Field `CH1_OVER` writer - 2:2\\]
Channel 1 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
pub type Ch1OverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_UNDER` reader - 3:3\\]
Channel 1 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
pub type Ch1UnderR = crate::BitReader;
#[doc = "Field `CH1_UNDER` writer - 3:3\\]
Channel 1 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
pub type Ch1UnderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TIME_OUT` reader - 4:4\\]
Channel 1 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
pub type Ch1TimeOutR = crate::BitReader;
#[doc = "Field `CH1_TIME_OUT` writer - 4:4\\]
Channel 1 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
pub type Ch1TimeOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_CCIT` reader - 8:8\\]
Channel 2 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
pub type Ch2CcitR = crate::BitReader;
#[doc = "Field `CH2_CCIT` writer - 8:8\\]
Channel 2 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
pub type Ch2CcitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_CRC_FAIL` reader - 9:9\\]
Channel 2 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
pub type Ch2CrcFailR = crate::BitReader;
#[doc = "Field `CH2_CRC_FAIL` writer - 9:9\\]
Channel 2 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
pub type Ch2CrcFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_OVER` reader - 10:10\\]
Channel 2 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
pub type Ch2OverR = crate::BitReader;
#[doc = "Field `CH2_OVER` writer - 10:10\\]
Channel 2 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
pub type Ch2OverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_UNDER` reader - 11:11\\]
Channel 2 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
pub type Ch2UnderR = crate::BitReader;
#[doc = "Field `CH2_UNDER` writer - 11:11\\]
Channel 2 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
pub type Ch2UnderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TIME_OUT` reader - 12:12\\]
Channel 2 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
pub type Ch2TimeOutR = crate::BitReader;
#[doc = "Field `CH2_TIME_OUT` writer - 12:12\\]
Channel 2 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
pub type Ch2TimeOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_CCIT` reader - 16:16\\]
Channel 3 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
pub type Ch3CcitR = crate::BitReader;
#[doc = "Field `CH3_CCIT` writer - 16:16\\]
Channel 3 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
pub type Ch3CcitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_CRC_FAIL` reader - 17:17\\]
Channel 3 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
pub type Ch3CrcFailR = crate::BitReader;
#[doc = "Field `CH3_CRC_FAIL` writer - 17:17\\]
Channel 3 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
pub type Ch3CrcFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_OVER` reader - 18:18\\]
Channel 3 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
pub type Ch3OverR = crate::BitReader;
#[doc = "Field `CH3_OVER` writer - 18:18\\]
Channel 3 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
pub type Ch3OverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_UNDER` reader - 19:19\\]
Channel 3 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
pub type Ch3UnderR = crate::BitReader;
#[doc = "Field `CH3_UNDER` writer - 19:19\\]
Channel 3 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
pub type Ch3UnderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TIME_OUT` reader - 20:20\\]
Channel 3 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
pub type Ch3TimeOutR = crate::BitReader;
#[doc = "Field `CH3_TIME_OUT` writer - 20:20\\]
Channel 3 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
pub type Ch3TimeOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CCIT` reader - 24:24\\]
Channel 4 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
pub type Ch4CcitR = crate::BitReader;
#[doc = "Field `CH4_CCIT` writer - 24:24\\]
Channel 4 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
pub type Ch4CcitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CRC_FAIL` reader - 25:25\\]
Channel 4 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
pub type Ch4CrcFailR = crate::BitReader;
#[doc = "Field `CH4_CRC_FAIL` writer - 25:25\\]
Channel 4 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
pub type Ch4CrcFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_OVER` reader - 26:26\\]
Channel 4 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
pub type Ch4OverR = crate::BitReader;
#[doc = "Field `CH4_OVER` writer - 26:26\\]
Channel 4 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
pub type Ch4OverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_UNDER` reader - 27:27\\]
Channel 4 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
pub type Ch4UnderR = crate::BitReader;
#[doc = "Field `CH4_UNDER` writer - 27:27\\]
Channel 4 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
pub type Ch4UnderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_TIME_OUT` reader - 28:28\\]
Channel 4 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
pub type Ch4TimeOutR = crate::BitReader;
#[doc = "Field `CH4_TIME_OUT` writer - 28:28\\]
Channel 4 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
pub type Ch4TimeOutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel 1 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
    #[inline(always)]
    pub fn ch1_ccit(&self) -> Ch1CcitR {
        Ch1CcitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel 1 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
    #[inline(always)]
    pub fn ch1_crc_fail(&self) -> Ch1CrcFailR {
        Ch1CrcFailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel 1 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
    #[inline(always)]
    pub fn ch1_over(&self) -> Ch1OverR {
        Ch1OverR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 1 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
    #[inline(always)]
    pub fn ch1_under(&self) -> Ch1UnderR {
        Ch1UnderR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
    #[inline(always)]
    pub fn ch1_time_out(&self) -> Ch1TimeOutR {
        Ch1TimeOutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 2 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
    #[inline(always)]
    pub fn ch2_ccit(&self) -> Ch2CcitR {
        Ch2CcitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Channel 2 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
    #[inline(always)]
    pub fn ch2_crc_fail(&self) -> Ch2CrcFailR {
        Ch2CrcFailR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Channel 2 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
    #[inline(always)]
    pub fn ch2_over(&self) -> Ch2OverR {
        Ch2OverR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Channel 2 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
    #[inline(always)]
    pub fn ch2_under(&self) -> Ch2UnderR {
        Ch2UnderR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Channel 2 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
    #[inline(always)]
    pub fn ch2_time_out(&self) -> Ch2TimeOutR {
        Ch2TimeOutR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Channel 3 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
    #[inline(always)]
    pub fn ch3_ccit(&self) -> Ch3CcitR {
        Ch3CcitR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Channel 3 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
    #[inline(always)]
    pub fn ch3_crc_fail(&self) -> Ch3CrcFailR {
        Ch3CrcFailR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Channel 3 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
    #[inline(always)]
    pub fn ch3_over(&self) -> Ch3OverR {
        Ch3OverR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Channel 3 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
    #[inline(always)]
    pub fn ch3_under(&self) -> Ch3UnderR {
        Ch3UnderR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Channel 3 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
    #[inline(always)]
    pub fn ch3_time_out(&self) -> Ch3TimeOutR {
        Ch3TimeOutR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Channel 4 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
    #[inline(always)]
    pub fn ch4_ccit(&self) -> Ch4CcitR {
        Ch4CcitR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Channel 4 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
    #[inline(always)]
    pub fn ch4_crc_fail(&self) -> Ch4CrcFailR {
        Ch4CrcFailR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Channel 4 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
    #[inline(always)]
    pub fn ch4_over(&self) -> Ch4OverR {
        Ch4OverR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Channel 4 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
    #[inline(always)]
    pub fn ch4_under(&self) -> Ch4UnderR {
        Ch4UnderR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Channel 4 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
    #[inline(always)]
    pub fn ch4_time_out(&self) -> Ch4TimeOutR {
        Ch4TimeOutR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel 1 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_ccit(&mut self) -> Ch1CcitW<Mcrc64RegsCrcStatusSpec> {
        Ch1CcitW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel 1 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_crc_fail(&mut self) -> Ch1CrcFailW<Mcrc64RegsCrcStatusSpec> {
        Ch1CrcFailW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel 1 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_over(&mut self) -> Ch1OverW<Mcrc64RegsCrcStatusSpec> {
        Ch1OverW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 1 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_under(&mut self) -> Ch1UnderW<Mcrc64RegsCrcStatusSpec> {
        Ch1UnderW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_time_out(&mut self) -> Ch1TimeOutW<Mcrc64RegsCrcStatusSpec> {
        Ch1TimeOutW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 2 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_ccit(&mut self) -> Ch2CcitW<Mcrc64RegsCrcStatusSpec> {
        Ch2CcitW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Channel 2 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_crc_fail(&mut self) -> Ch2CrcFailW<Mcrc64RegsCrcStatusSpec> {
        Ch2CrcFailW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Channel 2 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_over(&mut self) -> Ch2OverW<Mcrc64RegsCrcStatusSpec> {
        Ch2OverW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Channel 2 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_under(&mut self) -> Ch2UnderW<Mcrc64RegsCrcStatusSpec> {
        Ch2UnderW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Channel 2 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_time_out(&mut self) -> Ch2TimeOutW<Mcrc64RegsCrcStatusSpec> {
        Ch2TimeOutW::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
Channel 3 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_ccit(&mut self) -> Ch3CcitW<Mcrc64RegsCrcStatusSpec> {
        Ch3CcitW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Channel 3 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_crc_fail(&mut self) -> Ch3CrcFailW<Mcrc64RegsCrcStatusSpec> {
        Ch3CrcFailW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Channel 3 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_over(&mut self) -> Ch3OverW<Mcrc64RegsCrcStatusSpec> {
        Ch3OverW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Channel 3 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_under(&mut self) -> Ch3UnderW<Mcrc64RegsCrcStatusSpec> {
        Ch3UnderW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Channel 3 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_time_out(&mut self) -> Ch3TimeOutW<Mcrc64RegsCrcStatusSpec> {
        Ch3TimeOutW::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
Channel 4 CRC Pattern Compression Complete Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is only set in Semi-CPU mode. 0 = No CRC pattern compression complete interrupt is active 1 = CRC pattern compression complete interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_ccit(&mut self) -> Ch4CcitW<Mcrc64RegsCrcStatusSpec> {
        Ch4CcitW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Channel 4 CRC Compare Fail Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_crc_fail(&mut self) -> Ch4CrcFailW<Mcrc64RegsCrcStatusSpec> {
        Ch4CrcFailW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Channel 4 CRC Overrun Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_over(&mut self) -> Ch4OverW<Mcrc64RegsCrcStatusSpec> {
        Ch4OverW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Channel 4 CRC Underrun Status Flag. This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_under(&mut self) -> Ch4UnderW<Mcrc64RegsCrcStatusSpec> {
        Ch4UnderW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Channel 4 CRC Timeout Status Flag This bit is cleared by writing a 1 to it only. Writing 0 has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_time_out(&mut self) -> Ch4TimeOutW<Mcrc64RegsCrcStatusSpec> {
        Ch4TimeOutW::new(self, 28)
    }
}
#[doc = "CRC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcStatusSpec;
impl crate::RegisterSpec for Mcrc64RegsCrcStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_status::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_status::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_STATUS to value 0"]
impl crate::Resettable for Mcrc64RegsCrcStatusSpec {
    const RESET_VALUE: u32 = 0;
}
