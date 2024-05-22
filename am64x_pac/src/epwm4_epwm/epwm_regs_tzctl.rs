#[doc = "Register `EPWM_REGS_TZCTL` reader"]
pub type R = crate::R<EpwmRegsTzctlSpec>;
#[doc = "Register `EPWM_REGS_TZCTL` writer"]
pub type W = crate::W<EpwmRegsTzctlSpec>;
#[doc = "Field `TZA` reader - 1:0\\]
When a trip event occurs the following action is taken on output EPWMxA Which trip-zone pins can cause an event is defined in the TZSEL register"]
pub type TzaR = crate::FieldReader;
#[doc = "Field `TZA` writer - 1:0\\]
When a trip event occurs the following action is taken on output EPWMxA Which trip-zone pins can cause an event is defined in the TZSEL register"]
pub type TzaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZB` reader - 3:2\\]
When a trip event occurs the following action is taken on output EPWMxB Which trip-zone pins can cause an event is defined in the TZSEL register"]
pub type TzbR = crate::FieldReader;
#[doc = "Field `TZB` writer - 3:2\\]
When a trip event occurs the following action is taken on output EPWMxB Which trip-zone pins can cause an event is defined in the TZSEL register"]
pub type TzbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
When a trip event occurs the following action is taken on output EPWMxA Which trip-zone pins can cause an event is defined in the TZSEL register"]
    #[inline(always)]
    pub fn tza(&self) -> TzaR {
        TzaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
When a trip event occurs the following action is taken on output EPWMxB Which trip-zone pins can cause an event is defined in the TZSEL register"]
    #[inline(always)]
    pub fn tzb(&self) -> TzbR {
        TzbR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
When a trip event occurs the following action is taken on output EPWMxA Which trip-zone pins can cause an event is defined in the TZSEL register"]
    #[inline(always)]
    #[must_use]
    pub fn tza(&mut self) -> TzaW<EpwmRegsTzctlSpec> {
        TzaW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
When a trip event occurs the following action is taken on output EPWMxB Which trip-zone pins can cause an event is defined in the TZSEL register"]
    #[inline(always)]
    #[must_use]
    pub fn tzb(&mut self) -> TzbW<EpwmRegsTzctlSpec> {
        TzbW::new(self, 2)
    }
}
#[doc = "Trip Zone Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tzctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tzctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsTzctlSpec;
impl crate::RegisterSpec for EpwmRegsTzctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_tzctl::R`](R) reader structure"]
impl crate::Readable for EpwmRegsTzctlSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_tzctl::W`](W) writer structure"]
impl crate::Writable for EpwmRegsTzctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_TZCTL to value 0"]
impl crate::Resettable for EpwmRegsTzctlSpec {
    const RESET_VALUE: u16 = 0;
}
