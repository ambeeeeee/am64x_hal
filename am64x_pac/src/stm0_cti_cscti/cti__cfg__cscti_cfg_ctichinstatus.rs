#[doc = "Register `CTI__CFG__CSCTI_CFG_CTICHINSTATUS` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCtichinstatusSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_CTICHINSTATUS` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCtichinstatusSpec>;
#[doc = "Field `CTICHINSTATUS` reader - 3:0\\]
Shows the status of the ctichin inputs. 1 = ctichin is active. 0 = ctichin is inactive. Because the register provides a view of the raw ctichin inputs, the reset value is unknown. There is one bit of the field for each channel input."]
pub type CtichinstatusR = crate::FieldReader;
#[doc = "Field `CTICHINSTATUS` writer - 3:0\\]
Shows the status of the ctichin inputs. 1 = ctichin is active. 0 = ctichin is inactive. Because the register provides a view of the raw ctichin inputs, the reset value is unknown. There is one bit of the field for each channel input."]
pub type CtichinstatusW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Shows the status of the ctichin inputs. 1 = ctichin is active. 0 = ctichin is inactive. Because the register provides a view of the raw ctichin inputs, the reset value is unknown. There is one bit of the field for each channel input."]
    #[inline(always)]
    pub fn ctichinstatus(&self) -> CtichinstatusR {
        CtichinstatusR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Shows the status of the ctichin inputs. 1 = ctichin is active. 0 = ctichin is inactive. Because the register provides a view of the raw ctichin inputs, the reset value is unknown. There is one bit of the field for each channel input."]
    #[inline(always)]
    #[must_use]
    pub fn ctichinstatus(&mut self) -> CtichinstatusW<Cti_Cfg_CsctiCfgCtichinstatusSpec> {
        CtichinstatusW::new(self, 0)
    }
}
#[doc = "The CTI Channel In Status Register provides the status of the ctichin inputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctichinstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctichinstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCtichinstatusSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCtichinstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_ctichinstatus::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCtichinstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_ctichinstatus::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCtichinstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_CTICHINSTATUS to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCtichinstatusSpec {
    const RESET_VALUE: u32 = 0;
}
