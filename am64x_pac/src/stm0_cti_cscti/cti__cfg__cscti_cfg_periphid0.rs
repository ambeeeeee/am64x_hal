#[doc = "Register `CTI__CFG__CSCTI_CFG_PERIPHID0` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgPeriphid0Spec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_PERIPHID0` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgPeriphid0Spec>;
#[doc = "Field `PART_0` reader - 7:0\\]
Bits \\[7 : 0\\]
of the component's part number. This is selected by the designer of the component."]
pub type Part0R = crate::FieldReader;
#[doc = "Field `PART_0` writer - 7:0\\]
Bits \\[7 : 0\\]
of the component's part number. This is selected by the designer of the component."]
pub type Part0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Bits \\[7 : 0\\]
of the component's part number. This is selected by the designer of the component."]
    #[inline(always)]
    pub fn part_0(&self) -> Part0R {
        Part0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Bits \\[7 : 0\\]
of the component's part number. This is selected by the designer of the component."]
    #[inline(always)]
    #[must_use]
    pub fn part_0(&mut self) -> Part0W<Cti_Cfg_CsctiCfgPeriphid0Spec> {
        Part0W::new(self, 0)
    }
}
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer specific part number.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_periphid0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_periphid0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgPeriphid0Spec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgPeriphid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_periphid0::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgPeriphid0Spec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_periphid0::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgPeriphid0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_PERIPHID0 to value 0x06"]
impl crate::Resettable for Cti_Cfg_CsctiCfgPeriphid0Spec {
    const RESET_VALUE: u32 = 0x06;
}
