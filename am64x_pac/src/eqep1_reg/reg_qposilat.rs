#[doc = "Register `REG_QPOSILAT` reader"]
pub type R = crate::R<RegQposilatSpec>;
#[doc = "Register `REG_QPOSILAT` writer"]
pub type W = crate::W<RegQposilatSpec>;
#[doc = "Field `QPOSILAT` reader - 31:0\\]
Index Position Latch The position-counter value is latched into this register on an index event as defined by the QEPCTL\\[IEL\\]
bits."]
pub type QposilatR = crate::FieldReader<u32>;
#[doc = "Field `QPOSILAT` writer - 31:0\\]
Index Position Latch The position-counter value is latched into this register on an index event as defined by the QEPCTL\\[IEL\\]
bits."]
pub type QposilatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Index Position Latch The position-counter value is latched into this register on an index event as defined by the QEPCTL\\[IEL\\]
bits."]
    #[inline(always)]
    pub fn qposilat(&self) -> QposilatR {
        QposilatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Index Position Latch The position-counter value is latched into this register on an index event as defined by the QEPCTL\\[IEL\\]
bits."]
    #[inline(always)]
    #[must_use]
    pub fn qposilat(&mut self) -> QposilatW<RegQposilatSpec> {
        QposilatW::new(self, 0)
    }
}
#[doc = "Index Position Latch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposilat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposilat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQposilatSpec;
impl crate::RegisterSpec for RegQposilatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_qposilat::R`](R) reader structure"]
impl crate::Readable for RegQposilatSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qposilat::W`](W) writer structure"]
impl crate::Writable for RegQposilatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_QPOSILAT to value 0"]
impl crate::Resettable for RegQposilatSpec {
    const RESET_VALUE: u32 = 0;
}
