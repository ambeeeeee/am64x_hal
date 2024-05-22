#[doc = "Register `CPSW_NUSS_VBUSP_ALIVE_REG` reader"]
pub type R = crate::R<CpswNussVbuspAliveRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_ALIVE_REG` writer"]
pub type W = crate::W<CpswNussVbuspAliveRegSpec>;
#[doc = "Field `ALIVE` reader - 31:0\\]
MDIO alive"]
pub type AliveR = crate::FieldReader<u32>;
#[doc = "Field `ALIVE` writer - 31:0\\]
MDIO alive"]
pub type AliveW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO alive"]
    #[inline(always)]
    pub fn alive(&self) -> AliveR {
        AliveR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO alive"]
    #[inline(always)]
    #[must_use]
    pub fn alive(&mut self) -> AliveW<CpswNussVbuspAliveRegSpec> {
        AliveW::new(self, 0)
    }
}
#[doc = "MDIO Alive Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_alive_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_alive_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspAliveRegSpec;
impl crate::RegisterSpec for CpswNussVbuspAliveRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_alive_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspAliveRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_alive_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspAliveRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_ALIVE_REG to value 0"]
impl crate::Resettable for CpswNussVbuspAliveRegSpec {
    const RESET_VALUE: u32 = 0;
}
