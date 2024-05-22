#[doc = "Register `CFG0_POK_VDDR_CORE_UV_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0PokVddrCoreUvCtrlProxySpec>;
#[doc = "Register `CFG0_POK_VDDR_CORE_UV_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0PokVddrCoreUvCtrlProxySpec>;
#[doc = "Field `POK_VDDR_CORE_UV_CTRL_POK_TRIM_PROXY` reader - 6:0\\]
POK Trim Bits"]
pub type PokVddrCoreUvCtrlPokTrimProxyR = crate::FieldReader;
#[doc = "Field `POK_VDDR_CORE_UV_CTRL_POK_TRIM_PROXY` writer - 6:0\\]
POK Trim Bits"]
pub type PokVddrCoreUvCtrlPokTrimProxyW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `POK_VDDR_CORE_UV_CTRL_OVER_VOLT_DET_PROXY` reader - 7:7\\]
Over / under voltage detection mode"]
pub type PokVddrCoreUvCtrlOverVoltDetProxyR = crate::BitReader;
#[doc = "Field `POK_VDDR_CORE_UV_CTRL_OVER_VOLT_DET_PROXY` writer - 7:7\\]
Over / under voltage detection mode"]
pub type PokVddrCoreUvCtrlOverVoltDetProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POK_VDDR_CORE_UV_CTRL_HYST_EN_PROXY` reader - 31:31\\]
Active POK hysteresis"]
pub type PokVddrCoreUvCtrlHystEnProxyR = crate::BitReader;
#[doc = "Field `POK_VDDR_CORE_UV_CTRL_HYST_EN_PROXY` writer - 31:31\\]
Active POK hysteresis"]
pub type PokVddrCoreUvCtrlHystEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
POK Trim Bits"]
    #[inline(always)]
    pub fn pok_vddr_core_uv_ctrl_pok_trim_proxy(&self) -> PokVddrCoreUvCtrlPokTrimProxyR {
        PokVddrCoreUvCtrlPokTrimProxyR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Over / under voltage detection mode"]
    #[inline(always)]
    pub fn pok_vddr_core_uv_ctrl_over_volt_det_proxy(&self) -> PokVddrCoreUvCtrlOverVoltDetProxyR {
        PokVddrCoreUvCtrlOverVoltDetProxyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Active POK hysteresis"]
    #[inline(always)]
    pub fn pok_vddr_core_uv_ctrl_hyst_en_proxy(&self) -> PokVddrCoreUvCtrlHystEnProxyR {
        PokVddrCoreUvCtrlHystEnProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
POK Trim Bits"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vddr_core_uv_ctrl_pok_trim_proxy(
        &mut self,
    ) -> PokVddrCoreUvCtrlPokTrimProxyW<Cfg0PokVddrCoreUvCtrlProxySpec> {
        PokVddrCoreUvCtrlPokTrimProxyW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Over / under voltage detection mode"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vddr_core_uv_ctrl_over_volt_det_proxy(
        &mut self,
    ) -> PokVddrCoreUvCtrlOverVoltDetProxyW<Cfg0PokVddrCoreUvCtrlProxySpec> {
        PokVddrCoreUvCtrlOverVoltDetProxyW::new(self, 7)
    }
    #[doc = "Bit 31 - 31:31\\]
Active POK hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vddr_core_uv_ctrl_hyst_en_proxy(
        &mut self,
    ) -> PokVddrCoreUvCtrlHystEnProxyW<Cfg0PokVddrCoreUvCtrlProxySpec> {
        PokVddrCoreUvCtrlHystEnProxyW::new(self, 31)
    }
}
#[doc = "CFG0_POK_VDDR_CORE_UV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddr_core_uv_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddr_core_uv_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PokVddrCoreUvCtrlProxySpec;
impl crate::RegisterSpec for Cfg0PokVddrCoreUvCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_pok_vddr_core_uv_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0PokVddrCoreUvCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_pok_vddr_core_uv_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0PokVddrCoreUvCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_POK_VDDR_CORE_UV_CTRL_PROXY to value 0x8000_0000"]
impl crate::Resettable for Cfg0PokVddrCoreUvCtrlProxySpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
