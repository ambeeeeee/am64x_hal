#[doc = "Register `APBADDR_ETM_CPU1_TRCPIDR0` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trcpidr0Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCPIDR0` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trcpidr0Spec>;
#[doc = "Field `PART_0` reader - 7:0\\]
Part number, bits\\[7:0\\]."]
pub type Part0R = crate::FieldReader;
#[doc = "Field `PART_0` writer - 7:0\\]
Part number, bits\\[7:0\\]."]
pub type Part0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RES0_TRCPIDR0_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Trcpidr0_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCPIDR0_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Trcpidr0_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Part number, bits\\[7:0\\]."]
    #[inline(always)]
    pub fn part_0(&self) -> Part0R {
        Part0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcpidr0_31_8(&self) -> Res0Trcpidr0_31_8R {
        Res0Trcpidr0_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Part number, bits\\[7:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn part_0(&mut self) -> Part0W<ApbaddrEtmCpu1Trcpidr0Spec> {
        Part0W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcpidr0_31_8(&mut self) -> Res0Trcpidr0_31_8W<ApbaddrEtmCpu1Trcpidr0Spec> {
        Res0Trcpidr0_31_8W::new(self, 8)
    }
}
#[doc = "Peripheral Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcpidr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcpidr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trcpidr0Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trcpidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcpidr0::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trcpidr0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcpidr0::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trcpidr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCPIDR0 to value 0x93"]
impl crate::Resettable for ApbaddrEtmCpu1Trcpidr0Spec {
    const RESET_VALUE: u32 = 0x93;
}
