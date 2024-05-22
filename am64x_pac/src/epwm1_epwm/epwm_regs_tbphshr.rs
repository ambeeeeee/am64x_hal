#[doc = "Register `EPWM_REGS_TBPHSHR` reader"]
pub type R = crate::R<EpwmRegsTbphshrSpec>;
#[doc = "Register `EPWM_REGS_TBPHSHR` writer"]
pub type W = crate::W<EpwmRegsTbphshrSpec>;
#[doc = "Field `TBPHSH` reader - 15:8\\]
Time-base phase high-resolution bits"]
pub type TbphshR = crate::FieldReader;
#[doc = "Field `TBPHSH` writer - 15:8\\]
Time-base phase high-resolution bits"]
pub type TbphshW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - 15:8\\]
Time-base phase high-resolution bits"]
    #[inline(always)]
    pub fn tbphsh(&self) -> TbphshR {
        TbphshR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15:8\\]
Time-base phase high-resolution bits"]
    #[inline(always)]
    #[must_use]
    pub fn tbphsh(&mut self) -> TbphshW<EpwmRegsTbphshrSpec> {
        TbphshW::new(self, 8)
    }
}
#[doc = "Time Base Phase High Resolution Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tbphshr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tbphshr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsTbphshrSpec;
impl crate::RegisterSpec for EpwmRegsTbphshrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_tbphshr::R`](R) reader structure"]
impl crate::Readable for EpwmRegsTbphshrSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_tbphshr::W`](W) writer structure"]
impl crate::Writable for EpwmRegsTbphshrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_TBPHSHR to value 0"]
impl crate::Resettable for EpwmRegsTbphshrSpec {
    const RESET_VALUE: u16 = 0;
}
