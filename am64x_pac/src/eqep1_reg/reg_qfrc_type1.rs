#[doc = "Register `REG_QFRC_TYPE1` reader"]
pub type R = crate::R<RegQfrcType1Spec>;
#[doc = "Register `REG_QFRC_TYPE1` writer"]
pub type W = crate::W<RegQfrcType1Spec>;
#[doc = "Field `PCE` reader - 1:1\\]
Force position counter error interrupt"]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - 1:1\\]
Force position counter error interrupt"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHE` reader - 2:2\\]
Force quadrature phase error interrupt"]
pub type PheR = crate::BitReader;
#[doc = "Field `PHE` writer - 2:2\\]
Force quadrature phase error interrupt"]
pub type PheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDC` reader - 3:3\\]
Force quadrature direction change interrupt"]
pub type QdcR = crate::BitReader;
#[doc = "Field `QDC` writer - 3:3\\]
Force quadrature direction change interrupt"]
pub type QdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTO` reader - 4:4\\]
Force watchdog time out interrupt"]
pub type WtoR = crate::BitReader;
#[doc = "Field `WTO` writer - 4:4\\]
Force watchdog time out interrupt"]
pub type WtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCU` reader - 5:5\\]
Force position counter underflow interrupt"]
pub type PcuR = crate::BitReader;
#[doc = "Field `PCU` writer - 5:5\\]
Force position counter underflow interrupt"]
pub type PcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCO` reader - 6:6\\]
Force position counter overflow interrupt"]
pub type PcoR = crate::BitReader;
#[doc = "Field `PCO` writer - 6:6\\]
Force position counter overflow interrupt"]
pub type PcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCR` reader - 7:7\\]
Force position-compare ready interrupt"]
pub type PcrR = crate::BitReader;
#[doc = "Field `PCR` writer - 7:7\\]
Force position-compare ready interrupt"]
pub type PcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCM` reader - 8:8\\]
Force position-compare match interrupt"]
pub type PcmR = crate::BitReader;
#[doc = "Field `PCM` writer - 8:8\\]
Force position-compare match interrupt"]
pub type PcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL` reader - 9:9\\]
Force strobe event latch interrupt"]
pub type SelR = crate::BitReader;
#[doc = "Field `SEL` writer - 9:9\\]
Force strobe event latch interrupt"]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEL` reader - 10:10\\]
Force index event latch interrupt"]
pub type IelR = crate::BitReader;
#[doc = "Field `IEL` writer - 10:10\\]
Force index event latch interrupt"]
pub type IelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTO` reader - 11:11\\]
Force unit time out interrupt"]
pub type UtoR = crate::BitReader;
#[doc = "Field `UTO` writer - 11:11\\]
Force unit time out interrupt"]
pub type UtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QMAE` reader - 12:12\\]
Force QMA error interrupt"]
pub type QmaeR = crate::BitReader;
#[doc = "Field `QMAE` writer - 12:12\\]
Force QMA error interrupt"]
pub type QmaeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
Force position counter error interrupt"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Force quadrature phase error interrupt"]
    #[inline(always)]
    pub fn phe(&self) -> PheR {
        PheR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Force quadrature direction change interrupt"]
    #[inline(always)]
    pub fn qdc(&self) -> QdcR {
        QdcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Force watchdog time out interrupt"]
    #[inline(always)]
    pub fn wto(&self) -> WtoR {
        WtoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Force position counter underflow interrupt"]
    #[inline(always)]
    pub fn pcu(&self) -> PcuR {
        PcuR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Force position counter overflow interrupt"]
    #[inline(always)]
    pub fn pco(&self) -> PcoR {
        PcoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Force position-compare ready interrupt"]
    #[inline(always)]
    pub fn pcr(&self) -> PcrR {
        PcrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Force position-compare match interrupt"]
    #[inline(always)]
    pub fn pcm(&self) -> PcmR {
        PcmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Force strobe event latch interrupt"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Force index event latch interrupt"]
    #[inline(always)]
    pub fn iel(&self) -> IelR {
        IelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Force unit time out interrupt"]
    #[inline(always)]
    pub fn uto(&self) -> UtoR {
        UtoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Force QMA error interrupt"]
    #[inline(always)]
    pub fn qmae(&self) -> QmaeR {
        QmaeR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Force position counter error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PceW<RegQfrcType1Spec> {
        PceW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Force quadrature phase error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn phe(&mut self) -> PheW<RegQfrcType1Spec> {
        PheW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Force quadrature direction change interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn qdc(&mut self) -> QdcW<RegQfrcType1Spec> {
        QdcW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Force watchdog time out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wto(&mut self) -> WtoW<RegQfrcType1Spec> {
        WtoW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Force position counter underflow interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pcu(&mut self) -> PcuW<RegQfrcType1Spec> {
        PcuW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Force position counter overflow interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pco(&mut self) -> PcoW<RegQfrcType1Spec> {
        PcoW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Force position-compare ready interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pcr(&mut self) -> PcrW<RegQfrcType1Spec> {
        PcrW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Force position-compare match interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pcm(&mut self) -> PcmW<RegQfrcType1Spec> {
        PcmW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Force strobe event latch interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<RegQfrcType1Spec> {
        SelW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Force index event latch interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn iel(&mut self) -> IelW<RegQfrcType1Spec> {
        IelW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Force unit time out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn uto(&mut self) -> UtoW<RegQfrcType1Spec> {
        UtoW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Force QMA error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn qmae(&mut self) -> QmaeW<RegQfrcType1Spec> {
        QmaeW::new(self, 12)
    }
}
#[doc = "QEP Interrupt Force\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qfrc_type1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qfrc_type1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQfrcType1Spec;
impl crate::RegisterSpec for RegQfrcType1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_qfrc_type1::R`](R) reader structure"]
impl crate::Readable for RegQfrcType1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_qfrc_type1::W`](W) writer structure"]
impl crate::Writable for RegQfrcType1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REG_QFRC_TYPE1 to value 0"]
impl crate::Resettable for RegQfrcType1Spec {
    const RESET_VALUE: u16 = 0;
}
