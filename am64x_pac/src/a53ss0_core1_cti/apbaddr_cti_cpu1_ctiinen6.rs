#[doc = "Register `APBADDR_CTI_CPU1_CTIINEN6` reader"]
pub type R = crate::R<ApbaddrCtiCpu1Ctiinen6Spec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTIINEN6` writer"]
pub type W = crate::W<ApbaddrCtiCpu1Ctiinen6Spec>;
#[doc = "Field `INENX` reader - 3:0\\]
Input trigger 6 to output channel &lt;x> enable"]
pub type InenxR = crate::FieldReader;
#[doc = "Field `INENX` writer - 3:0\\]
Input trigger 6 to output channel &lt;x> enable"]
pub type InenxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Input trigger 6 to output channel &lt;x> enable"]
    #[inline(always)]
    pub fn inenx(&self) -> InenxR {
        InenxR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Input trigger 6 to output channel &lt;x> enable"]
    #[inline(always)]
    #[must_use]
    pub fn inenx(&mut self) -> InenxW<ApbaddrCtiCpu1Ctiinen6Spec> {
        InenxW::new(self, 0)
    }
}
#[doc = "CTI Input Trigger to Output Channel Enable Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiinen6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiinen6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1Ctiinen6Spec;
impl crate::RegisterSpec for ApbaddrCtiCpu1Ctiinen6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_ctiinen6::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1Ctiinen6Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_ctiinen6::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1Ctiinen6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTIINEN6 to value 0"]
impl crate::Resettable for ApbaddrCtiCpu1Ctiinen6Spec {
    const RESET_VALUE: u32 = 0;
}
