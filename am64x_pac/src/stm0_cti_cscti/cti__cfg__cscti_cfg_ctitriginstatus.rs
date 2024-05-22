#[doc = "Register `CTI__CFG__CSCTI_CFG_CTITRIGINSTATUS` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCtitriginstatusSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_CTITRIGINSTATUS` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCtitriginstatusSpec>;
#[doc = "Field `TRIGINSTATUS` reader - 7:0\\]
Shows the status of the ctitrigin inputs. 1 = ctitrigin is active. 0 = ctitrigin is inactive. Because the register provides a view of the raw ctitrigin inputs, the reset value is unknown. There is one bit of the field for each trigger input."]
pub type TriginstatusR = crate::FieldReader;
#[doc = "Field `TRIGINSTATUS` writer - 7:0\\]
Shows the status of the ctitrigin inputs. 1 = ctitrigin is active. 0 = ctitrigin is inactive. Because the register provides a view of the raw ctitrigin inputs, the reset value is unknown. There is one bit of the field for each trigger input."]
pub type TriginstatusW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Shows the status of the ctitrigin inputs. 1 = ctitrigin is active. 0 = ctitrigin is inactive. Because the register provides a view of the raw ctitrigin inputs, the reset value is unknown. There is one bit of the field for each trigger input."]
    #[inline(always)]
    pub fn triginstatus(&self) -> TriginstatusR {
        TriginstatusR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Shows the status of the ctitrigin inputs. 1 = ctitrigin is active. 0 = ctitrigin is inactive. Because the register provides a view of the raw ctitrigin inputs, the reset value is unknown. There is one bit of the field for each trigger input."]
    #[inline(always)]
    #[must_use]
    pub fn triginstatus(&mut self) -> TriginstatusW<Cti_Cfg_CsctiCfgCtitriginstatusSpec> {
        TriginstatusW::new(self, 0)
    }
}
#[doc = "The CTI Trigger In Status Register provides the status of the ctitrigin inputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctitriginstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctitriginstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCtitriginstatusSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCtitriginstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_ctitriginstatus::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCtitriginstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_ctitriginstatus::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCtitriginstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_CTITRIGINSTATUS to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCtitriginstatusSpec {
    const RESET_VALUE: u32 = 0;
}
