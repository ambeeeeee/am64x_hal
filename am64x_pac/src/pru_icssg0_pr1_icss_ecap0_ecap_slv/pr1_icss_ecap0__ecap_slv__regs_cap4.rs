#[doc = "Register `PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP4` reader"]
pub type R = crate::R<Pr1IcssEcap0_EcapSlv_RegsCap4Spec>;
#[doc = "Register `PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP4` writer"]
pub type W = crate::W<Pr1IcssEcap0_EcapSlv_RegsCap4Spec>;
#[doc = "Field `CAP4` reader - 31:0\\]
IN CMP MODE THIS IS A TIME-STAMP CAPTURE REGISTERIN APMW MODE THIS IS THE COMPARE SHADOW \\[ACMP\\]
REGISTER USER UPDATES THE PWM COMPARE VALUE VIA THIS REGISTER IN THIS MODE CAP4 \\[ACMP\\]
SHADOWS CAP2"]
pub type Cap4R = crate::FieldReader<u32>;
#[doc = "Field `CAP4` writer - 31:0\\]
IN CMP MODE THIS IS A TIME-STAMP CAPTURE REGISTERIN APMW MODE THIS IS THE COMPARE SHADOW \\[ACMP\\]
REGISTER USER UPDATES THE PWM COMPARE VALUE VIA THIS REGISTER IN THIS MODE CAP4 \\[ACMP\\]
SHADOWS CAP2"]
pub type Cap4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
IN CMP MODE THIS IS A TIME-STAMP CAPTURE REGISTERIN APMW MODE THIS IS THE COMPARE SHADOW \\[ACMP\\]
REGISTER USER UPDATES THE PWM COMPARE VALUE VIA THIS REGISTER IN THIS MODE CAP4 \\[ACMP\\]
SHADOWS CAP2"]
    #[inline(always)]
    pub fn cap4(&self) -> Cap4R {
        Cap4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
IN CMP MODE THIS IS A TIME-STAMP CAPTURE REGISTERIN APMW MODE THIS IS THE COMPARE SHADOW \\[ACMP\\]
REGISTER USER UPDATES THE PWM COMPARE VALUE VIA THIS REGISTER IN THIS MODE CAP4 \\[ACMP\\]
SHADOWS CAP2"]
    #[inline(always)]
    #[must_use]
    pub fn cap4(&mut self) -> Cap4W<Pr1IcssEcap0_EcapSlv_RegsCap4Spec> {
        Cap4W::new(self, 0)
    }
}
#[doc = "CAPTURE-4 REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_cap4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_cap4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssEcap0_EcapSlv_RegsCap4Spec;
impl crate::RegisterSpec for Pr1IcssEcap0_EcapSlv_RegsCap4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_ecap0__ecap_slv__regs_cap4::R`](R) reader structure"]
impl crate::Readable for Pr1IcssEcap0_EcapSlv_RegsCap4Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_ecap0__ecap_slv__regs_cap4::W`](W) writer structure"]
impl crate::Writable for Pr1IcssEcap0_EcapSlv_RegsCap4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP4 to value 0"]
impl crate::Resettable for Pr1IcssEcap0_EcapSlv_RegsCap4Spec {
    const RESET_VALUE: u32 = 0;
}
