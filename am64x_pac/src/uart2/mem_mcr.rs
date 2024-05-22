#[doc = "Register `MEM_MCR` reader"]
pub type R = crate::R<MemMcrSpec>;
#[doc = "Register `MEM_MCR` writer"]
pub type W = crate::W<MemMcrSpec>;
#[doc = "Field `DTR` reader - "]
pub type DtrR = crate::BitReader;
#[doc = "Field `DTR` writer - "]
pub type DtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS` reader - 1:1\\]
In loop back controls MSR\\[4\\]. If auto-RTS is enabled the RTS* output is controlled by hardware flow control."]
pub type RtsR = crate::BitReader;
#[doc = "Field `RTS` writer - 1:1\\]
In loop back controls MSR\\[4\\]. If auto-RTS is enabled the RTS* output is controlled by hardware flow control."]
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI_STS_CH` reader - "]
pub type RiStsChR = crate::BitReader;
#[doc = "Field `RI_STS_CH` writer - "]
pub type RiStsChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD_STS_CH` reader - "]
pub type CdStsChR = crate::BitReader;
#[doc = "Field `CD_STS_CH` writer - "]
pub type CdStsChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK_EN` reader - "]
pub type LoopbackEnR = crate::BitReader;
#[doc = "Field `LOOPBACK_EN` writer - "]
pub type LoopbackEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XON_EN` reader - "]
pub type XonEnR = crate::BitReader;
#[doc = "Field `XON_EN` writer - "]
pub type XonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCR_TLR` reader - "]
pub type TcrTlrR = crate::BitReader;
#[doc = "Field `TCR_TLR` writer - "]
pub type TcrTlrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dtr(&self) -> DtrR {
        DtrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
In loop back controls MSR\\[4\\]. If auto-RTS is enabled the RTS* output is controlled by hardware flow control."]
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ri_sts_ch(&self) -> RiStsChR {
        RiStsChR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cd_sts_ch(&self) -> CdStsChR {
        CdStsChR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn loopback_en(&self) -> LoopbackEnR {
        LoopbackEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xon_en(&self) -> XonEnR {
        XonEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tcr_tlr(&self) -> TcrTlrR {
        TcrTlrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dtr(&mut self) -> DtrW<MemMcrSpec> {
        DtrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
In loop back controls MSR\\[4\\]. If auto-RTS is enabled the RTS* output is controlled by hardware flow control."]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RtsW<MemMcrSpec> {
        RtsW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ri_sts_ch(&mut self) -> RiStsChW<MemMcrSpec> {
        RiStsChW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cd_sts_ch(&mut self) -> CdStsChW<MemMcrSpec> {
        CdStsChW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn loopback_en(&mut self) -> LoopbackEnW<MemMcrSpec> {
        LoopbackEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn xon_en(&mut self) -> XonEnW<MemMcrSpec> {
        XonEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tcr_tlr(&mut self) -> TcrTlrW<MemMcrSpec> {
        TcrTlrW::new(self, 6)
    }
}
#[doc = "MCR\\[3:0\\]
controls the interface with the modem, data set or peripheral device that is emulating the modem.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemMcrSpec;
impl crate::RegisterSpec for MemMcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_mcr::R`](R) reader structure"]
impl crate::Readable for MemMcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_mcr::W`](W) writer structure"]
impl crate::Writable for MemMcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_MCR to value 0"]
impl crate::Resettable for MemMcrSpec {
    const RESET_VALUE: u32 = 0;
}
