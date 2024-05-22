#[doc = "Register `APBADDR_DBG_CPU1_EDDEVID1` reader"]
pub type R = crate::R<ApbaddrDbgCpu1Eddevid1Spec>;
#[doc = "Register `APBADDR_DBG_CPU1_EDDEVID1` writer"]
pub type W = crate::W<ApbaddrDbgCpu1Eddevid1Spec>;
#[doc = "Field `PCSROFFSET` reader - 3:0\\]
This field indicates the offset applied to PC samples returned by reads of EDPCSR. Permitted values of this field in v8-A are: 0000 EDPCSR not implemented. 0010 EDPCSR implemented, and samples have no offset applied and do not sample the instruction set state in AArch32 state."]
pub type PcsroffsetR = crate::FieldReader;
#[doc = "Field `PCSROFFSET` writer - 3:0\\]
This field indicates the offset applied to PC samples returned by reads of EDPCSR. Permitted values of this field in v8-A are: 0000 EDPCSR not implemented. 0010 EDPCSR implemented, and samples have no offset applied and do not sample the instruction set state in AArch32 state."]
pub type PcsroffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_EDDEVID1_31_4` reader - 31:4\\]
Reserved, RES0."]
pub type Res0Eddevid1_31_4R = crate::FieldReader<u32>;
#[doc = "Field `RES0_EDDEVID1_31_4` writer - 31:4\\]
Reserved, RES0."]
pub type Res0Eddevid1_31_4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This field indicates the offset applied to PC samples returned by reads of EDPCSR. Permitted values of this field in v8-A are: 0000 EDPCSR not implemented. 0010 EDPCSR implemented, and samples have no offset applied and do not sample the instruction set state in AArch32 state."]
    #[inline(always)]
    pub fn pcsroffset(&self) -> PcsroffsetR {
        PcsroffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_eddevid1_31_4(&self) -> Res0Eddevid1_31_4R {
        Res0Eddevid1_31_4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This field indicates the offset applied to PC samples returned by reads of EDPCSR. Permitted values of this field in v8-A are: 0000 EDPCSR not implemented. 0010 EDPCSR implemented, and samples have no offset applied and do not sample the instruction set state in AArch32 state."]
    #[inline(always)]
    #[must_use]
    pub fn pcsroffset(&mut self) -> PcsroffsetW<ApbaddrDbgCpu1Eddevid1Spec> {
        PcsroffsetW::new(self, 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_eddevid1_31_4(&mut self) -> Res0Eddevid1_31_4W<ApbaddrDbgCpu1Eddevid1Spec> {
        Res0Eddevid1_31_4W::new(self, 4)
    }
}
#[doc = "External Debug Device ID Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_eddevid1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_eddevid1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1Eddevid1Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu1Eddevid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_eddevid1::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1Eddevid1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_eddevid1::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1Eddevid1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_EDDEVID1 to value 0x02"]
impl crate::Resettable for ApbaddrDbgCpu1Eddevid1Spec {
    const RESET_VALUE: u32 = 0x02;
}
