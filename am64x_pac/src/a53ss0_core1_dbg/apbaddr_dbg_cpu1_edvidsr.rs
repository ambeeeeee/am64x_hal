#[doc = "Register `APBADDR_DBG_CPU1_EDVIDSR` reader"]
pub type R = crate::R<ApbaddrDbgCpu1EdvidsrSpec>;
#[doc = "Register `APBADDR_DBG_CPU1_EDVIDSR` writer"]
pub type W = crate::W<ApbaddrDbgCpu1EdvidsrSpec>;
#[doc = "Field `VMID` reader - 7:0\\]
VMID sample. The value of VTTBR_EL2.VMID associated with the most recent EDPCSR sample. If EDVIDSR.NS == 0 or EDVIDSR.E2 == 1, this field is RAZ."]
pub type VmidR = crate::FieldReader;
#[doc = "Field `VMID` writer - 7:0\\]
VMID sample. The value of VTTBR_EL2.VMID associated with the most recent EDPCSR sample. If EDVIDSR.NS == 0 or EDVIDSR.E2 == 1, this field is RAZ."]
pub type VmidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RES0_EDVIDSR_27_8` reader - 27:8\\]
Reserved, RES0."]
pub type Res0Edvidsr27_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_EDVIDSR_27_8` writer - 27:8\\]
Reserved, RES0."]
pub type Res0Edvidsr27_8W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `HV` reader - 28:28\\]
EDPCSR high half valid. Indicates whether bits \\[63_32\\]
of the most recent EDPCSR sample are valid. If EDVIDSR.HV == 0, the value of EDPCSR\\[63_32\\]
is RAZ."]
pub type HvR = crate::BitReader;
#[doc = "Field `HV` writer - 28:28\\]
EDPCSR high half valid. Indicates whether bits \\[63_32\\]
of the most recent EDPCSR sample are valid. If EDVIDSR.HV == 0, the value of EDPCSR\\[63_32\\]
is RAZ."]
pub type HvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E3` reader - 29:29\\]
Exception level 3 status sample. Indicates whether the most recent EDPCSR sample was associated with AArch64 EL3. If EDVIDSR.NS == 1 or the processor was in AArch32 state when EDPCSR was read, this bit is 0."]
pub type E3R = crate::BitReader;
#[doc = "Field `E3` writer - 29:29\\]
Exception level 3 status sample. Indicates whether the most recent EDPCSR sample was associated with AArch64 EL3. If EDVIDSR.NS == 1 or the processor was in AArch32 state when EDPCSR was read, this bit is 0."]
pub type E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E2` reader - 30:30\\]
Exception level 2 status sample. Indicates whether the most recent EDPCSR sample was associated with EL2. If EDVIDSR.NS == 0, this bit is 0."]
pub type E2R = crate::BitReader;
#[doc = "Field `E2` writer - 30:30\\]
Exception level 2 status sample. Indicates whether the most recent EDPCSR sample was associated with EL2. If EDVIDSR.NS == 0, this bit is 0."]
pub type E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NS` reader - 31:31\\]
Non-secure state sample. Indicates the security state associated with the most recent EDPCSR sample."]
pub type NsR = crate::BitReader;
#[doc = "Field `NS` writer - 31:31\\]
Non-secure state sample. Indicates the security state associated with the most recent EDPCSR sample."]
pub type NsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
VMID sample. The value of VTTBR_EL2.VMID associated with the most recent EDPCSR sample. If EDVIDSR.NS == 0 or EDVIDSR.E2 == 1, this field is RAZ."]
    #[inline(always)]
    pub fn vmid(&self) -> VmidR {
        VmidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:27 - 27:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edvidsr_27_8(&self) -> Res0Edvidsr27_8R {
        Res0Edvidsr27_8R::new((self.bits >> 8) & 0x000f_ffff)
    }
    #[doc = "Bit 28 - 28:28\\]
EDPCSR high half valid. Indicates whether bits \\[63_32\\]
of the most recent EDPCSR sample are valid. If EDVIDSR.HV == 0, the value of EDPCSR\\[63_32\\]
is RAZ."]
    #[inline(always)]
    pub fn hv(&self) -> HvR {
        HvR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Exception level 3 status sample. Indicates whether the most recent EDPCSR sample was associated with AArch64 EL3. If EDVIDSR.NS == 1 or the processor was in AArch32 state when EDPCSR was read, this bit is 0."]
    #[inline(always)]
    pub fn e3(&self) -> E3R {
        E3R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Exception level 2 status sample. Indicates whether the most recent EDPCSR sample was associated with EL2. If EDVIDSR.NS == 0, this bit is 0."]
    #[inline(always)]
    pub fn e2(&self) -> E2R {
        E2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Non-secure state sample. Indicates the security state associated with the most recent EDPCSR sample."]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
VMID sample. The value of VTTBR_EL2.VMID associated with the most recent EDPCSR sample. If EDVIDSR.NS == 0 or EDVIDSR.E2 == 1, this field is RAZ."]
    #[inline(always)]
    #[must_use]
    pub fn vmid(&mut self) -> VmidW<ApbaddrDbgCpu1EdvidsrSpec> {
        VmidW::new(self, 0)
    }
    #[doc = "Bits 8:27 - 27:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edvidsr_27_8(&mut self) -> Res0Edvidsr27_8W<ApbaddrDbgCpu1EdvidsrSpec> {
        Res0Edvidsr27_8W::new(self, 8)
    }
    #[doc = "Bit 28 - 28:28\\]
EDPCSR high half valid. Indicates whether bits \\[63_32\\]
of the most recent EDPCSR sample are valid. If EDVIDSR.HV == 0, the value of EDPCSR\\[63_32\\]
is RAZ."]
    #[inline(always)]
    #[must_use]
    pub fn hv(&mut self) -> HvW<ApbaddrDbgCpu1EdvidsrSpec> {
        HvW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Exception level 3 status sample. Indicates whether the most recent EDPCSR sample was associated with AArch64 EL3. If EDVIDSR.NS == 1 or the processor was in AArch32 state when EDPCSR was read, this bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn e3(&mut self) -> E3W<ApbaddrDbgCpu1EdvidsrSpec> {
        E3W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Exception level 2 status sample. Indicates whether the most recent EDPCSR sample was associated with EL2. If EDVIDSR.NS == 0, this bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn e2(&mut self) -> E2W<ApbaddrDbgCpu1EdvidsrSpec> {
        E2W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Non-secure state sample. Indicates the security state associated with the most recent EDPCSR sample."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NsW<ApbaddrDbgCpu1EdvidsrSpec> {
        NsW::new(self, 31)
    }
}
#[doc = "External Debug Virtual Context Sample Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edvidsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edvidsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1EdvidsrSpec;
impl crate::RegisterSpec for ApbaddrDbgCpu1EdvidsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_edvidsr::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1EdvidsrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_edvidsr::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1EdvidsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_EDVIDSR to value 0"]
impl crate::Resettable for ApbaddrDbgCpu1EdvidsrSpec {
    const RESET_VALUE: u32 = 0;
}
