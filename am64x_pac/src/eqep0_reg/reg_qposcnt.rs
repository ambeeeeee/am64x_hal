#[doc = "Register `REG_QPOSCNT` reader"]
pub type R = crate::R<RegQposcntSpec>;
#[doc = "Register `REG_QPOSCNT` writer"]
pub type W = crate::W<RegQposcntSpec>;
#[doc = "Field `QPOSCNT` reader - 31:0\\]
Position Counter This 32-bit position counter register counts up/down on every eQEP pulse based on direction input. This counter acts as a position integrator whose count value is proportional to position from a give reference point. This Register acts as a Read ONLY register while counter is counting up/down."]
pub type QposcntR = crate::FieldReader<u32>;
#[doc = "Field `QPOSCNT` writer - 31:0\\]
Position Counter This 32-bit position counter register counts up/down on every eQEP pulse based on direction input. This counter acts as a position integrator whose count value is proportional to position from a give reference point. This Register acts as a Read ONLY register while counter is counting up/down."]
pub type QposcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Position Counter This 32-bit position counter register counts up/down on every eQEP pulse based on direction input. This counter acts as a position integrator whose count value is proportional to position from a give reference point. This Register acts as a Read ONLY register while counter is counting up/down."]
    #[inline(always)]
    pub fn qposcnt(&self) -> QposcntR {
        QposcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Position Counter This 32-bit position counter register counts up/down on every eQEP pulse based on direction input. This counter acts as a position integrator whose count value is proportional to position from a give reference point. This Register acts as a Read ONLY register while counter is counting up/down."]
    #[inline(always)]
    #[must_use]
    pub fn qposcnt(&mut self) -> QposcntW<RegQposcntSpec> {
        QposcntW::new(self, 0)
    }
}
#[doc = "Position Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQposcntSpec;
impl crate::RegisterSpec for RegQposcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_qposcnt::R`](R) reader structure"]
impl crate::Readable for RegQposcntSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qposcnt::W`](W) writer structure"]
impl crate::Writable for RegQposcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_QPOSCNT to value 0"]
impl crate::Resettable for RegQposcntSpec {
    const RESET_VALUE: u32 = 0;
}
