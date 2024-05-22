#[doc = "Register `APBADDR_DBG_CPU1_EDDEVARCH` reader"]
pub type R = crate::R<ApbaddrDbgCpu1EddevarchSpec>;
#[doc = "Register `APBADDR_DBG_CPU1_EDDEVARCH` writer"]
pub type W = crate::W<ApbaddrDbgCpu1EddevarchSpec>;
#[doc = "Field `ARCHID` reader - 15:0\\]
Defines this part to be a v8-A debug component. For architectures defined by ARM this is further subdivided.For debug:Bits \\[15:12\\]
are the architecture version, 0x6.Bits \\[11:0\\]
are the architecture part number, 0xA15.This corresponds to debug architecture version v8-A."]
pub type ArchidR = crate::FieldReader<u16>;
#[doc = "Field `ARCHID` writer - 15:0\\]
Defines this part to be a v8-A debug component. For architectures defined by ARM this is further subdivided.For debug:Bits \\[15:12\\]
are the architecture version, 0x6.Bits \\[11:0\\]
are the architecture part number, 0xA15.This corresponds to debug architecture version v8-A."]
pub type ArchidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REVISION` reader - 19:16\\]
Defines the architecture revision. For architectures defined by ARM this is the minor revision.For debug, the revision defined by v8-A is 0x0.All other values are reserved."]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - 19:16\\]
Defines the architecture revision. For architectures defined by ARM this is the minor revision.For debug, the revision defined by v8-A is 0x0.All other values are reserved."]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRESENT` reader - 20:20\\]
When set to 1, indicates that the DEVARCH is present.This field is 1 in v8-A."]
pub type PresentR = crate::BitReader;
#[doc = "Field `PRESENT` writer - 20:20\\]
When set to 1, indicates that the DEVARCH is present.This field is 1 in v8-A."]
pub type PresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARCHITECT` reader - 31:21\\]
Defines the architecture of the component. For debug, this is ARM Limited.Bits \\[31:28\\]
are the JEP 106 continuation code, 0x4.Bits \\[27:21\\]
are the JEP 106 ID code, 0x3B."]
pub type ArchitectR = crate::FieldReader<u16>;
#[doc = "Field `ARCHITECT` writer - 31:21\\]
Defines the architecture of the component. For debug, this is ARM Limited.Bits \\[31:28\\]
are the JEP 106 continuation code, 0x4.Bits \\[27:21\\]
are the JEP 106 ID code, 0x3B."]
pub type ArchitectW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Defines this part to be a v8-A debug component. For architectures defined by ARM this is further subdivided.For debug:Bits \\[15:12\\]
are the architecture version, 0x6.Bits \\[11:0\\]
are the architecture part number, 0xA15.This corresponds to debug architecture version v8-A."]
    #[inline(always)]
    pub fn archid(&self) -> ArchidR {
        ArchidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the architecture revision. For architectures defined by ARM this is the minor revision.For debug, the revision defined by v8-A is 0x0.All other values are reserved."]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
When set to 1, indicates that the DEVARCH is present.This field is 1 in v8-A."]
    #[inline(always)]
    pub fn present(&self) -> PresentR {
        PresentR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - 31:21\\]
Defines the architecture of the component. For debug, this is ARM Limited.Bits \\[31:28\\]
are the JEP 106 continuation code, 0x4.Bits \\[27:21\\]
are the JEP 106 ID code, 0x3B."]
    #[inline(always)]
    pub fn architect(&self) -> ArchitectR {
        ArchitectR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Defines this part to be a v8-A debug component. For architectures defined by ARM this is further subdivided.For debug:Bits \\[15:12\\]
are the architecture version, 0x6.Bits \\[11:0\\]
are the architecture part number, 0xA15.This corresponds to debug architecture version v8-A."]
    #[inline(always)]
    #[must_use]
    pub fn archid(&mut self) -> ArchidW<ApbaddrDbgCpu1EddevarchSpec> {
        ArchidW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the architecture revision. For architectures defined by ARM this is the minor revision.For debug, the revision defined by v8-A is 0x0.All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<ApbaddrDbgCpu1EddevarchSpec> {
        RevisionW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
When set to 1, indicates that the DEVARCH is present.This field is 1 in v8-A."]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PresentW<ApbaddrDbgCpu1EddevarchSpec> {
        PresentW::new(self, 20)
    }
    #[doc = "Bits 21:31 - 31:21\\]
Defines the architecture of the component. For debug, this is ARM Limited.Bits \\[31:28\\]
are the JEP 106 continuation code, 0x4.Bits \\[27:21\\]
are the JEP 106 ID code, 0x3B."]
    #[inline(always)]
    #[must_use]
    pub fn architect(&mut self) -> ArchitectW<ApbaddrDbgCpu1EddevarchSpec> {
        ArchitectW::new(self, 21)
    }
}
#[doc = "External Debug Device Architecture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_eddevarch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_eddevarch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1EddevarchSpec;
impl crate::RegisterSpec for ApbaddrDbgCpu1EddevarchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_eddevarch::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1EddevarchSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_eddevarch::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1EddevarchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_EDDEVARCH to value 0xae32_7157"]
impl crate::Resettable for ApbaddrDbgCpu1EddevarchSpec {
    const RESET_VALUE: u32 = 0xae32_7157;
}
