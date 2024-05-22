#[doc = "Register `CFG0_RST_SRC_PROXY` reader"]
pub type R = crate::R<Cfg0RstSrcProxySpec>;
#[doc = "Register `CFG0_RST_SRC_PROXY` writer"]
pub type W = crate::W<Cfg0RstSrcProxySpec>;
#[doc = "Field `RST_SRC_MCU_RESET_PIN_PROXY` reader - 0:0\\]
Rest Caused by MCU Reset Pin"]
pub type RstSrcMcuResetPinProxyR = crate::BitReader;
#[doc = "Field `RST_SRC_MCU_RESET_PIN_PROXY` writer - 0:0\\]
Rest Caused by MCU Reset Pin"]
pub type RstSrcMcuResetPinProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_MAIN_RESET_REQ_PROXY` reader - 2:2\\]
Main Reset Pin"]
pub type RstSrcMainResetReqProxyR = crate::BitReader;
#[doc = "Field `RST_SRC_MAIN_RESET_REQ_PROXY` writer - 2:2\\]
Main Reset Pin"]
pub type RstSrcMainResetReqProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_THERMAL_RST_PROXY` reader - 4:4\\]
Thermal Reset"]
pub type RstSrcThermalRstProxyR = crate::BitReader;
#[doc = "Field `RST_SRC_THERMAL_RST_PROXY` writer - 4:4\\]
Thermal Reset"]
pub type RstSrcThermalRstProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_DEBUG_RST_PROXY` reader - 8:8\\]
Debug Subsystem Initiated Reset"]
pub type RstSrcDebugRstProxyR = crate::BitReader;
#[doc = "Field `RST_SRC_DEBUG_RST_PROXY` writer - 8:8\\]
Debug Subsystem Initiated Reset"]
pub type RstSrcDebugRstProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_COLD_OUT_RST_PROXY` reader - 12:12\\]
DMSC Cold Reset"]
pub type RstSrcColdOutRstProxyR = crate::BitReader;
#[doc = "Field `RST_SRC_COLD_OUT_RST_PROXY` writer - 12:12\\]
DMSC Cold Reset"]
pub type RstSrcColdOutRstProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_WARM_OUT_RST_PROXY` reader - 13:13\\]
DMSC Warm Reset"]
pub type RstSrcWarmOutRstProxyR = crate::BitReader;
#[doc = "Field `RST_SRC_WARM_OUT_RST_PROXY` writer - 13:13\\]
DMSC Warm Reset"]
pub type RstSrcWarmOutRstProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_SW_MCU_WARMRST_PROXY` reader - 16:16\\]
Software Warm Reset"]
pub type RstSrcSwMcuWarmrstProxyR = crate::BitReader;
#[doc = "Field `RST_SRC_SW_MCU_WARMRST_PROXY` writer - 16:16\\]
Software Warm Reset"]
pub type RstSrcSwMcuWarmrstProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_SW_MAIN_WARMRST_FROM_MCU_PROXY` reader - 20:20\\]
Software Main Warm Reset From MCU CTRL MMR"]
pub type RstSrcSwMainWarmrstFromMcuProxyR = crate::BitReader;
#[doc = "Field `RST_SRC_SW_MAIN_WARMRST_FROM_MCU_PROXY` writer - 20:20\\]
Software Main Warm Reset From MCU CTRL MMR"]
pub type RstSrcSwMainWarmrstFromMcuProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_SW_MAIN_WARMRST_FROM_MAIN_PROXY` reader - 21:21\\]
Software Main Warm Reset from MAIN CTRL MMR"]
pub type RstSrcSwMainWarmrstFromMainProxyR = crate::BitReader;
#[doc = "Field `RST_SRC_SW_MAIN_WARMRST_FROM_MAIN_PROXY` writer - 21:21\\]
Software Main Warm Reset from MAIN CTRL MMR"]
pub type RstSrcSwMainWarmrstFromMainProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_SW_MAIN_POR_FROM_MCU_PROXY` reader - 24:24\\]
Software Main Power On Reset From MCU CTRL MMR"]
pub type RstSrcSwMainPorFromMcuProxyR = crate::BitReader;
#[doc = "Field `RST_SRC_SW_MAIN_POR_FROM_MCU_PROXY` writer - 24:24\\]
Software Main Power On Reset From MCU CTRL MMR"]
pub type RstSrcSwMainPorFromMcuProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_SW_MAIN_POR_FROM_MAIN_PROXY` reader - 25:25\\]
Software Main Power On Reset From MAIN CTRL MMR"]
pub type RstSrcSwMainPorFromMainProxyR = crate::BitReader;
#[doc = "Field `RST_SRC_SW_MAIN_POR_FROM_MAIN_PROXY` writer - 25:25\\]
Software Main Power On Reset From MAIN CTRL MMR"]
pub type RstSrcSwMainPorFromMainProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_MAIN_ESM_ERROR_PROXY` reader - 30:30\\]
Reset Caused by Main ESM Error"]
pub type RstSrcMainEsmErrorProxyR = crate::BitReader;
#[doc = "Field `RST_SRC_MAIN_ESM_ERROR_PROXY` writer - 30:30\\]
Reset Caused by Main ESM Error"]
pub type RstSrcMainEsmErrorProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_SAFETY_ERROR_PROXY` reader - 31:31\\]
Reset Caused by MCU ESM Error"]
pub type RstSrcSafetyErrorProxyR = crate::BitReader;
#[doc = "Field `RST_SRC_SAFETY_ERROR_PROXY` writer - 31:31\\]
Reset Caused by MCU ESM Error"]
pub type RstSrcSafetyErrorProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Rest Caused by MCU Reset Pin"]
    #[inline(always)]
    pub fn rst_src_mcu_reset_pin_proxy(&self) -> RstSrcMcuResetPinProxyR {
        RstSrcMcuResetPinProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Main Reset Pin"]
    #[inline(always)]
    pub fn rst_src_main_reset_req_proxy(&self) -> RstSrcMainResetReqProxyR {
        RstSrcMainResetReqProxyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Thermal Reset"]
    #[inline(always)]
    pub fn rst_src_thermal_rst_proxy(&self) -> RstSrcThermalRstProxyR {
        RstSrcThermalRstProxyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Debug Subsystem Initiated Reset"]
    #[inline(always)]
    pub fn rst_src_debug_rst_proxy(&self) -> RstSrcDebugRstProxyR {
        RstSrcDebugRstProxyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
DMSC Cold Reset"]
    #[inline(always)]
    pub fn rst_src_cold_out_rst_proxy(&self) -> RstSrcColdOutRstProxyR {
        RstSrcColdOutRstProxyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
DMSC Warm Reset"]
    #[inline(always)]
    pub fn rst_src_warm_out_rst_proxy(&self) -> RstSrcWarmOutRstProxyR {
        RstSrcWarmOutRstProxyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Software Warm Reset"]
    #[inline(always)]
    pub fn rst_src_sw_mcu_warmrst_proxy(&self) -> RstSrcSwMcuWarmrstProxyR {
        RstSrcSwMcuWarmrstProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Software Main Warm Reset From MCU CTRL MMR"]
    #[inline(always)]
    pub fn rst_src_sw_main_warmrst_from_mcu_proxy(&self) -> RstSrcSwMainWarmrstFromMcuProxyR {
        RstSrcSwMainWarmrstFromMcuProxyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Software Main Warm Reset from MAIN CTRL MMR"]
    #[inline(always)]
    pub fn rst_src_sw_main_warmrst_from_main_proxy(&self) -> RstSrcSwMainWarmrstFromMainProxyR {
        RstSrcSwMainWarmrstFromMainProxyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Software Main Power On Reset From MCU CTRL MMR"]
    #[inline(always)]
    pub fn rst_src_sw_main_por_from_mcu_proxy(&self) -> RstSrcSwMainPorFromMcuProxyR {
        RstSrcSwMainPorFromMcuProxyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Software Main Power On Reset From MAIN CTRL MMR"]
    #[inline(always)]
    pub fn rst_src_sw_main_por_from_main_proxy(&self) -> RstSrcSwMainPorFromMainProxyR {
        RstSrcSwMainPorFromMainProxyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Reset Caused by Main ESM Error"]
    #[inline(always)]
    pub fn rst_src_main_esm_error_proxy(&self) -> RstSrcMainEsmErrorProxyR {
        RstSrcMainEsmErrorProxyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Reset Caused by MCU ESM Error"]
    #[inline(always)]
    pub fn rst_src_safety_error_proxy(&self) -> RstSrcSafetyErrorProxyR {
        RstSrcSafetyErrorProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Rest Caused by MCU Reset Pin"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_mcu_reset_pin_proxy(&mut self) -> RstSrcMcuResetPinProxyW<Cfg0RstSrcProxySpec> {
        RstSrcMcuResetPinProxyW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Main Reset Pin"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_main_reset_req_proxy(
        &mut self,
    ) -> RstSrcMainResetReqProxyW<Cfg0RstSrcProxySpec> {
        RstSrcMainResetReqProxyW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
Thermal Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_thermal_rst_proxy(&mut self) -> RstSrcThermalRstProxyW<Cfg0RstSrcProxySpec> {
        RstSrcThermalRstProxyW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Debug Subsystem Initiated Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_debug_rst_proxy(&mut self) -> RstSrcDebugRstProxyW<Cfg0RstSrcProxySpec> {
        RstSrcDebugRstProxyW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
DMSC Cold Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_cold_out_rst_proxy(&mut self) -> RstSrcColdOutRstProxyW<Cfg0RstSrcProxySpec> {
        RstSrcColdOutRstProxyW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
DMSC Warm Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_warm_out_rst_proxy(&mut self) -> RstSrcWarmOutRstProxyW<Cfg0RstSrcProxySpec> {
        RstSrcWarmOutRstProxyW::new(self, 13)
    }
    #[doc = "Bit 16 - 16:16\\]
Software Warm Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_sw_mcu_warmrst_proxy(
        &mut self,
    ) -> RstSrcSwMcuWarmrstProxyW<Cfg0RstSrcProxySpec> {
        RstSrcSwMcuWarmrstProxyW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Software Main Warm Reset From MCU CTRL MMR"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_sw_main_warmrst_from_mcu_proxy(
        &mut self,
    ) -> RstSrcSwMainWarmrstFromMcuProxyW<Cfg0RstSrcProxySpec> {
        RstSrcSwMainWarmrstFromMcuProxyW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Software Main Warm Reset from MAIN CTRL MMR"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_sw_main_warmrst_from_main_proxy(
        &mut self,
    ) -> RstSrcSwMainWarmrstFromMainProxyW<Cfg0RstSrcProxySpec> {
        RstSrcSwMainWarmrstFromMainProxyW::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
Software Main Power On Reset From MCU CTRL MMR"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_sw_main_por_from_mcu_proxy(
        &mut self,
    ) -> RstSrcSwMainPorFromMcuProxyW<Cfg0RstSrcProxySpec> {
        RstSrcSwMainPorFromMcuProxyW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Software Main Power On Reset From MAIN CTRL MMR"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_sw_main_por_from_main_proxy(
        &mut self,
    ) -> RstSrcSwMainPorFromMainProxyW<Cfg0RstSrcProxySpec> {
        RstSrcSwMainPorFromMainProxyW::new(self, 25)
    }
    #[doc = "Bit 30 - 30:30\\]
Reset Caused by Main ESM Error"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_main_esm_error_proxy(
        &mut self,
    ) -> RstSrcMainEsmErrorProxyW<Cfg0RstSrcProxySpec> {
        RstSrcMainEsmErrorProxyW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Reset Caused by MCU ESM Error"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_safety_error_proxy(&mut self) -> RstSrcSafetyErrorProxyW<Cfg0RstSrcProxySpec> {
        RstSrcSafetyErrorProxyW::new(self, 31)
    }
}
#[doc = "CFG0_RST_SRC_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_src_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_src_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0RstSrcProxySpec;
impl crate::RegisterSpec for Cfg0RstSrcProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_rst_src_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0RstSrcProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_rst_src_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0RstSrcProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_RST_SRC_PROXY to value 0"]
impl crate::Resettable for Cfg0RstSrcProxySpec {
    const RESET_VALUE: u32 = 0;
}
