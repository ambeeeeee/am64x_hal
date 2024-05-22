#[doc = "Register `APBADDR_CTI_CPU0_CTIAPPSET` reader"]
pub type R = crate::R<ApbaddrCtiCpu0CtiappsetSpec>;
#[doc = "Register `APBADDR_CTI_CPU0_CTIAPPSET` writer"]
pub type W = crate::W<ApbaddrCtiCpu0CtiappsetSpec>;
#[doc = "Field `CTIAPPSETX` reader - 3:0\\]
Application trigger &lt;x> enable"]
pub type CtiappsetxR = crate::FieldReader;
#[doc = "Field `CTIAPPSETX` writer - 3:0\\]
Application trigger &lt;x> enable"]
pub type CtiappsetxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Application trigger &lt;x> enable"]
    #[inline(always)]
    pub fn ctiappsetx(&self) -> CtiappsetxR {
        CtiappsetxR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Application trigger &lt;x> enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctiappsetx(&mut self) -> CtiappsetxW<ApbaddrCtiCpu0CtiappsetSpec> {
        CtiappsetxW::new(self, 0)
    }
}
#[doc = "CTI Application Trigger Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiappset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiappset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu0CtiappsetSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu0CtiappsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu0_ctiappset::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu0CtiappsetSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu0_ctiappset::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu0CtiappsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU0_CTIAPPSET to value 0"]
impl crate::Resettable for ApbaddrCtiCpu0CtiappsetSpec {
    const RESET_VALUE: u32 = 0;
}
