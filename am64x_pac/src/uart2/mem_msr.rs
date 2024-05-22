#[doc = "Register `MEM_MSR` reader"]
pub type R = crate::R<MemMsrSpec>;
#[doc = "Register `MEM_MSR` writer"]
pub type W = crate::W<MemMsrSpec>;
#[doc = "Field `CTS_STS` reader - "]
pub type CtsStsR = crate::BitReader;
#[doc = "Field `CTS_STS` writer - "]
pub type CtsStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR_STS` reader - "]
pub type DsrStsR = crate::BitReader;
#[doc = "Field `DSR_STS` writer - "]
pub type DsrStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI_STS` reader - 2:2\\]
Indicates that RI* input \\[or MCR\\[2\\]
in loop back\\]
has changed state from low to high. Cleared on a read."]
pub type RiStsR = crate::BitReader;
#[doc = "Field `RI_STS` writer - 2:2\\]
Indicates that RI* input \\[or MCR\\[2\\]
in loop back\\]
has changed state from low to high. Cleared on a read."]
pub type RiStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCD_STS` reader - 3:3\\]
Indicates that DCD* input \\[or MCR\\[3\\]
in loop back\\]
has changed. Cleared on a read."]
pub type DcdStsR = crate::BitReader;
#[doc = "Field `DCD_STS` writer - 3:3\\]
Indicates that DCD* input \\[or MCR\\[3\\]
in loop back\\]
has changed. Cleared on a read."]
pub type DcdStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NCTS_STS` reader - 4:4\\]
This bit is the complement of the CTS* input. In loop-back mode it is equivalent to MCR\\[1\\]"]
pub type NctsStsR = crate::BitReader;
#[doc = "Field `NCTS_STS` writer - 4:4\\]
This bit is the complement of the CTS* input. In loop-back mode it is equivalent to MCR\\[1\\]"]
pub type NctsStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDSR_STS` reader - 5:5\\]
This bit is the complement of the DSR* input. In loop-back mode, it is equivalent to MCR\\[0\\]"]
pub type NdsrStsR = crate::BitReader;
#[doc = "Field `NDSR_STS` writer - 5:5\\]
This bit is the complement of the DSR* input. In loop-back mode, it is equivalent to MCR\\[0\\]"]
pub type NdsrStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRI_STS` reader - 6:6\\]
This bit is the complement of the RI* input. In loop-back mode it is equivalent to MCR\\[2\\]"]
pub type NriStsR = crate::BitReader;
#[doc = "Field `NRI_STS` writer - 6:6\\]
This bit is the complement of the RI* input. In loop-back mode it is equivalent to MCR\\[2\\]"]
pub type NriStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NCD_STS` reader - 7:7\\]
This bit is the complement of the DCD* input. In loop-back mode it is equivalent to MCR\\[3\\]"]
pub type NcdStsR = crate::BitReader;
#[doc = "Field `NCD_STS` writer - 7:7\\]
This bit is the complement of the DCD* input. In loop-back mode it is equivalent to MCR\\[3\\]"]
pub type NcdStsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cts_sts(&self) -> CtsStsR {
        CtsStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dsr_sts(&self) -> DsrStsR {
        DsrStsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates that RI* input \\[or MCR\\[2\\]
in loop back\\]
has changed state from low to high. Cleared on a read."]
    #[inline(always)]
    pub fn ri_sts(&self) -> RiStsR {
        RiStsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates that DCD* input \\[or MCR\\[3\\]
in loop back\\]
has changed. Cleared on a read."]
    #[inline(always)]
    pub fn dcd_sts(&self) -> DcdStsR {
        DcdStsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
This bit is the complement of the CTS* input. In loop-back mode it is equivalent to MCR\\[1\\]"]
    #[inline(always)]
    pub fn ncts_sts(&self) -> NctsStsR {
        NctsStsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit is the complement of the DSR* input. In loop-back mode, it is equivalent to MCR\\[0\\]"]
    #[inline(always)]
    pub fn ndsr_sts(&self) -> NdsrStsR {
        NdsrStsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
This bit is the complement of the RI* input. In loop-back mode it is equivalent to MCR\\[2\\]"]
    #[inline(always)]
    pub fn nri_sts(&self) -> NriStsR {
        NriStsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
This bit is the complement of the DCD* input. In loop-back mode it is equivalent to MCR\\[3\\]"]
    #[inline(always)]
    pub fn ncd_sts(&self) -> NcdStsR {
        NcdStsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cts_sts(&mut self) -> CtsStsW<MemMsrSpec> {
        CtsStsW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dsr_sts(&mut self) -> DsrStsW<MemMsrSpec> {
        DsrStsW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates that RI* input \\[or MCR\\[2\\]
in loop back\\]
has changed state from low to high. Cleared on a read."]
    #[inline(always)]
    #[must_use]
    pub fn ri_sts(&mut self) -> RiStsW<MemMsrSpec> {
        RiStsW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates that DCD* input \\[or MCR\\[3\\]
in loop back\\]
has changed. Cleared on a read."]
    #[inline(always)]
    #[must_use]
    pub fn dcd_sts(&mut self) -> DcdStsW<MemMsrSpec> {
        DcdStsW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
This bit is the complement of the CTS* input. In loop-back mode it is equivalent to MCR\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ncts_sts(&mut self) -> NctsStsW<MemMsrSpec> {
        NctsStsW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit is the complement of the DSR* input. In loop-back mode, it is equivalent to MCR\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ndsr_sts(&mut self) -> NdsrStsW<MemMsrSpec> {
        NdsrStsW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
This bit is the complement of the RI* input. In loop-back mode it is equivalent to MCR\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn nri_sts(&mut self) -> NriStsW<MemMsrSpec> {
        NriStsW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
This bit is the complement of the DCD* input. In loop-back mode it is equivalent to MCR\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ncd_sts(&mut self) -> NcdStsW<MemMsrSpec> {
        NcdStsW::new(self, 7)
    }
}
#[doc = "This register provides information about the current state of the control lines from the modem, data set or peripheral device to the LH. It also indicates when a control input from the modem changes state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_msr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_msr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemMsrSpec;
impl crate::RegisterSpec for MemMsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_msr::R`](R) reader structure"]
impl crate::Readable for MemMsrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_msr::W`](W) writer structure"]
impl crate::Writable for MemMsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_MSR to value 0"]
impl crate::Resettable for MemMsrSpec {
    const RESET_VALUE: u32 = 0;
}
