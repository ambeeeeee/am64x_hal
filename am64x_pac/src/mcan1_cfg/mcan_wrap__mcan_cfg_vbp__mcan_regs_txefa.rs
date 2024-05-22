#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFA` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsTxefaSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFA` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsTxefaSpec>;
#[doc = "Field `EFAI` reader - 4:0\\]
Event FIFO Acknowledge Index"]
pub type EfaiR = crate::FieldReader;
#[doc = "Field `EFAI` writer - 4:0\\]
Event FIFO Acknowledge Index"]
pub type EfaiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Event FIFO Acknowledge Index"]
    #[inline(always)]
    pub fn efai(&self) -> EfaiR {
        EfaiR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Event FIFO Acknowledge Index"]
    #[inline(always)]
    #[must_use]
    pub fn efai(&mut self) -> EfaiW<McanWrap_McanCfgVbp_McanRegsTxefaSpec> {
        EfaiW::new(self, 0)
    }
}
#[doc = "Tx event FIFO acknowledge last index of read elements, updates get index and fill level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsTxefaSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsTxefaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefa::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsTxefaSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefa::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsTxefaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFA to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsTxefaSpec {
    const RESET_VALUE: u32 = 0;
}
