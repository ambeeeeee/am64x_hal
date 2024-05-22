#[doc = "Register `CTI__CFG__CSCTI_CFG_CTIINEN7` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCtiinen7Spec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_CTIINEN7` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCtiinen7Spec>;
#[doc = "Field `TRIGINEN` reader - 3:0\\]
Enables a cross trigger event to the corresponding channel when an ctitrigin is activated. There is one bit of the field for each of the four channels. When a 1 is written to a bit in this register, it enables the ctitrigin signal to generate an event on the respective channel of the CTM. For example, TRIGINEN\\[0\\]
set to 1 enables ctitrigin onto channel 0. Writing a 0 to any of the bits in this register disables the ctitrigin signal from generating an event on the respective channel of the CTM. Reading this register returns the programmed value."]
pub type TriginenR = crate::FieldReader;
#[doc = "Field `TRIGINEN` writer - 3:0\\]
Enables a cross trigger event to the corresponding channel when an ctitrigin is activated. There is one bit of the field for each of the four channels. When a 1 is written to a bit in this register, it enables the ctitrigin signal to generate an event on the respective channel of the CTM. For example, TRIGINEN\\[0\\]
set to 1 enables ctitrigin onto channel 0. Writing a 0 to any of the bits in this register disables the ctitrigin signal from generating an event on the respective channel of the CTM. Reading this register returns the programmed value."]
pub type TriginenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Enables a cross trigger event to the corresponding channel when an ctitrigin is activated. There is one bit of the field for each of the four channels. When a 1 is written to a bit in this register, it enables the ctitrigin signal to generate an event on the respective channel of the CTM. For example, TRIGINEN\\[0\\]
set to 1 enables ctitrigin onto channel 0. Writing a 0 to any of the bits in this register disables the ctitrigin signal from generating an event on the respective channel of the CTM. Reading this register returns the programmed value."]
    #[inline(always)]
    pub fn triginen(&self) -> TriginenR {
        TriginenR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Enables a cross trigger event to the corresponding channel when an ctitrigin is activated. There is one bit of the field for each of the four channels. When a 1 is written to a bit in this register, it enables the ctitrigin signal to generate an event on the respective channel of the CTM. For example, TRIGINEN\\[0\\]
set to 1 enables ctitrigin onto channel 0. Writing a 0 to any of the bits in this register disables the ctitrigin signal from generating an event on the respective channel of the CTM. Reading this register returns the programmed value."]
    #[inline(always)]
    #[must_use]
    pub fn triginen(&mut self) -> TriginenW<Cti_Cfg_CsctiCfgCtiinen7Spec> {
        TriginenW::new(self, 0)
    }
}
#[doc = "The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiinen7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiinen7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCtiinen7Spec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCtiinen7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_ctiinen7::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCtiinen7Spec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_ctiinen7::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCtiinen7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_CTIINEN7 to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCtiinen7Spec {
    const RESET_VALUE: u32 = 0;
}
