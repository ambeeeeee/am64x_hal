#[doc = "Register `CTI__CFG__CSCTI_CFG_DEVTYPE` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgDevtypeSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_DEVTYPE` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgDevtypeSpec>;
#[doc = "Field `MAJOR_TYPE` reader - 3:0\\]
Major classification grouping for this debug/trace component"]
pub type MajorTypeR = crate::FieldReader;
#[doc = "Field `MAJOR_TYPE` writer - 3:0\\]
Major classification grouping for this debug/trace component"]
pub type MajorTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SUB_TYPE` reader - 7:4\\]
Sub-classification within the major category"]
pub type SubTypeR = crate::FieldReader;
#[doc = "Field `SUB_TYPE` writer - 7:4\\]
Sub-classification within the major category"]
pub type SubTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Major classification grouping for this debug/trace component"]
    #[inline(always)]
    pub fn major_type(&self) -> MajorTypeR {
        MajorTypeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Sub-classification within the major category"]
    #[inline(always)]
    pub fn sub_type(&self) -> SubTypeR {
        SubTypeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Major classification grouping for this debug/trace component"]
    #[inline(always)]
    #[must_use]
    pub fn major_type(&mut self) -> MajorTypeW<Cti_Cfg_CsctiCfgDevtypeSpec> {
        MajorTypeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Sub-classification within the major category"]
    #[inline(always)]
    #[must_use]
    pub fn sub_type(&mut self) -> SubTypeW<Cti_Cfg_CsctiCfgDevtypeSpec> {
        SubTypeW::new(self, 4)
    }
}
#[doc = "It provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_devtype::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_devtype::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgDevtypeSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgDevtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_devtype::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgDevtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_devtype::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgDevtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_DEVTYPE to value 0x14"]
impl crate::Resettable for Cti_Cfg_CsctiCfgDevtypeSpec {
    const RESET_VALUE: u32 = 0x14;
}
