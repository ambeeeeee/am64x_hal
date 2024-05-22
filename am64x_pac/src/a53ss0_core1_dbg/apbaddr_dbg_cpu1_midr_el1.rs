#[doc = "Register `APBADDR_DBG_CPU1_MIDR_EL1` reader"]
pub type R = crate::R<ApbaddrDbgCpu1MidrEl1Spec>;
#[doc = "Register `APBADDR_DBG_CPU1_MIDR_EL1` writer"]
pub type W = crate::W<ApbaddrDbgCpu1MidrEl1Spec>;
#[doc = "Field `REVISION` reader - 3:0\\]
An IMPLEMENTATION DEFINED revision number for the device"]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - 3:0\\]
An IMPLEMENTATION DEFINED revision number for the device"]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PARTNUM` reader - 15:4\\]
An IMPLEMENTATION DEFINED primary part number for the device. On processors implemented by ARM if the top four bits of the primary part number are 0x0 or 0x7 the variant and architecture are encoded differently"]
pub type PartnumR = crate::FieldReader<u16>;
#[doc = "Field `PARTNUM` writer - 15:4\\]
An IMPLEMENTATION DEFINED primary part number for the device. On processors implemented by ARM if the top four bits of the primary part number are 0x0 or 0x7 the variant and architecture are encoded differently"]
pub type PartnumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ARCHITECTURE` reader - "]
pub type ArchitectureR = crate::FieldReader;
#[doc = "Field `ARCHITECTURE` writer - "]
pub type ArchitectureW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VARIANT` reader - 23:20\\]
An IMPLEMENTATION DEFINED variant number. Typically this field is used to distinguish between different product variants or major revisions of a product."]
pub type VariantR = crate::FieldReader;
#[doc = "Field `VARIANT` writer - 23:20\\]
An IMPLEMENTATION DEFINED variant number. Typically this field is used to distinguish between different product variants or major revisions of a product."]
pub type VariantW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IMPLEMENTER` reader - 31:24\\]
The Implementer code. This field must hold an implementer code that has been assigned by ARM."]
pub type ImplementerR = crate::FieldReader;
#[doc = "Field `IMPLEMENTER` writer - 31:24\\]
The Implementer code. This field must hold an implementer code that has been assigned by ARM."]
pub type ImplementerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
An IMPLEMENTATION DEFINED revision number for the device"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
An IMPLEMENTATION DEFINED primary part number for the device. On processors implemented by ARM if the top four bits of the primary part number are 0x0 or 0x7 the variant and architecture are encoded differently"]
    #[inline(always)]
    pub fn partnum(&self) -> PartnumR {
        PartnumR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn architecture(&self) -> ArchitectureR {
        ArchitectureR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
An IMPLEMENTATION DEFINED variant number. Typically this field is used to distinguish between different product variants or major revisions of a product."]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
The Implementer code. This field must hold an implementer code that has been assigned by ARM."]
    #[inline(always)]
    pub fn implementer(&self) -> ImplementerR {
        ImplementerR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
An IMPLEMENTATION DEFINED revision number for the device"]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<ApbaddrDbgCpu1MidrEl1Spec> {
        RevisionW::new(self, 0)
    }
    #[doc = "Bits 4:15 - 15:4\\]
An IMPLEMENTATION DEFINED primary part number for the device. On processors implemented by ARM if the top four bits of the primary part number are 0x0 or 0x7 the variant and architecture are encoded differently"]
    #[inline(always)]
    #[must_use]
    pub fn partnum(&mut self) -> PartnumW<ApbaddrDbgCpu1MidrEl1Spec> {
        PartnumW::new(self, 4)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn architecture(&mut self) -> ArchitectureW<ApbaddrDbgCpu1MidrEl1Spec> {
        ArchitectureW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
An IMPLEMENTATION DEFINED variant number. Typically this field is used to distinguish between different product variants or major revisions of a product."]
    #[inline(always)]
    #[must_use]
    pub fn variant(&mut self) -> VariantW<ApbaddrDbgCpu1MidrEl1Spec> {
        VariantW::new(self, 20)
    }
    #[doc = "Bits 24:31 - 31:24\\]
The Implementer code. This field must hold an implementer code that has been assigned by ARM."]
    #[inline(always)]
    #[must_use]
    pub fn implementer(&mut self) -> ImplementerW<ApbaddrDbgCpu1MidrEl1Spec> {
        ImplementerW::new(self, 24)
    }
}
#[doc = "Main ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_midr_el1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_midr_el1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1MidrEl1Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu1MidrEl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_midr_el1::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1MidrEl1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_midr_el1::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1MidrEl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_MIDR_EL1 to value 0x6517_3314"]
impl crate::Resettable for ApbaddrDbgCpu1MidrEl1Spec {
    const RESET_VALUE: u32 = 0x6517_3314;
}
