#[doc = "Register `APBADDR_ETM_CPU0_TRCPIDR4` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trcpidr4Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCPIDR4` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trcpidr4Spec>;
#[doc = "Field `DES_2` reader - 3:0\\]
Designer, JEP106 continuation code. For ARM Limited, this field is 0b0100."]
pub type Des2R = crate::FieldReader;
#[doc = "Field `DES_2` writer - 3:0\\]
Designer, JEP106 continuation code. For ARM Limited, this field is 0b0100."]
pub type Des2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SIZE` reader - 7:4\\]
Size of the component. RES0. This indicates that the ETM memory map occupies 4KB."]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - 7:4\\]
Size of the component. RES0. This indicates that the ETM memory map occupies 4KB."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_TRCPIDR4_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Trcpidr4_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCPIDR4_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Trcpidr4_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Designer, JEP106 continuation code. For ARM Limited, this field is 0b0100."]
    #[inline(always)]
    pub fn des_2(&self) -> Des2R {
        Des2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Size of the component. RES0. This indicates that the ETM memory map occupies 4KB."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcpidr4_31_8(&self) -> Res0Trcpidr4_31_8R {
        Res0Trcpidr4_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Designer, JEP106 continuation code. For ARM Limited, this field is 0b0100."]
    #[inline(always)]
    #[must_use]
    pub fn des_2(&mut self) -> Des2W<ApbaddrEtmCpu0Trcpidr4Spec> {
        Des2W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Size of the component. RES0. This indicates that the ETM memory map occupies 4KB."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<ApbaddrEtmCpu0Trcpidr4Spec> {
        SizeW::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcpidr4_31_8(&mut self) -> Res0Trcpidr4_31_8W<ApbaddrEtmCpu0Trcpidr4Spec> {
        Res0Trcpidr4_31_8W::new(self, 8)
    }
}
#[doc = "Peripheral Identification Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcpidr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcpidr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trcpidr4Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trcpidr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcpidr4::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trcpidr4Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcpidr4::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trcpidr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCPIDR4 to value 0x04"]
impl crate::Resettable for ApbaddrEtmCpu0Trcpidr4Spec {
    const RESET_VALUE: u32 = 0x04;
}
