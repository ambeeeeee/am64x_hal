#[doc = "Register `CFG_pll2_HSDIV_CTRL9` reader"]
pub type R = crate::R<CfgPll2HsdivCtrl9Spec>;
#[doc = "Register `CFG_pll2_HSDIV_CTRL9` writer"]
pub type W = crate::W<CfgPll2HsdivCtrl9Spec>;
#[doc = "Field `HSDIV` reader - 6:0\\]
CLKOUT0 divider value (HSDIV1+1) Allowed values are 0-127"]
pub type HsdivR = crate::FieldReader;
#[doc = "Field `HSDIV` writer - 6:0\\]
CLKOUT0 divider value (HSDIV1+1) Allowed values are 0-127"]
pub type HsdivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SYNC_DIS` reader - 8:8\\]
Disable divider synchronization logic 0 - Changes to DIV value synchronized to prevent glitches 1 - Changes are asynchronous"]
pub type SyncDisR = crate::BitReader;
#[doc = "Field `SYNC_DIS` writer - 8:8\\]
Disable divider synchronization logic 0 - Changes to DIV value synchronized to prevent glitches 1 - Changes are asynchronous"]
pub type SyncDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUT_EN` reader - 15:15\\]
CLKOUT1 enable 1'b0 - Synchronously disable CLKOUT1 1'b1 - Synchronously enable CLKOUT1"]
pub type ClkoutEnR = crate::BitReader;
#[doc = "Field `CLKOUT_EN` writer - 15:15\\]
CLKOUT1 enable 1'b0 - Synchronously disable CLKOUT1 1'b1 - Synchronously enable CLKOUT1"]
pub type ClkoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - 31:31\\]
SSM reset. When set to 1 the SSM modulator is in resetl"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - 31:31\\]
SSM reset. When set to 1 the SSM modulator is in resetl"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
CLKOUT0 divider value (HSDIV1+1) Allowed values are 0-127"]
    #[inline(always)]
    pub fn hsdiv(&self) -> HsdivR {
        HsdivR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Disable divider synchronization logic 0 - Changes to DIV value synchronized to prevent glitches 1 - Changes are asynchronous"]
    #[inline(always)]
    pub fn sync_dis(&self) -> SyncDisR {
        SyncDisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
CLKOUT1 enable 1'b0 - Synchronously disable CLKOUT1 1'b1 - Synchronously enable CLKOUT1"]
    #[inline(always)]
    pub fn clkout_en(&self) -> ClkoutEnR {
        ClkoutEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
SSM reset. When set to 1 the SSM modulator is in resetl"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
CLKOUT0 divider value (HSDIV1+1) Allowed values are 0-127"]
    #[inline(always)]
    #[must_use]
    pub fn hsdiv(&mut self) -> HsdivW<CfgPll2HsdivCtrl9Spec> {
        HsdivW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Disable divider synchronization logic 0 - Changes to DIV value synchronized to prevent glitches 1 - Changes are asynchronous"]
    #[inline(always)]
    #[must_use]
    pub fn sync_dis(&mut self) -> SyncDisW<CfgPll2HsdivCtrl9Spec> {
        SyncDisW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
CLKOUT1 enable 1'b0 - Synchronously disable CLKOUT1 1'b1 - Synchronously enable CLKOUT1"]
    #[inline(always)]
    #[must_use]
    pub fn clkout_en(&mut self) -> ClkoutEnW<CfgPll2HsdivCtrl9Spec> {
        ClkoutEnW::new(self, 15)
    }
    #[doc = "Bit 31 - 31:31\\]
SSM reset. When set to 1 the SSM modulator is in resetl"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<CfgPll2HsdivCtrl9Spec> {
        ResetW::new(self, 31)
    }
}
#[doc = "CFG_pll2_HSDIV_CTRL9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_hsdiv_ctrl9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_hsdiv_ctrl9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll2HsdivCtrl9Spec;
impl crate::RegisterSpec for CfgPll2HsdivCtrl9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll2_hsdiv_ctrl9::R`](R) reader structure"]
impl crate::Readable for CfgPll2HsdivCtrl9Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll2_hsdiv_ctrl9::W`](W) writer structure"]
impl crate::Writable for CfgPll2HsdivCtrl9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll2_HSDIV_CTRL9 to value 0"]
impl crate::Resettable for CfgPll2HsdivCtrl9Spec {
    const RESET_VALUE: u32 = 0;
}
