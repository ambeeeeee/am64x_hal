#[doc = "Register `APBADDR_CTI_CPU1_CTITRIGINSTATUS` reader"]
pub type R = crate::R<ApbaddrCtiCpu1CtitriginstatusSpec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTITRIGINSTATUS` writer"]
pub type W = crate::W<ApbaddrCtiCpu1CtitriginstatusSpec>;
#[doc = "Field `TRINN` reader - 7:0\\]
Provides the status of the trigger inputs"]
pub type TrinnR = crate::FieldReader;
#[doc = "Field `TRINN` writer - 7:0\\]
Provides the status of the trigger inputs"]
pub type TrinnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Provides the status of the trigger inputs"]
    #[inline(always)]
    pub fn trinn(&self) -> TrinnR {
        TrinnR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Provides the status of the trigger inputs"]
    #[inline(always)]
    #[must_use]
    pub fn trinn(&mut self) -> TrinnW<ApbaddrCtiCpu1CtitriginstatusSpec> {
        TrinnW::new(self, 0)
    }
}
#[doc = "CTI Trigger In Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctitriginstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctitriginstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1CtitriginstatusSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu1CtitriginstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_ctitriginstatus::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1CtitriginstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_ctitriginstatus::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1CtitriginstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTITRIGINSTATUS to value 0"]
impl crate::Resettable for ApbaddrCtiCpu1CtitriginstatusSpec {
    const RESET_VALUE: u32 = 0;
}
