#[doc = "Register `APBADDR_CTI_CPU1_CTICLAIMCLR` reader"]
pub type R = crate::R<ApbaddrCtiCpu1CticlaimclrSpec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTICLAIMCLR` writer"]
pub type W = crate::W<ApbaddrCtiCpu1CticlaimclrSpec>;
#[doc = "Field `CLAIMX` reader - 3:0\\]
Clear CLAIM tag"]
pub type ClaimxR = crate::FieldReader;
#[doc = "Field `CLAIMX` writer - 3:0\\]
Clear CLAIM tag"]
pub type ClaimxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Clear CLAIM tag"]
    #[inline(always)]
    pub fn claimx(&self) -> ClaimxR {
        ClaimxR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Clear CLAIM tag"]
    #[inline(always)]
    #[must_use]
    pub fn claimx(&mut self) -> ClaimxW<ApbaddrCtiCpu1CticlaimclrSpec> {
        ClaimxW::new(self, 0)
    }
}
#[doc = "CTI Claim Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_cticlaimclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_cticlaimclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1CticlaimclrSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu1CticlaimclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_cticlaimclr::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1CticlaimclrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_cticlaimclr::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1CticlaimclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTICLAIMCLR to value 0"]
impl crate::Resettable for ApbaddrCtiCpu1CticlaimclrSpec {
    const RESET_VALUE: u32 = 0;
}
