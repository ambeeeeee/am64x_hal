#[doc = "Register `MCRC64_REGS_CRC_BUSY` reader"]
pub type R = crate::R<Mcrc64RegsCrcBusySpec>;
#[doc = "Register `MCRC64_REGS_CRC_BUSY` writer"]
pub type W = crate::W<Mcrc64RegsCrcBusySpec>;
#[doc = "Field `CH1_BUSY` reader - 0:0\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
pub type Ch1BusyR = crate::BitReader;
#[doc = "Field `CH1_BUSY` writer - 0:0\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
pub type Ch1BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_BUSY` reader - 8:8\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
pub type Ch2BusyR = crate::BitReader;
#[doc = "Field `CH2_BUSY` writer - 8:8\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
pub type Ch2BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_BUSY` reader - 16:16\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
pub type Ch3BusyR = crate::BitReader;
#[doc = "Field `CH3_BUSY` writer - 16:16\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
pub type Ch3BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_BUSY` reader - 24:24\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
pub type Ch4BusyR = crate::BitReader;
#[doc = "Field `CH4_BUSY` writer - 24:24\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
pub type Ch4BusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
    #[inline(always)]
    pub fn ch1_busy(&self) -> Ch1BusyR {
        Ch1BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
    #[inline(always)]
    pub fn ch2_busy(&self) -> Ch2BusyR {
        Ch2BusyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
    #[inline(always)]
    pub fn ch3_busy(&self) -> Ch3BusyR {
        Ch3BusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
    #[inline(always)]
    pub fn ch4_busy(&self) -> Ch4BusyR {
        Ch4BusyR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_busy(&mut self) -> Ch1BusyW<Mcrc64RegsCrcBusySpec> {
        Ch1BusyW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_busy(&mut self) -> Ch2BusyW<Mcrc64RegsCrcBusySpec> {
        Ch2BusyW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_busy(&mut self) -> Ch3BusyW<Mcrc64RegsCrcBusySpec> {
        Ch3BusyW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_busy(&mut self) -> Ch4BusyW<Mcrc64RegsCrcBusySpec> {
        Ch4BusyW::new(self, 24)
    }
}
#[doc = "CRC Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_busy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_busy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcBusySpec;
impl crate::RegisterSpec for Mcrc64RegsCrcBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_busy::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcBusySpec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_busy::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcBusySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_BUSY to value 0"]
impl crate::Resettable for Mcrc64RegsCrcBusySpec {
    const RESET_VALUE: u32 = 0;
}
