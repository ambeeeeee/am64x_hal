#[doc = "Register `CFG0_TIMER7_CTRL` reader"]
pub type R = crate::R<Cfg0Timer7CtrlSpec>;
#[doc = "Register `CFG0_TIMER7_CTRL` writer"]
pub type W = crate::W<Cfg0Timer7CtrlSpec>;
#[doc = "Field `TIMER7_CTRL_CASCADE_EN` reader - 8:8\\]
Activates cascading of TIMER7 to TIMER6"]
pub type Timer7CtrlCascadeEnR = crate::BitReader;
#[doc = "Field `TIMER7_CTRL_CASCADE_EN` writer - 8:8\\]
Activates cascading of TIMER7 to TIMER6"]
pub type Timer7CtrlCascadeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 8:8\\]
Activates cascading of TIMER7 to TIMER6"]
    #[inline(always)]
    pub fn timer7_ctrl_cascade_en(&self) -> Timer7CtrlCascadeEnR {
        Timer7CtrlCascadeEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
Activates cascading of TIMER7 to TIMER6"]
    #[inline(always)]
    #[must_use]
    pub fn timer7_ctrl_cascade_en(&mut self) -> Timer7CtrlCascadeEnW<Cfg0Timer7CtrlSpec> {
        Timer7CtrlCascadeEnW::new(self, 8)
    }
}
#[doc = "CFG0_TIMER7_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer7_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer7_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Timer7CtrlSpec;
impl crate::RegisterSpec for Cfg0Timer7CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_timer7_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Timer7CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_timer7_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Timer7CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_TIMER7_CTRL to value 0"]
impl crate::Resettable for Cfg0Timer7CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
