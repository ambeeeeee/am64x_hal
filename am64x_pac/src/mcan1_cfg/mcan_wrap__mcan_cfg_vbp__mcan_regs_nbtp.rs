#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_NBTP` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsNbtpSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_NBTP` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsNbtpSpec>;
#[doc = "Field `NTSEG2` reader - 6:0\\]
Nominal Time segment after sample point"]
pub type Ntseg2R = crate::FieldReader;
#[doc = "Field `NTSEG2` writer - 6:0\\]
Nominal Time segment after sample point"]
pub type Ntseg2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NTSEG1` reader - 15:8\\]
Nominal Time segment before sample point"]
pub type Ntseg1R = crate::FieldReader;
#[doc = "Field `NTSEG1` writer - 15:8\\]
Nominal Time segment before sample point"]
pub type Ntseg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NBRP` reader - 24:16\\]
Nominal Baud Rate Prescaler"]
pub type NbrpR = crate::FieldReader<u16>;
#[doc = "Field `NBRP` writer - 24:16\\]
Nominal Baud Rate Prescaler"]
pub type NbrpW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NSJW` reader - 31:25\\]
Nominal Resynchronization Jump Width"]
pub type NsjwR = crate::FieldReader;
#[doc = "Field `NSJW` writer - 31:25\\]
Nominal Resynchronization Jump Width"]
pub type NsjwW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Nominal Time segment after sample point"]
    #[inline(always)]
    pub fn ntseg2(&self) -> Ntseg2R {
        Ntseg2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Nominal Time segment before sample point"]
    #[inline(always)]
    pub fn ntseg1(&self) -> Ntseg1R {
        Ntseg1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Nominal Baud Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&self) -> NbrpR {
        NbrpR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Nominal Resynchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&self) -> NsjwR {
        NsjwR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Nominal Time segment after sample point"]
    #[inline(always)]
    #[must_use]
    pub fn ntseg2(&mut self) -> Ntseg2W<McanWrap_McanCfgVbp_McanRegsNbtpSpec> {
        Ntseg2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Nominal Time segment before sample point"]
    #[inline(always)]
    #[must_use]
    pub fn ntseg1(&mut self) -> Ntseg1W<McanWrap_McanCfgVbp_McanRegsNbtpSpec> {
        Ntseg1W::new(self, 8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Nominal Baud Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn nbrp(&mut self) -> NbrpW<McanWrap_McanCfgVbp_McanRegsNbtpSpec> {
        NbrpW::new(self, 16)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Nominal Resynchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn nsjw(&mut self) -> NsjwW<McanWrap_McanCfgVbp_McanRegsNbtpSpec> {
        NsjwW::new(self, 25)
    }
}
#[doc = "Configuration of arbitration phase bit timing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_nbtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_nbtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsNbtpSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsNbtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_nbtp::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsNbtpSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_nbtp::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsNbtpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_NBTP to value 0x0600_1003"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsNbtpSpec {
    const RESET_VALUE: u32 = 0x0600_1003;
}
