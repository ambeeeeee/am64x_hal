#[doc = "Register `APBADDR_CTI_CPU1_CTICLAIMSET` reader"]
pub type R = crate::R<ApbaddrCtiCpu1CticlaimsetSpec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTICLAIMSET` writer"]
pub type W = crate::W<ApbaddrCtiCpu1CticlaimsetSpec>;
#[doc = "Field `CLAIMX` reader - 3:0\\]
CLAIM tag set bit"]
pub type ClaimxR = crate::FieldReader;
#[doc = "Field `CLAIMX` writer - 3:0\\]
CLAIM tag set bit"]
pub type ClaimxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
CLAIM tag set bit"]
    #[inline(always)]
    pub fn claimx(&self) -> ClaimxR {
        ClaimxR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
CLAIM tag set bit"]
    #[inline(always)]
    #[must_use]
    pub fn claimx(&mut self) -> ClaimxW<ApbaddrCtiCpu1CticlaimsetSpec> {
        ClaimxW::new(self, 0)
    }
}
#[doc = "CTI Claim Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_cticlaimset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_cticlaimset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1CticlaimsetSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu1CticlaimsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_cticlaimset::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1CticlaimsetSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_cticlaimset::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1CticlaimsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTICLAIMSET to value 0x15"]
impl crate::Resettable for ApbaddrCtiCpu1CticlaimsetSpec {
    const RESET_VALUE: u32 = 0x15;
}
