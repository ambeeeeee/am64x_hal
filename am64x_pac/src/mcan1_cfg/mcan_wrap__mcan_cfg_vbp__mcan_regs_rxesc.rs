#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXESC` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsRxescSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXESC` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsRxescSpec>;
#[doc = "Field `F0DS` reader - 2:0\\]
Rx FIFO 0 Data Field Size"]
pub type F0dsR = crate::FieldReader;
#[doc = "Field `F0DS` writer - 2:0\\]
Rx FIFO 0 Data Field Size"]
pub type F0dsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `F1DS` reader - 6:4\\]
Rx FIFO 1 Data Field Size"]
pub type F1dsR = crate::FieldReader;
#[doc = "Field `F1DS` writer - 6:4\\]
Rx FIFO 1 Data Field Size"]
pub type F1dsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RBDS` reader - 10:8\\]
Rx Buffer data Field Size"]
pub type RbdsR = crate::FieldReader;
#[doc = "Field `RBDS` writer - 10:8\\]
Rx Buffer data Field Size"]
pub type RbdsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Rx FIFO 0 Data Field Size"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0dsR {
        F0dsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Rx FIFO 1 Data Field Size"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1dsR {
        F1dsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Rx Buffer data Field Size"]
    #[inline(always)]
    pub fn rbds(&self) -> RbdsR {
        RbdsR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Rx FIFO 0 Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn f0ds(&mut self) -> F0dsW<McanWrap_McanCfgVbp_McanRegsRxescSpec> {
        F0dsW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Rx FIFO 1 Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn f1ds(&mut self) -> F1dsW<McanWrap_McanCfgVbp_McanRegsRxescSpec> {
        F1dsW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Rx Buffer data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn rbds(&mut self) -> RbdsW<McanWrap_McanCfgVbp_McanRegsRxescSpec> {
        RbdsW::new(self, 8)
    }
}
#[doc = "Configure data field size for storage of accepted frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxesc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxesc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsRxescSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsRxescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxesc::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsRxescSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxesc::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsRxescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXESC to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsRxescSpec {
    const RESET_VALUE: u32 = 0;
}
