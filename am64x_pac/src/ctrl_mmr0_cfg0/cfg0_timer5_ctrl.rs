#[doc = "Register `CFG0_TIMER5_CTRL` reader"]
pub type R = crate::R<Cfg0Timer5CtrlSpec>;
#[doc = "Register `CFG0_TIMER5_CTRL` writer"]
pub type W = crate::W<Cfg0Timer5CtrlSpec>;
#[doc = "Field `TIMER5_CTRL_CASCADE_EN` reader - 8:8\\]
Activates cascading of TIMER5 to TIMER4"]
pub type Timer5CtrlCascadeEnR = crate::BitReader;
#[doc = "Field `TIMER5_CTRL_CASCADE_EN` writer - 8:8\\]
Activates cascading of TIMER5 to TIMER4"]
pub type Timer5CtrlCascadeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 8:8\\]
Activates cascading of TIMER5 to TIMER4"]
    #[inline(always)]
    pub fn timer5_ctrl_cascade_en(&self) -> Timer5CtrlCascadeEnR {
        Timer5CtrlCascadeEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
Activates cascading of TIMER5 to TIMER4"]
    #[inline(always)]
    #[must_use]
    pub fn timer5_ctrl_cascade_en(&mut self) -> Timer5CtrlCascadeEnW<Cfg0Timer5CtrlSpec> {
        Timer5CtrlCascadeEnW::new(self, 8)
    }
}
#[doc = "CFG0_TIMER5_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer5_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer5_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Timer5CtrlSpec;
impl crate::RegisterSpec for Cfg0Timer5CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_timer5_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Timer5CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_timer5_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Timer5CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_TIMER5_CTRL to value 0"]
impl crate::Resettable for Cfg0Timer5CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
