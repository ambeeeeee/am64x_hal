#[doc = "Register `MCRC64_REGS_CRC_CTRL2` reader"]
pub type R = crate::R<Mcrc64RegsCrcCtrl2Spec>;
#[doc = "Register `MCRC64_REGS_CRC_CTRL2` writer"]
pub type W = crate::W<Mcrc64RegsCrcCtrl2Spec>;
#[doc = "Field `CH1_MODE` reader - 1:0\\]
Channel 1 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode"]
pub type Ch1ModeR = crate::FieldReader;
#[doc = "Field `CH1_MODE` writer - 1:0\\]
Channel 1 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode"]
pub type Ch1ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_TRACEEN` reader - 4:4\\]
Channel 1 Data Trace Enable When set, the channel is put into data trace mode. The channel snoops on the CPU VBUSM, ITCM, DTCM buses for any read transaction. Any read data on these buses is compressed by the PSA Signature Register. When suspend is on, the PSA Signature Register does not compress any read data on these buses. 0 = Data Trace disable 1 = Data Trace enable"]
pub type Ch1TraceenR = crate::BitReader;
#[doc = "Field `CH1_TRACEEN` writer - 4:4\\]
Channel 1 Data Trace Enable When set, the channel is put into data trace mode. The channel snoops on the CPU VBUSM, ITCM, DTCM buses for any read transaction. Any read data on these buses is compressed by the PSA Signature Register. When suspend is on, the PSA Signature Register does not compress any read data on these buses. 0 = Data Trace disable 1 = Data Trace enable"]
pub type Ch1TraceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_MODE` reader - 9:8\\]
Channel 2 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode"]
pub type Ch2ModeR = crate::FieldReader;
#[doc = "Field `CH2_MODE` writer - 9:8\\]
Channel 2 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode"]
pub type Ch2ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH3_MODE` reader - 17:16\\]
Channel 3 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode"]
pub type Ch3ModeR = crate::FieldReader;
#[doc = "Field `CH3_MODE` writer - 17:16\\]
Channel 3 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode"]
pub type Ch3ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH4_MODE` reader - 25:24\\]
Channel 4 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode For all four channels the seed value can be first planted into PSA register before the Channel Mode is switched to Full-CPU mode since host CPU controls the amount of data for compression. During AUTO mode, the PSA register is automatically reset to zero at the end of each sector compression."]
pub type Ch4ModeR = crate::FieldReader;
#[doc = "Field `CH4_MODE` writer - 25:24\\]
Channel 4 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode For all four channels the seed value can be first planted into PSA register before the Channel Mode is switched to Full-CPU mode since host CPU controls the amount of data for compression. During AUTO mode, the PSA register is automatically reset to zero at the end of each sector compression."]
pub type Ch4ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Channel 1 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode"]
    #[inline(always)]
    pub fn ch1_mode(&self) -> Ch1ModeR {
        Ch1ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 Data Trace Enable When set, the channel is put into data trace mode. The channel snoops on the CPU VBUSM, ITCM, DTCM buses for any read transaction. Any read data on these buses is compressed by the PSA Signature Register. When suspend is on, the PSA Signature Register does not compress any read data on these buses. 0 = Data Trace disable 1 = Data Trace enable"]
    #[inline(always)]
    pub fn ch1_traceen(&self) -> Ch1TraceenR {
        Ch1TraceenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Channel 2 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode"]
    #[inline(always)]
    pub fn ch2_mode(&self) -> Ch2ModeR {
        Ch2ModeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Channel 3 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode"]
    #[inline(always)]
    pub fn ch3_mode(&self) -> Ch3ModeR {
        Ch3ModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Channel 4 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode For all four channels the seed value can be first planted into PSA register before the Channel Mode is switched to Full-CPU mode since host CPU controls the amount of data for compression. During AUTO mode, the PSA register is automatically reset to zero at the end of each sector compression."]
    #[inline(always)]
    pub fn ch4_mode(&self) -> Ch4ModeR {
        Ch4ModeR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Channel 1 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_mode(&mut self) -> Ch1ModeW<Mcrc64RegsCrcCtrl2Spec> {
        Ch1ModeW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 Data Trace Enable When set, the channel is put into data trace mode. The channel snoops on the CPU VBUSM, ITCM, DTCM buses for any read transaction. Any read data on these buses is compressed by the PSA Signature Register. When suspend is on, the PSA Signature Register does not compress any read data on these buses. 0 = Data Trace disable 1 = Data Trace enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_traceen(&mut self) -> Ch1TraceenW<Mcrc64RegsCrcCtrl2Spec> {
        Ch1TraceenW::new(self, 4)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Channel 2 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_mode(&mut self) -> Ch2ModeW<Mcrc64RegsCrcCtrl2Spec> {
        Ch2ModeW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Channel 3 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_mode(&mut self) -> Ch3ModeW<Mcrc64RegsCrcCtrl2Spec> {
        Ch3ModeW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Channel 4 Mode. 00 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register. 01 = AUTO mode 10 = Semi-CPU mode 11 = Full-CPU mode For all four channels the seed value can be first planted into PSA register before the Channel Mode is switched to Full-CPU mode since host CPU controls the amount of data for compression. During AUTO mode, the PSA register is automatically reset to zero at the end of each sector compression."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_mode(&mut self) -> Ch4ModeW<Mcrc64RegsCrcCtrl2Spec> {
        Ch4ModeW::new(self, 24)
    }
}
#[doc = "Data capture mode is especially useful when it is used in conjunction when data trace (CH1_TRACEEN) for channel 1. The seed value can be planted in PSA Signature Register during data capture mode by writing a seed value into PSA Signature Register. Data trace enable bit is then set to snoop and compress any read transaction on Data buses. When trace enable bit is set, CH1_MODE is automatically reset to data capture mode. To change mode from one to another in the middle of an on-going mode user should take the following steps:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcCtrl2Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_ctrl2::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_ctrl2::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcCtrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_CTRL2 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcCtrl2Spec {
    const RESET_VALUE: u32 = 0;
}
