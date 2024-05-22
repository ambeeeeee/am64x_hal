#[doc = "Register `APBADDR_CTI_CPU0_CTIOUTEN4` reader"]
pub type R = crate::R<ApbaddrCtiCpu0Ctiouten4Spec>;
#[doc = "Register `APBADDR_CTI_CPU0_CTIOUTEN4` writer"]
pub type W = crate::W<ApbaddrCtiCpu0Ctiouten4Spec>;
#[doc = "Field `OUTENX` reader - 3:0\\]
Input channel &lt;x> to output trigger 4 enable"]
pub type OutenxR = crate::FieldReader;
#[doc = "Field `OUTENX` writer - 3:0\\]
Input channel &lt;x> to output trigger 4 enable"]
pub type OutenxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Input channel &lt;x> to output trigger 4 enable"]
    #[inline(always)]
    pub fn outenx(&self) -> OutenxR {
        OutenxR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Input channel &lt;x> to output trigger 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn outenx(&mut self) -> OutenxW<ApbaddrCtiCpu0Ctiouten4Spec> {
        OutenxW::new(self, 0)
    }
}
#[doc = "CTI Input Channel to Output Trigger Enable Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiouten4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiouten4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu0Ctiouten4Spec;
impl crate::RegisterSpec for ApbaddrCtiCpu0Ctiouten4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu0_ctiouten4::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu0Ctiouten4Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu0_ctiouten4::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu0Ctiouten4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU0_CTIOUTEN4 to value 0"]
impl crate::Resettable for ApbaddrCtiCpu0Ctiouten4Spec {
    const RESET_VALUE: u32 = 0;
}
