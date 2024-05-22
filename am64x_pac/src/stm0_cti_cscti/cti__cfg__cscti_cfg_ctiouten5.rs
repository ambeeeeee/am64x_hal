#[doc = "Register `CTI__CFG__CSCTI_CFG_CTIOUTEN5` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCtiouten5Spec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_CTIOUTEN5` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCtiouten5Spec>;
#[doc = "Field `TRIGOUTEN` reader - 3:0\\]
Changing the value of this bit from a 0 to a 1 enables a channel event for the corresponding channel to generate a ctitrigout\\[5\\]
output. There is one bit of the field for each of the four channels. When a 1 is written to a bit in this register, the channel input (ctichin) from the CTM is routed to the ctitrigout output. For example, enabling bit 0 enables ctichin\\[0\\]
to cause a trigger event on the ctitrigout\\[5\\]
output. When a 0 is written to any of the bits in this register, the channel input (ctichin) from the CTM is not routed to the ctitrigout output. Reading this register returns the programmed value."]
pub type TrigoutenR = crate::FieldReader;
#[doc = "Field `TRIGOUTEN` writer - 3:0\\]
Changing the value of this bit from a 0 to a 1 enables a channel event for the corresponding channel to generate a ctitrigout\\[5\\]
output. There is one bit of the field for each of the four channels. When a 1 is written to a bit in this register, the channel input (ctichin) from the CTM is routed to the ctitrigout output. For example, enabling bit 0 enables ctichin\\[0\\]
to cause a trigger event on the ctitrigout\\[5\\]
output. When a 0 is written to any of the bits in this register, the channel input (ctichin) from the CTM is not routed to the ctitrigout output. Reading this register returns the programmed value."]
pub type TrigoutenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Changing the value of this bit from a 0 to a 1 enables a channel event for the corresponding channel to generate a ctitrigout\\[5\\]
output. There is one bit of the field for each of the four channels. When a 1 is written to a bit in this register, the channel input (ctichin) from the CTM is routed to the ctitrigout output. For example, enabling bit 0 enables ctichin\\[0\\]
to cause a trigger event on the ctitrigout\\[5\\]
output. When a 0 is written to any of the bits in this register, the channel input (ctichin) from the CTM is not routed to the ctitrigout output. Reading this register returns the programmed value."]
    #[inline(always)]
    pub fn trigouten(&self) -> TrigoutenR {
        TrigoutenR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Changing the value of this bit from a 0 to a 1 enables a channel event for the corresponding channel to generate a ctitrigout\\[5\\]
output. There is one bit of the field for each of the four channels. When a 1 is written to a bit in this register, the channel input (ctichin) from the CTM is routed to the ctitrigout output. For example, enabling bit 0 enables ctichin\\[0\\]
to cause a trigger event on the ctitrigout\\[5\\]
output. When a 0 is written to any of the bits in this register, the channel input (ctichin) from the CTM is not routed to the ctitrigout output. Reading this register returns the programmed value."]
    #[inline(always)]
    #[must_use]
    pub fn trigouten(&mut self) -> TrigoutenW<Cti_Cfg_CsctiCfgCtiouten5Spec> {
        TrigoutenW::new(self, 0)
    }
}
#[doc = "The CTI Channel to Trigger 5 Enable Registers define which channels can generate a ctitrigout\\[5\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiouten5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiouten5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCtiouten5Spec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCtiouten5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_ctiouten5::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCtiouten5Spec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_ctiouten5::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCtiouten5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_CTIOUTEN5 to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCtiouten5Spec {
    const RESET_VALUE: u32 = 0;
}
