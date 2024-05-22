#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_XIDFC` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsXidfcSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_XIDFC` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsXidfcSpec>;
#[doc = "Field `FLESA` reader - 15:2\\]
Filter List Extended Start Address"]
pub type FlesaR = crate::FieldReader<u16>;
#[doc = "Field `FLESA` writer - 15:2\\]
Filter List Extended Start Address"]
pub type FlesaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LSE` reader - 22:16\\]
List Size Extended"]
pub type LseR = crate::FieldReader;
#[doc = "Field `LSE` writer - 22:16\\]
List Size Extended"]
pub type LseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 2:15 - 15:2\\]
Filter List Extended Start Address"]
    #[inline(always)]
    pub fn flesa(&self) -> FlesaR {
        FlesaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - 22:16\\]
List Size Extended"]
    #[inline(always)]
    pub fn lse(&self) -> LseR {
        LseR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - 15:2\\]
Filter List Extended Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn flesa(&mut self) -> FlesaW<McanWrap_McanCfgVbp_McanRegsXidfcSpec> {
        FlesaW::new(self, 2)
    }
    #[doc = "Bits 16:22 - 22:16\\]
List Size Extended"]
    #[inline(always)]
    #[must_use]
    pub fn lse(&mut self) -> LseW<McanWrap_McanCfgVbp_McanRegsXidfcSpec> {
        LseW::new(self, 16)
    }
}
#[doc = "Number of filter elements, pointer to start of filter list\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_xidfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_xidfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsXidfcSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsXidfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_xidfc::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsXidfcSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_xidfc::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsXidfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_XIDFC to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsXidfcSpec {
    const RESET_VALUE: u32 = 0;
}
