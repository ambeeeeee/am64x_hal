#[doc = "Register `PR1_ICSS_ECAP0__ECAP_SLV__REGS_CNTPHS` reader"]
pub type R = crate::R<Pr1IcssEcap0_EcapSlv_RegsCntphsSpec>;
#[doc = "Register `PR1_ICSS_ECAP0__ECAP_SLV__REGS_CNTPHS` writer"]
pub type W = crate::W<Pr1IcssEcap0_EcapSlv_RegsCntphsSpec>;
#[doc = "Field `CNTPHS` reader - 31:0\\]
COUNTER PHASE VALUE REGISTER THAT CAN BE PROGRAMMED FOR PHASE LAG/LEAD THIS REGISTER SHADOWS TSCNT AND IS LOADED INTO TSCNT UPON EITHER A SYNCI EVENT OR S/W FORCE VIA A CONTROL BITUSED TO ACHIEVE PHASE CONTROL SYNC WITH RESPECT TO OTHER ECAP AND EPWM TIME-BASES"]
pub type CntphsR = crate::FieldReader<u32>;
#[doc = "Field `CNTPHS` writer - 31:0\\]
COUNTER PHASE VALUE REGISTER THAT CAN BE PROGRAMMED FOR PHASE LAG/LEAD THIS REGISTER SHADOWS TSCNT AND IS LOADED INTO TSCNT UPON EITHER A SYNCI EVENT OR S/W FORCE VIA A CONTROL BITUSED TO ACHIEVE PHASE CONTROL SYNC WITH RESPECT TO OTHER ECAP AND EPWM TIME-BASES"]
pub type CntphsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
COUNTER PHASE VALUE REGISTER THAT CAN BE PROGRAMMED FOR PHASE LAG/LEAD THIS REGISTER SHADOWS TSCNT AND IS LOADED INTO TSCNT UPON EITHER A SYNCI EVENT OR S/W FORCE VIA A CONTROL BITUSED TO ACHIEVE PHASE CONTROL SYNC WITH RESPECT TO OTHER ECAP AND EPWM TIME-BASES"]
    #[inline(always)]
    pub fn cntphs(&self) -> CntphsR {
        CntphsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
COUNTER PHASE VALUE REGISTER THAT CAN BE PROGRAMMED FOR PHASE LAG/LEAD THIS REGISTER SHADOWS TSCNT AND IS LOADED INTO TSCNT UPON EITHER A SYNCI EVENT OR S/W FORCE VIA A CONTROL BITUSED TO ACHIEVE PHASE CONTROL SYNC WITH RESPECT TO OTHER ECAP AND EPWM TIME-BASES"]
    #[inline(always)]
    #[must_use]
    pub fn cntphs(&mut self) -> CntphsW<Pr1IcssEcap0_EcapSlv_RegsCntphsSpec> {
        CntphsW::new(self, 0)
    }
}
#[doc = "COUNTER PHASE CONTROL REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_cntphs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_cntphs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssEcap0_EcapSlv_RegsCntphsSpec;
impl crate::RegisterSpec for Pr1IcssEcap0_EcapSlv_RegsCntphsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_ecap0__ecap_slv__regs_cntphs::R`](R) reader structure"]
impl crate::Readable for Pr1IcssEcap0_EcapSlv_RegsCntphsSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_ecap0__ecap_slv__regs_cntphs::W`](W) writer structure"]
impl crate::Writable for Pr1IcssEcap0_EcapSlv_RegsCntphsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_ECAP0__ECAP_SLV__REGS_CNTPHS to value 0"]
impl crate::Resettable for Pr1IcssEcap0_EcapSlv_RegsCntphsSpec {
    const RESET_VALUE: u32 = 0;
}
