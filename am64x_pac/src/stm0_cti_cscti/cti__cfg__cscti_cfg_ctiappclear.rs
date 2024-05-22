#[doc = "Register `CTI__CFG__CSCTI_CFG_CTIAPPCLEAR` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCtiappclearSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_CTIAPPCLEAR` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCtiappclearSpec>;
#[doc = "Field `APPCLEAR` reader - 3:0\\]
Clears corresponding bits in the CTIAPPSET register. There is one bit of the register for each channel. When a 1 is written to a bit in this register, the corresponding application trigger is disabled in the CTIAPPSET register. Writing a 0 to any of the bits in this register has no effect."]
pub type AppclearR = crate::FieldReader;
#[doc = "Field `APPCLEAR` writer - 3:0\\]
Clears corresponding bits in the CTIAPPSET register. There is one bit of the register for each channel. When a 1 is written to a bit in this register, the corresponding application trigger is disabled in the CTIAPPSET register. Writing a 0 to any of the bits in this register has no effect."]
pub type AppclearW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Clears corresponding bits in the CTIAPPSET register. There is one bit of the register for each channel. When a 1 is written to a bit in this register, the corresponding application trigger is disabled in the CTIAPPSET register. Writing a 0 to any of the bits in this register has no effect."]
    #[inline(always)]
    pub fn appclear(&self) -> AppclearR {
        AppclearR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Clears corresponding bits in the CTIAPPSET register. There is one bit of the register for each channel. When a 1 is written to a bit in this register, the corresponding application trigger is disabled in the CTIAPPSET register. Writing a 0 to any of the bits in this register has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn appclear(&mut self) -> AppclearW<Cti_Cfg_CsctiCfgCtiappclearSpec> {
        AppclearW::new(self, 0)
    }
}
#[doc = "The CTI Interrupt Acknowledge Register is write-only. A write to this register causes a channel event to be cleared, corresponding to the bit written to.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiappclear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiappclear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCtiappclearSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCtiappclearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_ctiappclear::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCtiappclearSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_ctiappclear::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCtiappclearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_CTIAPPCLEAR to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCtiappclearSpec {
    const RESET_VALUE: u32 = 0;
}
