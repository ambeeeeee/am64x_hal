#[doc = "Register `CFG0_POK_VDDSHV_MCU_1P8_UV_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0PokVddshvMcu1p8UvCtrlProxySpec>;
#[doc = "Register `CFG0_POK_VDDSHV_MCU_1P8_UV_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0PokVddshvMcu1p8UvCtrlProxySpec>;
#[doc = "Field `POK_VDDSHV_MCU_1P8_UV_CTRL_POK_TRIM_PROXY` reader - 6:0\\]
POK Trim Bits"]
pub type PokVddshvMcu1p8UvCtrlPokTrimProxyR = crate::FieldReader;
#[doc = "Field `POK_VDDSHV_MCU_1P8_UV_CTRL_POK_TRIM_PROXY` writer - 6:0\\]
POK Trim Bits"]
pub type PokVddshvMcu1p8UvCtrlPokTrimProxyW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `POK_VDDSHV_MCU_1P8_UV_CTRL_OVER_VOLT_DET_PROXY` reader - 7:7\\]
Over / under voltage detection mode"]
pub type PokVddshvMcu1p8UvCtrlOverVoltDetProxyR = crate::BitReader;
#[doc = "Field `POK_VDDSHV_MCU_1P8_UV_CTRL_OVER_VOLT_DET_PROXY` writer - 7:7\\]
Over / under voltage detection mode"]
pub type PokVddshvMcu1p8UvCtrlOverVoltDetProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POK_VDDSHV_MCU_1P8_UV_CTRL_HYST_EN_PROXY` reader - 31:31\\]
Active POK hysteresis"]
pub type PokVddshvMcu1p8UvCtrlHystEnProxyR = crate::BitReader;
#[doc = "Field `POK_VDDSHV_MCU_1P8_UV_CTRL_HYST_EN_PROXY` writer - 31:31\\]
Active POK hysteresis"]
pub type PokVddshvMcu1p8UvCtrlHystEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
POK Trim Bits"]
    #[inline(always)]
    pub fn pok_vddshv_mcu_1p8_uv_ctrl_pok_trim_proxy(&self) -> PokVddshvMcu1p8UvCtrlPokTrimProxyR {
        PokVddshvMcu1p8UvCtrlPokTrimProxyR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Over / under voltage detection mode"]
    #[inline(always)]
    pub fn pok_vddshv_mcu_1p8_uv_ctrl_over_volt_det_proxy(
        &self,
    ) -> PokVddshvMcu1p8UvCtrlOverVoltDetProxyR {
        PokVddshvMcu1p8UvCtrlOverVoltDetProxyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Active POK hysteresis"]
    #[inline(always)]
    pub fn pok_vddshv_mcu_1p8_uv_ctrl_hyst_en_proxy(&self) -> PokVddshvMcu1p8UvCtrlHystEnProxyR {
        PokVddshvMcu1p8UvCtrlHystEnProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
POK Trim Bits"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vddshv_mcu_1p8_uv_ctrl_pok_trim_proxy(
        &mut self,
    ) -> PokVddshvMcu1p8UvCtrlPokTrimProxyW<Cfg0PokVddshvMcu1p8UvCtrlProxySpec> {
        PokVddshvMcu1p8UvCtrlPokTrimProxyW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Over / under voltage detection mode"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vddshv_mcu_1p8_uv_ctrl_over_volt_det_proxy(
        &mut self,
    ) -> PokVddshvMcu1p8UvCtrlOverVoltDetProxyW<Cfg0PokVddshvMcu1p8UvCtrlProxySpec> {
        PokVddshvMcu1p8UvCtrlOverVoltDetProxyW::new(self, 7)
    }
    #[doc = "Bit 31 - 31:31\\]
Active POK hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vddshv_mcu_1p8_uv_ctrl_hyst_en_proxy(
        &mut self,
    ) -> PokVddshvMcu1p8UvCtrlHystEnProxyW<Cfg0PokVddshvMcu1p8UvCtrlProxySpec> {
        PokVddshvMcu1p8UvCtrlHystEnProxyW::new(self, 31)
    }
}
#[doc = "CFG0_POK_VDDSHV_MCU_1P8_UV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_mcu_1p8_uv_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_mcu_1p8_uv_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PokVddshvMcu1p8UvCtrlProxySpec;
impl crate::RegisterSpec for Cfg0PokVddshvMcu1p8UvCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_pok_vddshv_mcu_1p8_uv_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0PokVddshvMcu1p8UvCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_pok_vddshv_mcu_1p8_uv_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0PokVddshvMcu1p8UvCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_POK_VDDSHV_MCU_1P8_UV_CTRL_PROXY to value 0x8000_0000"]
impl crate::Resettable for Cfg0PokVddshvMcu1p8UvCtrlProxySpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
