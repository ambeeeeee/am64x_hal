#[doc = "Register `APBADDR_CTI_CPU0_CTICIDR3` reader"]
pub type R = crate::R<ApbaddrCtiCpu0Cticidr3Spec>;
#[doc = "Register `APBADDR_CTI_CPU0_CTICIDR3` writer"]
pub type W = crate::W<ApbaddrCtiCpu0Cticidr3Spec>;
#[doc = "Field `PRMBL_3` reader - 7:0\\]
Preamble. Must read as 0xB1."]
pub type Prmbl3R = crate::FieldReader;
#[doc = "Field `PRMBL_3` writer - 7:0\\]
Preamble. Must read as 0xB1."]
pub type Prmbl3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RES0_CTICIDR3_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Cticidr3_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_CTICIDR3_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Cticidr3_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Preamble. Must read as 0xB1."]
    #[inline(always)]
    pub fn prmbl_3(&self) -> Prmbl3R {
        Prmbl3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_cticidr3_31_8(&self) -> Res0Cticidr3_31_8R {
        Res0Cticidr3_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Preamble. Must read as 0xB1."]
    #[inline(always)]
    #[must_use]
    pub fn prmbl_3(&mut self) -> Prmbl3W<ApbaddrCtiCpu0Cticidr3Spec> {
        Prmbl3W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_cticidr3_31_8(&mut self) -> Res0Cticidr3_31_8W<ApbaddrCtiCpu0Cticidr3Spec> {
        Res0Cticidr3_31_8W::new(self, 8)
    }
}
#[doc = "CTI Component Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_cticidr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_cticidr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu0Cticidr3Spec;
impl crate::RegisterSpec for ApbaddrCtiCpu0Cticidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu0_cticidr3::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu0Cticidr3Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu0_cticidr3::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu0Cticidr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU0_CTICIDR3 to value 0x0177"]
impl crate::Resettable for ApbaddrCtiCpu0Cticidr3Spec {
    const RESET_VALUE: u32 = 0x0177;
}
