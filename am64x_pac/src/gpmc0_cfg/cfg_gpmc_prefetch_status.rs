#[doc = "Register `CFG_GPMC_PREFETCH_STATUS` reader"]
pub type R = crate::R<CfgGpmcPrefetchStatusSpec>;
#[doc = "Register `CFG_GPMC_PREFETCH_STATUS` writer"]
pub type W = crate::W<CfgGpmcPrefetchStatusSpec>;
#[doc = "Field `COUNTVALUE` reader - 13:0\\]
Number of remaining bytes to be read or to be written by the engine according to the TransferCount value \\[0x0000 corresponds to 0 byte remaining to be read or to be written, 0x0001 corresponds to 1 byte remaining to be read or to be written, &amp;, 0x2000 corresponds to 8 Kbytes remaining to be read or to be written\\]"]
pub type CountvalueR = crate::FieldReader<u16>;
#[doc = "Field `COUNTVALUE` writer - 13:0\\]
Number of remaining bytes to be read or to be written by the engine according to the TransferCount value \\[0x0000 corresponds to 0 byte remaining to be read or to be written, 0x0001 corresponds to 1 byte remaining to be read or to be written, &amp;, 0x2000 corresponds to 8 Kbytes remaining to be read or to be written\\]"]
pub type CountvalueW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `FIFOTHRESHOLDSTATUS` reader - 16:16\\]
Set when FIFOPointer exceeds FIFOThreshold value"]
pub type FifothresholdstatusR = crate::BitReader;
#[doc = "Field `FIFOTHRESHOLDSTATUS` writer - 16:16\\]
Set when FIFOPointer exceeds FIFOThreshold value"]
pub type FifothresholdstatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOPOINTER` reader - 30:24\\]
Number of available bytes to be read or number of free empty byte places to be written \\[0x00 corresponds to 0 byte available to be read or 0 free empty place to be written, &amp;, 0x40 corresponds to 64 bytes available to be read or 64 empty places to be written\\]"]
pub type FifopointerR = crate::FieldReader;
#[doc = "Field `FIFOPOINTER` writer - 30:24\\]
Number of available bytes to be read or number of free empty byte places to be written \\[0x00 corresponds to 0 byte available to be read or 0 free empty place to be written, &amp;, 0x40 corresponds to 64 bytes available to be read or 64 empty places to be written\\]"]
pub type FifopointerW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Number of remaining bytes to be read or to be written by the engine according to the TransferCount value \\[0x0000 corresponds to 0 byte remaining to be read or to be written, 0x0001 corresponds to 1 byte remaining to be read or to be written, &amp;, 0x2000 corresponds to 8 Kbytes remaining to be read or to be written\\]"]
    #[inline(always)]
    pub fn countvalue(&self) -> CountvalueR {
        CountvalueR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Set when FIFOPointer exceeds FIFOThreshold value"]
    #[inline(always)]
    pub fn fifothresholdstatus(&self) -> FifothresholdstatusR {
        FifothresholdstatusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Number of available bytes to be read or number of free empty byte places to be written \\[0x00 corresponds to 0 byte available to be read or 0 free empty place to be written, &amp;, 0x40 corresponds to 64 bytes available to be read or 64 empty places to be written\\]"]
    #[inline(always)]
    pub fn fifopointer(&self) -> FifopointerR {
        FifopointerR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Number of remaining bytes to be read or to be written by the engine according to the TransferCount value \\[0x0000 corresponds to 0 byte remaining to be read or to be written, 0x0001 corresponds to 1 byte remaining to be read or to be written, &amp;, 0x2000 corresponds to 8 Kbytes remaining to be read or to be written\\]"]
    #[inline(always)]
    #[must_use]
    pub fn countvalue(&mut self) -> CountvalueW<CfgGpmcPrefetchStatusSpec> {
        CountvalueW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Set when FIFOPointer exceeds FIFOThreshold value"]
    #[inline(always)]
    #[must_use]
    pub fn fifothresholdstatus(&mut self) -> FifothresholdstatusW<CfgGpmcPrefetchStatusSpec> {
        FifothresholdstatusW::new(self, 16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Number of available bytes to be read or number of free empty byte places to be written \\[0x00 corresponds to 0 byte available to be read or 0 free empty place to be written, &amp;, 0x40 corresponds to 64 bytes available to be read or 64 empty places to be written\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fifopointer(&mut self) -> FifopointerW<CfgGpmcPrefetchStatusSpec> {
        FifopointerW::new(self, 24)
    }
}
#[doc = "Prefetch engine status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_prefetch_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_prefetch_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcPrefetchStatusSpec;
impl crate::RegisterSpec for CfgGpmcPrefetchStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_prefetch_status::R`](R) reader structure"]
impl crate::Readable for CfgGpmcPrefetchStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_prefetch_status::W`](W) writer structure"]
impl crate::Writable for CfgGpmcPrefetchStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_PREFETCH_STATUS to value 0"]
impl crate::Resettable for CfgGpmcPrefetchStatusSpec {
    const RESET_VALUE: u32 = 0;
}
