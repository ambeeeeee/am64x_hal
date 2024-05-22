#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF1C` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsRxf1cSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF1C` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsRxf1cSpec>;
#[doc = "Field `F1SA` reader - 15:2\\]
Rx FIFO 1 Start Address"]
pub type F1saR = crate::FieldReader<u16>;
#[doc = "Field `F1SA` writer - 15:2\\]
Rx FIFO 1 Start Address"]
pub type F1saW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `F1S` reader - 22:16\\]
Rx FIFO 1 Size"]
pub type F1sR = crate::FieldReader;
#[doc = "Field `F1S` writer - 22:16\\]
Rx FIFO 1 Size"]
pub type F1sW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F1WM` reader - 30:24\\]
Rx FIFO 1 Watermark"]
pub type F1wmR = crate::FieldReader;
#[doc = "Field `F1WM` writer - 30:24\\]
Rx FIFO 1 Watermark"]
pub type F1wmW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F1OM` reader - 31:31\\]
Rx FIFO 1 Operation Mode"]
pub type F1omR = crate::BitReader;
#[doc = "Field `F1OM` writer - 31:31\\]
Rx FIFO 1 Operation Mode"]
pub type F1omW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - 15:2\\]
Rx FIFO 1 Start Address"]
    #[inline(always)]
    pub fn f1sa(&self) -> F1saR {
        F1saR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Rx FIFO 1 Size"]
    #[inline(always)]
    pub fn f1s(&self) -> F1sR {
        F1sR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Rx FIFO 1 Watermark"]
    #[inline(always)]
    pub fn f1wm(&self) -> F1wmR {
        F1wmR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Rx FIFO 1 Operation Mode"]
    #[inline(always)]
    pub fn f1om(&self) -> F1omR {
        F1omR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - 15:2\\]
Rx FIFO 1 Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn f1sa(&mut self) -> F1saW<McanWrap_McanCfgVbp_McanRegsRxf1cSpec> {
        F1saW::new(self, 2)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Rx FIFO 1 Size"]
    #[inline(always)]
    #[must_use]
    pub fn f1s(&mut self) -> F1sW<McanWrap_McanCfgVbp_McanRegsRxf1cSpec> {
        F1sW::new(self, 16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Rx FIFO 1 Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn f1wm(&mut self) -> F1wmW<McanWrap_McanCfgVbp_McanRegsRxf1cSpec> {
        F1wmW::new(self, 24)
    }
    #[doc = "Bit 31 - 31:31\\]
Rx FIFO 1 Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn f1om(&mut self) -> F1omW<McanWrap_McanCfgVbp_McanRegsRxf1cSpec> {
        F1omW::new(self, 31)
    }
}
#[doc = "FIFO 1 operation mode, watermark, size and start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsRxf1cSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsRxf1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1c::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsRxf1cSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf1c::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsRxf1cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF1C to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsRxf1cSpec {
    const RESET_VALUE: u32 = 0;
}
