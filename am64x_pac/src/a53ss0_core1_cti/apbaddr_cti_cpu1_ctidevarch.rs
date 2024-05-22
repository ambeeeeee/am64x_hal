#[doc = "Register `APBADDR_CTI_CPU1_CTIDEVARCH` reader"]
pub type R = crate::R<ApbaddrCtiCpu1CtidevarchSpec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTIDEVARCH` writer"]
pub type W = crate::W<ApbaddrCtiCpu1CtidevarchSpec>;
#[doc = "Field `ARCHID` reader - 15:0\\]
Defines this part to be a v8-A debug component. For architectures defined by ARM this is further subdivided.For CTI:Bits \\[15:12\\]
are the architecture version, 0x1.Bits \\[11:0\\]
are the architecture part number, 0xA14.This corresponds to CTI architecture version CTIv2."]
pub type ArchidR = crate::FieldReader<u16>;
#[doc = "Field `ARCHID` writer - 15:0\\]
Defines this part to be a v8-A debug component. For architectures defined by ARM this is further subdivided.For CTI:Bits \\[15:12\\]
are the architecture version, 0x1.Bits \\[11:0\\]
are the architecture part number, 0xA14.This corresponds to CTI architecture version CTIv2."]
pub type ArchidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REVISION` reader - 19:16\\]
Defines the architecture revision. For architectures defined by ARM this is the minor revision.For CTI, the revision defined by v8-A is 0x0.All other values are reserved."]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - 19:16\\]
Defines the architecture revision. For architectures defined by ARM this is the minor revision.For CTI, the revision defined by v8-A is 0x0.All other values are reserved."]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRESENT` reader - 20:20\\]
When set to 1, indicates that the DEVARCH is present.This field is 1 in v8-A."]
pub type PresentR = crate::BitReader;
#[doc = "Field `PRESENT` writer - 20:20\\]
When set to 1, indicates that the DEVARCH is present.This field is 1 in v8-A."]
pub type PresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARCHITECT` reader - 31:21\\]
Defines the architecture of the component. For CTI, this is ARM Limited.Bits \\[31:28\\]
are the JEP 106 continuation code, 0x4.Bits \\[27:21\\]
are the JEP 106 ID code, 0x3B."]
pub type ArchitectR = crate::FieldReader<u16>;
#[doc = "Field `ARCHITECT` writer - 31:21\\]
Defines the architecture of the component. For CTI, this is ARM Limited.Bits \\[31:28\\]
are the JEP 106 continuation code, 0x4.Bits \\[27:21\\]
are the JEP 106 ID code, 0x3B."]
pub type ArchitectW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Defines this part to be a v8-A debug component. For architectures defined by ARM this is further subdivided.For CTI:Bits \\[15:12\\]
are the architecture version, 0x1.Bits \\[11:0\\]
are the architecture part number, 0xA14.This corresponds to CTI architecture version CTIv2."]
    #[inline(always)]
    pub fn archid(&self) -> ArchidR {
        ArchidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the architecture revision. For architectures defined by ARM this is the minor revision.For CTI, the revision defined by v8-A is 0x0.All other values are reserved."]
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
Defines the architecture of the component. For CTI, this is ARM Limited.Bits \\[31:28\\]
are the JEP 106 continuation code, 0x4.Bits \\[27:21\\]
are the JEP 106 ID code, 0x3B."]
    #[inline(always)]
    pub fn architect(&self) -> ArchitectR {
        ArchitectR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Defines this part to be a v8-A debug component. For architectures defined by ARM this is further subdivided.For CTI:Bits \\[15:12\\]
are the architecture version, 0x1.Bits \\[11:0\\]
are the architecture part number, 0xA14.This corresponds to CTI architecture version CTIv2."]
    #[inline(always)]
    #[must_use]
    pub fn archid(&mut self) -> ArchidW<ApbaddrCtiCpu1CtidevarchSpec> {
        ArchidW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the architecture revision. For architectures defined by ARM this is the minor revision.For CTI, the revision defined by v8-A is 0x0.All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<ApbaddrCtiCpu1CtidevarchSpec> {
        RevisionW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
When set to 1, indicates that the DEVARCH is present.This field is 1 in v8-A."]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PresentW<ApbaddrCtiCpu1CtidevarchSpec> {
        PresentW::new(self, 20)
    }
    #[doc = "Bits 21:31 - 31:21\\]
Defines the architecture of the component. For CTI, this is ARM Limited.Bits \\[31:28\\]
are the JEP 106 continuation code, 0x4.Bits \\[27:21\\]
are the JEP 106 ID code, 0x3B."]
    #[inline(always)]
    #[must_use]
    pub fn architect(&mut self) -> ArchitectW<ApbaddrCtiCpu1CtidevarchSpec> {
        ArchitectW::new(self, 21)
    }
}
#[doc = "CTI Device Architecture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctidevarch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctidevarch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1CtidevarchSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu1CtidevarchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_ctidevarch::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1CtidevarchSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_ctidevarch::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1CtidevarchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTIDEVARCH to value 0xae30_6676"]
impl crate::Resettable for ApbaddrCtiCpu1CtidevarchSpec {
    const RESET_VALUE: u32 = 0xae30_6676;
}
