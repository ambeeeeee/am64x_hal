#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_XIDAM` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsXidamSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_XIDAM` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsXidamSpec>;
#[doc = "Field `EIDM` reader - 28:0\\]
Extended ID Mask"]
pub type EidmR = crate::FieldReader<u32>;
#[doc = "Field `EIDM` writer - 28:0\\]
Extended ID Mask"]
pub type EidmW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - 28:0\\]
Extended ID Mask"]
    #[inline(always)]
    pub fn eidm(&self) -> EidmR {
        EidmR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - 28:0\\]
Extended ID Mask"]
    #[inline(always)]
    #[must_use]
    pub fn eidm(&mut self) -> EidmW<McanWrap_McanCfgVbp_McanRegsXidamSpec> {
        EidmW::new(self, 0)
    }
}
#[doc = "29-bit logical AND mask for J1939\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_xidam::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_xidam::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsXidamSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsXidamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_xidam::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsXidamSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_xidam::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsXidamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_XIDAM to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsXidamSpec {
    const RESET_VALUE: u32 = 0;
}
