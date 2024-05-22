#[doc = "Register `REG_QPOSLAT` reader"]
pub type R = crate::R<RegQposlatSpec>;
#[doc = "Register `REG_QPOSLAT` writer"]
pub type W = crate::W<RegQposlatSpec>;
#[doc = "Field `QPOSLAT` reader - 31:0\\]
Position Latch The position-counter value is latched into this register on a unit time out event."]
pub type QposlatR = crate::FieldReader<u32>;
#[doc = "Field `QPOSLAT` writer - 31:0\\]
Position Latch The position-counter value is latched into this register on a unit time out event."]
pub type QposlatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Position Latch The position-counter value is latched into this register on a unit time out event."]
    #[inline(always)]
    pub fn qposlat(&self) -> QposlatR {
        QposlatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Position Latch The position-counter value is latched into this register on a unit time out event."]
    #[inline(always)]
    #[must_use]
    pub fn qposlat(&mut self) -> QposlatW<RegQposlatSpec> {
        QposlatW::new(self, 0)
    }
}
#[doc = "Position Latch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposlat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposlat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQposlatSpec;
impl crate::RegisterSpec for RegQposlatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_qposlat::R`](R) reader structure"]
impl crate::Readable for RegQposlatSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qposlat::W`](W) writer structure"]
impl crate::Writable for RegQposlatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_QPOSLAT to value 0"]
impl crate::Resettable for RegQposlatSpec {
    const RESET_VALUE: u32 = 0;
}
