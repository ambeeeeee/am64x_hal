#[doc = "Register `APBADDR_CTI_CPU1_CTIAPPCLEAR` reader"]
pub type R = crate::R<ApbaddrCtiCpu1CtiappclearSpec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTIAPPCLEAR` writer"]
pub type W = crate::W<ApbaddrCtiCpu1CtiappclearSpec>;
#[doc = "Field `CTIAPPCLEARX` reader - 3:0\\]
Application trigger &lt;x> disable"]
pub type CtiappclearxR = crate::FieldReader;
#[doc = "Field `CTIAPPCLEARX` writer - 3:0\\]
Application trigger &lt;x> disable"]
pub type CtiappclearxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Application trigger &lt;x> disable"]
    #[inline(always)]
    pub fn ctiappclearx(&self) -> CtiappclearxR {
        CtiappclearxR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Application trigger &lt;x> disable"]
    #[inline(always)]
    #[must_use]
    pub fn ctiappclearx(&mut self) -> CtiappclearxW<ApbaddrCtiCpu1CtiappclearSpec> {
        CtiappclearxW::new(self, 0)
    }
}
#[doc = "CTI Application Trigger Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiappclear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiappclear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1CtiappclearSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu1CtiappclearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_ctiappclear::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1CtiappclearSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_ctiappclear::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1CtiappclearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTIAPPCLEAR to value 0"]
impl crate::Resettable for ApbaddrCtiCpu1CtiappclearSpec {
    const RESET_VALUE: u32 = 0;
}
