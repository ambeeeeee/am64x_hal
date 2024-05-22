#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TDCR` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsTdcrSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TDCR` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsTdcrSpec>;
#[doc = "Field `TDCF` reader - 6:0\\]
Transmitter Delay Compensation Filter Window Length"]
pub type TdcfR = crate::FieldReader;
#[doc = "Field `TDCF` writer - 6:0\\]
Transmitter Delay Compensation Filter Window Length"]
pub type TdcfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TDCO` reader - 14:8\\]
Transmitter Delay Compensation Offset"]
pub type TdcoR = crate::FieldReader;
#[doc = "Field `TDCO` writer - 14:8\\]
Transmitter Delay Compensation Offset"]
pub type TdcoW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Transmitter Delay Compensation Filter Window Length"]
    #[inline(always)]
    pub fn tdcf(&self) -> TdcfR {
        TdcfR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Transmitter Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(&self) -> TdcoR {
        TdcoR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Transmitter Delay Compensation Filter Window Length"]
    #[inline(always)]
    #[must_use]
    pub fn tdcf(&mut self) -> TdcfW<McanWrap_McanCfgVbp_McanRegsTdcrSpec> {
        TdcfW::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Transmitter Delay Compensation Offset"]
    #[inline(always)]
    #[must_use]
    pub fn tdco(&mut self) -> TdcoW<McanWrap_McanCfgVbp_McanRegsTdcrSpec> {
        TdcoW::new(self, 8)
    }
}
#[doc = "configuration of transmitter delay compensation offset and filter window length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsTdcrSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsTdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tdcr::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsTdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tdcr::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsTdcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TDCR to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsTdcrSpec {
    const RESET_VALUE: u32 = 0;
}
