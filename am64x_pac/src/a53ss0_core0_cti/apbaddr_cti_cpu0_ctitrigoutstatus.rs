#[doc = "Register `APBADDR_CTI_CPU0_CTITRIGOUTSTATUS` reader"]
pub type R = crate::R<ApbaddrCtiCpu0CtitrigoutstatusSpec>;
#[doc = "Register `APBADDR_CTI_CPU0_CTITRIGOUTSTATUS` writer"]
pub type W = crate::W<ApbaddrCtiCpu0CtitrigoutstatusSpec>;
#[doc = "Field `TROUTN` reader - 7:0\\]
Provides the status of the trigger outputs"]
pub type TroutnR = crate::FieldReader;
#[doc = "Field `TROUTN` writer - 7:0\\]
Provides the status of the trigger outputs"]
pub type TroutnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Provides the status of the trigger outputs"]
    #[inline(always)]
    pub fn troutn(&self) -> TroutnR {
        TroutnR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Provides the status of the trigger outputs"]
    #[inline(always)]
    #[must_use]
    pub fn troutn(&mut self) -> TroutnW<ApbaddrCtiCpu0CtitrigoutstatusSpec> {
        TroutnW::new(self, 0)
    }
}
#[doc = "CTI Trigger Out Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctitrigoutstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctitrigoutstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu0CtitrigoutstatusSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu0CtitrigoutstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu0_ctitrigoutstatus::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu0CtitrigoutstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu0_ctitrigoutstatus::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu0CtitrigoutstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU0_CTITRIGOUTSTATUS to value 0"]
impl crate::Resettable for ApbaddrCtiCpu0CtitrigoutstatusSpec {
    const RESET_VALUE: u32 = 0;
}
