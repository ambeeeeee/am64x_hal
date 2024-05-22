#[doc = "Register `APBADDR_CTI_CPU0_CTIPIDR4` reader"]
pub type R = crate::R<ApbaddrCtiCpu0Ctipidr4Spec>;
#[doc = "Register `APBADDR_CTI_CPU0_CTIPIDR4` writer"]
pub type W = crate::W<ApbaddrCtiCpu0Ctipidr4Spec>;
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
#[doc = "Field `RES0_CTIPIDR4_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Ctipidr4_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_CTIPIDR4_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Ctipidr4_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
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
    pub fn res0_ctipidr4_31_8(&self) -> Res0Ctipidr4_31_8R {
        Res0Ctipidr4_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Designer, JEP106 continuation code, least significant nibble. For ARM Limited, this field is 0b0100."]
    #[inline(always)]
    #[must_use]
    pub fn des_2(&mut self) -> Des2W<ApbaddrCtiCpu0Ctipidr4Spec> {
        Des2W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Size of the component. RAZ. Log2 of the number of 4KB pages from the start of the component to the end of the component ID registers."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<ApbaddrCtiCpu0Ctipidr4Spec> {
        SizeW::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_ctipidr4_31_8(&mut self) -> Res0Ctipidr4_31_8W<ApbaddrCtiCpu0Ctipidr4Spec> {
        Res0Ctipidr4_31_8W::new(self, 8)
    }
}
#[doc = "CTI Peripheral Identification Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctipidr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctipidr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu0Ctipidr4Spec;
impl crate::RegisterSpec for ApbaddrCtiCpu0Ctipidr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu0_ctipidr4::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu0Ctipidr4Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu0_ctipidr4::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu0Ctipidr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU0_CTIPIDR4 to value 0x04"]
impl crate::Resettable for ApbaddrCtiCpu0Ctipidr4Spec {
    const RESET_VALUE: u32 = 0x04;
}
