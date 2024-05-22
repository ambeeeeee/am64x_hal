#[doc = "Register `CTI__CFG__CSCTI_CFG_CTITRIGOUTSTATUS` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCtitrigoutstatusSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_CTITRIGOUTSTATUS` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCtitrigoutstatusSpec>;
#[doc = "Field `TRIGOUTSTATUS` reader - 7:0\\]
Shows the status of the ctitrigout outputs. 1 = ctitrigout is active. 0 = ctitrigout is inactive. There is one bit of the field for each trigger output."]
pub type TrigoutstatusR = crate::FieldReader;
#[doc = "Field `TRIGOUTSTATUS` writer - 7:0\\]
Shows the status of the ctitrigout outputs. 1 = ctitrigout is active. 0 = ctitrigout is inactive. There is one bit of the field for each trigger output."]
pub type TrigoutstatusW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Shows the status of the ctitrigout outputs. 1 = ctitrigout is active. 0 = ctitrigout is inactive. There is one bit of the field for each trigger output."]
    #[inline(always)]
    pub fn trigoutstatus(&self) -> TrigoutstatusR {
        TrigoutstatusR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Shows the status of the ctitrigout outputs. 1 = ctitrigout is active. 0 = ctitrigout is inactive. There is one bit of the field for each trigger output."]
    #[inline(always)]
    #[must_use]
    pub fn trigoutstatus(&mut self) -> TrigoutstatusW<Cti_Cfg_CsctiCfgCtitrigoutstatusSpec> {
        TrigoutstatusW::new(self, 0)
    }
}
#[doc = "The CTI Trigger Out Status Register provides the status of the ctitrigout outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctitrigoutstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctitrigoutstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCtitrigoutstatusSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCtitrigoutstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_ctitrigoutstatus::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCtitrigoutstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_ctitrigoutstatus::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCtitrigoutstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_CTITRIGOUTSTATUS to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCtitrigoutstatusSpec {
    const RESET_VALUE: u32 = 0;
}
