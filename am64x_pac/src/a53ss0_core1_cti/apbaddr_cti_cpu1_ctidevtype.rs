#[doc = "Register `APBADDR_CTI_CPU1_CTIDEVTYPE` reader"]
pub type R = crate::R<ApbaddrCtiCpu1CtidevtypeSpec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTIDEVTYPE` writer"]
pub type W = crate::W<ApbaddrCtiCpu1CtidevtypeSpec>;
#[doc = "Field `MAJOR` reader - 3:0\\]
Major type. Must read as 0x4 to indicate this is a cross-trigger component."]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - 3:0\\]
Major type. Must read as 0x4 to indicate this is a cross-trigger component."]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SUB` reader - 7:4\\]
Subtype. Must read as 0x1 to indicate this is a processor component."]
pub type SubR = crate::FieldReader;
#[doc = "Field `SUB` writer - 7:4\\]
Subtype. Must read as 0x1 to indicate this is a processor component."]
pub type SubW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_CTIDEVTYPE_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Ctidevtype31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_CTIDEVTYPE_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Ctidevtype31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Major type. Must read as 0x4 to indicate this is a cross-trigger component."]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Subtype. Must read as 0x1 to indicate this is a processor component."]
    #[inline(always)]
    pub fn sub(&self) -> SubR {
        SubR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_ctidevtype_31_8(&self) -> Res0Ctidevtype31_8R {
        Res0Ctidevtype31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Major type. Must read as 0x4 to indicate this is a cross-trigger component."]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<ApbaddrCtiCpu1CtidevtypeSpec> {
        MajorW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Subtype. Must read as 0x1 to indicate this is a processor component."]
    #[inline(always)]
    #[must_use]
    pub fn sub(&mut self) -> SubW<ApbaddrCtiCpu1CtidevtypeSpec> {
        SubW::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_ctidevtype_31_8(&mut self) -> Res0Ctidevtype31_8W<ApbaddrCtiCpu1CtidevtypeSpec> {
        Res0Ctidevtype31_8W::new(self, 8)
    }
}
#[doc = "CTI Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctidevtype::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctidevtype::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1CtidevtypeSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu1CtidevtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_ctidevtype::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1CtidevtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_ctidevtype::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1CtidevtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTIDEVTYPE to value 0x14"]
impl crate::Resettable for ApbaddrCtiCpu1CtidevtypeSpec {
    const RESET_VALUE: u32 = 0x14;
}
