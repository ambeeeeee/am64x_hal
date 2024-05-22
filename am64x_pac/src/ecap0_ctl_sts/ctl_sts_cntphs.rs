#[doc = "Register `CTL_STS_CNTPHS` reader"]
pub type R = crate::R<CtlStsCntphsSpec>;
#[doc = "Register `CTL_STS_CNTPHS` writer"]
pub type W = crate::W<CtlStsCntphsSpec>;
#[doc = "Field `CNTPHS` reader - 31:0\\]
Counter Phase value register that can be programmed for phase Lag/Lead. This register shadows TSCNT and is loaded into TSCNT upon either a SYNCI event or S/W force via a control bit. Used to achieve Phase control sync with respect to other ECAP and EPWM time-bases."]
pub type CntphsR = crate::FieldReader<u32>;
#[doc = "Field `CNTPHS` writer - 31:0\\]
Counter Phase value register that can be programmed for phase Lag/Lead. This register shadows TSCNT and is loaded into TSCNT upon either a SYNCI event or S/W force via a control bit. Used to achieve Phase control sync with respect to other ECAP and EPWM time-bases."]
pub type CntphsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Counter Phase value register that can be programmed for phase Lag/Lead. This register shadows TSCNT and is loaded into TSCNT upon either a SYNCI event or S/W force via a control bit. Used to achieve Phase control sync with respect to other ECAP and EPWM time-bases."]
    #[inline(always)]
    pub fn cntphs(&self) -> CntphsR {
        CntphsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Counter Phase value register that can be programmed for phase Lag/Lead. This register shadows TSCNT and is loaded into TSCNT upon either a SYNCI event or S/W force via a control bit. Used to achieve Phase control sync with respect to other ECAP and EPWM time-bases."]
    #[inline(always)]
    #[must_use]
    pub fn cntphs(&mut self) -> CntphsW<CtlStsCntphsSpec> {
        CntphsW::new(self, 0)
    }
}
#[doc = "CTL_STS_CNTPHS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_cntphs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_cntphs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlStsCntphsSpec;
impl crate::RegisterSpec for CtlStsCntphsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl_sts_cntphs::R`](R) reader structure"]
impl crate::Readable for CtlStsCntphsSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl_sts_cntphs::W`](W) writer structure"]
impl crate::Writable for CtlStsCntphsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL_STS_CNTPHS to value 0"]
impl crate::Resettable for CtlStsCntphsSpec {
    const RESET_VALUE: u32 = 0;
}
