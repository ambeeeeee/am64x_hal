#[doc = "Register `REG_QUTMR` reader"]
pub type R = crate::R<RegQutmrSpec>;
#[doc = "Register `REG_QUTMR` writer"]
pub type W = crate::W<RegQutmrSpec>;
#[doc = "Field `QUTMR` reader - 31:0\\]
QEP Unit TimerThis register acts as time base for unit time event generation. When this timer value matches the unit time period value a unit time event is generated."]
pub type QutmrR = crate::FieldReader<u32>;
#[doc = "Field `QUTMR` writer - 31:0\\]
QEP Unit TimerThis register acts as time base for unit time event generation. When this timer value matches the unit time period value a unit time event is generated."]
pub type QutmrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
QEP Unit TimerThis register acts as time base for unit time event generation. When this timer value matches the unit time period value a unit time event is generated."]
    #[inline(always)]
    pub fn qutmr(&self) -> QutmrR {
        QutmrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
QEP Unit TimerThis register acts as time base for unit time event generation. When this timer value matches the unit time period value a unit time event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn qutmr(&mut self) -> QutmrW<RegQutmrSpec> {
        QutmrW::new(self, 0)
    }
}
#[doc = "QEP Unit Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qutmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qutmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQutmrSpec;
impl crate::RegisterSpec for RegQutmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_qutmr::R`](R) reader structure"]
impl crate::Readable for RegQutmrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qutmr::W`](W) writer structure"]
impl crate::Writable for RegQutmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_QUTMR to value 0"]
impl crate::Resettable for RegQutmrSpec {
    const RESET_VALUE: u32 = 0;
}
