#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXBC` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsRxbcSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXBC` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsRxbcSpec>;
#[doc = "Field `RBSA` reader - 15:2\\]
Rx Buffer Start Address"]
pub type RbsaR = crate::FieldReader<u16>;
#[doc = "Field `RBSA` writer - 15:2\\]
Rx Buffer Start Address"]
pub type RbsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 2:15 - 15:2\\]
Rx Buffer Start Address"]
    #[inline(always)]
    pub fn rbsa(&self) -> RbsaR {
        RbsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:15 - 15:2\\]
Rx Buffer Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn rbsa(&mut self) -> RbsaW<McanWrap_McanCfgVbp_McanRegsRxbcSpec> {
        RbsaW::new(self, 2)
    }
}
#[doc = "Start address of Rx buffer section\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxbc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxbc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsRxbcSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsRxbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxbc::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsRxbcSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxbc::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsRxbcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXBC to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsRxbcSpec {
    const RESET_VALUE: u32 = 0;
}
