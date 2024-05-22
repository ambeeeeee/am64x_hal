#[doc = "Register `REG_QCTMR` reader"]
pub type R = crate::R<RegQctmrSpec>;
#[doc = "Register `REG_QCTMR` writer"]
pub type W = crate::W<RegQctmrSpec>;
#[doc = "Field `QCTMR` reader - 15:0\\]
This register provides time base for edge capture unit."]
pub type QctmrR = crate::FieldReader<u16>;
#[doc = "Field `QCTMR` writer - 15:0\\]
This register provides time base for edge capture unit."]
pub type QctmrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
This register provides time base for edge capture unit."]
    #[inline(always)]
    pub fn qctmr(&self) -> QctmrR {
        QctmrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
This register provides time base for edge capture unit."]
    #[inline(always)]
    #[must_use]
    pub fn qctmr(&mut self) -> QctmrW<RegQctmrSpec> {
        QctmrW::new(self, 0)
    }
}
#[doc = "QEP Capture Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qctmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qctmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQctmrSpec;
impl crate::RegisterSpec for RegQctmrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_qctmr::R`](R) reader structure"]
impl crate::Readable for RegQctmrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qctmr::W`](W) writer structure"]
impl crate::Writable for RegQctmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REG_QCTMR to value 0"]
impl crate::Resettable for RegQctmrSpec {
    const RESET_VALUE: u16 = 0;
}
