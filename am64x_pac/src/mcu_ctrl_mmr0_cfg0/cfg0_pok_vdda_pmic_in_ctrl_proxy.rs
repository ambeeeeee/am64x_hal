#[doc = "Register `CFG0_POK_VDDA_PMIC_IN_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0PokVddaPmicInCtrlProxySpec>;
#[doc = "Register `CFG0_POK_VDDA_PMIC_IN_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0PokVddaPmicInCtrlProxySpec>;
#[doc = "Field `POK_VDDA_PMIC_IN_CTRL_OVER_VOLT_DET_PROXY` reader - 15:15\\]
Over / under voltage detection mode"]
pub type PokVddaPmicInCtrlOverVoltDetProxyR = crate::BitReader;
#[doc = "Field `POK_VDDA_PMIC_IN_CTRL_OVER_VOLT_DET_PROXY` writer - 15:15\\]
Over / under voltage detection mode"]
pub type PokVddaPmicInCtrlOverVoltDetProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POK_VDDA_PMIC_IN_CTRL_HYST_EN_PROXY` reader - 31:31\\]
Active POK hysteresis"]
pub type PokVddaPmicInCtrlHystEnProxyR = crate::BitReader;
#[doc = "Field `POK_VDDA_PMIC_IN_CTRL_HYST_EN_PROXY` writer - 31:31\\]
Active POK hysteresis"]
pub type PokVddaPmicInCtrlHystEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 15 - 15:15\\]
Over / under voltage detection mode"]
    #[inline(always)]
    pub fn pok_vdda_pmic_in_ctrl_over_volt_det_proxy(&self) -> PokVddaPmicInCtrlOverVoltDetProxyR {
        PokVddaPmicInCtrlOverVoltDetProxyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Active POK hysteresis"]
    #[inline(always)]
    pub fn pok_vdda_pmic_in_ctrl_hyst_en_proxy(&self) -> PokVddaPmicInCtrlHystEnProxyR {
        PokVddaPmicInCtrlHystEnProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - 15:15\\]
Over / under voltage detection mode"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vdda_pmic_in_ctrl_over_volt_det_proxy(
        &mut self,
    ) -> PokVddaPmicInCtrlOverVoltDetProxyW<Cfg0PokVddaPmicInCtrlProxySpec> {
        PokVddaPmicInCtrlOverVoltDetProxyW::new(self, 15)
    }
    #[doc = "Bit 31 - 31:31\\]
Active POK hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vdda_pmic_in_ctrl_hyst_en_proxy(
        &mut self,
    ) -> PokVddaPmicInCtrlHystEnProxyW<Cfg0PokVddaPmicInCtrlProxySpec> {
        PokVddaPmicInCtrlHystEnProxyW::new(self, 31)
    }
}
#[doc = "CFG0_POK_VDDA_PMIC_IN_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdda_pmic_in_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdda_pmic_in_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PokVddaPmicInCtrlProxySpec;
impl crate::RegisterSpec for Cfg0PokVddaPmicInCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_pok_vdda_pmic_in_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0PokVddaPmicInCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_pok_vdda_pmic_in_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0PokVddaPmicInCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_POK_VDDA_PMIC_IN_CTRL_PROXY to value 0x8000_0000"]
impl crate::Resettable for Cfg0PokVddaPmicInCtrlProxySpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
