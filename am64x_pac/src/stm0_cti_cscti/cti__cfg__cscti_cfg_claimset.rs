#[doc = "Register `CTI__CFG__CSCTI_CFG_CLAIMSET` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgClaimsetSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_CLAIMSET` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgClaimsetSpec>;
#[doc = "Field `CLAIMSET` reader - 3:0\\]
This claim tag bit is implemented"]
pub type ClaimsetR = crate::FieldReader;
#[doc = "Field `CLAIMSET` writer - 3:0\\]
This claim tag bit is implemented"]
pub type ClaimsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This claim tag bit is implemented"]
    #[inline(always)]
    pub fn claimset(&self) -> ClaimsetR {
        ClaimsetR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This claim tag bit is implemented"]
    #[inline(always)]
    #[must_use]
    pub fn claimset(&mut self) -> ClaimsetW<Cti_Cfg_CsctiCfgClaimsetSpec> {
        ClaimsetW::new(self, 0)
    }
}
#[doc = "This is used in conjunction with Claim Tag Clear Register, CLAIMCLR. This register forms one half of the Claim Tag value. This location allows individual bits to be set, write, and returns the number of bits that can be set, read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_claimset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_claimset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgClaimsetSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgClaimsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_claimset::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgClaimsetSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_claimset::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgClaimsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_CLAIMSET to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgClaimsetSpec {
    const RESET_VALUE: u32 = 0;
}
