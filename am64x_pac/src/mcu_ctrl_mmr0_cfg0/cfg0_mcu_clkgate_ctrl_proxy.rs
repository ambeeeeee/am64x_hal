#[doc = "Register `CFG0_MCU_CLKGATE_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0McuClkgateCtrlProxySpec>;
#[doc = "Register `CFG0_MCU_CLKGATE_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0McuClkgateCtrlProxySpec>;
#[doc = "Field `MCU_CLKGATE_CTRL_MCU_CBA_NOGATE_PROXY` reader - 0:0\\]
MCU domain Data bus (mcu_cbass) clock gate deactivate."]
pub type McuClkgateCtrlMcuCbaNogateProxyR = crate::BitReader;
#[doc = "Field `MCU_CLKGATE_CTRL_MCU_CBA_NOGATE_PROXY` writer - 0:0\\]
MCU domain Data bus (mcu_cbass) clock gate deactivate."]
pub type McuClkgateCtrlMcuCbaNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_CLKGATE_CTRL_MCU_CBA_ECC_AGGR_NOGATE_PROXY` reader - 1:1\\]
MCU domain Pulsar clock gate deactivate."]
pub type McuClkgateCtrlMcuCbaEccAggrNogateProxyR = crate::BitReader;
#[doc = "Field `MCU_CLKGATE_CTRL_MCU_CBA_ECC_AGGR_NOGATE_PROXY` writer - 1:1\\]
MCU domain Pulsar clock gate deactivate."]
pub type McuClkgateCtrlMcuCbaEccAggrNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_CLKGATE_CTRL_MCU_M4FSS_NOGATE_PROXY` reader - 8:8\\]
MCU domain M4FSS0 clock gate deactivate."]
pub type McuClkgateCtrlMcuM4fssNogateProxyR = crate::BitReader;
#[doc = "Field `MCU_CLKGATE_CTRL_MCU_M4FSS_NOGATE_PROXY` writer - 8:8\\]
MCU domain M4FSS0 clock gate deactivate."]
pub type McuClkgateCtrlMcuM4fssNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MCU domain Data bus (mcu_cbass) clock gate deactivate."]
    #[inline(always)]
    pub fn mcu_clkgate_ctrl_mcu_cba_nogate_proxy(&self) -> McuClkgateCtrlMcuCbaNogateProxyR {
        McuClkgateCtrlMcuCbaNogateProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MCU domain Pulsar clock gate deactivate."]
    #[inline(always)]
    pub fn mcu_clkgate_ctrl_mcu_cba_ecc_aggr_nogate_proxy(
        &self,
    ) -> McuClkgateCtrlMcuCbaEccAggrNogateProxyR {
        McuClkgateCtrlMcuCbaEccAggrNogateProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
MCU domain M4FSS0 clock gate deactivate."]
    #[inline(always)]
    pub fn mcu_clkgate_ctrl_mcu_m4fss_nogate_proxy(&self) -> McuClkgateCtrlMcuM4fssNogateProxyR {
        McuClkgateCtrlMcuM4fssNogateProxyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MCU domain Data bus (mcu_cbass) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_clkgate_ctrl_mcu_cba_nogate_proxy(
        &mut self,
    ) -> McuClkgateCtrlMcuCbaNogateProxyW<Cfg0McuClkgateCtrlProxySpec> {
        McuClkgateCtrlMcuCbaNogateProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MCU domain Pulsar clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_clkgate_ctrl_mcu_cba_ecc_aggr_nogate_proxy(
        &mut self,
    ) -> McuClkgateCtrlMcuCbaEccAggrNogateProxyW<Cfg0McuClkgateCtrlProxySpec> {
        McuClkgateCtrlMcuCbaEccAggrNogateProxyW::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
MCU domain M4FSS0 clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_clkgate_ctrl_mcu_m4fss_nogate_proxy(
        &mut self,
    ) -> McuClkgateCtrlMcuM4fssNogateProxyW<Cfg0McuClkgateCtrlProxySpec> {
        McuClkgateCtrlMcuM4fssNogateProxyW::new(self, 8)
    }
}
#[doc = "CFG0_MCU_CLKGATE_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_clkgate_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_clkgate_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuClkgateCtrlProxySpec;
impl crate::RegisterSpec for Cfg0McuClkgateCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_clkgate_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0McuClkgateCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_clkgate_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0McuClkgateCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_CLKGATE_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0McuClkgateCtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
