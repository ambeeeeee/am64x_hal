#[doc = "Register `APBADDR_DBG_CPU0_EDPIDR4` reader"]
pub type R = crate::R<ApbaddrDbgCpu0Edpidr4Spec>;
#[doc = "Register `APBADDR_DBG_CPU0_EDPIDR4` writer"]
pub type W = crate::W<ApbaddrDbgCpu0Edpidr4Spec>;
#[doc = "Field `DES_2` reader - 3:0\\]
Designer, JEP106 continuation code, least significant nibble. For ARM Limited, this field is 0b0100."]
pub type Des2R = crate::FieldReader;
#[doc = "Field `DES_2` writer - 3:0\\]
Designer, JEP106 continuation code, least significant nibble. For ARM Limited, this field is 0b0100."]
pub type Des2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SIZE` reader - 7:4\\]
Size of the component. RAZ. Log2 of the number of 4KB pages from the start of the component to the end of the component ID registers."]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - 7:4\\]
Size of the component. RAZ. Log2 of the number of 4KB pages from the start of the component to the end of the component ID registers."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_EDPIDR4_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Edpidr4_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_EDPIDR4_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Edpidr4_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Designer, JEP106 continuation code, least significant nibble. For ARM Limited, this field is 0b0100."]
    #[inline(always)]
    pub fn des_2(&self) -> Des2R {
        Des2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Size of the component. RAZ. Log2 of the number of 4KB pages from the start of the component to the end of the component ID registers."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edpidr4_31_8(&self) -> Res0Edpidr4_31_8R {
        Res0Edpidr4_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Designer, JEP106 continuation code, least significant nibble. For ARM Limited, this field is 0b0100."]
    #[inline(always)]
    #[must_use]
    pub fn des_2(&mut self) -> Des2W<ApbaddrDbgCpu0Edpidr4Spec> {
        Des2W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Size of the component. RAZ. Log2 of the number of 4KB pages from the start of the component to the end of the component ID registers."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<ApbaddrDbgCpu0Edpidr4Spec> {
        SizeW::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edpidr4_31_8(&mut self) -> Res0Edpidr4_31_8W<ApbaddrDbgCpu0Edpidr4Spec> {
        Res0Edpidr4_31_8W::new(self, 8)
    }
}
#[doc = "External Debug Peripheral Identification Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edpidr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edpidr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0Edpidr4Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu0Edpidr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_edpidr4::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0Edpidr4Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_edpidr4::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0Edpidr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_EDPIDR4 to value 0x04"]
impl crate::Resettable for ApbaddrDbgCpu0Edpidr4Spec {
    const RESET_VALUE: u32 = 0x04;
}
