#[doc = "Register `APBADDR_CTI_CPU1_CTIGATE` reader"]
pub type R = crate::R<ApbaddrCtiCpu1CtigateSpec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTIGATE` writer"]
pub type W = crate::W<ApbaddrCtiCpu1CtigateSpec>;
#[doc = "Field `GATEX` reader - 3:0\\]
Determines whether events on channels propagate through the CTM to other ECT components or from the CTM into the CTI"]
pub type GatexR = crate::FieldReader;
#[doc = "Field `GATEX` writer - 3:0\\]
Determines whether events on channels propagate through the CTM to other ECT components or from the CTM into the CTI"]
pub type GatexW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Determines whether events on channels propagate through the CTM to other ECT components or from the CTM into the CTI"]
    #[inline(always)]
    pub fn gatex(&self) -> GatexR {
        GatexR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Determines whether events on channels propagate through the CTM to other ECT components or from the CTM into the CTI"]
    #[inline(always)]
    #[must_use]
    pub fn gatex(&mut self) -> GatexW<ApbaddrCtiCpu1CtigateSpec> {
        GatexW::new(self, 0)
    }
}
#[doc = "CTI Channel Gate Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctigate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctigate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1CtigateSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu1CtigateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_ctigate::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1CtigateSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_ctigate::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1CtigateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTIGATE to value 0x15"]
impl crate::Resettable for ApbaddrCtiCpu1CtigateSpec {
    const RESET_VALUE: u32 = 0x15;
}
