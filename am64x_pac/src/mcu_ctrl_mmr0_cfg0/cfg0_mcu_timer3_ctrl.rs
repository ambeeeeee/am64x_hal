#[doc = "Register `CFG0_MCU_TIMER3_CTRL` reader"]
pub type R = crate::R<Cfg0McuTimer3CtrlSpec>;
#[doc = "Register `CFG0_MCU_TIMER3_CTRL` writer"]
pub type W = crate::W<Cfg0McuTimer3CtrlSpec>;
#[doc = "Field `MCU_TIMER3_CTRL_CASCADE_EN` reader - 8:8\\]
Actives cascading of TIMER3 to TIMER2"]
pub type McuTimer3CtrlCascadeEnR = crate::BitReader;
#[doc = "Field `MCU_TIMER3_CTRL_CASCADE_EN` writer - 8:8\\]
Actives cascading of TIMER3 to TIMER2"]
pub type McuTimer3CtrlCascadeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 8:8\\]
Actives cascading of TIMER3 to TIMER2"]
    #[inline(always)]
    pub fn mcu_timer3_ctrl_cascade_en(&self) -> McuTimer3CtrlCascadeEnR {
        McuTimer3CtrlCascadeEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
Actives cascading of TIMER3 to TIMER2"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_timer3_ctrl_cascade_en(&mut self) -> McuTimer3CtrlCascadeEnW<Cfg0McuTimer3CtrlSpec> {
        McuTimer3CtrlCascadeEnW::new(self, 8)
    }
}
#[doc = "CFG0_MCU_TIMER3_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer3_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer3_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuTimer3CtrlSpec;
impl crate::RegisterSpec for Cfg0McuTimer3CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_timer3_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0McuTimer3CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_timer3_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0McuTimer3CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_TIMER3_CTRL to value 0"]
impl crate::Resettable for Cfg0McuTimer3CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
