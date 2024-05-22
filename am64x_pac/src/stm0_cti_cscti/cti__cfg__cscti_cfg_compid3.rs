#[doc = "Register `CTI__CFG__CSCTI_CFG_COMPID3` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCompid3Spec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_COMPID3` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCompid3Spec>;
#[doc = "Field `PRMBL_3` reader - 7:0\\]
Contains bits \\[31 : 24\\]
of the component identification"]
pub type Prmbl3R = crate::FieldReader;
#[doc = "Field `PRMBL_3` writer - 7:0\\]
Contains bits \\[31 : 24\\]
of the component identification"]
pub type Prmbl3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Contains bits \\[31 : 24\\]
of the component identification"]
    #[inline(always)]
    pub fn prmbl_3(&self) -> Prmbl3R {
        Prmbl3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Contains bits \\[31 : 24\\]
of the component identification"]
    #[inline(always)]
    #[must_use]
    pub fn prmbl_3(&mut self) -> Prmbl3W<Cti_Cfg_CsctiCfgCompid3Spec> {
        Prmbl3W::new(self, 0)
    }
}
#[doc = "A component identification register, that indicates that the identification registers are present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_compid3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_compid3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCompid3Spec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCompid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_compid3::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCompid3Spec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_compid3::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCompid3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_COMPID3 to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCompid3Spec {
    const RESET_VALUE: u32 = 0;
}
