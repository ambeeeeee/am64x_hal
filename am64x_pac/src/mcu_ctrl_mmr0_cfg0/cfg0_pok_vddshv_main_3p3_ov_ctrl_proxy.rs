#[doc = "Register `CFG0_POK_VDDSHV_MAIN_3P3_OV_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0PokVddshvMain3p3OvCtrlProxySpec>;
#[doc = "Register `CFG0_POK_VDDSHV_MAIN_3P3_OV_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0PokVddshvMain3p3OvCtrlProxySpec>;
#[doc = "Field `POK_VDDSHV_MAIN_3P3_OV_CTRL_POK_TRIM_PROXY` reader - 6:0\\]
POK Trim Bits"]
pub type PokVddshvMain3p3OvCtrlPokTrimProxyR = crate::FieldReader;
#[doc = "Field `POK_VDDSHV_MAIN_3P3_OV_CTRL_POK_TRIM_PROXY` writer - 6:0\\]
POK Trim Bits"]
pub type PokVddshvMain3p3OvCtrlPokTrimProxyW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `POK_VDDSHV_MAIN_3P3_OV_CTRL_OVER_VOLT_DET_PROXY` reader - 7:7\\]
Over / under voltage detection mode"]
pub type PokVddshvMain3p3OvCtrlOverVoltDetProxyR = crate::BitReader;
#[doc = "Field `POK_VDDSHV_MAIN_3P3_OV_CTRL_OVER_VOLT_DET_PROXY` writer - 7:7\\]
Over / under voltage detection mode"]
pub type PokVddshvMain3p3OvCtrlOverVoltDetProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POK_VDDSHV_MAIN_3P3_OV_CTRL_HYST_EN_PROXY` reader - 31:31\\]
Active POK hysteresis"]
pub type PokVddshvMain3p3OvCtrlHystEnProxyR = crate::BitReader;
#[doc = "Field `POK_VDDSHV_MAIN_3P3_OV_CTRL_HYST_EN_PROXY` writer - 31:31\\]
Active POK hysteresis"]
pub type PokVddshvMain3p3OvCtrlHystEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
POK Trim Bits"]
    #[inline(always)]
    pub fn pok_vddshv_main_3p3_ov_ctrl_pok_trim_proxy(
        &self,
    ) -> PokVddshvMain3p3OvCtrlPokTrimProxyR {
        PokVddshvMain3p3OvCtrlPokTrimProxyR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Over / under voltage detection mode"]
    #[inline(always)]
    pub fn pok_vddshv_main_3p3_ov_ctrl_over_volt_det_proxy(
        &self,
    ) -> PokVddshvMain3p3OvCtrlOverVoltDetProxyR {
        PokVddshvMain3p3OvCtrlOverVoltDetProxyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Active POK hysteresis"]
    #[inline(always)]
    pub fn pok_vddshv_main_3p3_ov_ctrl_hyst_en_proxy(&self) -> PokVddshvMain3p3OvCtrlHystEnProxyR {
        PokVddshvMain3p3OvCtrlHystEnProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
POK Trim Bits"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vddshv_main_3p3_ov_ctrl_pok_trim_proxy(
        &mut self,
    ) -> PokVddshvMain3p3OvCtrlPokTrimProxyW<Cfg0PokVddshvMain3p3OvCtrlProxySpec> {
        PokVddshvMain3p3OvCtrlPokTrimProxyW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Over / under voltage detection mode"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vddshv_main_3p3_ov_ctrl_over_volt_det_proxy(
        &mut self,
    ) -> PokVddshvMain3p3OvCtrlOverVoltDetProxyW<Cfg0PokVddshvMain3p3OvCtrlProxySpec> {
        PokVddshvMain3p3OvCtrlOverVoltDetProxyW::new(self, 7)
    }
    #[doc = "Bit 31 - 31:31\\]
Active POK hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vddshv_main_3p3_ov_ctrl_hyst_en_proxy(
        &mut self,
    ) -> PokVddshvMain3p3OvCtrlHystEnProxyW<Cfg0PokVddshvMain3p3OvCtrlProxySpec> {
        PokVddshvMain3p3OvCtrlHystEnProxyW::new(self, 31)
    }
}
#[doc = "CFG0_POK_VDDSHV_MAIN_3P3_OV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_main_3p3_ov_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_main_3p3_ov_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PokVddshvMain3p3OvCtrlProxySpec;
impl crate::RegisterSpec for Cfg0PokVddshvMain3p3OvCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_pok_vddshv_main_3p3_ov_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0PokVddshvMain3p3OvCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_pok_vddshv_main_3p3_ov_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0PokVddshvMain3p3OvCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_POK_VDDSHV_MAIN_3P3_OV_CTRL_PROXY to value 0x8000_0000"]
impl crate::Resettable for Cfg0PokVddshvMain3p3OvCtrlProxySpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
