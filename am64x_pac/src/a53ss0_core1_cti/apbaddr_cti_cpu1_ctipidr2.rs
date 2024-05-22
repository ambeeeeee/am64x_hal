#[doc = "Register `APBADDR_CTI_CPU1_CTIPIDR2` reader"]
pub type R = crate::R<ApbaddrCtiCpu1Ctipidr2Spec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTIPIDR2` writer"]
pub type W = crate::W<ApbaddrCtiCpu1Ctipidr2Spec>;
#[doc = "Field `DES_1` reader - 2:0\\]
Designer, most significant bits of JEP106 ID code. For ARM Limited, this field is 0b011."]
pub type Des1R = crate::FieldReader;
#[doc = "Field `DES_1` writer - 2:0\\]
Designer, most significant bits of JEP106 ID code. For ARM Limited, this field is 0b011."]
pub type Des1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JEDEC` reader - 3:3\\]
RAO. Indicates a JEP106 identity code is used."]
pub type JedecR = crate::BitReader;
#[doc = "Field `JEDEC` writer - 3:3\\]
RAO. Indicates a JEP106 identity code is used."]
pub type JedecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REVISION` reader - 7:4\\]
Part major revision. Parts can also use this field to extend Part number to 16-bits."]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - 7:4\\]
Part major revision. Parts can also use this field to extend Part number to 16-bits."]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_CTIPIDR2_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Ctipidr2_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_CTIPIDR2_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Ctipidr2_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Designer, most significant bits of JEP106 ID code. For ARM Limited, this field is 0b011."]
    #[inline(always)]
    pub fn des_1(&self) -> Des1R {
        Des1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
RAO. Indicates a JEP106 identity code is used."]
    #[inline(always)]
    pub fn jedec(&self) -> JedecR {
        JedecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Part major revision. Parts can also use this field to extend Part number to 16-bits."]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_ctipidr2_31_8(&self) -> Res0Ctipidr2_31_8R {
        Res0Ctipidr2_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Designer, most significant bits of JEP106 ID code. For ARM Limited, this field is 0b011."]
    #[inline(always)]
    #[must_use]
    pub fn des_1(&mut self) -> Des1W<ApbaddrCtiCpu1Ctipidr2Spec> {
        Des1W::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
RAO. Indicates a JEP106 identity code is used."]
    #[inline(always)]
    #[must_use]
    pub fn jedec(&mut self) -> JedecW<ApbaddrCtiCpu1Ctipidr2Spec> {
        JedecW::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Part major revision. Parts can also use this field to extend Part number to 16-bits."]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<ApbaddrCtiCpu1Ctipidr2Spec> {
        RevisionW::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_ctipidr2_31_8(&mut self) -> Res0Ctipidr2_31_8W<ApbaddrCtiCpu1Ctipidr2Spec> {
        Res0Ctipidr2_31_8W::new(self, 8)
    }
}
#[doc = "CTI Peripheral Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctipidr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctipidr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1Ctipidr2Spec;
impl crate::RegisterSpec for ApbaddrCtiCpu1Ctipidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_ctipidr2::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1Ctipidr2Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_ctipidr2::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1Ctipidr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTIPIDR2 to value 0x4b"]
impl crate::Resettable for ApbaddrCtiCpu1Ctipidr2Spec {
    const RESET_VALUE: u32 = 0x4b;
}
