#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_DBTP` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsDbtpSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_DBTP` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsDbtpSpec>;
#[doc = "Field `DSJW` reader - 3:0\\]
Data resynchronization Jump Width"]
pub type DsjwR = crate::FieldReader;
#[doc = "Field `DSJW` writer - 3:0\\]
Data resynchronization Jump Width"]
pub type DsjwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG2` reader - 7:4\\]
Data time segment after sample point"]
pub type Dtseg2R = crate::FieldReader;
#[doc = "Field `DTSEG2` writer - 7:4\\]
Data time segment after sample point"]
pub type Dtseg2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG1` reader - 12:8\\]
Data time segment before sample point"]
pub type Dtseg1R = crate::FieldReader;
#[doc = "Field `DTSEG1` writer - 12:8\\]
Data time segment before sample point"]
pub type Dtseg1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBRP` reader - 20:16\\]
Data Baud Rate Prescaler"]
pub type DbrpR = crate::FieldReader;
#[doc = "Field `DBRP` writer - 20:16\\]
Data Baud Rate Prescaler"]
pub type DbrpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TDC` reader - 23:23\\]
Transmitter Delay Compensation"]
pub type TdcR = crate::BitReader;
#[doc = "Field `TDC` writer - 23:23\\]
Transmitter Delay Compensation"]
pub type TdcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Data resynchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(&self) -> DsjwR {
        DsjwR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Data time segment after sample point"]
    #[inline(always)]
    pub fn dtseg2(&self) -> Dtseg2R {
        Dtseg2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Data time segment before sample point"]
    #[inline(always)]
    pub fn dtseg1(&self) -> Dtseg1R {
        Dtseg1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Data Baud Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(&self) -> DbrpR {
        DbrpR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Transmitter Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TdcR {
        TdcR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Data resynchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn dsjw(&mut self) -> DsjwW<McanWrap_McanCfgVbp_McanRegsDbtpSpec> {
        DsjwW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Data time segment after sample point"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg2(&mut self) -> Dtseg2W<McanWrap_McanCfgVbp_McanRegsDbtpSpec> {
        Dtseg2W::new(self, 4)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Data time segment before sample point"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg1(&mut self) -> Dtseg1W<McanWrap_McanCfgVbp_McanRegsDbtpSpec> {
        Dtseg1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Data Baud Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn dbrp(&mut self) -> DbrpW<McanWrap_McanCfgVbp_McanRegsDbtpSpec> {
        DbrpW::new(self, 16)
    }
    #[doc = "Bit 23 - 23:23\\]
Transmitter Delay Compensation"]
    #[inline(always)]
    #[must_use]
    pub fn tdc(&mut self) -> TdcW<McanWrap_McanCfgVbp_McanRegsDbtpSpec> {
        TdcW::new(self, 23)
    }
}
#[doc = "Configuration of data phase bit timing, transmitter delay compensation enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_dbtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_dbtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsDbtpSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsDbtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_dbtp::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsDbtpSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_dbtp::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsDbtpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_DBTP to value 0x1033"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsDbtpSpec {
    const RESET_VALUE: u32 = 0x1033;
}
