#[doc = "Register `APBADDR_ETM_CPU1_TRCPIDR1` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trcpidr1Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCPIDR1` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trcpidr1Spec>;
#[doc = "Field `PART_1` reader - 3:0\\]
Part number, bits\\[11:8\\]."]
pub type Part1R = crate::FieldReader;
#[doc = "Field `PART_1` writer - 3:0\\]
Part number, bits\\[11:8\\]."]
pub type Part1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DES_0` reader - 7:4\\]
Designer, bits\\[3:0\\]
of JEP106 ID code. For ARM Limited, this field is 0b1011."]
pub type Des0R = crate::FieldReader;
#[doc = "Field `DES_0` writer - 7:4\\]
Designer, bits\\[3:0\\]
of JEP106 ID code. For ARM Limited, this field is 0b1011."]
pub type Des0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_TRCPIDR1_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Trcpidr1_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCPIDR1_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Trcpidr1_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Part number, bits\\[11:8\\]."]
    #[inline(always)]
    pub fn part_1(&self) -> Part1R {
        Part1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Designer, bits\\[3:0\\]
of JEP106 ID code. For ARM Limited, this field is 0b1011."]
    #[inline(always)]
    pub fn des_0(&self) -> Des0R {
        Des0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcpidr1_31_8(&self) -> Res0Trcpidr1_31_8R {
        Res0Trcpidr1_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Part number, bits\\[11:8\\]."]
    #[inline(always)]
    #[must_use]
    pub fn part_1(&mut self) -> Part1W<ApbaddrEtmCpu1Trcpidr1Spec> {
        Part1W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Designer, bits\\[3:0\\]
of JEP106 ID code. For ARM Limited, this field is 0b1011."]
    #[inline(always)]
    #[must_use]
    pub fn des_0(&mut self) -> Des0W<ApbaddrEtmCpu1Trcpidr1Spec> {
        Des0W::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcpidr1_31_8(&mut self) -> Res0Trcpidr1_31_8W<ApbaddrEtmCpu1Trcpidr1Spec> {
        Res0Trcpidr1_31_8W::new(self, 8)
    }
}
#[doc = "Peripheral Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcpidr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcpidr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trcpidr1Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trcpidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcpidr1::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trcpidr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcpidr1::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trcpidr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCPIDR1 to value 0x0119"]
impl crate::Resettable for ApbaddrEtmCpu1Trcpidr1Spec {
    const RESET_VALUE: u32 = 0x0119;
}
