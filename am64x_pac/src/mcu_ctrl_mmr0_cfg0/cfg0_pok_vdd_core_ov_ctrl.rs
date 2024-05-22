#[doc = "Register `CFG0_POK_VDD_CORE_OV_CTRL` reader"]
pub type R = crate::R<Cfg0PokVddCoreOvCtrlSpec>;
#[doc = "Register `CFG0_POK_VDD_CORE_OV_CTRL` writer"]
pub type W = crate::W<Cfg0PokVddCoreOvCtrlSpec>;
#[doc = "Field `POK_VDD_CORE_OV_CTRL_POK_TRIM` reader - 6:0\\]
POK Trim Bits"]
pub type PokVddCoreOvCtrlPokTrimR = crate::FieldReader;
#[doc = "Field `POK_VDD_CORE_OV_CTRL_POK_TRIM` writer - 6:0\\]
POK Trim Bits"]
pub type PokVddCoreOvCtrlPokTrimW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `POK_VDD_CORE_OV_CTRL_OVER_VOLT_DET` reader - 7:7\\]
Over / under voltage detection mode"]
pub type PokVddCoreOvCtrlOverVoltDetR = crate::BitReader;
#[doc = "Field `POK_VDD_CORE_OV_CTRL_OVER_VOLT_DET` writer - 7:7\\]
Over / under voltage detection mode"]
pub type PokVddCoreOvCtrlOverVoltDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POK_VDD_CORE_OV_CTRL_HYST_EN` reader - 31:31\\]
Active POK hysteresis"]
pub type PokVddCoreOvCtrlHystEnR = crate::BitReader;
#[doc = "Field `POK_VDD_CORE_OV_CTRL_HYST_EN` writer - 31:31\\]
Active POK hysteresis"]
pub type PokVddCoreOvCtrlHystEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
POK Trim Bits"]
    #[inline(always)]
    pub fn pok_vdd_core_ov_ctrl_pok_trim(&self) -> PokVddCoreOvCtrlPokTrimR {
        PokVddCoreOvCtrlPokTrimR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Over / under voltage detection mode"]
    #[inline(always)]
    pub fn pok_vdd_core_ov_ctrl_over_volt_det(&self) -> PokVddCoreOvCtrlOverVoltDetR {
        PokVddCoreOvCtrlOverVoltDetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Active POK hysteresis"]
    #[inline(always)]
    pub fn pok_vdd_core_ov_ctrl_hyst_en(&self) -> PokVddCoreOvCtrlHystEnR {
        PokVddCoreOvCtrlHystEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
POK Trim Bits"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vdd_core_ov_ctrl_pok_trim(
        &mut self,
    ) -> PokVddCoreOvCtrlPokTrimW<Cfg0PokVddCoreOvCtrlSpec> {
        PokVddCoreOvCtrlPokTrimW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Over / under voltage detection mode"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vdd_core_ov_ctrl_over_volt_det(
        &mut self,
    ) -> PokVddCoreOvCtrlOverVoltDetW<Cfg0PokVddCoreOvCtrlSpec> {
        PokVddCoreOvCtrlOverVoltDetW::new(self, 7)
    }
    #[doc = "Bit 31 - 31:31\\]
Active POK hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vdd_core_ov_ctrl_hyst_en(
        &mut self,
    ) -> PokVddCoreOvCtrlHystEnW<Cfg0PokVddCoreOvCtrlSpec> {
        PokVddCoreOvCtrlHystEnW::new(self, 31)
    }
}
#[doc = "CFG0_POK_VDD_CORE_OV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdd_core_ov_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdd_core_ov_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PokVddCoreOvCtrlSpec;
impl crate::RegisterSpec for Cfg0PokVddCoreOvCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_pok_vdd_core_ov_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0PokVddCoreOvCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_pok_vdd_core_ov_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0PokVddCoreOvCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_POK_VDD_CORE_OV_CTRL to value 0x8000_0000"]
impl crate::Resettable for Cfg0PokVddCoreOvCtrlSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
