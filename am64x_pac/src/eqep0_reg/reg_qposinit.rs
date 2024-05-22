#[doc = "Register `REG_QPOSINIT` reader"]
pub type R = crate::R<RegQposinitSpec>;
#[doc = "Register `REG_QPOSINIT` writer"]
pub type W = crate::W<RegQposinitSpec>;
#[doc = "Field `QPOSINIT` reader - 31:0\\]
Position Counter InitThis register contains the position value that is used to initialize the position counter based on external strobe or index event. The position counter can be initialized through software. Writes to this register should always be full 32-bit writes."]
pub type QposinitR = crate::FieldReader<u32>;
#[doc = "Field `QPOSINIT` writer - 31:0\\]
Position Counter InitThis register contains the position value that is used to initialize the position counter based on external strobe or index event. The position counter can be initialized through software. Writes to this register should always be full 32-bit writes."]
pub type QposinitW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Position Counter InitThis register contains the position value that is used to initialize the position counter based on external strobe or index event. The position counter can be initialized through software. Writes to this register should always be full 32-bit writes."]
    #[inline(always)]
    pub fn qposinit(&self) -> QposinitR {
        QposinitR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Position Counter InitThis register contains the position value that is used to initialize the position counter based on external strobe or index event. The position counter can be initialized through software. Writes to this register should always be full 32-bit writes."]
    #[inline(always)]
    #[must_use]
    pub fn qposinit(&mut self) -> QposinitW<RegQposinitSpec> {
        QposinitW::new(self, 0)
    }
}
#[doc = "Position Counter Init\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposinit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposinit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQposinitSpec;
impl crate::RegisterSpec for RegQposinitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_qposinit::R`](R) reader structure"]
impl crate::Readable for RegQposinitSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qposinit::W`](W) writer structure"]
impl crate::Writable for RegQposinitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_QPOSINIT to value 0"]
impl crate::Resettable for RegQposinitSpec {
    const RESET_VALUE: u32 = 0;
}
