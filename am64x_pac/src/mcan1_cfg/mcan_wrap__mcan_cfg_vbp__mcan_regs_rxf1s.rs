#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF1S` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsRxf1sSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF1S` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsRxf1sSpec>;
#[doc = "Field `F1FL` reader - 6:0\\]
Rx FIFO 1 Fill Level"]
pub type F1flR = crate::FieldReader;
#[doc = "Field `F1FL` writer - 6:0\\]
Rx FIFO 1 Fill Level"]
pub type F1flW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F1GI` reader - 13:8\\]
Rx FIFO 1 Get Index"]
pub type F1giR = crate::FieldReader;
#[doc = "Field `F1GI` writer - 13:8\\]
Rx FIFO 1 Get Index"]
pub type F1giW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `F1PI` reader - 21:16\\]
Rx FIFO 1 Put Index"]
pub type F1piR = crate::FieldReader;
#[doc = "Field `F1PI` writer - 21:16\\]
Rx FIFO 1 Put Index"]
pub type F1piW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `F1F` reader - 24:24\\]
Rx FIFO 1 Full"]
pub type F1fR = crate::BitReader;
#[doc = "Field `F1F` writer - 24:24\\]
Rx FIFO 1 Full"]
pub type F1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1L` reader - 25:25\\]
Rx FIFO 1 Message Lost"]
pub type Rf1lR = crate::BitReader;
#[doc = "Field `RF1L` writer - 25:25\\]
Rx FIFO 1 Message Lost"]
pub type Rf1lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMS` reader - 31:30\\]
Debug Message Status"]
pub type DmsR = crate::FieldReader;
#[doc = "Field `DMS` writer - 31:30\\]
Debug Message Status"]
pub type DmsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Rx FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1flR {
        F1flR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Rx FIFO 1 Get Index"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1giR {
        F1giR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Rx FIFO 1 Put Index"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1piR {
        F1piR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Rx FIFO 1 Full"]
    #[inline(always)]
    pub fn f1f(&self) -> F1fR {
        F1fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> Rf1lR {
        Rf1lR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Debug Message Status"]
    #[inline(always)]
    pub fn dms(&self) -> DmsR {
        DmsR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Rx FIFO 1 Fill Level"]
    #[inline(always)]
    #[must_use]
    pub fn f1fl(&mut self) -> F1flW<McanWrap_McanCfgVbp_McanRegsRxf1sSpec> {
        F1flW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Rx FIFO 1 Get Index"]
    #[inline(always)]
    #[must_use]
    pub fn f1gi(&mut self) -> F1giW<McanWrap_McanCfgVbp_McanRegsRxf1sSpec> {
        F1giW::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Rx FIFO 1 Put Index"]
    #[inline(always)]
    #[must_use]
    pub fn f1pi(&mut self) -> F1piW<McanWrap_McanCfgVbp_McanRegsRxf1sSpec> {
        F1piW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Rx FIFO 1 Full"]
    #[inline(always)]
    #[must_use]
    pub fn f1f(&mut self) -> F1fW<McanWrap_McanCfgVbp_McanRegsRxf1sSpec> {
        F1fW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Rx FIFO 1 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf1l(&mut self) -> Rf1lW<McanWrap_McanCfgVbp_McanRegsRxf1sSpec> {
        Rf1lW::new(self, 25)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Debug Message Status"]
    #[inline(always)]
    #[must_use]
    pub fn dms(&mut self) -> DmsW<McanWrap_McanCfgVbp_McanRegsRxf1sSpec> {
        DmsW::new(self, 30)
    }
}
#[doc = "FIFO 1 message lost/full indication, put index, get index and fill level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsRxf1sSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsRxf1sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1s::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsRxf1sSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1s::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsRxf1sSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF1S to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsRxf1sSpec {
    const RESET_VALUE: u32 = 0;
}
