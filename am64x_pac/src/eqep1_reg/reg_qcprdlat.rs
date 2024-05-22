#[doc = "Register `REG_QCPRDLAT` reader"]
pub type R = crate::R<RegQcprdlatSpec>;
#[doc = "Register `REG_QCPRDLAT` writer"]
pub type W = crate::W<RegQcprdlatSpec>;
#[doc = "Field `QCPRDLAT` reader - 15:0\\]
eQEP capture period value can be latched into this register on two events viz., unit timeout event, reading the eQEP position counter."]
pub type QcprdlatR = crate::FieldReader<u16>;
#[doc = "Field `QCPRDLAT` writer - 15:0\\]
eQEP capture period value can be latched into this register on two events viz., unit timeout event, reading the eQEP position counter."]
pub type QcprdlatW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
eQEP capture period value can be latched into this register on two events viz., unit timeout event, reading the eQEP position counter."]
    #[inline(always)]
    pub fn qcprdlat(&self) -> QcprdlatR {
        QcprdlatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
eQEP capture period value can be latched into this register on two events viz., unit timeout event, reading the eQEP position counter."]
    #[inline(always)]
    #[must_use]
    pub fn qcprdlat(&mut self) -> QcprdlatW<RegQcprdlatSpec> {
        QcprdlatW::new(self, 0)
    }
}
#[doc = "QEP Capture Period Latch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qcprdlat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qcprdlat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQcprdlatSpec;
impl crate::RegisterSpec for RegQcprdlatSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_qcprdlat::R`](R) reader structure"]
impl crate::Readable for RegQcprdlatSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qcprdlat::W`](W) writer structure"]
impl crate::Writable for RegQcprdlatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REG_QCPRDLAT to value 0"]
impl crate::Resettable for RegQcprdlatSpec {
    const RESET_VALUE: u16 = 0;
}
