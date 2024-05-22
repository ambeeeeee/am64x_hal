#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0S` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsRxf0sSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0S` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsRxf0sSpec>;
#[doc = "Field `F0FL` reader - 6:0\\]
Rx FIFO 0 Fill Level"]
pub type F0flR = crate::FieldReader;
#[doc = "Field `F0FL` writer - 6:0\\]
Rx FIFO 0 Fill Level"]
pub type F0flW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F0GI` reader - 13:8\\]
Rx FIFO 0 Get Index"]
pub type F0giR = crate::FieldReader;
#[doc = "Field `F0GI` writer - 13:8\\]
Rx FIFO 0 Get Index"]
pub type F0giW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `F0PI` reader - 21:16\\]
Rx FIFO 0 Put Index"]
pub type F0piR = crate::FieldReader;
#[doc = "Field `F0PI` writer - 21:16\\]
Rx FIFO 0 Put Index"]
pub type F0piW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `F0F` reader - 24:24\\]
Rx FIFO 0 Full"]
pub type F0fR = crate::BitReader;
#[doc = "Field `F0F` writer - 24:24\\]
Rx FIFO 0 Full"]
pub type F0fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0L` reader - 25:25\\]
Rx FIFO 0 Message Lost"]
pub type Rf0lR = crate::BitReader;
#[doc = "Field `RF0L` writer - 25:25\\]
Rx FIFO 0 Message Lost"]
pub type Rf0lW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Rx FIFO 0 Fill Level"]
    #[inline(always)]
    pub fn f0fl(&self) -> F0flR {
        F0flR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Rx FIFO 0 Get Index"]
    #[inline(always)]
    pub fn f0gi(&self) -> F0giR {
        F0giR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Rx FIFO 0 Put Index"]
    #[inline(always)]
    pub fn f0pi(&self) -> F0piR {
        F0piR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0fR {
        F0fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> Rf0lR {
        Rf0lR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Rx FIFO 0 Fill Level"]
    #[inline(always)]
    #[must_use]
    pub fn f0fl(&mut self) -> F0flW<McanWrap_McanCfgVbp_McanRegsRxf0sSpec> {
        F0flW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Rx FIFO 0 Get Index"]
    #[inline(always)]
    #[must_use]
    pub fn f0gi(&mut self) -> F0giW<McanWrap_McanCfgVbp_McanRegsRxf0sSpec> {
        F0giW::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Rx FIFO 0 Put Index"]
    #[inline(always)]
    #[must_use]
    pub fn f0pi(&mut self) -> F0piW<McanWrap_McanCfgVbp_McanRegsRxf0sSpec> {
        F0piW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Rx FIFO 0 Full"]
    #[inline(always)]
    #[must_use]
    pub fn f0f(&mut self) -> F0fW<McanWrap_McanCfgVbp_McanRegsRxf0sSpec> {
        F0fW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Rx FIFO 0 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> Rf0lW<McanWrap_McanCfgVbp_McanRegsRxf0sSpec> {
        Rf0lW::new(self, 25)
    }
}
#[doc = "FIFO 0 message lost/full indication, put index, get index and fill level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsRxf0sSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsRxf0sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0s::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsRxf0sSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0s::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsRxf0sSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0S to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsRxf0sSpec {
    const RESET_VALUE: u32 = 0;
}
