#[doc = "Register `CFG0_POK_VDDA_PMIC_IN_CTRL` reader"]
pub type R = crate::R<Cfg0PokVddaPmicInCtrlSpec>;
#[doc = "Register `CFG0_POK_VDDA_PMIC_IN_CTRL` writer"]
pub type W = crate::W<Cfg0PokVddaPmicInCtrlSpec>;
#[doc = "Field `POK_VDDA_PMIC_IN_CTRL_OVER_VOLT_DET` reader - 15:15\\]
Over / under voltage detection mode"]
pub type PokVddaPmicInCtrlOverVoltDetR = crate::BitReader;
#[doc = "Field `POK_VDDA_PMIC_IN_CTRL_OVER_VOLT_DET` writer - 15:15\\]
Over / under voltage detection mode"]
pub type PokVddaPmicInCtrlOverVoltDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POK_VDDA_PMIC_IN_CTRL_HYST_EN` reader - 31:31\\]
Active POK hysteresis"]
pub type PokVddaPmicInCtrlHystEnR = crate::BitReader;
#[doc = "Field `POK_VDDA_PMIC_IN_CTRL_HYST_EN` writer - 31:31\\]
Active POK hysteresis"]
pub type PokVddaPmicInCtrlHystEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 15 - 15:15\\]
Over / under voltage detection mode"]
    #[inline(always)]
    pub fn pok_vdda_pmic_in_ctrl_over_volt_det(&self) -> PokVddaPmicInCtrlOverVoltDetR {
        PokVddaPmicInCtrlOverVoltDetR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Active POK hysteresis"]
    #[inline(always)]
    pub fn pok_vdda_pmic_in_ctrl_hyst_en(&self) -> PokVddaPmicInCtrlHystEnR {
        PokVddaPmicInCtrlHystEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - 15:15\\]
Over / under voltage detection mode"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vdda_pmic_in_ctrl_over_volt_det(
        &mut self,
    ) -> PokVddaPmicInCtrlOverVoltDetW<Cfg0PokVddaPmicInCtrlSpec> {
        PokVddaPmicInCtrlOverVoltDetW::new(self, 15)
    }
    #[doc = "Bit 31 - 31:31\\]
Active POK hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vdda_pmic_in_ctrl_hyst_en(
        &mut self,
    ) -> PokVddaPmicInCtrlHystEnW<Cfg0PokVddaPmicInCtrlSpec> {
        PokVddaPmicInCtrlHystEnW::new(self, 31)
    }
}
#[doc = "CFG0_POK_VDDA_PMIC_IN_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdda_pmic_in_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdda_pmic_in_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PokVddaPmicInCtrlSpec;
impl crate::RegisterSpec for Cfg0PokVddaPmicInCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_pok_vdda_pmic_in_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0PokVddaPmicInCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_pok_vdda_pmic_in_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0PokVddaPmicInCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_POK_VDDA_PMIC_IN_CTRL to value 0x8000_0000"]
impl crate::Resettable for Cfg0PokVddaPmicInCtrlSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
