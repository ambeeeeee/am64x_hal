#[doc = "Register `APBADDR_ETM_CPU1_TRCDEVARCH` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcdevarchSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCDEVARCH` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcdevarchSpec>;
#[doc = "Field `ARCHID` reader - 15:0\\]
Defines this part to be a v8-A debug component. For architectures defined by ARM this is further subdivided.For trace, bits \\[15:12\\]
are the architecture version, 0x4; bits \\[11:0\\]
are the architecture part number, 0xA13.This corresponds to trace architecture version ETMv4."]
pub type ArchidR = crate::FieldReader<u16>;
#[doc = "Field `ARCHID` writer - 15:0\\]
Defines this part to be a v8-A debug component. For architectures defined by ARM this is further subdivided.For trace, bits \\[15:12\\]
are the architecture version, 0x4; bits \\[11:0\\]
are the architecture part number, 0xA13.This corresponds to trace architecture version ETMv4."]
pub type ArchidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REVISION` reader - 19:16\\]
Defines the architecture revision. For architectures defined by ARM this is the minor revision.For trace, the revision defined by ETMv4 is 0x0.All other values are reserved."]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - 19:16\\]
Defines the architecture revision. For architectures defined by ARM this is the minor revision.For trace, the revision defined by ETMv4 is 0x0.All other values are reserved."]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRESENT` reader - 20:20\\]
When set to 1, indicates that the DEVARCH is present.This field is RAO."]
pub type PresentR = crate::BitReader;
#[doc = "Field `PRESENT` writer - 20:20\\]
When set to 1, indicates that the DEVARCH is present.This field is RAO."]
pub type PresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARCHITECT` reader - 31:21\\]
Defines the architecture of the component. For trace, this is ARM Limited.Bits \\[31:28\\]
are the JEP 106 continuation code, 0x4.Bits \\[27:21\\]
are the JEP 106 ID code, 0x3B."]
pub type ArchitectR = crate::FieldReader<u16>;
#[doc = "Field `ARCHITECT` writer - 31:21\\]
Defines the architecture of the component. For trace, this is ARM Limited.Bits \\[31:28\\]
are the JEP 106 continuation code, 0x4.Bits \\[27:21\\]
are the JEP 106 ID code, 0x3B."]
pub type ArchitectW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Defines this part to be a v8-A debug component. For architectures defined by ARM this is further subdivided.For trace, bits \\[15:12\\]
are the architecture version, 0x4; bits \\[11:0\\]
are the architecture part number, 0xA13.This corresponds to trace architecture version ETMv4."]
    #[inline(always)]
    pub fn archid(&self) -> ArchidR {
        ArchidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the architecture revision. For architectures defined by ARM this is the minor revision.For trace, the revision defined by ETMv4 is 0x0.All other values are reserved."]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
When set to 1, indicates that the DEVARCH is present.This field is RAO."]
    #[inline(always)]
    pub fn present(&self) -> PresentR {
        PresentR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - 31:21\\]
Defines the architecture of the component. For trace, this is ARM Limited.Bits \\[31:28\\]
are the JEP 106 continuation code, 0x4.Bits \\[27:21\\]
are the JEP 106 ID code, 0x3B."]
    #[inline(always)]
    pub fn architect(&self) -> ArchitectR {
        ArchitectR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Defines this part to be a v8-A debug component. For architectures defined by ARM this is further subdivided.For trace, bits \\[15:12\\]
are the architecture version, 0x4; bits \\[11:0\\]
are the architecture part number, 0xA13.This corresponds to trace architecture version ETMv4."]
    #[inline(always)]
    #[must_use]
    pub fn archid(&mut self) -> ArchidW<ApbaddrEtmCpu1TrcdevarchSpec> {
        ArchidW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the architecture revision. For architectures defined by ARM this is the minor revision.For trace, the revision defined by ETMv4 is 0x0.All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<ApbaddrEtmCpu1TrcdevarchSpec> {
        RevisionW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
When set to 1, indicates that the DEVARCH is present.This field is RAO."]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PresentW<ApbaddrEtmCpu1TrcdevarchSpec> {
        PresentW::new(self, 20)
    }
    #[doc = "Bits 21:31 - 31:21\\]
Defines the architecture of the component. For trace, this is ARM Limited.Bits \\[31:28\\]
are the JEP 106 continuation code, 0x4.Bits \\[27:21\\]
are the JEP 106 ID code, 0x3B."]
    #[inline(always)]
    #[must_use]
    pub fn architect(&mut self) -> ArchitectW<ApbaddrEtmCpu1TrcdevarchSpec> {
        ArchitectW::new(self, 21)
    }
}
#[doc = "Device Architecture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcdevarch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcdevarch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcdevarchSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcdevarchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcdevarch::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcdevarchSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcdevarch::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcdevarchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCDEVARCH to value 0xae31_8963"]
impl crate::Resettable for ApbaddrEtmCpu1TrcdevarchSpec {
    const RESET_VALUE: u32 = 0xae31_8963;
}
