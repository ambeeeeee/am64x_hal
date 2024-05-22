#[doc = "Register `CFG0_CLAIMREG_P2_R3` reader"]
pub type R = crate::R<Cfg0ClaimregP2R3Spec>;
#[doc = "Register `CFG0_CLAIMREG_P2_R3` writer"]
pub type W = crate::W<Cfg0ClaimregP2R3Spec>;
#[doc = "Field `CLAIMREG_P2_R3` reader - 31:0\\]
Claim bits for Partition 2"]
pub type ClaimregP2R3R = crate::FieldReader<u32>;
#[doc = "Field `CLAIMREG_P2_R3` writer - 31:0\\]
Claim bits for Partition 2"]
pub type ClaimregP2R3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Claim bits for Partition 2"]
    #[inline(always)]
    pub fn claimreg_p2_r3(&self) -> ClaimregP2R3R {
        ClaimregP2R3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Claim bits for Partition 2"]
    #[inline(always)]
    #[must_use]
    pub fn claimreg_p2_r3(&mut self) -> ClaimregP2R3W<Cfg0ClaimregP2R3Spec> {
        ClaimregP2R3W::new(self, 0)
    }
}
#[doc = "CFG0_CLAIMREG_P2_R3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0ClaimregP2R3Spec;
impl crate::RegisterSpec for Cfg0ClaimregP2R3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_claimreg_p2_r3::R`](R) reader structure"]
impl crate::Readable for Cfg0ClaimregP2R3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_claimreg_p2_r3::W`](W) writer structure"]
impl crate::Writable for Cfg0ClaimregP2R3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CLAIMREG_P2_R3 to value 0"]
impl crate::Resettable for Cfg0ClaimregP2R3Spec {
    const RESET_VALUE: u32 = 0;
}
