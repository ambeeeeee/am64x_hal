#[doc = "Register `REG_QUPRD` reader"]
pub type R = crate::R<RegQuprdSpec>;
#[doc = "Register `REG_QUPRD` writer"]
pub type W = crate::W<RegQuprdSpec>;
#[doc = "Field `QUPRD` reader - 31:0\\]
QEP Unit PeriodThis register contains the period count for the unit timer to generate periodic unit time events. These events latch the eQEP position information at periodic intervals and optionally generate an interrupt. Writes to this register should always be full 32-bit writes."]
pub type QuprdR = crate::FieldReader<u32>;
#[doc = "Field `QUPRD` writer - 31:0\\]
QEP Unit PeriodThis register contains the period count for the unit timer to generate periodic unit time events. These events latch the eQEP position information at periodic intervals and optionally generate an interrupt. Writes to this register should always be full 32-bit writes."]
pub type QuprdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
QEP Unit PeriodThis register contains the period count for the unit timer to generate periodic unit time events. These events latch the eQEP position information at periodic intervals and optionally generate an interrupt. Writes to this register should always be full 32-bit writes."]
    #[inline(always)]
    pub fn quprd(&self) -> QuprdR {
        QuprdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
QEP Unit PeriodThis register contains the period count for the unit timer to generate periodic unit time events. These events latch the eQEP position information at periodic intervals and optionally generate an interrupt. Writes to this register should always be full 32-bit writes."]
    #[inline(always)]
    #[must_use]
    pub fn quprd(&mut self) -> QuprdW<RegQuprdSpec> {
        QuprdW::new(self, 0)
    }
}
#[doc = "QEP Unit Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_quprd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_quprd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQuprdSpec;
impl crate::RegisterSpec for RegQuprdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_quprd::R`](R) reader structure"]
impl crate::Readable for RegQuprdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_quprd::W`](W) writer structure"]
impl crate::Writable for RegQuprdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_QUPRD to value 0"]
impl crate::Resettable for RegQuprdSpec {
    const RESET_VALUE: u32 = 0;
}
