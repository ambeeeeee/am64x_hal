#[doc = "Register `APBADDR_CTI_CPU0_CTICHOUTSTATUS` reader"]
pub type R = crate::R<ApbaddrCtiCpu0CtichoutstatusSpec>;
#[doc = "Register `APBADDR_CTI_CPU0_CTICHOUTSTATUS` writer"]
pub type W = crate::W<ApbaddrCtiCpu0CtichoutstatusSpec>;
#[doc = "Field `CHOUTN` reader - 3:0\\]
Provides the status of the ECT channel outputs from the CTI"]
pub type ChoutnR = crate::FieldReader;
#[doc = "Field `CHOUTN` writer - 3:0\\]
Provides the status of the ECT channel outputs from the CTI"]
pub type ChoutnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Provides the status of the ECT channel outputs from the CTI"]
    #[inline(always)]
    pub fn choutn(&self) -> ChoutnR {
        ChoutnR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Provides the status of the ECT channel outputs from the CTI"]
    #[inline(always)]
    #[must_use]
    pub fn choutn(&mut self) -> ChoutnW<ApbaddrCtiCpu0CtichoutstatusSpec> {
        ChoutnW::new(self, 0)
    }
}
#[doc = "CTI Channel Out Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctichoutstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctichoutstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu0CtichoutstatusSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu0CtichoutstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu0_ctichoutstatus::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu0CtichoutstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu0_ctichoutstatus::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu0CtichoutstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU0_CTICHOUTSTATUS to value 0"]
impl crate::Resettable for ApbaddrCtiCpu0CtichoutstatusSpec {
    const RESET_VALUE: u32 = 0;
}
