#[doc = "Register `REG_QDECCTL_TYPE2` reader"]
pub type R = crate::R<RegQdecctlType2Spec>;
#[doc = "Register `REG_QDECCTL_TYPE2` writer"]
pub type W = crate::W<RegQdecctlType2Spec>;
#[doc = "Field `QIDIRE` reader - 0:0\\]
0 - Compatible mode, Behavior same as existing devices1 - Enhancement for Direction change during Index will be enabled"]
pub type QidireR = crate::BitReader;
#[doc = "Field `QIDIRE` writer - 0:0\\]
0 - Compatible mode, Behavior same as existing devices1 - Enhancement for Direction change during Index will be enabled"]
pub type QidireW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSP` reader - 5:5\\]
QEPS input polarity"]
pub type QspR = crate::BitReader;
#[doc = "Field `QSP` writer - 5:5\\]
QEPS input polarity"]
pub type QspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QIP` reader - 6:6\\]
QEPI input polarity"]
pub type QipR = crate::BitReader;
#[doc = "Field `QIP` writer - 6:6\\]
QEPI input polarity"]
pub type QipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QBP` reader - 7:7\\]
QEPB input polarity"]
pub type QbpR = crate::BitReader;
#[doc = "Field `QBP` writer - 7:7\\]
QEPB input polarity"]
pub type QbpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QAP` reader - 8:8\\]
QEPA input polarity"]
pub type QapR = crate::BitReader;
#[doc = "Field `QAP` writer - 8:8\\]
QEPA input polarity"]
pub type QapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGATE` reader - 9:9\\]
Index pulse gating option"]
pub type IgateR = crate::BitReader;
#[doc = "Field `IGATE` writer - 9:9\\]
Index pulse gating option"]
pub type IgateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP` reader - 10:10\\]
CLK/DIR Signal Source for Position Counter"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - 10:10\\]
CLK/DIR Signal Source for Position Counter"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XCR` reader - 11:11\\]
External Clock Rate"]
pub type XcrR = crate::BitReader;
#[doc = "Field `XCR` writer - 11:11\\]
External Clock Rate"]
pub type XcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPSEL` reader - 12:12\\]
Sync output pin selection"]
pub type SpselR = crate::BitReader;
#[doc = "Field `SPSEL` writer - 12:12\\]
Sync output pin selection"]
pub type SpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOEN` reader - 13:13\\]
Sync output-enable"]
pub type SoenR = crate::BitReader;
#[doc = "Field `SOEN` writer - 13:13\\]
Sync output-enable"]
pub type SoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSRC` reader - 15:14\\]
Position-counter source selection"]
pub type QsrcR = crate::FieldReader;
#[doc = "Field `QSRC` writer - 15:14\\]
Position-counter source selection"]
pub type QsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0 - Compatible mode, Behavior same as existing devices1 - Enhancement for Direction change during Index will be enabled"]
    #[inline(always)]
    pub fn qidire(&self) -> QidireR {
        QidireR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
QEPS input polarity"]
    #[inline(always)]
    pub fn qsp(&self) -> QspR {
        QspR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
QEPI input polarity"]
    #[inline(always)]
    pub fn qip(&self) -> QipR {
        QipR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
QEPB input polarity"]
    #[inline(always)]
    pub fn qbp(&self) -> QbpR {
        QbpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
QEPA input polarity"]
    #[inline(always)]
    pub fn qap(&self) -> QapR {
        QapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Index pulse gating option"]
    #[inline(always)]
    pub fn igate(&self) -> IgateR {
        IgateR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
CLK/DIR Signal Source for Position Counter"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
External Clock Rate"]
    #[inline(always)]
    pub fn xcr(&self) -> XcrR {
        XcrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Sync output pin selection"]
    #[inline(always)]
    pub fn spsel(&self) -> SpselR {
        SpselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Sync output-enable"]
    #[inline(always)]
    pub fn soen(&self) -> SoenR {
        SoenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Position-counter source selection"]
    #[inline(always)]
    pub fn qsrc(&self) -> QsrcR {
        QsrcR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0 - Compatible mode, Behavior same as existing devices1 - Enhancement for Direction change during Index will be enabled"]
    #[inline(always)]
    #[must_use]
    pub fn qidire(&mut self) -> QidireW<RegQdecctlType2Spec> {
        QidireW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
QEPS input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn qsp(&mut self) -> QspW<RegQdecctlType2Spec> {
        QspW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
QEPI input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn qip(&mut self) -> QipW<RegQdecctlType2Spec> {
        QipW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
QEPB input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn qbp(&mut self) -> QbpW<RegQdecctlType2Spec> {
        QbpW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
QEPA input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn qap(&mut self) -> QapW<RegQdecctlType2Spec> {
        QapW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Index pulse gating option"]
    #[inline(always)]
    #[must_use]
    pub fn igate(&mut self) -> IgateW<RegQdecctlType2Spec> {
        IgateW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
CLK/DIR Signal Source for Position Counter"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SwapW<RegQdecctlType2Spec> {
        SwapW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
External Clock Rate"]
    #[inline(always)]
    #[must_use]
    pub fn xcr(&mut self) -> XcrW<RegQdecctlType2Spec> {
        XcrW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Sync output pin selection"]
    #[inline(always)]
    #[must_use]
    pub fn spsel(&mut self) -> SpselW<RegQdecctlType2Spec> {
        SpselW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Sync output-enable"]
    #[inline(always)]
    #[must_use]
    pub fn soen(&mut self) -> SoenW<RegQdecctlType2Spec> {
        SoenW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Position-counter source selection"]
    #[inline(always)]
    #[must_use]
    pub fn qsrc(&mut self) -> QsrcW<RegQdecctlType2Spec> {
        QsrcW::new(self, 14)
    }
}
#[doc = "Quadrature Decoder Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qdecctl_type2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qdecctl_type2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQdecctlType2Spec;
impl crate::RegisterSpec for RegQdecctlType2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_qdecctl_type2::R`](R) reader structure"]
impl crate::Readable for RegQdecctlType2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_qdecctl_type2::W`](W) writer structure"]
impl crate::Writable for RegQdecctlType2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REG_QDECCTL_TYPE2 to value 0"]
impl crate::Resettable for RegQdecctlType2Spec {
    const RESET_VALUE: u16 = 0;
}
