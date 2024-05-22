#[doc = "Register `CPSW_NUSS_VBUSP_CLAUS45_REG` reader"]
pub type R = crate::R<CpswNussVbuspClaus45RegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_CLAUS45_REG` writer"]
pub type W = crate::W<CpswNussVbuspClaus45RegSpec>;
#[doc = "Field `CLAUSE45` reader - 31:0\\]
MDIO Clause 45"]
pub type Clause45R = crate::FieldReader<u32>;
#[doc = "Field `CLAUSE45` writer - 31:0\\]
MDIO Clause 45"]
pub type Clause45W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO Clause 45"]
    #[inline(always)]
    pub fn clause45(&self) -> Clause45R {
        Clause45R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO Clause 45"]
    #[inline(always)]
    #[must_use]
    pub fn clause45(&mut self) -> Clause45W<CpswNussVbuspClaus45RegSpec> {
        Clause45W::new(self, 0)
    }
}
#[doc = "MDIO Clause45 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_claus45_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_claus45_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspClaus45RegSpec;
impl crate::RegisterSpec for CpswNussVbuspClaus45RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_claus45_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspClaus45RegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_claus45_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspClaus45RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_CLAUS45_REG to value 0"]
impl crate::Resettable for CpswNussVbuspClaus45RegSpec {
    const RESET_VALUE: u32 = 0;
}
