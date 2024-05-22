#[doc = "Register `CTI__CFG__CSCTI_CFG_COMPID1` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCompid1Spec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_COMPID1` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCompid1Spec>;
#[doc = "Field `PRMBL_1` reader - 3:0\\]
Contains bits \\[11 : 8\\]
of the component identification"]
pub type Prmbl1R = crate::FieldReader;
#[doc = "Field `PRMBL_1` writer - 3:0\\]
Contains bits \\[11 : 8\\]
of the component identification"]
pub type Prmbl1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLASS` reader - 7:4\\]
Class of the component. E.g. ROM table, CoreSight component etc. Constitutes bits \\[15 : 12\\]
of the component identification."]
pub type ClassR = crate::FieldReader;
#[doc = "Field `CLASS` writer - 7:4\\]
Class of the component. E.g. ROM table, CoreSight component etc. Constitutes bits \\[15 : 12\\]
of the component identification."]
pub type ClassW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Contains bits \\[11 : 8\\]
of the component identification"]
    #[inline(always)]
    pub fn prmbl_1(&self) -> Prmbl1R {
        Prmbl1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Class of the component. E.g. ROM table, CoreSight component etc. Constitutes bits \\[15 : 12\\]
of the component identification."]
    #[inline(always)]
    pub fn class(&self) -> ClassR {
        ClassR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Contains bits \\[11 : 8\\]
of the component identification"]
    #[inline(always)]
    #[must_use]
    pub fn prmbl_1(&mut self) -> Prmbl1W<Cti_Cfg_CsctiCfgCompid1Spec> {
        Prmbl1W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Class of the component. E.g. ROM table, CoreSight component etc. Constitutes bits \\[15 : 12\\]
of the component identification."]
    #[inline(always)]
    #[must_use]
    pub fn class(&mut self) -> ClassW<Cti_Cfg_CsctiCfgCompid1Spec> {
        ClassW::new(self, 4)
    }
}
#[doc = "A component identification register, that indicates that the identification registers are present. This register also indicates the component class.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_compid1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_compid1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCompid1Spec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCompid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_compid1::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCompid1Spec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_compid1::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCompid1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_COMPID1 to value 0x90"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCompid1Spec {
    const RESET_VALUE: u32 = 0x90;
}
