#[doc = "Register `CTI__CFG__CSCTI_CFG_ITTRIGINACK` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgIttriginackSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_ITTRIGINACK` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgIttriginackSpec>;
#[doc = "Field `CTTRIGINACK` reader - 7:0\\]
Set the value of the CTTRIGINACK outputs."]
pub type CttriginackR = crate::FieldReader;
#[doc = "Field `CTTRIGINACK` writer - 7:0\\]
Set the value of the CTTRIGINACK outputs."]
pub type CttriginackW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Set the value of the CTTRIGINACK outputs."]
    #[inline(always)]
    pub fn cttriginack(&self) -> CttriginackR {
        CttriginackR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Set the value of the CTTRIGINACK outputs."]
    #[inline(always)]
    #[must_use]
    pub fn cttriginack(&mut self) -> CttriginackW<Cti_Cfg_CsctiCfgIttriginackSpec> {
        CttriginackW::new(self, 0)
    }
}
#[doc = "This register is a write-only register. It can be used to set the value of the CTTRIGINACK outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ittriginack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ittriginack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgIttriginackSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgIttriginackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_ittriginack::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgIttriginackSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_ittriginack::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgIttriginackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_ITTRIGINACK to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgIttriginackSpec {
    const RESET_VALUE: u32 = 0;
}
