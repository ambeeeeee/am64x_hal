#[doc = "Register `CFG0_HFOSC0_CLKOUT_32K_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Hfosc0Clkout32kCtrlProxySpec>;
#[doc = "Register `CFG0_HFOSC0_CLKOUT_32K_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Hfosc0Clkout32kCtrlProxySpec>;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_HSDIV_PROXY` reader - 6:0\\]
HFOSC0_CLKOUT_32K divider:"]
pub type Hfosc0Clkout32kCtrlHsdivProxyR = crate::FieldReader;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_HSDIV_PROXY` writer - 6:0\\]
HFOSC0_CLKOUT_32K divider:"]
pub type Hfosc0Clkout32kCtrlHsdivProxyW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_SYNC_DIS_PROXY` reader - 8:8\\]
HFOSC0_CLKOUT_32K Synchronize Deactivate"]
pub type Hfosc0Clkout32kCtrlSyncDisProxyR = crate::BitReader;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_SYNC_DIS_PROXY` writer - 8:8\\]
HFOSC0_CLKOUT_32K Synchronize Deactivate"]
pub type Hfosc0Clkout32kCtrlSyncDisProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_CLKOUT_EN_PROXY` reader - 15:15\\]
HFOSC0_CLKOUT_32K active:"]
pub type Hfosc0Clkout32kCtrlClkoutEnProxyR = crate::BitReader;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_CLKOUT_EN_PROXY` writer - 15:15\\]
HFOSC0_CLKOUT_32K active:"]
pub type Hfosc0Clkout32kCtrlClkoutEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_RESET_PROXY` reader - 31:31\\]
Asynchronous Divider Reset"]
pub type Hfosc0Clkout32kCtrlResetProxyR = crate::BitReader;
#[doc = "Field `HFOSC0_CLKOUT_32K_CTRL_RESET_PROXY` writer - 31:31\\]
Asynchronous Divider Reset"]
pub type Hfosc0Clkout32kCtrlResetProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
HFOSC0_CLKOUT_32K divider:"]
    #[inline(always)]
    pub fn hfosc0_clkout_32k_ctrl_hsdiv_proxy(&self) -> Hfosc0Clkout32kCtrlHsdivProxyR {
        Hfosc0Clkout32kCtrlHsdivProxyR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
HFOSC0_CLKOUT_32K Synchronize Deactivate"]
    #[inline(always)]
    pub fn hfosc0_clkout_32k_ctrl_sync_dis_proxy(&self) -> Hfosc0Clkout32kCtrlSyncDisProxyR {
        Hfosc0Clkout32kCtrlSyncDisProxyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
HFOSC0_CLKOUT_32K active:"]
    #[inline(always)]
    pub fn hfosc0_clkout_32k_ctrl_clkout_en_proxy(&self) -> Hfosc0Clkout32kCtrlClkoutEnProxyR {
        Hfosc0Clkout32kCtrlClkoutEnProxyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Asynchronous Divider Reset"]
    #[inline(always)]
    pub fn hfosc0_clkout_32k_ctrl_reset_proxy(&self) -> Hfosc0Clkout32kCtrlResetProxyR {
        Hfosc0Clkout32kCtrlResetProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
HFOSC0_CLKOUT_32K divider:"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_clkout_32k_ctrl_hsdiv_proxy(
        &mut self,
    ) -> Hfosc0Clkout32kCtrlHsdivProxyW<Cfg0Hfosc0Clkout32kCtrlProxySpec> {
        Hfosc0Clkout32kCtrlHsdivProxyW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
HFOSC0_CLKOUT_32K Synchronize Deactivate"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_clkout_32k_ctrl_sync_dis_proxy(
        &mut self,
    ) -> Hfosc0Clkout32kCtrlSyncDisProxyW<Cfg0Hfosc0Clkout32kCtrlProxySpec> {
        Hfosc0Clkout32kCtrlSyncDisProxyW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
HFOSC0_CLKOUT_32K active:"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_clkout_32k_ctrl_clkout_en_proxy(
        &mut self,
    ) -> Hfosc0Clkout32kCtrlClkoutEnProxyW<Cfg0Hfosc0Clkout32kCtrlProxySpec> {
        Hfosc0Clkout32kCtrlClkoutEnProxyW::new(self, 15)
    }
    #[doc = "Bit 31 - 31:31\\]
Asynchronous Divider Reset"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_clkout_32k_ctrl_reset_proxy(
        &mut self,
    ) -> Hfosc0Clkout32kCtrlResetProxyW<Cfg0Hfosc0Clkout32kCtrlProxySpec> {
        Hfosc0Clkout32kCtrlResetProxyW::new(self, 31)
    }
}
#[doc = "CFG0_HFOSC0_CLKOUT_32K_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_hfosc0_clkout_32k_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_hfosc0_clkout_32k_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Hfosc0Clkout32kCtrlProxySpec;
impl crate::RegisterSpec for Cfg0Hfosc0Clkout32kCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_hfosc0_clkout_32k_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Hfosc0Clkout32kCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_hfosc0_clkout_32k_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Hfosc0Clkout32kCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_HFOSC0_CLKOUT_32K_CTRL_PROXY to value 0x8104"]
impl crate::Resettable for Cfg0Hfosc0Clkout32kCtrlProxySpec {
    const RESET_VALUE: u32 = 0x8104;
}
