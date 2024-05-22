#[doc = "Register `CFG_GPMC_PREFETCH_CONFIG1` reader"]
pub type R = crate::R<CfgGpmcPrefetchConfig1Spec>;
#[doc = "Register `CFG_GPMC_PREFETCH_CONFIG1` writer"]
pub type W = crate::W<CfgGpmcPrefetchConfig1Spec>;
#[doc = "Field `ACCESSMODE` reader - 0:0\\]
Selects pre-fetch read or write posting accesses"]
pub type AccessmodeR = crate::BitReader;
#[doc = "Field `ACCESSMODE` writer - 0:0\\]
Selects pre-fetch read or write posting accesses"]
pub type AccessmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMODE` reader - 2:2\\]
Selects interrupt synchronization or DMA request synchronization"]
pub type DmamodeR = crate::BitReader;
#[doc = "Field `DMAMODE` writer - 2:2\\]
Selects interrupt synchronization or DMA request synchronization"]
pub type DmamodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCHROMODE` reader - 3:3\\]
Selects when the engine starts the access to CS"]
pub type SynchromodeR = crate::BitReader;
#[doc = "Field `SYNCHROMODE` writer - 3:3\\]
Selects when the engine starts the access to CS"]
pub type SynchromodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITPINSELECTOR` reader - 5:4\\]
Select which wait pin edge detector should start the engine in synchronized mode"]
pub type WaitpinselectorR = crate::FieldReader;
#[doc = "Field `WAITPINSELECTOR` writer - 5:4\\]
Select which wait pin edge detector should start the engine in synchronized mode"]
pub type WaitpinselectorW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENABLEENGINE` reader - 7:7\\]
Enables the Prefetch Postwite engine"]
pub type EnableengineR = crate::BitReader;
#[doc = "Field `ENABLEENGINE` writer - 7:7\\]
Enables the Prefetch Postwite engine"]
pub type EnableengineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOTHRESHOLD` reader - 14:8\\]
Selects the maximum number of bytes read from the FIFO or written to the FIFO by the host on a DMA or interrupt request \\[0x00 corresponds to 0 byte, 0x01 corresponds to 1 byte, &amp;, 0x40 corresponds to 64 bytes\\]"]
pub type FifothresholdR = crate::FieldReader;
#[doc = "Field `FIFOTHRESHOLD` writer - 14:8\\]
Selects the maximum number of bytes read from the FIFO or written to the FIFO by the host on a DMA or interrupt request \\[0x00 corresponds to 0 byte, 0x01 corresponds to 1 byte, &amp;, 0x40 corresponds to 64 bytes\\]"]
pub type FifothresholdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PFPWWEIGHTEDPRIO` reader - 19:16\\]
When an arbitration occurs between a direct memory access and a PFPW engine access, the direct memory access is always serviced. If the PFPWEnRoundRobin is enabled, 0x0 means : the next access is granted to the PFPW engine, 0x1 means : the two next accesses are granted to the PFPW engine, ..., 0xF means : the 16 next accesses are granted to the PFPW engine."]
pub type PfpwweightedprioR = crate::FieldReader;
#[doc = "Field `PFPWWEIGHTEDPRIO` writer - 19:16\\]
When an arbitration occurs between a direct memory access and a PFPW engine access, the direct memory access is always serviced. If the PFPWEnRoundRobin is enabled, 0x0 means : the next access is granted to the PFPW engine, 0x1 means : the two next accesses are granted to the PFPW engine, ..., 0xF means : the 16 next accesses are granted to the PFPW engine."]
pub type PfpwweightedprioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PFPWENROUNDROBIN` reader - 23:23\\]
Enables the PFPW RoundRobin arbitration"]
pub type PfpwenroundrobinR = crate::BitReader;
#[doc = "Field `PFPWENROUNDROBIN` writer - 23:23\\]
Enables the PFPW RoundRobin arbitration"]
pub type PfpwenroundrobinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENGINECSSELECTOR` reader - 26:24\\]
Selects the CS where Prefetch Postwrite engine is active \\[0x0 corresponds toCS0, 0x1 corresponds to CS1, &amp;, 0x7 corresponds to CS7\\]"]
pub type EnginecsselectorR = crate::FieldReader;
#[doc = "Field `ENGINECSSELECTOR` writer - 26:24\\]
Selects the CS where Prefetch Postwrite engine is active \\[0x0 corresponds toCS0, 0x1 corresponds to CS1, &amp;, 0x7 corresponds to CS7\\]"]
pub type EnginecsselectorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ENABLEOPTIMIZEDACCESS` reader - 27:27\\]
Enables access cycle optimization"]
pub type EnableoptimizedaccessR = crate::BitReader;
#[doc = "Field `ENABLEOPTIMIZEDACCESS` writer - 27:27\\]
Enables access cycle optimization"]
pub type EnableoptimizedaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CYCLEOPTIMIZATION` reader - 30:28\\]
Define the number of GPMC.FCLK cycles to be substracted from RdCycleTime, WrCycleTime, AccessTime, CSRdOffTime, CSWrOffTime, ADVRdOffTime, ADVWrOffTime, OEOffTime, WEOffTime \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x7 corresponds to 7 GPMC.FCLK cycles\\]"]
pub type CycleoptimizationR = crate::FieldReader;
#[doc = "Field `CYCLEOPTIMIZATION` writer - 30:28\\]
Define the number of GPMC.FCLK cycles to be substracted from RdCycleTime, WrCycleTime, AccessTime, CSRdOffTime, CSWrOffTime, ADVRdOffTime, ADVWrOffTime, OEOffTime, WEOffTime \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x7 corresponds to 7 GPMC.FCLK cycles\\]"]
pub type CycleoptimizationW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects pre-fetch read or write posting accesses"]
    #[inline(always)]
    pub fn accessmode(&self) -> AccessmodeR {
        AccessmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Selects interrupt synchronization or DMA request synchronization"]
    #[inline(always)]
    pub fn dmamode(&self) -> DmamodeR {
        DmamodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Selects when the engine starts the access to CS"]
    #[inline(always)]
    pub fn synchromode(&self) -> SynchromodeR {
        SynchromodeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select which wait pin edge detector should start the engine in synchronized mode"]
    #[inline(always)]
    pub fn waitpinselector(&self) -> WaitpinselectorR {
        WaitpinselectorR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Enables the Prefetch Postwite engine"]
    #[inline(always)]
    pub fn enableengine(&self) -> EnableengineR {
        EnableengineR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Selects the maximum number of bytes read from the FIFO or written to the FIFO by the host on a DMA or interrupt request \\[0x00 corresponds to 0 byte, 0x01 corresponds to 1 byte, &amp;, 0x40 corresponds to 64 bytes\\]"]
    #[inline(always)]
    pub fn fifothreshold(&self) -> FifothresholdR {
        FifothresholdR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
When an arbitration occurs between a direct memory access and a PFPW engine access, the direct memory access is always serviced. If the PFPWEnRoundRobin is enabled, 0x0 means : the next access is granted to the PFPW engine, 0x1 means : the two next accesses are granted to the PFPW engine, ..., 0xF means : the 16 next accesses are granted to the PFPW engine."]
    #[inline(always)]
    pub fn pfpwweightedprio(&self) -> PfpwweightedprioR {
        PfpwweightedprioR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Enables the PFPW RoundRobin arbitration"]
    #[inline(always)]
    pub fn pfpwenroundrobin(&self) -> PfpwenroundrobinR {
        PfpwenroundrobinR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Selects the CS where Prefetch Postwrite engine is active \\[0x0 corresponds toCS0, 0x1 corresponds to CS1, &amp;, 0x7 corresponds to CS7\\]"]
    #[inline(always)]
    pub fn enginecsselector(&self) -> EnginecsselectorR {
        EnginecsselectorR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
Enables access cycle optimization"]
    #[inline(always)]
    pub fn enableoptimizedaccess(&self) -> EnableoptimizedaccessR {
        EnableoptimizedaccessR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Define the number of GPMC.FCLK cycles to be substracted from RdCycleTime, WrCycleTime, AccessTime, CSRdOffTime, CSWrOffTime, ADVRdOffTime, ADVWrOffTime, OEOffTime, WEOffTime \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x7 corresponds to 7 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn cycleoptimization(&self) -> CycleoptimizationR {
        CycleoptimizationR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects pre-fetch read or write posting accesses"]
    #[inline(always)]
    #[must_use]
    pub fn accessmode(&mut self) -> AccessmodeW<CfgGpmcPrefetchConfig1Spec> {
        AccessmodeW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Selects interrupt synchronization or DMA request synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn dmamode(&mut self) -> DmamodeW<CfgGpmcPrefetchConfig1Spec> {
        DmamodeW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Selects when the engine starts the access to CS"]
    #[inline(always)]
    #[must_use]
    pub fn synchromode(&mut self) -> SynchromodeW<CfgGpmcPrefetchConfig1Spec> {
        SynchromodeW::new(self, 3)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select which wait pin edge detector should start the engine in synchronized mode"]
    #[inline(always)]
    #[must_use]
    pub fn waitpinselector(&mut self) -> WaitpinselectorW<CfgGpmcPrefetchConfig1Spec> {
        WaitpinselectorW::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
Enables the Prefetch Postwite engine"]
    #[inline(always)]
    #[must_use]
    pub fn enableengine(&mut self) -> EnableengineW<CfgGpmcPrefetchConfig1Spec> {
        EnableengineW::new(self, 7)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Selects the maximum number of bytes read from the FIFO or written to the FIFO by the host on a DMA or interrupt request \\[0x00 corresponds to 0 byte, 0x01 corresponds to 1 byte, &amp;, 0x40 corresponds to 64 bytes\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fifothreshold(&mut self) -> FifothresholdW<CfgGpmcPrefetchConfig1Spec> {
        FifothresholdW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
When an arbitration occurs between a direct memory access and a PFPW engine access, the direct memory access is always serviced. If the PFPWEnRoundRobin is enabled, 0x0 means : the next access is granted to the PFPW engine, 0x1 means : the two next accesses are granted to the PFPW engine, ..., 0xF means : the 16 next accesses are granted to the PFPW engine."]
    #[inline(always)]
    #[must_use]
    pub fn pfpwweightedprio(&mut self) -> PfpwweightedprioW<CfgGpmcPrefetchConfig1Spec> {
        PfpwweightedprioW::new(self, 16)
    }
    #[doc = "Bit 23 - 23:23\\]
Enables the PFPW RoundRobin arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn pfpwenroundrobin(&mut self) -> PfpwenroundrobinW<CfgGpmcPrefetchConfig1Spec> {
        PfpwenroundrobinW::new(self, 23)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Selects the CS where Prefetch Postwrite engine is active \\[0x0 corresponds toCS0, 0x1 corresponds to CS1, &amp;, 0x7 corresponds to CS7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn enginecsselector(&mut self) -> EnginecsselectorW<CfgGpmcPrefetchConfig1Spec> {
        EnginecsselectorW::new(self, 24)
    }
    #[doc = "Bit 27 - 27:27\\]
Enables access cycle optimization"]
    #[inline(always)]
    #[must_use]
    pub fn enableoptimizedaccess(&mut self) -> EnableoptimizedaccessW<CfgGpmcPrefetchConfig1Spec> {
        EnableoptimizedaccessW::new(self, 27)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Define the number of GPMC.FCLK cycles to be substracted from RdCycleTime, WrCycleTime, AccessTime, CSRdOffTime, CSWrOffTime, ADVRdOffTime, ADVWrOffTime, OEOffTime, WEOffTime \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x7 corresponds to 7 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cycleoptimization(&mut self) -> CycleoptimizationW<CfgGpmcPrefetchConfig1Spec> {
        CycleoptimizationW::new(self, 28)
    }
}
#[doc = "Prefetch engine configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_prefetch_config1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_prefetch_config1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcPrefetchConfig1Spec;
impl crate::RegisterSpec for CfgGpmcPrefetchConfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_prefetch_config1::R`](R) reader structure"]
impl crate::Readable for CfgGpmcPrefetchConfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_prefetch_config1::W`](W) writer structure"]
impl crate::Writable for CfgGpmcPrefetchConfig1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_PREFETCH_CONFIG1 to value 0x6400"]
impl crate::Resettable for CfgGpmcPrefetchConfig1Spec {
    const RESET_VALUE: u32 = 0x6400;
}
