#[doc = "Register `CFG0_TIMER9_CTRL` reader"]
pub type R = crate::R<Cfg0Timer9CtrlSpec>;
#[doc = "Register `CFG0_TIMER9_CTRL` writer"]
pub type W = crate::W<Cfg0Timer9CtrlSpec>;
#[doc = "Field `TIMER9_CTRL_CASCADE_EN` reader - 8:8\\]
Activates cascading of TIMER9 to TIMER8"]
pub type Timer9CtrlCascadeEnR = crate::BitReader;
#[doc = "Field `TIMER9_CTRL_CASCADE_EN` writer - 8:8\\]
Activates cascading of TIMER9 to TIMER8"]
pub type Timer9CtrlCascadeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 8:8\\]
Activates cascading of TIMER9 to TIMER8"]
    #[inline(always)]
    pub fn timer9_ctrl_cascade_en(&self) -> Timer9CtrlCascadeEnR {
        Timer9CtrlCascadeEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
Activates cascading of TIMER9 to TIMER8"]
    #[inline(always)]
    #[must_use]
    pub fn timer9_ctrl_cascade_en(&mut self) -> Timer9CtrlCascadeEnW<Cfg0Timer9CtrlSpec> {
        Timer9CtrlCascadeEnW::new(self, 8)
    }
}
#[doc = "CFG0_TIMER9_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer9_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer9_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Timer9CtrlSpec;
impl crate::RegisterSpec for Cfg0Timer9CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_timer9_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Timer9CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_timer9_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Timer9CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_TIMER9_CTRL to value 0"]
impl crate::Resettable for Cfg0Timer9CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
