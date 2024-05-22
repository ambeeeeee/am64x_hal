#[doc = "Register `APBADDR_CTI_CPU1_CTILSR` reader"]
pub type R = crate::R<ApbaddrCtiCpu1CtilsrSpec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTILSR` writer"]
pub type W = crate::W<ApbaddrCtiCpu1CtilsrSpec>;
#[doc = "Field `SLI` reader - 0:0\\]
Software lock implemented. For an access to LSR that is not a memory-mapped access, this field is RAZ. For memory-mapped accesses, the value of this field is IMPLEMENTATION DEFINED. Permitted values are: 0 Software lock not implemented or not memory-mapped access. 1 Software lock implemented and memory-mapped access."]
pub type SliR = crate::BitReader;
#[doc = "Field `SLI` writer - 0:0\\]
Software lock implemented. For an access to LSR that is not a memory-mapped access, this field is RAZ. For memory-mapped accesses, the value of this field is IMPLEMENTATION DEFINED. Permitted values are: 0 Software lock not implemented or not memory-mapped access. 1 Software lock implemented and memory-mapped access."]
pub type SliW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLK` reader - 1:1\\]
Software lock status for this component. For an access to LSR that is not a memory-mapped access, or when the software lock is not implemented, this field is RES0.For memory-mapped accesses when the software lock is implemented, possible values of this field are: 0 Lock clear. Writes are permitted to this component's registers. 1 Lock set. Writes to this component's registers are ignored, and reads have no side effects."]
pub type SlkR = crate::BitReader;
#[doc = "Field `SLK` writer - 1:1\\]
Software lock status for this component. For an access to LSR that is not a memory-mapped access, or when the software lock is not implemented, this field is RES0.For memory-mapped accesses when the software lock is implemented, possible values of this field are: 0 Lock clear. Writes are permitted to this component's registers. 1 Lock set. Writes to this component's registers are ignored, and reads have no side effects."]
pub type SlkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NTT` reader - 2:2\\]
Not thirty-two bit access required. RAZ."]
pub type NttR = crate::BitReader;
#[doc = "Field `NTT` writer - 2:2\\]
Not thirty-two bit access required. RAZ."]
pub type NttW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_CTILSR_31_3` reader - 31:3\\]
Reserved, RES0."]
pub type Res0Ctilsr31_3R = crate::FieldReader<u32>;
#[doc = "Field `RES0_CTILSR_31_3` writer - 31:3\\]
Reserved, RES0."]
pub type Res0Ctilsr31_3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software lock implemented. For an access to LSR that is not a memory-mapped access, this field is RAZ. For memory-mapped accesses, the value of this field is IMPLEMENTATION DEFINED. Permitted values are: 0 Software lock not implemented or not memory-mapped access. 1 Software lock implemented and memory-mapped access."]
    #[inline(always)]
    pub fn sli(&self) -> SliR {
        SliR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software lock status for this component. For an access to LSR that is not a memory-mapped access, or when the software lock is not implemented, this field is RES0.For memory-mapped accesses when the software lock is implemented, possible values of this field are: 0 Lock clear. Writes are permitted to this component's registers. 1 Lock set. Writes to this component's registers are ignored, and reads have no side effects."]
    #[inline(always)]
    pub fn slk(&self) -> SlkR {
        SlkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Not thirty-two bit access required. RAZ."]
    #[inline(always)]
    pub fn ntt(&self) -> NttR {
        NttR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_ctilsr_31_3(&self) -> Res0Ctilsr31_3R {
        Res0Ctilsr31_3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software lock implemented. For an access to LSR that is not a memory-mapped access, this field is RAZ. For memory-mapped accesses, the value of this field is IMPLEMENTATION DEFINED. Permitted values are: 0 Software lock not implemented or not memory-mapped access. 1 Software lock implemented and memory-mapped access."]
    #[inline(always)]
    #[must_use]
    pub fn sli(&mut self) -> SliW<ApbaddrCtiCpu1CtilsrSpec> {
        SliW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software lock status for this component. For an access to LSR that is not a memory-mapped access, or when the software lock is not implemented, this field is RES0.For memory-mapped accesses when the software lock is implemented, possible values of this field are: 0 Lock clear. Writes are permitted to this component's registers. 1 Lock set. Writes to this component's registers are ignored, and reads have no side effects."]
    #[inline(always)]
    #[must_use]
    pub fn slk(&mut self) -> SlkW<ApbaddrCtiCpu1CtilsrSpec> {
        SlkW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Not thirty-two bit access required. RAZ."]
    #[inline(always)]
    #[must_use]
    pub fn ntt(&mut self) -> NttW<ApbaddrCtiCpu1CtilsrSpec> {
        NttW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_ctilsr_31_3(&mut self) -> Res0Ctilsr31_3W<ApbaddrCtiCpu1CtilsrSpec> {
        Res0Ctilsr31_3W::new(self, 3)
    }
}
#[doc = "CTI Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctilsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctilsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1CtilsrSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu1CtilsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_ctilsr::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1CtilsrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_ctilsr::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1CtilsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTILSR to value 0"]
impl crate::Resettable for ApbaddrCtiCpu1CtilsrSpec {
    const RESET_VALUE: u32 = 0;
}
