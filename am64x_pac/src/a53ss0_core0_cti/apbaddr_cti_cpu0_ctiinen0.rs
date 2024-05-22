#[doc = "Register `APBADDR_CTI_CPU0_CTIINEN0` reader"]
pub type R = crate::R<ApbaddrCtiCpu0Ctiinen0Spec>;
#[doc = "Register `APBADDR_CTI_CPU0_CTIINEN0` writer"]
pub type W = crate::W<ApbaddrCtiCpu0Ctiinen0Spec>;
#[doc = "Field `INENX` reader - 3:0\\]
Input trigger 0 to output channel &lt;x> enable"]
pub type InenxR = crate::FieldReader;
#[doc = "Field `INENX` writer - 3:0\\]
Input trigger 0 to output channel &lt;x> enable"]
pub type InenxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Input trigger 0 to output channel &lt;x> enable"]
    #[inline(always)]
    pub fn inenx(&self) -> InenxR {
        InenxR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Input trigger 0 to output channel &lt;x> enable"]
    #[inline(always)]
    #[must_use]
    pub fn inenx(&mut self) -> InenxW<ApbaddrCtiCpu0Ctiinen0Spec> {
        InenxW::new(self, 0)
    }
}
#[doc = "CTI Input Trigger to Output Channel Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiinen0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiinen0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu0Ctiinen0Spec;
impl crate::RegisterSpec for ApbaddrCtiCpu0Ctiinen0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu0_ctiinen0::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu0Ctiinen0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu0_ctiinen0::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu0Ctiinen0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU0_CTIINEN0 to value 0"]
impl crate::Resettable for ApbaddrCtiCpu0Ctiinen0Spec {
    const RESET_VALUE: u32 = 0;
}
