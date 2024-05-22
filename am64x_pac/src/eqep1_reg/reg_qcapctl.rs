#[doc = "Register `REG_QCAPCTL` reader"]
pub type R = crate::R<RegQcapctlSpec>;
#[doc = "Register `REG_QCAPCTL` writer"]
pub type W = crate::W<RegQcapctlSpec>;
#[doc = "Field `UPPS` reader - 3:0\\]
Unit position event prescaler"]
pub type UppsR = crate::FieldReader;
#[doc = "Field `UPPS` writer - 3:0\\]
Unit position event prescaler"]
pub type UppsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CCPS` reader - 6:4\\]
eQEP capture timer clock prescaler"]
pub type CcpsR = crate::FieldReader;
#[doc = "Field `CCPS` writer - 6:4\\]
eQEP capture timer clock prescaler"]
pub type CcpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CEN` reader - 15:15\\]
Enable eQEP capture"]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - 15:15\\]
Enable eQEP capture"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Unit position event prescaler"]
    #[inline(always)]
    pub fn upps(&self) -> UppsR {
        UppsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
eQEP capture timer clock prescaler"]
    #[inline(always)]
    pub fn ccps(&self) -> CcpsR {
        CcpsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Enable eQEP capture"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Unit position event prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn upps(&mut self) -> UppsW<RegQcapctlSpec> {
        UppsW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
eQEP capture timer clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ccps(&mut self) -> CcpsW<RegQcapctlSpec> {
        CcpsW::new(self, 4)
    }
    #[doc = "Bit 15 - 15:15\\]
Enable eQEP capture"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CenW<RegQcapctlSpec> {
        CenW::new(self, 15)
    }
}
#[doc = "Qaudrature Capture Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qcapctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qcapctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQcapctlSpec;
impl crate::RegisterSpec for RegQcapctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_qcapctl::R`](R) reader structure"]
impl crate::Readable for RegQcapctlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qcapctl::W`](W) writer structure"]
impl crate::Writable for RegQcapctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REG_QCAPCTL to value 0"]
impl crate::Resettable for RegQcapctlSpec {
    const RESET_VALUE: u16 = 0;
}
