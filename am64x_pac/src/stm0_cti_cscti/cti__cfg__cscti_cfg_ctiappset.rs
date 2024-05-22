#[doc = "Register `CTI__CFG__CSCTI_CFG_CTIAPPSET` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCtiappsetSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_CTIAPPSET` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCtiappsetSpec>;
#[doc = "Field `APPSET` reader - 3:0\\]
Setting a bit HIGH generates a channel event for the selected channel. There is one bit of the register for each channel. Read : 0 = application trigger inactive (reset). 1 = application trigger active. Write : 0 = no effect. 1 = generate channel event."]
pub type AppsetR = crate::FieldReader;
#[doc = "Field `APPSET` writer - 3:0\\]
Setting a bit HIGH generates a channel event for the selected channel. There is one bit of the register for each channel. Read : 0 = application trigger inactive (reset). 1 = application trigger active. Write : 0 = no effect. 1 = generate channel event."]
pub type AppsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Setting a bit HIGH generates a channel event for the selected channel. There is one bit of the register for each channel. Read : 0 = application trigger inactive (reset). 1 = application trigger active. Write : 0 = no effect. 1 = generate channel event."]
    #[inline(always)]
    pub fn appset(&self) -> AppsetR {
        AppsetR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Setting a bit HIGH generates a channel event for the selected channel. There is one bit of the register for each channel. Read : 0 = application trigger inactive (reset). 1 = application trigger active. Write : 0 = no effect. 1 = generate channel event."]
    #[inline(always)]
    #[must_use]
    pub fn appset(&mut self) -> AppsetW<Cti_Cfg_CsctiCfgCtiappsetSpec> {
        AppsetW::new(self, 0)
    }
}
#[doc = "The CTI Application Trigger Set Register is read/write. A write to this register causes a channel event to be raised, corresponding to the bit written to.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiappset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiappset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCtiappsetSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCtiappsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_ctiappset::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCtiappsetSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_ctiappset::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCtiappsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_CTIAPPSET to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCtiappsetSpec {
    const RESET_VALUE: u32 = 0;
}
