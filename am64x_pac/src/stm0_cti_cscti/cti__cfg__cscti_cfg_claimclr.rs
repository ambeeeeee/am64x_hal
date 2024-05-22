#[doc = "Register `CTI__CFG__CSCTI_CFG_CLAIMCLR` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgClaimclrSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_CLAIMCLR` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgClaimclrSpec>;
#[doc = "Field `CLAIMCLR` reader - 3:0\\]
The value present reflects the current setting of the Claim Tag."]
pub type ClaimclrR = crate::FieldReader;
#[doc = "Field `CLAIMCLR` writer - 3:0\\]
The value present reflects the current setting of the Claim Tag."]
pub type ClaimclrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
The value present reflects the current setting of the Claim Tag."]
    #[inline(always)]
    pub fn claimclr(&self) -> ClaimclrR {
        ClaimclrR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
The value present reflects the current setting of the Claim Tag."]
    #[inline(always)]
    #[must_use]
    pub fn claimclr(&mut self) -> ClaimclrW<Cti_Cfg_CsctiCfgClaimclrSpec> {
        ClaimclrW::new(self, 0)
    }
}
#[doc = "This register is used in conjunction with Claim Tag Set Register, CLAIMSET. This register forms one half of the Claim Tag value. This location enables individual bits to be cleared, write, and returns the current Claim Tag value, read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_claimclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_claimclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgClaimclrSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgClaimclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_claimclr::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgClaimclrSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_claimclr::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgClaimclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_CLAIMCLR to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgClaimclrSpec {
    const RESET_VALUE: u32 = 0;
}
