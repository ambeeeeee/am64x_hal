#[doc = "Register `CFG0_ADC0_CTRL` reader"]
pub type R = crate::R<Cfg0Adc0CtrlSpec>;
#[doc = "Register `CFG0_ADC0_CTRL` writer"]
pub type W = crate::W<Cfg0Adc0CtrlSpec>;
#[doc = "Field `ADC0_CTRL_TRIG_SEL` reader - 4:0\\]
Selects the source of the ADC hardware event trigger"]
pub type Adc0CtrlTrigSelR = crate::FieldReader;
#[doc = "Field `ADC0_CTRL_TRIG_SEL` writer - 4:0\\]
Selects the source of the ADC hardware event trigger"]
pub type Adc0CtrlTrigSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC0_CTRL_GPI_MODE_EN` reader - 16:16\\]
Activates ADC0 data pins to be used as general purpose inputs when set. This signal is tied to the en_dig_test input of MCU_ADC0"]
pub type Adc0CtrlGpiModeEnR = crate::BitReader;
#[doc = "Field `ADC0_CTRL_GPI_MODE_EN` writer - 16:16\\]
Activates ADC0 data pins to be used as general purpose inputs when set. This signal is tied to the en_dig_test input of MCU_ADC0"]
pub type Adc0CtrlGpiModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Selects the source of the ADC hardware event trigger"]
    #[inline(always)]
    pub fn adc0_ctrl_trig_sel(&self) -> Adc0CtrlTrigSelR {
        Adc0CtrlTrigSelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Activates ADC0 data pins to be used as general purpose inputs when set. This signal is tied to the en_dig_test input of MCU_ADC0"]
    #[inline(always)]
    pub fn adc0_ctrl_gpi_mode_en(&self) -> Adc0CtrlGpiModeEnR {
        Adc0CtrlGpiModeEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Selects the source of the ADC hardware event trigger"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_ctrl_trig_sel(&mut self) -> Adc0CtrlTrigSelW<Cfg0Adc0CtrlSpec> {
        Adc0CtrlTrigSelW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Activates ADC0 data pins to be used as general purpose inputs when set. This signal is tied to the en_dig_test input of MCU_ADC0"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_ctrl_gpi_mode_en(&mut self) -> Adc0CtrlGpiModeEnW<Cfg0Adc0CtrlSpec> {
        Adc0CtrlGpiModeEnW::new(self, 16)
    }
}
#[doc = "CFG0_ADC0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_adc0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_adc0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Adc0CtrlSpec;
impl crate::RegisterSpec for Cfg0Adc0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_adc0_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Adc0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_adc0_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Adc0CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_ADC0_CTRL to value 0"]
impl crate::Resettable for Cfg0Adc0CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
