#[doc = "Register `CTL_STS_TSCNT` reader"]
pub type R = crate::R<CtlStsTscntSpec>;
#[doc = "Register `CTL_STS_TSCNT` writer"]
pub type W = crate::W<CtlStsTscntSpec>;
#[doc = "Field `TSCNT` reader - 31:0\\]
Active 32 bit Counter register which is used as the Capture time-base"]
pub type TscntR = crate::FieldReader<u32>;
#[doc = "Field `TSCNT` writer - 31:0\\]
Active 32 bit Counter register which is used as the Capture time-base"]
pub type TscntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Active 32 bit Counter register which is used as the Capture time-base"]
    #[inline(always)]
    pub fn tscnt(&self) -> TscntR {
        TscntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Active 32 bit Counter register which is used as the Capture time-base"]
    #[inline(always)]
    #[must_use]
    pub fn tscnt(&mut self) -> TscntW<CtlStsTscntSpec> {
        TscntW::new(self, 0)
    }
}
#[doc = "CTL_STS_TSCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_tscnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_tscnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlStsTscntSpec;
impl crate::RegisterSpec for CtlStsTscntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl_sts_tscnt::R`](R) reader structure"]
impl crate::Readable for CtlStsTscntSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl_sts_tscnt::W`](W) writer structure"]
impl crate::Writable for CtlStsTscntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL_STS_TSCNT to value 0"]
impl crate::Resettable for CtlStsTscntSpec {
    const RESET_VALUE: u32 = 0;
}
