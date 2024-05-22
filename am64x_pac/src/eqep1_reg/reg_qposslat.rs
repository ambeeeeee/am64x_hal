#[doc = "Register `REG_QPOSSLAT` reader"]
pub type R = crate::R<RegQposslatSpec>;
#[doc = "Register `REG_QPOSSLAT` writer"]
pub type W = crate::W<RegQposslatSpec>;
#[doc = "Field `QPOSSLAT` reader - 31:0\\]
Strobe Position Latch The position-counter value is latched into this register on a strobe event as defined by the QEPCTL\\[SEL\\]
bits."]
pub type QposslatR = crate::FieldReader<u32>;
#[doc = "Field `QPOSSLAT` writer - 31:0\\]
Strobe Position Latch The position-counter value is latched into this register on a strobe event as defined by the QEPCTL\\[SEL\\]
bits."]
pub type QposslatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Strobe Position Latch The position-counter value is latched into this register on a strobe event as defined by the QEPCTL\\[SEL\\]
bits."]
    #[inline(always)]
    pub fn qposslat(&self) -> QposslatR {
        QposslatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Strobe Position Latch The position-counter value is latched into this register on a strobe event as defined by the QEPCTL\\[SEL\\]
bits."]
    #[inline(always)]
    #[must_use]
    pub fn qposslat(&mut self) -> QposslatW<RegQposslatSpec> {
        QposslatW::new(self, 0)
    }
}
#[doc = "Strobe Position Latch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposslat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposslat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQposslatSpec;
impl crate::RegisterSpec for RegQposslatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_qposslat::R`](R) reader structure"]
impl crate::Readable for RegQposslatSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qposslat::W`](W) writer structure"]
impl crate::Writable for RegQposslatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_QPOSSLAT to value 0"]
impl crate::Resettable for RegQposslatSpec {
    const RESET_VALUE: u32 = 0;
}
