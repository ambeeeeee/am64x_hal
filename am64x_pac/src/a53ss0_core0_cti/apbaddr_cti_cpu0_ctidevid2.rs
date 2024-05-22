#[doc = "Register `APBADDR_CTI_CPU0_CTIDEVID2` reader"]
pub type R = crate::R<ApbaddrCtiCpu0Ctidevid2Spec>;
#[doc = "Register `APBADDR_CTI_CPU0_CTIDEVID2` writer"]
pub type W = crate::W<ApbaddrCtiCpu0Ctidevid2Spec>;
#[doc = "Field `RES0_CTIDEVID2_31_0` reader - 31:0\\]
Reserved, RES0."]
pub type Res0Ctidevid2_31_0R = crate::FieldReader<u32>;
#[doc = "Field `RES0_CTIDEVID2_31_0` writer - 31:0\\]
Reserved, RES0."]
pub type Res0Ctidevid2_31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_ctidevid2_31_0(&self) -> Res0Ctidevid2_31_0R {
        Res0Ctidevid2_31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_ctidevid2_31_0(&mut self) -> Res0Ctidevid2_31_0W<ApbaddrCtiCpu0Ctidevid2Spec> {
        Res0Ctidevid2_31_0W::new(self, 0)
    }
}
#[doc = "CTI Device ID Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctidevid2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctidevid2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu0Ctidevid2Spec;
impl crate::RegisterSpec for ApbaddrCtiCpu0Ctidevid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu0_ctidevid2::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu0Ctidevid2Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu0_ctidevid2::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu0Ctidevid2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU0_CTIDEVID2 to value 0"]
impl crate::Resettable for ApbaddrCtiCpu0Ctidevid2Spec {
    const RESET_VALUE: u32 = 0;
}
