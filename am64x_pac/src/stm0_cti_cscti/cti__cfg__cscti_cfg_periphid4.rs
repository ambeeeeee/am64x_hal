#[doc = "Register `CTI__CFG__CSCTI_CFG_PERIPHID4` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgPeriphid4Spec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_PERIPHID4` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgPeriphid4Spec>;
#[doc = "Field `DES_2` reader - 3:0\\]
JEDEC continuation code indicating the designer of the component (along with the identity code)"]
pub type Des2R = crate::FieldReader;
#[doc = "Field `DES_2` writer - 3:0\\]
JEDEC continuation code indicating the designer of the component (along with the identity code)"]
pub type Des2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SIZE` reader - 7:4\\]
This is a 4-bit value that indicates the total contiguous size of the memory window used by this component in powers of 2 from the standard 4KB. If a component only requires the standard 4KB then this should read as 0x0, 4KB only, for 8KB set to 0x1, 16KB == 0x2, 32KB == 0x3, and so on."]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - 7:4\\]
This is a 4-bit value that indicates the total contiguous size of the memory window used by this component in powers of 2 from the standard 4KB. If a component only requires the standard 4KB then this should read as 0x0, 4KB only, for 8KB set to 0x1, 16KB == 0x2, 32KB == 0x3, and so on."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
JEDEC continuation code indicating the designer of the component (along with the identity code)"]
    #[inline(always)]
    pub fn des_2(&self) -> Des2R {
        Des2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
This is a 4-bit value that indicates the total contiguous size of the memory window used by this component in powers of 2 from the standard 4KB. If a component only requires the standard 4KB then this should read as 0x0, 4KB only, for 8KB set to 0x1, 16KB == 0x2, 32KB == 0x3, and so on."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
JEDEC continuation code indicating the designer of the component (along with the identity code)"]
    #[inline(always)]
    #[must_use]
    pub fn des_2(&mut self) -> Des2W<Cti_Cfg_CsctiCfgPeriphid4Spec> {
        Des2W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
This is a 4-bit value that indicates the total contiguous size of the memory window used by this component in powers of 2 from the standard 4KB. If a component only requires the standard 4KB then this should read as 0x0, 4KB only, for 8KB set to 0x1, 16KB == 0x2, 32KB == 0x3, and so on."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<Cti_Cfg_CsctiCfgPeriphid4Spec> {
        SizeW::new(self, 4)
    }
}
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_periphid4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_periphid4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgPeriphid4Spec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgPeriphid4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_periphid4::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgPeriphid4Spec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_periphid4::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgPeriphid4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_PERIPHID4 to value 0x04"]
impl crate::Resettable for Cti_Cfg_CsctiCfgPeriphid4Spec {
    const RESET_VALUE: u32 = 0x04;
}
