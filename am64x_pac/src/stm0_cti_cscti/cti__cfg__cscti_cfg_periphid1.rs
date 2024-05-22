#[doc = "Register `CTI__CFG__CSCTI_CFG_PERIPHID1` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgPeriphid1Spec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_PERIPHID1` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgPeriphid1Spec>;
#[doc = "Field `PART_1` reader - 3:0\\]
Bits \\[11 : 8\\]
of the component's part number. This is selected by the designer of the component."]
pub type Part1R = crate::FieldReader;
#[doc = "Field `PART_1` writer - 3:0\\]
Bits \\[11 : 8\\]
of the component's part number. This is selected by the designer of the component."]
pub type Part1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DES_0` reader - 7:4\\]
Bits 3 : 0 of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
pub type Des0R = crate::FieldReader;
#[doc = "Field `DES_0` writer - 7:4\\]
Bits 3 : 0 of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
pub type Des0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Bits \\[11 : 8\\]
of the component's part number. This is selected by the designer of the component."]
    #[inline(always)]
    pub fn part_1(&self) -> Part1R {
        Part1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Bits 3 : 0 of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
    #[inline(always)]
    pub fn des_0(&self) -> Des0R {
        Des0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Bits \\[11 : 8\\]
of the component's part number. This is selected by the designer of the component."]
    #[inline(always)]
    #[must_use]
    pub fn part_1(&mut self) -> Part1W<Cti_Cfg_CsctiCfgPeriphid1Spec> {
        Part1W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Bits 3 : 0 of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
    #[inline(always)]
    #[must_use]
    pub fn des_0(&mut self) -> Des0W<Cti_Cfg_CsctiCfgPeriphid1Spec> {
        Des0W::new(self, 4)
    }
}
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_periphid1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_periphid1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgPeriphid1Spec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgPeriphid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_periphid1::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgPeriphid1Spec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_periphid1::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgPeriphid1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_PERIPHID1 to value 0x09"]
impl crate::Resettable for Cti_Cfg_CsctiCfgPeriphid1Spec {
    const RESET_VALUE: u32 = 0x09;
}
