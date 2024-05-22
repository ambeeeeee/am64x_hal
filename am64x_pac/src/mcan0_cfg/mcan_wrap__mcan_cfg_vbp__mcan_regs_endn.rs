#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ENDN` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsEndnSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ENDN` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsEndnSpec>;
#[doc = "Field `ETV` reader - 31:0\\]
Endianess test value"]
pub type EtvR = crate::FieldReader<u32>;
#[doc = "Field `ETV` writer - 31:0\\]
Endianess test value"]
pub type EtvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Endianess test value"]
    #[inline(always)]
    pub fn etv(&self) -> EtvR {
        EtvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Endianess test value"]
    #[inline(always)]
    #[must_use]
    pub fn etv(&mut self) -> EtvW<McanWrap_McanCfgVbp_McanRegsEndnSpec> {
        EtvW::new(self, 0)
    }
}
#[doc = "Constant 0x8765 4321\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_endn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_endn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsEndnSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsEndnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_endn::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsEndnSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_endn::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsEndnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ENDN to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsEndnSpec {
    const RESET_VALUE: u32 = 0;
}
