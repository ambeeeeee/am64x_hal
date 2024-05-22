#[doc = "Register `CPSW_NUSS_VBUSP_MR_ADV_ABILITY_REG` reader"]
pub type R = crate::R<CpswNussVbuspMrAdvAbilityRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_MR_ADV_ABILITY_REG` writer"]
pub type W = crate::W<CpswNussVbuspMrAdvAbilityRegSpec>;
#[doc = "Field `MR_ADV_ABILITY` reader - 15:0\\]
Advertised ability"]
pub type MrAdvAbilityR = crate::FieldReader<u16>;
#[doc = "Field `MR_ADV_ABILITY` writer - 15:0\\]
Advertised ability"]
pub type MrAdvAbilityW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Advertised ability"]
    #[inline(always)]
    pub fn mr_adv_ability(&self) -> MrAdvAbilityR {
        MrAdvAbilityR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Advertised ability"]
    #[inline(always)]
    #[must_use]
    pub fn mr_adv_ability(&mut self) -> MrAdvAbilityW<CpswNussVbuspMrAdvAbilityRegSpec> {
        MrAdvAbilityW::new(self, 0)
    }
}
#[doc = "SGMII MR Advertized Ability Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_mr_adv_ability_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_mr_adv_ability_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspMrAdvAbilityRegSpec;
impl crate::RegisterSpec for CpswNussVbuspMrAdvAbilityRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_mr_adv_ability_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspMrAdvAbilityRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_mr_adv_ability_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspMrAdvAbilityRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_MR_ADV_ABILITY_REG to value 0"]
impl crate::Resettable for CpswNussVbuspMrAdvAbilityRegSpec {
    const RESET_VALUE: u32 = 0;
}
