#[doc = "Register `REG_QWDPRD` reader"]
pub type R = crate::R<RegQwdprdSpec>;
#[doc = "Register `REG_QWDPRD` writer"]
pub type W = crate::W<RegQwdprdSpec>;
#[doc = "Field `QWDPRD` reader - 15:0\\]
QEP Watchdog Period This register contains the time-out count for the eQEP peripheral watch dog timer.When the watchdog timer value matches the watchdog period value, a watchdog timeout interrupt is generated."]
pub type QwdprdR = crate::FieldReader<u16>;
#[doc = "Field `QWDPRD` writer - 15:0\\]
QEP Watchdog Period This register contains the time-out count for the eQEP peripheral watch dog timer.When the watchdog timer value matches the watchdog period value, a watchdog timeout interrupt is generated."]
pub type QwdprdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
QEP Watchdog Period This register contains the time-out count for the eQEP peripheral watch dog timer.When the watchdog timer value matches the watchdog period value, a watchdog timeout interrupt is generated."]
    #[inline(always)]
    pub fn qwdprd(&self) -> QwdprdR {
        QwdprdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
QEP Watchdog Period This register contains the time-out count for the eQEP peripheral watch dog timer.When the watchdog timer value matches the watchdog period value, a watchdog timeout interrupt is generated."]
    #[inline(always)]
    #[must_use]
    pub fn qwdprd(&mut self) -> QwdprdW<RegQwdprdSpec> {
        QwdprdW::new(self, 0)
    }
}
#[doc = "QEP Watchdog Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qwdprd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qwdprd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQwdprdSpec;
impl crate::RegisterSpec for RegQwdprdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_qwdprd::R`](R) reader structure"]
impl crate::Readable for RegQwdprdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qwdprd::W`](W) writer structure"]
impl crate::Writable for RegQwdprdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REG_QWDPRD to value 0"]
impl crate::Resettable for RegQwdprdSpec {
    const RESET_VALUE: u16 = 0;
}
