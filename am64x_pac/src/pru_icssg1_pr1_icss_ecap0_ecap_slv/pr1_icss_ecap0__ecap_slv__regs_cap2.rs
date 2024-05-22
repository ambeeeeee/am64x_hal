#[doc = "Register `PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP2` reader"]
pub type R = crate::R<Pr1IcssEcap0_EcapSlv_RegsCap2Spec>;
#[doc = "Register `PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP2` writer"]
pub type W = crate::W<Pr1IcssEcap0_EcapSlv_RegsCap2Spec>;
#[doc = "Field `CAP2` reader - 31:0\\]
THIS REGISTER CAN BE LOADED \\[WRITTEN\\]
BY :TIME-STAMP \\[IE COUNTER VALUE\\]
DURING A CAPTURE EVENTS/W MAY BE USEFUL FOR TEST PURPOSESACMP SHADOW REGISTER \\[IE CAP4\\]
WHEN USED IN APWM MODE"]
pub type Cap2R = crate::FieldReader<u32>;
#[doc = "Field `CAP2` writer - 31:0\\]
THIS REGISTER CAN BE LOADED \\[WRITTEN\\]
BY :TIME-STAMP \\[IE COUNTER VALUE\\]
DURING A CAPTURE EVENTS/W MAY BE USEFUL FOR TEST PURPOSESACMP SHADOW REGISTER \\[IE CAP4\\]
WHEN USED IN APWM MODE"]
pub type Cap2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
THIS REGISTER CAN BE LOADED \\[WRITTEN\\]
BY :TIME-STAMP \\[IE COUNTER VALUE\\]
DURING A CAPTURE EVENTS/W MAY BE USEFUL FOR TEST PURPOSESACMP SHADOW REGISTER \\[IE CAP4\\]
WHEN USED IN APWM MODE"]
    #[inline(always)]
    pub fn cap2(&self) -> Cap2R {
        Cap2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
THIS REGISTER CAN BE LOADED \\[WRITTEN\\]
BY :TIME-STAMP \\[IE COUNTER VALUE\\]
DURING A CAPTURE EVENTS/W MAY BE USEFUL FOR TEST PURPOSESACMP SHADOW REGISTER \\[IE CAP4\\]
WHEN USED IN APWM MODE"]
    #[inline(always)]
    #[must_use]
    pub fn cap2(&mut self) -> Cap2W<Pr1IcssEcap0_EcapSlv_RegsCap2Spec> {
        Cap2W::new(self, 0)
    }
}
#[doc = "CAPTURE-2 REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_cap2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_cap2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssEcap0_EcapSlv_RegsCap2Spec;
impl crate::RegisterSpec for Pr1IcssEcap0_EcapSlv_RegsCap2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_ecap0__ecap_slv__regs_cap2::R`](R) reader structure"]
impl crate::Readable for Pr1IcssEcap0_EcapSlv_RegsCap2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_ecap0__ecap_slv__regs_cap2::W`](W) writer structure"]
impl crate::Writable for Pr1IcssEcap0_EcapSlv_RegsCap2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP2 to value 0"]
impl crate::Resettable for Pr1IcssEcap0_EcapSlv_RegsCap2Spec {
    const RESET_VALUE: u32 = 0;
}
