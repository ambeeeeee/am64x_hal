#[doc = "Register `APBADDR_CTI_CPU0_CTIAPPPULSE` reader"]
pub type R = crate::R<ApbaddrCtiCpu0CtiapppulseSpec>;
#[doc = "Register `APBADDR_CTI_CPU0_CTIAPPPULSE` writer"]
pub type W = crate::W<ApbaddrCtiCpu0CtiapppulseSpec>;
#[doc = "Field `CTIAPPPULSEX` reader - 3:0\\]
Generate event pulse on ECT channel &lt;x>."]
pub type CtiapppulsexR = crate::FieldReader;
#[doc = "Field `CTIAPPPULSEX` writer - 3:0\\]
Generate event pulse on ECT channel &lt;x>."]
pub type CtiapppulsexW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Generate event pulse on ECT channel &lt;x>."]
    #[inline(always)]
    pub fn ctiapppulsex(&self) -> CtiapppulsexR {
        CtiapppulsexR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Generate event pulse on ECT channel &lt;x>."]
    #[inline(always)]
    #[must_use]
    pub fn ctiapppulsex(&mut self) -> CtiapppulsexW<ApbaddrCtiCpu0CtiapppulseSpec> {
        CtiapppulsexW::new(self, 0)
    }
}
#[doc = "CTI Application Pulse Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiapppulse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiapppulse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu0CtiapppulseSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu0CtiapppulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu0_ctiapppulse::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu0CtiapppulseSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu0_ctiapppulse::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu0CtiapppulseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU0_CTIAPPPULSE to value 0"]
impl crate::Resettable for ApbaddrCtiCpu0CtiapppulseSpec {
    const RESET_VALUE: u32 = 0;
}
