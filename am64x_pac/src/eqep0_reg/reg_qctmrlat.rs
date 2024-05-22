#[doc = "Register `REG_QCTMRLAT` reader"]
pub type R = crate::R<RegQctmrlatSpec>;
#[doc = "Register `REG_QCTMRLAT` writer"]
pub type W = crate::W<RegQctmrlatSpec>;
#[doc = "Field `QCTMRLAT` reader - 15:0\\]
The eQEP capture timer value can be latched into this register on two events viz., unit timeout event, reading the eQEP position counter."]
pub type QctmrlatR = crate::FieldReader<u16>;
#[doc = "Field `QCTMRLAT` writer - 15:0\\]
The eQEP capture timer value can be latched into this register on two events viz., unit timeout event, reading the eQEP position counter."]
pub type QctmrlatW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The eQEP capture timer value can be latched into this register on two events viz., unit timeout event, reading the eQEP position counter."]
    #[inline(always)]
    pub fn qctmrlat(&self) -> QctmrlatR {
        QctmrlatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
The eQEP capture timer value can be latched into this register on two events viz., unit timeout event, reading the eQEP position counter."]
    #[inline(always)]
    #[must_use]
    pub fn qctmrlat(&mut self) -> QctmrlatW<RegQctmrlatSpec> {
        QctmrlatW::new(self, 0)
    }
}
#[doc = "QEP Capture Latch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qctmrlat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qctmrlat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQctmrlatSpec;
impl crate::RegisterSpec for RegQctmrlatSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_qctmrlat::R`](R) reader structure"]
impl crate::Readable for RegQctmrlatSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qctmrlat::W`](W) writer structure"]
impl crate::Writable for RegQctmrlatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REG_QCTMRLAT to value 0"]
impl crate::Resettable for RegQctmrlatSpec {
    const RESET_VALUE: u16 = 0;
}
