#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0A` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsRxf0aSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0A` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsRxf0aSpec>;
#[doc = "Field `F0AI` reader - 5:0\\]
Rx FIFO 0 Acknowledge Index"]
pub type F0aiR = crate::FieldReader;
#[doc = "Field `F0AI` writer - 5:0\\]
Rx FIFO 0 Acknowledge Index"]
pub type F0aiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    pub fn f0ai(&self) -> F0aiR {
        F0aiR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    #[must_use]
    pub fn f0ai(&mut self) -> F0aiW<McanWrap_McanCfgVbp_McanRegsRxf0aSpec> {
        F0aiW::new(self, 0)
    }
}
#[doc = "FIFO 0 acknowledge last index of read buffers, updates get index and fill level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsRxf0aSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsRxf0aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0a::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsRxf0aSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0a::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsRxf0aSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0A to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsRxf0aSpec {
    const RESET_VALUE: u32 = 0;
}
