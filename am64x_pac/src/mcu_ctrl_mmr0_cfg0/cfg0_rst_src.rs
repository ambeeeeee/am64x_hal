#[doc = "Register `CFG0_RST_SRC` reader"]
pub type R = crate::R<Cfg0RstSrcSpec>;
#[doc = "Register `CFG0_RST_SRC` writer"]
pub type W = crate::W<Cfg0RstSrcSpec>;
#[doc = "Field `RST_SRC_MCU_RESET_PIN` reader - 0:0\\]
Rest Caused by MCU Reset Pin"]
pub type RstSrcMcuResetPinR = crate::BitReader;
#[doc = "Field `RST_SRC_MCU_RESET_PIN` writer - 0:0\\]
Rest Caused by MCU Reset Pin"]
pub type RstSrcMcuResetPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_MAIN_RESET_REQ` reader - 2:2\\]
Main Reset Pin"]
pub type RstSrcMainResetReqR = crate::BitReader;
#[doc = "Field `RST_SRC_MAIN_RESET_REQ` writer - 2:2\\]
Main Reset Pin"]
pub type RstSrcMainResetReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_THERMAL_RST` reader - 4:4\\]
Thermal Reset"]
pub type RstSrcThermalRstR = crate::BitReader;
#[doc = "Field `RST_SRC_THERMAL_RST` writer - 4:4\\]
Thermal Reset"]
pub type RstSrcThermalRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_DEBUG_RST` reader - 8:8\\]
Debug Subsystem Initiated Reset"]
pub type RstSrcDebugRstR = crate::BitReader;
#[doc = "Field `RST_SRC_DEBUG_RST` writer - 8:8\\]
Debug Subsystem Initiated Reset"]
pub type RstSrcDebugRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_COLD_OUT_RST` reader - 12:12\\]
DMSC Cold Reset"]
pub type RstSrcColdOutRstR = crate::BitReader;
#[doc = "Field `RST_SRC_COLD_OUT_RST` writer - 12:12\\]
DMSC Cold Reset"]
pub type RstSrcColdOutRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_WARM_OUT_RST` reader - 13:13\\]
DMSC Warm Reset"]
pub type RstSrcWarmOutRstR = crate::BitReader;
#[doc = "Field `RST_SRC_WARM_OUT_RST` writer - 13:13\\]
DMSC Warm Reset"]
pub type RstSrcWarmOutRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_SW_MCU_WARMRST` reader - 16:16\\]
Software Warm Reset"]
pub type RstSrcSwMcuWarmrstR = crate::BitReader;
#[doc = "Field `RST_SRC_SW_MCU_WARMRST` writer - 16:16\\]
Software Warm Reset"]
pub type RstSrcSwMcuWarmrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_SW_MAIN_WARMRST_FROM_MCU` reader - 20:20\\]
Software Main Warm Reset From MCU CTRL MMR"]
pub type RstSrcSwMainWarmrstFromMcuR = crate::BitReader;
#[doc = "Field `RST_SRC_SW_MAIN_WARMRST_FROM_MCU` writer - 20:20\\]
Software Main Warm Reset From MCU CTRL MMR"]
pub type RstSrcSwMainWarmrstFromMcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_SW_MAIN_WARMRST_FROM_MAIN` reader - 21:21\\]
Software Main Warm Reset from MAIN CTRL MMR"]
pub type RstSrcSwMainWarmrstFromMainR = crate::BitReader;
#[doc = "Field `RST_SRC_SW_MAIN_WARMRST_FROM_MAIN` writer - 21:21\\]
Software Main Warm Reset from MAIN CTRL MMR"]
pub type RstSrcSwMainWarmrstFromMainW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_SW_MAIN_POR_FROM_MCU` reader - 24:24\\]
Software Main Power On Reset From MCU CTRL MMR"]
pub type RstSrcSwMainPorFromMcuR = crate::BitReader;
#[doc = "Field `RST_SRC_SW_MAIN_POR_FROM_MCU` writer - 24:24\\]
Software Main Power On Reset From MCU CTRL MMR"]
pub type RstSrcSwMainPorFromMcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_SW_MAIN_POR_FROM_MAIN` reader - 25:25\\]
Software Main Power On Reset From MAIN CTRL MMR"]
pub type RstSrcSwMainPorFromMainR = crate::BitReader;
#[doc = "Field `RST_SRC_SW_MAIN_POR_FROM_MAIN` writer - 25:25\\]
Software Main Power On Reset From MAIN CTRL MMR"]
pub type RstSrcSwMainPorFromMainW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_MAIN_ESM_ERROR` reader - 30:30\\]
Reset Caused by Main ESM Error"]
pub type RstSrcMainEsmErrorR = crate::BitReader;
#[doc = "Field `RST_SRC_MAIN_ESM_ERROR` writer - 30:30\\]
Reset Caused by Main ESM Error"]
pub type RstSrcMainEsmErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SRC_SAFETY_ERROR` reader - 31:31\\]
Reset Caused by MCU ESM Error"]
pub type RstSrcSafetyErrorR = crate::BitReader;
#[doc = "Field `RST_SRC_SAFETY_ERROR` writer - 31:31\\]
Reset Caused by MCU ESM Error"]
pub type RstSrcSafetyErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Rest Caused by MCU Reset Pin"]
    #[inline(always)]
    pub fn rst_src_mcu_reset_pin(&self) -> RstSrcMcuResetPinR {
        RstSrcMcuResetPinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Main Reset Pin"]
    #[inline(always)]
    pub fn rst_src_main_reset_req(&self) -> RstSrcMainResetReqR {
        RstSrcMainResetReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Thermal Reset"]
    #[inline(always)]
    pub fn rst_src_thermal_rst(&self) -> RstSrcThermalRstR {
        RstSrcThermalRstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Debug Subsystem Initiated Reset"]
    #[inline(always)]
    pub fn rst_src_debug_rst(&self) -> RstSrcDebugRstR {
        RstSrcDebugRstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
DMSC Cold Reset"]
    #[inline(always)]
    pub fn rst_src_cold_out_rst(&self) -> RstSrcColdOutRstR {
        RstSrcColdOutRstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
DMSC Warm Reset"]
    #[inline(always)]
    pub fn rst_src_warm_out_rst(&self) -> RstSrcWarmOutRstR {
        RstSrcWarmOutRstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Software Warm Reset"]
    #[inline(always)]
    pub fn rst_src_sw_mcu_warmrst(&self) -> RstSrcSwMcuWarmrstR {
        RstSrcSwMcuWarmrstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Software Main Warm Reset From MCU CTRL MMR"]
    #[inline(always)]
    pub fn rst_src_sw_main_warmrst_from_mcu(&self) -> RstSrcSwMainWarmrstFromMcuR {
        RstSrcSwMainWarmrstFromMcuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Software Main Warm Reset from MAIN CTRL MMR"]
    #[inline(always)]
    pub fn rst_src_sw_main_warmrst_from_main(&self) -> RstSrcSwMainWarmrstFromMainR {
        RstSrcSwMainWarmrstFromMainR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Software Main Power On Reset From MCU CTRL MMR"]
    #[inline(always)]
    pub fn rst_src_sw_main_por_from_mcu(&self) -> RstSrcSwMainPorFromMcuR {
        RstSrcSwMainPorFromMcuR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Software Main Power On Reset From MAIN CTRL MMR"]
    #[inline(always)]
    pub fn rst_src_sw_main_por_from_main(&self) -> RstSrcSwMainPorFromMainR {
        RstSrcSwMainPorFromMainR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Reset Caused by Main ESM Error"]
    #[inline(always)]
    pub fn rst_src_main_esm_error(&self) -> RstSrcMainEsmErrorR {
        RstSrcMainEsmErrorR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Reset Caused by MCU ESM Error"]
    #[inline(always)]
    pub fn rst_src_safety_error(&self) -> RstSrcSafetyErrorR {
        RstSrcSafetyErrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Rest Caused by MCU Reset Pin"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_mcu_reset_pin(&mut self) -> RstSrcMcuResetPinW<Cfg0RstSrcSpec> {
        RstSrcMcuResetPinW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Main Reset Pin"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_main_reset_req(&mut self) -> RstSrcMainResetReqW<Cfg0RstSrcSpec> {
        RstSrcMainResetReqW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
Thermal Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_thermal_rst(&mut self) -> RstSrcThermalRstW<Cfg0RstSrcSpec> {
        RstSrcThermalRstW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Debug Subsystem Initiated Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_debug_rst(&mut self) -> RstSrcDebugRstW<Cfg0RstSrcSpec> {
        RstSrcDebugRstW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
DMSC Cold Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_cold_out_rst(&mut self) -> RstSrcColdOutRstW<Cfg0RstSrcSpec> {
        RstSrcColdOutRstW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
DMSC Warm Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_warm_out_rst(&mut self) -> RstSrcWarmOutRstW<Cfg0RstSrcSpec> {
        RstSrcWarmOutRstW::new(self, 13)
    }
    #[doc = "Bit 16 - 16:16\\]
Software Warm Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_sw_mcu_warmrst(&mut self) -> RstSrcSwMcuWarmrstW<Cfg0RstSrcSpec> {
        RstSrcSwMcuWarmrstW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Software Main Warm Reset From MCU CTRL MMR"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_sw_main_warmrst_from_mcu(
        &mut self,
    ) -> RstSrcSwMainWarmrstFromMcuW<Cfg0RstSrcSpec> {
        RstSrcSwMainWarmrstFromMcuW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Software Main Warm Reset from MAIN CTRL MMR"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_sw_main_warmrst_from_main(
        &mut self,
    ) -> RstSrcSwMainWarmrstFromMainW<Cfg0RstSrcSpec> {
        RstSrcSwMainWarmrstFromMainW::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
Software Main Power On Reset From MCU CTRL MMR"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_sw_main_por_from_mcu(&mut self) -> RstSrcSwMainPorFromMcuW<Cfg0RstSrcSpec> {
        RstSrcSwMainPorFromMcuW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Software Main Power On Reset From MAIN CTRL MMR"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_sw_main_por_from_main(&mut self) -> RstSrcSwMainPorFromMainW<Cfg0RstSrcSpec> {
        RstSrcSwMainPorFromMainW::new(self, 25)
    }
    #[doc = "Bit 30 - 30:30\\]
Reset Caused by Main ESM Error"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_main_esm_error(&mut self) -> RstSrcMainEsmErrorW<Cfg0RstSrcSpec> {
        RstSrcMainEsmErrorW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Reset Caused by MCU ESM Error"]
    #[inline(always)]
    #[must_use]
    pub fn rst_src_safety_error(&mut self) -> RstSrcSafetyErrorW<Cfg0RstSrcSpec> {
        RstSrcSafetyErrorW::new(self, 31)
    }
}
#[doc = "CFG0_RST_SRC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_src::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_src::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0RstSrcSpec;
impl crate::RegisterSpec for Cfg0RstSrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_rst_src::R`](R) reader structure"]
impl crate::Readable for Cfg0RstSrcSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_rst_src::W`](W) writer structure"]
impl crate::Writable for Cfg0RstSrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_RST_SRC to value 0"]
impl crate::Resettable for Cfg0RstSrcSpec {
    const RESET_VALUE: u32 = 0;
}
