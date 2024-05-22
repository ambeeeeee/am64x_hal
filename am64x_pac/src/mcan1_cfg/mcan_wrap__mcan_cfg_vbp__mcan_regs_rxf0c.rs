#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0C` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsRxf0cSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0C` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsRxf0cSpec>;
#[doc = "Field `F0SA` reader - 15:2\\]
Rx FIFO 0 Start Address"]
pub type F0saR = crate::FieldReader<u16>;
#[doc = "Field `F0SA` writer - 15:2\\]
Rx FIFO 0 Start Address"]
pub type F0saW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `F0S` reader - 22:16\\]
Rx FIFO 0 Size"]
pub type F0sR = crate::FieldReader;
#[doc = "Field `F0S` writer - 22:16\\]
Rx FIFO 0 Size"]
pub type F0sW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F0WM` reader - 30:24\\]
Rx FIFO 0 Watermark"]
pub type F0wmR = crate::FieldReader;
#[doc = "Field `F0WM` writer - 30:24\\]
Rx FIFO 0 Watermark"]
pub type F0wmW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F0OM` reader - 31:31\\]
Rx FIFO 0 Operation Mode"]
pub type F0omR = crate::BitReader;
#[doc = "Field `F0OM` writer - 31:31\\]
Rx FIFO 0 Operation Mode"]
pub type F0omW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - 15:2\\]
Rx FIFO 0 Start Address"]
    #[inline(always)]
    pub fn f0sa(&self) -> F0saR {
        F0saR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Rx FIFO 0 Size"]
    #[inline(always)]
    pub fn f0s(&self) -> F0sR {
        F0sR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Rx FIFO 0 Watermark"]
    #[inline(always)]
    pub fn f0wm(&self) -> F0wmR {
        F0wmR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Rx FIFO 0 Operation Mode"]
    #[inline(always)]
    pub fn f0om(&self) -> F0omR {
        F0omR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - 15:2\\]
Rx FIFO 0 Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn f0sa(&mut self) -> F0saW<McanWrap_McanCfgVbp_McanRegsRxf0cSpec> {
        F0saW::new(self, 2)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Rx FIFO 0 Size"]
    #[inline(always)]
    #[must_use]
    pub fn f0s(&mut self) -> F0sW<McanWrap_McanCfgVbp_McanRegsRxf0cSpec> {
        F0sW::new(self, 16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Rx FIFO 0 Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn f0wm(&mut self) -> F0wmW<McanWrap_McanCfgVbp_McanRegsRxf0cSpec> {
        F0wmW::new(self, 24)
    }
    #[doc = "Bit 31 - 31:31\\]
Rx FIFO 0 Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn f0om(&mut self) -> F0omW<McanWrap_McanCfgVbp_McanRegsRxf0cSpec> {
        F0omW::new(self, 31)
    }
}
#[doc = "FIFO 0 operation mode, watermark, size and start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsRxf0cSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsRxf0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0c::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsRxf0cSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rxf0c::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsRxf0cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RXF0C to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsRxf0cSpec {
    const RESET_VALUE: u32 = 0;
}
