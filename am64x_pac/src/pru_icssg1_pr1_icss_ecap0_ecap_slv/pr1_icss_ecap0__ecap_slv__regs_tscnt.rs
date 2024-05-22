#[doc = "Register `PR1_ICSS_ECAP0__ECAP_SLV__REGS_TSCNT` reader"]
pub type R = crate::R<Pr1IcssEcap0_EcapSlv_RegsTscntSpec>;
#[doc = "Register `PR1_ICSS_ECAP0__ECAP_SLV__REGS_TSCNT` writer"]
pub type W = crate::W<Pr1IcssEcap0_EcapSlv_RegsTscntSpec>;
#[doc = "Field `TSCNT` reader - 31:0\\]
ACTIVE 32 BIT COUNTER REGISTER WHICH IS USED AS THE CAPTURE TIME-BASE"]
pub type TscntR = crate::FieldReader<u32>;
#[doc = "Field `TSCNT` writer - 31:0\\]
ACTIVE 32 BIT COUNTER REGISTER WHICH IS USED AS THE CAPTURE TIME-BASE"]
pub type TscntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ACTIVE 32 BIT COUNTER REGISTER WHICH IS USED AS THE CAPTURE TIME-BASE"]
    #[inline(always)]
    pub fn tscnt(&self) -> TscntR {
        TscntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ACTIVE 32 BIT COUNTER REGISTER WHICH IS USED AS THE CAPTURE TIME-BASE"]
    #[inline(always)]
    #[must_use]
    pub fn tscnt(&mut self) -> TscntW<Pr1IcssEcap0_EcapSlv_RegsTscntSpec> {
        TscntW::new(self, 0)
    }
}
#[doc = "TIME STAMP COUNTER REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_tscnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_tscnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssEcap0_EcapSlv_RegsTscntSpec;
impl crate::RegisterSpec for Pr1IcssEcap0_EcapSlv_RegsTscntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_ecap0__ecap_slv__regs_tscnt::R`](R) reader structure"]
impl crate::Readable for Pr1IcssEcap0_EcapSlv_RegsTscntSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_ecap0__ecap_slv__regs_tscnt::W`](W) writer structure"]
impl crate::Writable for Pr1IcssEcap0_EcapSlv_RegsTscntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_ECAP0__ECAP_SLV__REGS_TSCNT to value 0"]
impl crate::Resettable for Pr1IcssEcap0_EcapSlv_RegsTscntSpec {
    const RESET_VALUE: u32 = 0;
}
