#[doc = "Register `REG_QPOSMAX` reader"]
pub type R = crate::R<RegQposmaxSpec>;
#[doc = "Register `REG_QPOSMAX` writer"]
pub type W = crate::W<RegQposmaxSpec>;
#[doc = "Field `QPOSMAX` reader - 31:0\\]
Maximum Position CountThis register contains the maximum position counter value. Writes to this register should always be full 32-bit writes."]
pub type QposmaxR = crate::FieldReader<u32>;
#[doc = "Field `QPOSMAX` writer - 31:0\\]
Maximum Position CountThis register contains the maximum position counter value. Writes to this register should always be full 32-bit writes."]
pub type QposmaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Maximum Position CountThis register contains the maximum position counter value. Writes to this register should always be full 32-bit writes."]
    #[inline(always)]
    pub fn qposmax(&self) -> QposmaxR {
        QposmaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Maximum Position CountThis register contains the maximum position counter value. Writes to this register should always be full 32-bit writes."]
    #[inline(always)]
    #[must_use]
    pub fn qposmax(&mut self) -> QposmaxW<RegQposmaxSpec> {
        QposmaxW::new(self, 0)
    }
}
#[doc = "Maximum Position Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposmax::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposmax::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQposmaxSpec;
impl crate::RegisterSpec for RegQposmaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_qposmax::R`](R) reader structure"]
impl crate::Readable for RegQposmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qposmax::W`](W) writer structure"]
impl crate::Writable for RegQposmaxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_QPOSMAX to value 0"]
impl crate::Resettable for RegQposmaxSpec {
    const RESET_VALUE: u32 = 0;
}
