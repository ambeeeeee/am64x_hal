#[doc = "Register `PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP3` reader"]
pub type R = crate::R<Pr1IcssEcap0_EcapSlv_RegsCap3Spec>;
#[doc = "Register `PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP3` writer"]
pub type W = crate::W<Pr1IcssEcap0_EcapSlv_RegsCap3Spec>;
#[doc = "Field `CAP3` reader - 31:0\\]
IN CMP MODE THIS IS A TIME-STAMP CAPTURE REGISTERIN APMW MODE THIS IS THE PERIOD SHADOW \\[APER\\]
REGISTER USER UPDATES THE PWM PERIOD VALUE VIA THIS REGISTER IN THIS MODE CAP3 \\[APRD\\]
SHADOWS CAP1"]
pub type Cap3R = crate::FieldReader<u32>;
#[doc = "Field `CAP3` writer - 31:0\\]
IN CMP MODE THIS IS A TIME-STAMP CAPTURE REGISTERIN APMW MODE THIS IS THE PERIOD SHADOW \\[APER\\]
REGISTER USER UPDATES THE PWM PERIOD VALUE VIA THIS REGISTER IN THIS MODE CAP3 \\[APRD\\]
SHADOWS CAP1"]
pub type Cap3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
IN CMP MODE THIS IS A TIME-STAMP CAPTURE REGISTERIN APMW MODE THIS IS THE PERIOD SHADOW \\[APER\\]
REGISTER USER UPDATES THE PWM PERIOD VALUE VIA THIS REGISTER IN THIS MODE CAP3 \\[APRD\\]
SHADOWS CAP1"]
    #[inline(always)]
    pub fn cap3(&self) -> Cap3R {
        Cap3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
IN CMP MODE THIS IS A TIME-STAMP CAPTURE REGISTERIN APMW MODE THIS IS THE PERIOD SHADOW \\[APER\\]
REGISTER USER UPDATES THE PWM PERIOD VALUE VIA THIS REGISTER IN THIS MODE CAP3 \\[APRD\\]
SHADOWS CAP1"]
    #[inline(always)]
    #[must_use]
    pub fn cap3(&mut self) -> Cap3W<Pr1IcssEcap0_EcapSlv_RegsCap3Spec> {
        Cap3W::new(self, 0)
    }
}
#[doc = "CAPTURE-3 REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_cap3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_cap3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssEcap0_EcapSlv_RegsCap3Spec;
impl crate::RegisterSpec for Pr1IcssEcap0_EcapSlv_RegsCap3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_ecap0__ecap_slv__regs_cap3::R`](R) reader structure"]
impl crate::Readable for Pr1IcssEcap0_EcapSlv_RegsCap3Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_ecap0__ecap_slv__regs_cap3::W`](W) writer structure"]
impl crate::Writable for Pr1IcssEcap0_EcapSlv_RegsCap3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP3 to value 0"]
impl crate::Resettable for Pr1IcssEcap0_EcapSlv_RegsCap3Spec {
    const RESET_VALUE: u32 = 0;
}
