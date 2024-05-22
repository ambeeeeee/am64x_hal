#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_HPMS` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsHpmsSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_HPMS` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsHpmsSpec>;
#[doc = "Field `BIDX` reader - 5:0\\]
Buffer Index"]
pub type BidxR = crate::FieldReader;
#[doc = "Field `BIDX` writer - 5:0\\]
Buffer Index"]
pub type BidxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MSI` reader - 7:6\\]
Message Storage Indicator"]
pub type MsiR = crate::FieldReader;
#[doc = "Field `MSI` writer - 7:6\\]
Message Storage Indicator"]
pub type MsiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIDX` reader - 14:8\\]
Filter Index"]
pub type FidxR = crate::FieldReader;
#[doc = "Field `FIDX` writer - 14:8\\]
Filter Index"]
pub type FidxW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FLST` reader - 15:15\\]
Filter List"]
pub type FlstR = crate::BitReader;
#[doc = "Field `FLST` writer - 15:15\\]
Filter List"]
pub type FlstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Buffer Index"]
    #[inline(always)]
    pub fn bidx(&self) -> BidxR {
        BidxR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Message Storage Indicator"]
    #[inline(always)]
    pub fn msi(&self) -> MsiR {
        MsiR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Filter Index"]
    #[inline(always)]
    pub fn fidx(&self) -> FidxR {
        FidxR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Filter List"]
    #[inline(always)]
    pub fn flst(&self) -> FlstR {
        FlstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Buffer Index"]
    #[inline(always)]
    #[must_use]
    pub fn bidx(&mut self) -> BidxW<McanWrap_McanCfgVbp_McanRegsHpmsSpec> {
        BidxW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Message Storage Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn msi(&mut self) -> MsiW<McanWrap_McanCfgVbp_McanRegsHpmsSpec> {
        MsiW::new(self, 6)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Filter Index"]
    #[inline(always)]
    #[must_use]
    pub fn fidx(&mut self) -> FidxW<McanWrap_McanCfgVbp_McanRegsHpmsSpec> {
        FidxW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
Filter List"]
    #[inline(always)]
    #[must_use]
    pub fn flst(&mut self) -> FlstW<McanWrap_McanCfgVbp_McanRegsHpmsSpec> {
        FlstW::new(self, 15)
    }
}
#[doc = "Status monitoring of incoming high priority messages\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_hpms::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_hpms::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsHpmsSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsHpmsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_hpms::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsHpmsSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_hpms::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsHpmsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_HPMS to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsHpmsSpec {
    const RESET_VALUE: u32 = 0;
}
