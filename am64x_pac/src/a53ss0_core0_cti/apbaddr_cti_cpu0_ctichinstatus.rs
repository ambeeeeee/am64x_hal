#[doc = "Register `APBADDR_CTI_CPU0_CTICHINSTATUS` reader"]
pub type R = crate::R<ApbaddrCtiCpu0CtichinstatusSpec>;
#[doc = "Register `APBADDR_CTI_CPU0_CTICHINSTATUS` writer"]
pub type W = crate::W<ApbaddrCtiCpu0CtichinstatusSpec>;
#[doc = "Field `CHINN` reader - 3:0\\]
Provides the raw status of the ECT channel inputs to the CTI"]
pub type ChinnR = crate::FieldReader;
#[doc = "Field `CHINN` writer - 3:0\\]
Provides the raw status of the ECT channel inputs to the CTI"]
pub type ChinnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Provides the raw status of the ECT channel inputs to the CTI"]
    #[inline(always)]
    pub fn chinn(&self) -> ChinnR {
        ChinnR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Provides the raw status of the ECT channel inputs to the CTI"]
    #[inline(always)]
    #[must_use]
    pub fn chinn(&mut self) -> ChinnW<ApbaddrCtiCpu0CtichinstatusSpec> {
        ChinnW::new(self, 0)
    }
}
#[doc = "CTI Channel In Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctichinstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctichinstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu0CtichinstatusSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu0CtichinstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu0_ctichinstatus::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu0CtichinstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu0_ctichinstatus::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu0CtichinstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU0_CTICHINSTATUS to value 0"]
impl crate::Resettable for ApbaddrCtiCpu0CtichinstatusSpec {
    const RESET_VALUE: u32 = 0;
}
