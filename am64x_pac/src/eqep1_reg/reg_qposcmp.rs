#[doc = "Register `REG_QPOSCMP` reader"]
pub type R = crate::R<RegQposcmpSpec>;
#[doc = "Register `REG_QPOSCMP` writer"]
pub type W = crate::W<RegQposcmpSpec>;
#[doc = "Field `QPOSCMP` reader - 31:0\\]
Position Compare The position-compare value in this register is compared with the position counter \\[QPOSCNT\\]
to generate sync output and/or interrupt on compare match."]
pub type QposcmpR = crate::FieldReader<u32>;
#[doc = "Field `QPOSCMP` writer - 31:0\\]
Position Compare The position-compare value in this register is compared with the position counter \\[QPOSCNT\\]
to generate sync output and/or interrupt on compare match."]
pub type QposcmpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Position Compare The position-compare value in this register is compared with the position counter \\[QPOSCNT\\]
to generate sync output and/or interrupt on compare match."]
    #[inline(always)]
    pub fn qposcmp(&self) -> QposcmpR {
        QposcmpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Position Compare The position-compare value in this register is compared with the position counter \\[QPOSCNT\\]
to generate sync output and/or interrupt on compare match."]
    #[inline(always)]
    #[must_use]
    pub fn qposcmp(&mut self) -> QposcmpW<RegQposcmpSpec> {
        QposcmpW::new(self, 0)
    }
}
#[doc = "Position Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposcmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposcmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQposcmpSpec;
impl crate::RegisterSpec for RegQposcmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_qposcmp::R`](R) reader structure"]
impl crate::Readable for RegQposcmpSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qposcmp::W`](W) writer structure"]
impl crate::Writable for RegQposcmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_QPOSCMP to value 0"]
impl crate::Resettable for RegQposcmpSpec {
    const RESET_VALUE: u32 = 0;
}
