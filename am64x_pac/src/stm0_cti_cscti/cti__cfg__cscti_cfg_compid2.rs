#[doc = "Register `CTI__CFG__CSCTI_CFG_COMPID2` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCompid2Spec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_COMPID2` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCompid2Spec>;
#[doc = "Field `PRMBL_2` reader - 7:0\\]
Contains bits \\[23 : 16\\]
of the component identification"]
pub type Prmbl2R = crate::FieldReader;
#[doc = "Field `PRMBL_2` writer - 7:0\\]
Contains bits \\[23 : 16\\]
of the component identification"]
pub type Prmbl2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Contains bits \\[23 : 16\\]
of the component identification"]
    #[inline(always)]
    pub fn prmbl_2(&self) -> Prmbl2R {
        Prmbl2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Contains bits \\[23 : 16\\]
of the component identification"]
    #[inline(always)]
    #[must_use]
    pub fn prmbl_2(&mut self) -> Prmbl2W<Cti_Cfg_CsctiCfgCompid2Spec> {
        Prmbl2W::new(self, 0)
    }
}
#[doc = "A component identification register, that indicates that the identification registers are present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_compid2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_compid2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCompid2Spec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCompid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_compid2::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCompid2Spec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_compid2::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCompid2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_COMPID2 to value 0x05"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCompid2Spec {
    const RESET_VALUE: u32 = 0x05;
}
