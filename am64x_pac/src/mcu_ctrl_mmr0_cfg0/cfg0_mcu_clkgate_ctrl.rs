#[doc = "Register `CFG0_MCU_CLKGATE_CTRL` reader"]
pub type R = crate::R<Cfg0McuClkgateCtrlSpec>;
#[doc = "Register `CFG0_MCU_CLKGATE_CTRL` writer"]
pub type W = crate::W<Cfg0McuClkgateCtrlSpec>;
#[doc = "Field `MCU_CLKGATE_CTRL_MCU_CBA_NOGATE` reader - 0:0\\]
MCU domain Data bus (mcu_cbass) clock gate deactivate."]
pub type McuClkgateCtrlMcuCbaNogateR = crate::BitReader;
#[doc = "Field `MCU_CLKGATE_CTRL_MCU_CBA_NOGATE` writer - 0:0\\]
MCU domain Data bus (mcu_cbass) clock gate deactivate."]
pub type McuClkgateCtrlMcuCbaNogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_CLKGATE_CTRL_MCU_CBA_ECC_AGGR_NOGATE` reader - 1:1\\]
MCU domain Pulsar clock gate deactivate."]
pub type McuClkgateCtrlMcuCbaEccAggrNogateR = crate::BitReader;
#[doc = "Field `MCU_CLKGATE_CTRL_MCU_CBA_ECC_AGGR_NOGATE` writer - 1:1\\]
MCU domain Pulsar clock gate deactivate."]
pub type McuClkgateCtrlMcuCbaEccAggrNogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_CLKGATE_CTRL_MCU_M4FSS_NOGATE` reader - 8:8\\]
MCU domain M4FSS0 clock gate deactivate."]
pub type McuClkgateCtrlMcuM4fssNogateR = crate::BitReader;
#[doc = "Field `MCU_CLKGATE_CTRL_MCU_M4FSS_NOGATE` writer - 8:8\\]
MCU domain M4FSS0 clock gate deactivate."]
pub type McuClkgateCtrlMcuM4fssNogateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MCU domain Data bus (mcu_cbass) clock gate deactivate."]
    #[inline(always)]
    pub fn mcu_clkgate_ctrl_mcu_cba_nogate(&self) -> McuClkgateCtrlMcuCbaNogateR {
        McuClkgateCtrlMcuCbaNogateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MCU domain Pulsar clock gate deactivate."]
    #[inline(always)]
    pub fn mcu_clkgate_ctrl_mcu_cba_ecc_aggr_nogate(&self) -> McuClkgateCtrlMcuCbaEccAggrNogateR {
        McuClkgateCtrlMcuCbaEccAggrNogateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
MCU domain M4FSS0 clock gate deactivate."]
    #[inline(always)]
    pub fn mcu_clkgate_ctrl_mcu_m4fss_nogate(&self) -> McuClkgateCtrlMcuM4fssNogateR {
        McuClkgateCtrlMcuM4fssNogateR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MCU domain Data bus (mcu_cbass) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_clkgate_ctrl_mcu_cba_nogate(
        &mut self,
    ) -> McuClkgateCtrlMcuCbaNogateW<Cfg0McuClkgateCtrlSpec> {
        McuClkgateCtrlMcuCbaNogateW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MCU domain Pulsar clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_clkgate_ctrl_mcu_cba_ecc_aggr_nogate(
        &mut self,
    ) -> McuClkgateCtrlMcuCbaEccAggrNogateW<Cfg0McuClkgateCtrlSpec> {
        McuClkgateCtrlMcuCbaEccAggrNogateW::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
MCU domain M4FSS0 clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_clkgate_ctrl_mcu_m4fss_nogate(
        &mut self,
    ) -> McuClkgateCtrlMcuM4fssNogateW<Cfg0McuClkgateCtrlSpec> {
        McuClkgateCtrlMcuM4fssNogateW::new(self, 8)
    }
}
#[doc = "CFG0_MCU_CLKGATE_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_clkgate_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_clkgate_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuClkgateCtrlSpec;
impl crate::RegisterSpec for Cfg0McuClkgateCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_clkgate_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0McuClkgateCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_clkgate_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0McuClkgateCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_CLKGATE_CTRL to value 0"]
impl crate::Resettable for Cfg0McuClkgateCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
