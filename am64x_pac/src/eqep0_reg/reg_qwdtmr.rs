#[doc = "Register `REG_QWDTMR` reader"]
pub type R = crate::R<RegQwdtmrSpec>;
#[doc = "Register `REG_QWDTMR` writer"]
pub type W = crate::W<RegQwdtmrSpec>;
#[doc = "Field `QWDTMR` reader - 15:0\\]
QEP Watchdog Timer This register acts as time base for the watchdog to detect motor stalls. When this timer value matches with the watchdog's period value a watchdog timeout interrupt is generated. This register is reset upon edge transition in quadrature-clock indicating the motion."]
pub type QwdtmrR = crate::FieldReader<u16>;
#[doc = "Field `QWDTMR` writer - 15:0\\]
QEP Watchdog Timer This register acts as time base for the watchdog to detect motor stalls. When this timer value matches with the watchdog's period value a watchdog timeout interrupt is generated. This register is reset upon edge transition in quadrature-clock indicating the motion."]
pub type QwdtmrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
QEP Watchdog Timer This register acts as time base for the watchdog to detect motor stalls. When this timer value matches with the watchdog's period value a watchdog timeout interrupt is generated. This register is reset upon edge transition in quadrature-clock indicating the motion."]
    #[inline(always)]
    pub fn qwdtmr(&self) -> QwdtmrR {
        QwdtmrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
QEP Watchdog Timer This register acts as time base for the watchdog to detect motor stalls. When this timer value matches with the watchdog's period value a watchdog timeout interrupt is generated. This register is reset upon edge transition in quadrature-clock indicating the motion."]
    #[inline(always)]
    #[must_use]
    pub fn qwdtmr(&mut self) -> QwdtmrW<RegQwdtmrSpec> {
        QwdtmrW::new(self, 0)
    }
}
#[doc = "QEP Watchdog Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qwdtmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qwdtmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQwdtmrSpec;
impl crate::RegisterSpec for RegQwdtmrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_qwdtmr::R`](R) reader structure"]
impl crate::Readable for RegQwdtmrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qwdtmr::W`](W) writer structure"]
impl crate::Writable for RegQwdtmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REG_QWDTMR to value 0"]
impl crate::Resettable for RegQwdtmrSpec {
    const RESET_VALUE: u16 = 0;
}
