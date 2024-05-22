#[doc = "Register `APBADDR_CTI_CPU1_CTIAPPPULSE` reader"]
pub type R = crate::R<ApbaddrCtiCpu1CtiapppulseSpec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTIAPPPULSE` writer"]
pub type W = crate::W<ApbaddrCtiCpu1CtiapppulseSpec>;
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
    pub fn ctiapppulsex(&mut self) -> CtiapppulsexW<ApbaddrCtiCpu1CtiapppulseSpec> {
        CtiapppulsexW::new(self, 0)
    }
}
#[doc = "CTI Application Pulse Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiapppulse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiapppulse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1CtiapppulseSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu1CtiapppulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_ctiapppulse::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1CtiapppulseSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_ctiapppulse::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1CtiapppulseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTIAPPPULSE to value 0"]
impl crate::Resettable for ApbaddrCtiCpu1CtiapppulseSpec {
    const RESET_VALUE: u32 = 0;
}
