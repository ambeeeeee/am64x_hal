#[doc = "Register `CFG0_HFOSC0_CLKOUT_32K_CTRL` reader"]
pub type R = crate::R<Cfg0Hfosc0Clkout32kCtrlSpec>;
#[doc = "Register `CFG0_HFOSC0_CLKOUT_32K_CTRL` writer"]
pub type W = crate::W<Cfg0Hfosc0Clkout32kCtrlSpec>;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_HSDIV` reader - 6:0\\]
HFOSC0_CLKOUT_32K divider:"]
pub type Hfosc0Clkout32kCtrlHsdivR = crate::FieldReader;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_HSDIV` writer - 6:0\\]
HFOSC0_CLKOUT_32K divider:"]
pub type Hfosc0Clkout32kCtrlHsdivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_SYNC_DIS` reader - 8:8\\]
HFOSC0_CLKOUT_32K Synchronize Deactivate"]
pub type Hfosc0Clkout32kCtrlSyncDisR = crate::BitReader;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_SYNC_DIS` writer - 8:8\\]
HFOSC0_CLKOUT_32K Synchronize Deactivate"]
pub type Hfosc0Clkout32kCtrlSyncDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_CLKOUT_EN` reader - 15:15\\]
HFOSC0_CLKOUT_32K active:"]
pub type Hfosc0Clkout32kCtrlClkoutEnR = crate::BitReader;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_CLKOUT_EN` writer - 15:15\\]
HFOSC0_CLKOUT_32K active:"]
pub type Hfosc0Clkout32kCtrlClkoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_RESET` reader - 31:31\\]
Asynchronous Divider Reset"]
pub type Hfosc0Clkout32kCtrlResetR = crate::BitReader;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_RESET` writer - 31:31\\]
Asynchronous Divider Reset"]
pub type Hfosc0Clkout32kCtrlResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
HFOSC0_CLKOUT_32K divider:"]
    #[inline(always)]
    pub fn hfosc0_clkout_32k_ctrl_hsdiv(&self) -> Hfosc0Clkout32kCtrlHsdivR {
        Hfosc0Clkout32kCtrlHsdivR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
HFOSC0_CLKOUT_32K Synchronize Deactivate"]
    #[inline(always)]
    pub fn hfosc0_clkout_32k_ctrl_sync_dis(&self) -> Hfosc0Clkout32kCtrlSyncDisR {
        Hfosc0Clkout32kCtrlSyncDisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
HFOSC0_CLKOUT_32K active:"]
    #[inline(always)]
    pub fn hfosc0_clkout_32k_ctrl_clkout_en(&self) -> Hfosc0Clkout32kCtrlClkoutEnR {
        Hfosc0Clkout32kCtrlClkoutEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Asynchronous Divider Reset"]
    #[inline(always)]
    pub fn hfosc0_clkout_32k_ctrl_reset(&self) -> Hfosc0Clkout32kCtrlResetR {
        Hfosc0Clkout32kCtrlResetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
HFOSC0_CLKOUT_32K divider:"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_clkout_32k_ctrl_hsdiv(
        &mut self,
    ) -> Hfosc0Clkout32kCtrlHsdivW<Cfg0Hfosc0Clkout32kCtrlSpec> {
        Hfosc0Clkout32kCtrlHsdivW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
HFOSC0_CLKOUT_32K Synchronize Deactivate"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_clkout_32k_ctrl_sync_dis(
        &mut self,
    ) -> Hfosc0Clkout32kCtrlSyncDisW<Cfg0Hfosc0Clkout32kCtrlSpec> {
        Hfosc0Clkout32kCtrlSyncDisW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
HFOSC0_CLKOUT_32K active:"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_clkout_32k_ctrl_clkout_en(
        &mut self,
    ) -> Hfosc0Clkout32kCtrlClkoutEnW<Cfg0Hfosc0Clkout32kCtrlSpec> {
        Hfosc0Clkout32kCtrlClkoutEnW::new(self, 15)
    }
    #[doc = "Bit 31 - 31:31\\]
Asynchronous Divider Reset"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_clkout_32k_ctrl_reset(
        &mut self,
    ) -> Hfosc0Clkout32kCtrlResetW<Cfg0Hfosc0Clkout32kCtrlSpec> {
        Hfosc0Clkout32kCtrlResetW::new(self, 31)
    }
}
#[doc = "CFG0_HFOSC0_CLKOUT_32K_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_hfosc0_clkout_32k_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_hfosc0_clkout_32k_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Hfosc0Clkout32kCtrlSpec;
impl crate::RegisterSpec for Cfg0Hfosc0Clkout32kCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_hfosc0_clkout_32k_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Hfosc0Clkout32kCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_hfosc0_clkout_32k_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Hfosc0Clkout32kCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_HFOSC0_CLKOUT_32K_CTRL to value 0x8104"]
impl crate::Resettable for Cfg0Hfosc0Clkout32kCtrlSpec {
    const RESET_VALUE: u32 = 0x8104;
}
