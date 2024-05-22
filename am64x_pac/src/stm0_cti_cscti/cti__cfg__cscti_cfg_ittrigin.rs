#[doc = "Register `CTI__CFG__CSCTI_CFG_ITTRIGIN` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgIttriginSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_ITTRIGIN` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgIttriginSpec>;
#[doc = "Field `CTTRIGIN` reader - 7:0\\]
Read the values of the CTTRIGIN inputs."]
pub type CttriginR = crate::FieldReader;
#[doc = "Field `CTTRIGIN` writer - 7:0\\]
Read the values of the CTTRIGIN inputs."]
pub type CttriginW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Read the values of the CTTRIGIN inputs."]
    #[inline(always)]
    pub fn cttrigin(&self) -> CttriginR {
        CttriginR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Read the values of the CTTRIGIN inputs."]
    #[inline(always)]
    #[must_use]
    pub fn cttrigin(&mut self) -> CttriginW<Cti_Cfg_CsctiCfgIttriginSpec> {
        CttriginW::new(self, 0)
    }
}
#[doc = "This register is a read-only register. It can be used to read the values of the CTTRIGIN inputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ittrigin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ittrigin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgIttriginSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgIttriginSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_ittrigin::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgIttriginSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_ittrigin::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgIttriginSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_ITTRIGIN to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgIttriginSpec {
    const RESET_VALUE: u32 = 0;
}
