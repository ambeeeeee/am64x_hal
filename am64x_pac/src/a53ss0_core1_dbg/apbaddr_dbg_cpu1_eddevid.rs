#[doc = "Register `APBADDR_DBG_CPU1_EDDEVID` reader"]
pub type R = crate::R<ApbaddrDbgCpu1EddevidSpec>;
#[doc = "Register `APBADDR_DBG_CPU1_EDDEVID` writer"]
pub type W = crate::W<ApbaddrDbgCpu1EddevidSpec>;
#[doc = "Field `PCSAMPLE` reader - 3:0\\]
Indicates the level of Sample-based profiling support using external debug registers 40 through 43. Permitted values of this field in v8-A are: 0000 Architecture-defined form of Sample-based profiling not implemented. 0010 EDPCSR and EDCIDSR are implemented \\[only permitted if EL3 and EL2 are not implemented\\]. 0011 EDPCSR, EDCIDSR, and EDVIDSR are implemented. All other values are reserved."]
pub type PcsampleR = crate::FieldReader;
#[doc = "Field `PCSAMPLE` writer - 3:0\\]
Indicates the level of Sample-based profiling support using external debug registers 40 through 43. Permitted values of this field in v8-A are: 0000 Architecture-defined form of Sample-based profiling not implemented. 0010 EDPCSR and EDCIDSR are implemented \\[only permitted if EL3 and EL2 are not implemented\\]. 0011 EDPCSR, EDCIDSR, and EDVIDSR are implemented. All other values are reserved."]
pub type PcsampleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_EDDEVID_23_4` reader - 23:4\\]
Reserved, RES0."]
pub type Res0Eddevid23_4R = crate::FieldReader<u32>;
#[doc = "Field `RES0_EDDEVID_23_4` writer - 23:4\\]
Reserved, RES0."]
pub type Res0Eddevid23_4W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `AUXREGS` reader - 27:24\\]
Indicates support for Auxiliary registers. Permitted values for this field are: 0000 None supported. 0001 Support for External Debug Auxiliary Control Register, EDACR. All other values are reserved."]
pub type AuxregsR = crate::FieldReader;
#[doc = "Field `AUXREGS` writer - 27:24\\]
Indicates support for Auxiliary registers. Permitted values for this field are: 0000 None supported. 0001 Support for External Debug Auxiliary Control Register, EDACR. All other values are reserved."]
pub type AuxregsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_EDDEVID_31_28` reader - 31:28\\]
Reserved, RES0."]
pub type Res0Eddevid31_28R = crate::FieldReader;
#[doc = "Field `RES0_EDDEVID_31_28` writer - 31:28\\]
Reserved, RES0."]
pub type Res0Eddevid31_28W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the level of Sample-based profiling support using external debug registers 40 through 43. Permitted values of this field in v8-A are: 0000 Architecture-defined form of Sample-based profiling not implemented. 0010 EDPCSR and EDCIDSR are implemented \\[only permitted if EL3 and EL2 are not implemented\\]. 0011 EDPCSR, EDCIDSR, and EDVIDSR are implemented. All other values are reserved."]
    #[inline(always)]
    pub fn pcsample(&self) -> PcsampleR {
        PcsampleR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - 23:4\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_eddevid_23_4(&self) -> Res0Eddevid23_4R {
        Res0Eddevid23_4R::new((self.bits >> 4) & 0x000f_ffff)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates support for Auxiliary registers. Permitted values for this field are: 0000 None supported. 0001 Support for External Debug Auxiliary Control Register, EDACR. All other values are reserved."]
    #[inline(always)]
    pub fn auxregs(&self) -> AuxregsR {
        AuxregsR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_eddevid_31_28(&self) -> Res0Eddevid31_28R {
        Res0Eddevid31_28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the level of Sample-based profiling support using external debug registers 40 through 43. Permitted values of this field in v8-A are: 0000 Architecture-defined form of Sample-based profiling not implemented. 0010 EDPCSR and EDCIDSR are implemented \\[only permitted if EL3 and EL2 are not implemented\\]. 0011 EDPCSR, EDCIDSR, and EDVIDSR are implemented. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn pcsample(&mut self) -> PcsampleW<ApbaddrDbgCpu1EddevidSpec> {
        PcsampleW::new(self, 0)
    }
    #[doc = "Bits 4:23 - 23:4\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_eddevid_23_4(&mut self) -> Res0Eddevid23_4W<ApbaddrDbgCpu1EddevidSpec> {
        Res0Eddevid23_4W::new(self, 4)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates support for Auxiliary registers. Permitted values for this field are: 0000 None supported. 0001 Support for External Debug Auxiliary Control Register, EDACR. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn auxregs(&mut self) -> AuxregsW<ApbaddrDbgCpu1EddevidSpec> {
        AuxregsW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_eddevid_31_28(&mut self) -> Res0Eddevid31_28W<ApbaddrDbgCpu1EddevidSpec> {
        Res0Eddevid31_28W::new(self, 28)
    }
}
#[doc = "External Debug Device ID Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_eddevid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_eddevid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1EddevidSpec;
impl crate::RegisterSpec for ApbaddrDbgCpu1EddevidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_eddevid::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1EddevidSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_eddevid::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1EddevidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_EDDEVID to value 0x03"]
impl crate::Resettable for ApbaddrDbgCpu1EddevidSpec {
    const RESET_VALUE: u32 = 0x03;
}
