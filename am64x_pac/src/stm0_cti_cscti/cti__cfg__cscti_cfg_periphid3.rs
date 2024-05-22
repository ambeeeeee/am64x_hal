#[doc = "Register `CTI__CFG__CSCTI_CFG_PERIPHID3` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgPeriphid3Spec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_PERIPHID3` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgPeriphid3Spec>;
#[doc = "Field `CMOD` reader - 3:0\\]
Where the component is reusable IP, this value indicates if the customer has modified the behavior of the component. In most cases this field is zero."]
pub type CmodR = crate::FieldReader;
#[doc = "Field `CMOD` writer - 3:0\\]
Where the component is reusable IP, this value indicates if the customer has modified the behavior of the component. In most cases this field is zero."]
pub type CmodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REVAND` reader - 7:4\\]
This field indicates minor errata fixes specific to this design, for example metal fixes after implementation. In most cases this field is zero. It is recommended that component designers ensure this field can be changed by a metal fix if required, for example by driving it from registers that reset to zero."]
pub type RevandR = crate::FieldReader;
#[doc = "Field `REVAND` writer - 7:4\\]
This field indicates minor errata fixes specific to this design, for example metal fixes after implementation. In most cases this field is zero. It is recommended that component designers ensure this field can be changed by a metal fix if required, for example by driving it from registers that reset to zero."]
pub type RevandW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Where the component is reusable IP, this value indicates if the customer has modified the behavior of the component. In most cases this field is zero."]
    #[inline(always)]
    pub fn cmod(&self) -> CmodR {
        CmodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
This field indicates minor errata fixes specific to this design, for example metal fixes after implementation. In most cases this field is zero. It is recommended that component designers ensure this field can be changed by a metal fix if required, for example by driving it from registers that reset to zero."]
    #[inline(always)]
    pub fn revand(&self) -> RevandR {
        RevandR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Where the component is reusable IP, this value indicates if the customer has modified the behavior of the component. In most cases this field is zero."]
    #[inline(always)]
    #[must_use]
    pub fn cmod(&mut self) -> CmodW<Cti_Cfg_CsctiCfgPeriphid3Spec> {
        CmodW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
This field indicates minor errata fixes specific to this design, for example metal fixes after implementation. In most cases this field is zero. It is recommended that component designers ensure this field can be changed by a metal fix if required, for example by driving it from registers that reset to zero."]
    #[inline(always)]
    #[must_use]
    pub fn revand(&mut self) -> RevandW<Cti_Cfg_CsctiCfgPeriphid3Spec> {
        RevandW::new(self, 4)
    }
}
#[doc = "Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer Modified fields.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_periphid3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_periphid3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgPeriphid3Spec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgPeriphid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_periphid3::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgPeriphid3Spec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_periphid3::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgPeriphid3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_PERIPHID3 to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgPeriphid3Spec {
    const RESET_VALUE: u32 = 0;
}
