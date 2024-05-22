#[doc = "Register `REG_QEPSTS_TYPE1` reader"]
pub type R = crate::R<RegQepstsType1Spec>;
#[doc = "Register `REG_QEPSTS_TYPE1` writer"]
pub type W = crate::W<RegQepstsType1Spec>;
#[doc = "Field `PCEF` reader - 0:0\\]
Position counter error flag. This bit is not sticky and it is updated for every index event."]
pub type PcefR = crate::BitReader;
#[doc = "Field `PCEF` writer - 0:0\\]
Position counter error flag. This bit is not sticky and it is updated for every index event."]
pub type PcefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIMF` reader - 1:1\\]
First index marker flag"]
pub type FimfR = crate::BitReader;
#[doc = "Field `FIMF` writer - 1:1\\]
First index marker flag"]
pub type FimfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDEF` reader - 2:2\\]
Capture direction error flag"]
pub type CdefR = crate::BitReader;
#[doc = "Field `CDEF` writer - 2:2\\]
Capture direction error flag"]
pub type CdefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COEF` reader - 3:3\\]
Capture overflow error flag"]
pub type CoefR = crate::BitReader;
#[doc = "Field `COEF` writer - 3:3\\]
Capture overflow error flag"]
pub type CoefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDLF` reader - 4:4\\]
eQEP direction latch flag"]
pub type QdlfR = crate::BitReader;
#[doc = "Field `QDLF` writer - 4:4\\]
eQEP direction latch flag"]
pub type QdlfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDF` reader - 5:5\\]
Quadrature direction flag"]
pub type QdfR = crate::BitReader;
#[doc = "Field `QDF` writer - 5:5\\]
Quadrature direction flag"]
pub type QdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIDF` reader - 6:6\\]
Direction on the first index markerStatus of the direction is latched on the first index event marker."]
pub type FidfR = crate::BitReader;
#[doc = "Field `FIDF` writer - 6:6\\]
Direction on the first index markerStatus of the direction is latched on the first index event marker."]
pub type FidfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPEVNT` reader - 7:7\\]
Unit position event flag"]
pub type UpevntR = crate::BitReader;
#[doc = "Field `UPEVNT` writer - 7:7\\]
Unit position event flag"]
pub type UpevntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Position counter error flag. This bit is not sticky and it is updated for every index event."]
    #[inline(always)]
    pub fn pcef(&self) -> PcefR {
        PcefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
First index marker flag"]
    #[inline(always)]
    pub fn fimf(&self) -> FimfR {
        FimfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Capture direction error flag"]
    #[inline(always)]
    pub fn cdef(&self) -> CdefR {
        CdefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Capture overflow error flag"]
    #[inline(always)]
    pub fn coef(&self) -> CoefR {
        CoefR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
eQEP direction latch flag"]
    #[inline(always)]
    pub fn qdlf(&self) -> QdlfR {
        QdlfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Quadrature direction flag"]
    #[inline(always)]
    pub fn qdf(&self) -> QdfR {
        QdfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Direction on the first index markerStatus of the direction is latched on the first index event marker."]
    #[inline(always)]
    pub fn fidf(&self) -> FidfR {
        FidfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Unit position event flag"]
    #[inline(always)]
    pub fn upevnt(&self) -> UpevntR {
        UpevntR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Position counter error flag. This bit is not sticky and it is updated for every index event."]
    #[inline(always)]
    #[must_use]
    pub fn pcef(&mut self) -> PcefW<RegQepstsType1Spec> {
        PcefW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
First index marker flag"]
    #[inline(always)]
    #[must_use]
    pub fn fimf(&mut self) -> FimfW<RegQepstsType1Spec> {
        FimfW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Capture direction error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cdef(&mut self) -> CdefW<RegQepstsType1Spec> {
        CdefW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Capture overflow error flag"]
    #[inline(always)]
    #[must_use]
    pub fn coef(&mut self) -> CoefW<RegQepstsType1Spec> {
        CoefW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
eQEP direction latch flag"]
    #[inline(always)]
    #[must_use]
    pub fn qdlf(&mut self) -> QdlfW<RegQepstsType1Spec> {
        QdlfW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Quadrature direction flag"]
    #[inline(always)]
    #[must_use]
    pub fn qdf(&mut self) -> QdfW<RegQepstsType1Spec> {
        QdfW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Direction on the first index markerStatus of the direction is latched on the first index event marker."]
    #[inline(always)]
    #[must_use]
    pub fn fidf(&mut self) -> FidfW<RegQepstsType1Spec> {
        FidfW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Unit position event flag"]
    #[inline(always)]
    #[must_use]
    pub fn upevnt(&mut self) -> UpevntW<RegQepstsType1Spec> {
        UpevntW::new(self, 7)
    }
}
#[doc = "QEP Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qepsts_type1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qepsts_type1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQepstsType1Spec;
impl crate::RegisterSpec for RegQepstsType1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_qepsts_type1::R`](R) reader structure"]
impl crate::Readable for RegQepstsType1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_qepsts_type1::W`](W) writer structure"]
impl crate::Writable for RegQepstsType1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REG_QEPSTS_TYPE1 to value 0x80"]
impl crate::Resettable for RegQepstsType1Spec {
    const RESET_VALUE: u16 = 0x80;
}
