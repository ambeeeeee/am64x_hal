#[doc = "Register `CTI__CFG__CSCTI_CFG_PERIPHID2` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgPeriphid2Spec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_PERIPHID2` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgPeriphid2Spec>;
#[doc = "Field `DES_1` reader - 2:0\\]
Bits 6 : 4 of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
pub type Des1R = crate::FieldReader;
#[doc = "Field `DES_1` writer - 2:0\\]
Bits 6 : 4 of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
pub type Des1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JEDEC` reader - 3:3\\]
Always set. Indicates that a JEDEC assigned value is used"]
pub type JedecR = crate::BitReader;
#[doc = "Field `JEDEC` writer - 3:3\\]
Always set. Indicates that a JEDEC assigned value is used"]
pub type JedecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REVISION` reader - 7:4\\]
The Revision field is an incremental value starting at 0x0 for the first design of this component. This only increases by 1 for both major and minor revisions and is simply used as a look-up to establish the exact major/minor revision."]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - 7:4\\]
The Revision field is an incremental value starting at 0x0 for the first design of this component. This only increases by 1 for both major and minor revisions and is simply used as a look-up to establish the exact major/minor revision."]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Bits 6 : 4 of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
    #[inline(always)]
    pub fn des_1(&self) -> Des1R {
        Des1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Always set. Indicates that a JEDEC assigned value is used"]
    #[inline(always)]
    pub fn jedec(&self) -> JedecR {
        JedecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
The Revision field is an incremental value starting at 0x0 for the first design of this component. This only increases by 1 for both major and minor revisions and is simply used as a look-up to establish the exact major/minor revision."]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Bits 6 : 4 of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
    #[inline(always)]
    #[must_use]
    pub fn des_1(&mut self) -> Des1W<Cti_Cfg_CsctiCfgPeriphid2Spec> {
        Des1W::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Always set. Indicates that a JEDEC assigned value is used"]
    #[inline(always)]
    #[must_use]
    pub fn jedec(&mut self) -> JedecW<Cti_Cfg_CsctiCfgPeriphid2Spec> {
        JedecW::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
The Revision field is an incremental value starting at 0x0 for the first design of this component. This only increases by 1 for both major and minor revisions and is simply used as a look-up to establish the exact major/minor revision."]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<Cti_Cfg_CsctiCfgPeriphid2Spec> {
        RevisionW::new(self, 4)
    }
}
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_periphid2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_periphid2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgPeriphid2Spec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgPeriphid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_periphid2::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgPeriphid2Spec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_periphid2::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgPeriphid2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_PERIPHID2 to value 0x48"]
impl crate::Resettable for Cti_Cfg_CsctiCfgPeriphid2Spec {
    const RESET_VALUE: u32 = 0x48;
}
