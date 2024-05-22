#[doc = "Register `REG_QCPRD` reader"]
pub type R = crate::R<RegQcprdSpec>;
#[doc = "Register `REG_QCPRD` writer"]
pub type W = crate::W<RegQcprdSpec>;
#[doc = "Field `QCPRD` reader - 15:0\\]
This register holds the period count value between the last successive eQEP position events"]
pub type QcprdR = crate::FieldReader<u16>;
#[doc = "Field `QCPRD` writer - 15:0\\]
This register holds the period count value between the last successive eQEP position events"]
pub type QcprdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
This register holds the period count value between the last successive eQEP position events"]
    #[inline(always)]
    pub fn qcprd(&self) -> QcprdR {
        QcprdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
This register holds the period count value between the last successive eQEP position events"]
    #[inline(always)]
    #[must_use]
    pub fn qcprd(&mut self) -> QcprdW<RegQcprdSpec> {
        QcprdW::new(self, 0)
    }
}
#[doc = "QEP Capture Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qcprd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qcprd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQcprdSpec;
impl crate::RegisterSpec for RegQcprdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_qcprd::R`](R) reader structure"]
impl crate::Readable for RegQcprdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qcprd::W`](W) writer structure"]
impl crate::Writable for RegQcprdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REG_QCPRD to value 0"]
impl crate::Resettable for RegQcprdSpec {
    const RESET_VALUE: u16 = 0;
}
