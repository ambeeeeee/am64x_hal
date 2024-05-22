#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXESC` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsTxescSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXESC` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsTxescSpec>;
#[doc = "Field `TBDS` reader - 2:0\\]
Tx Buffer Data Field Size"]
pub type TbdsR = crate::FieldReader;
#[doc = "Field `TBDS` writer - 2:0\\]
Tx Buffer Data Field Size"]
pub type TbdsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Tx Buffer Data Field Size"]
    #[inline(always)]
    pub fn tbds(&self) -> TbdsR {
        TbdsR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Tx Buffer Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn tbds(&mut self) -> TbdsW<McanWrap_McanCfgVbp_McanRegsTxescSpec> {
        TbdsW::new(self, 0)
    }
}
#[doc = "Configure data field size for frame transmission\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txesc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txesc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsTxescSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsTxescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txesc::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsTxescSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txesc::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsTxescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXESC to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsTxescSpec {
    const RESET_VALUE: u32 = 0;
}
