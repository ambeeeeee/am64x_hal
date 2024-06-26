#[doc = "Register `CFG0_POK_VDDS_DDRIO_UV_CTRL` reader"]
pub type R = crate::R<Cfg0PokVddsDdrioUvCtrlSpec>;
#[doc = "Register `CFG0_POK_VDDS_DDRIO_UV_CTRL` writer"]
pub type W = crate::W<Cfg0PokVddsDdrioUvCtrlSpec>;
#[doc = "Field `POK_VDDS_DDRIO_UV_CTRL_POK_TRIM` reader - 6:0\\]
POK Trim Bits"]
pub type PokVddsDdrioUvCtrlPokTrimR = crate::FieldReader;
#[doc = "Field `POK_VDDS_DDRIO_UV_CTRL_POK_TRIM` writer - 6:0\\]
POK Trim Bits"]
pub type PokVddsDdrioUvCtrlPokTrimW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `POK_VDDS_DDRIO_UV_CTRL_OVER_VOLT_DET` reader - 7:7\\]
Over / under voltage detection mode"]
pub type PokVddsDdrioUvCtrlOverVoltDetR = crate::BitReader;
#[doc = "Field `POK_VDDS_DDRIO_UV_CTRL_OVER_VOLT_DET` writer - 7:7\\]
Over / under voltage detection mode"]
pub type PokVddsDdrioUvCtrlOverVoltDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POK_VDDS_DDRIO_UV_CTRL_HYST_EN` reader - 31:31\\]
Active POK hysteresis"]
pub type PokVddsDdrioUvCtrlHystEnR = crate::BitReader;
#[doc = "Field `POK_VDDS_DDRIO_UV_CTRL_HYST_EN` writer - 31:31\\]
Active POK hysteresis"]
pub type PokVddsDdrioUvCtrlHystEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
POK Trim Bits"]
    #[inline(always)]
    pub fn pok_vdds_ddrio_uv_ctrl_pok_trim(&self) -> PokVddsDdrioUvCtrlPokTrimR {
        PokVddsDdrioUvCtrlPokTrimR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Over / under voltage detection mode"]
    #[inline(always)]
    pub fn pok_vdds_ddrio_uv_ctrl_over_volt_det(&self) -> PokVddsDdrioUvCtrlOverVoltDetR {
        PokVddsDdrioUvCtrlOverVoltDetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Active POK hysteresis"]
    #[inline(always)]
    pub fn pok_vdds_ddrio_uv_ctrl_hyst_en(&self) -> PokVddsDdrioUvCtrlHystEnR {
        PokVddsDdrioUvCtrlHystEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
POK Trim Bits"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vdds_ddrio_uv_ctrl_pok_trim(
        &mut self,
    ) -> PokVddsDdrioUvCtrlPokTrimW<Cfg0PokVddsDdrioUvCtrlSpec> {
        PokVddsDdrioUvCtrlPokTrimW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Over / under voltage detection mode"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vdds_ddrio_uv_ctrl_over_volt_det(
        &mut self,
    ) -> PokVddsDdrioUvCtrlOverVoltDetW<Cfg0PokVddsDdrioUvCtrlSpec> {
        PokVddsDdrioUvCtrlOverVoltDetW::new(self, 7)
    }
    #[doc = "Bit 31 - 31:31\\]
Active POK hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn pok_vdds_ddrio_uv_ctrl_hyst_en(
        &mut self,
    ) -> PokVddsDdrioUvCtrlHystEnW<Cfg0PokVddsDdrioUvCtrlSpec> {
        PokVddsDdrioUvCtrlHystEnW::new(self, 31)
    }
}
#[doc = "CFG0_POK_VDDS_DDRIO_UV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdds_ddrio_uv_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdds_ddrio_uv_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PokVddsDdrioUvCtrlSpec;
impl crate::RegisterSpec for Cfg0PokVddsDdrioUvCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_pok_vdds_ddrio_uv_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0PokVddsDdrioUvCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_pok_vdds_ddrio_uv_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0PokVddsDdrioUvCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_POK_VDDS_DDRIO_UV_CTRL to value 0x8000_0000"]
impl crate::Resettable for Cfg0PokVddsDdrioUvCtrlSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
