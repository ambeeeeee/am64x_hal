#[doc = "Register `PR1_CFG__SLV__REGS_pwm3` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPwm3Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pwm3` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPwm3Spec>;
#[doc = "Field `PWM3_DEBOUNCE_VALUE` reader - 7:0\\]
debounce counter , defines the number of core_clk required for the pulse not to get rejected"]
pub type Pwm3DebounceValueR = crate::FieldReader;
#[doc = "Field `PWM3_DEBOUNCE_VALUE` writer - 7:0\\]
debounce counter , defines the number of core_clk required for the pulse not to get rejected"]
pub type Pwm3DebounceValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWM3_TRIP_MASK` reader - 16:8\\]
SW mask for safety trip, one hot"]
pub type Pwm3TripMaskR = crate::FieldReader<u16>;
#[doc = "Field `PWM3_TRIP_MASK` writer - 16:8\\]
SW mask for safety trip, one hot"]
pub type Pwm3TripMaskW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PWM3_TRIP_CMP0_EN` reader - 17:17\\]
CMP0 reset safety trip clear enable"]
pub type Pwm3TripCmp0EnR = crate::BitReader;
#[doc = "Field `PWM3_TRIP_CMP0_EN` writer - 17:17\\]
CMP0 reset safety trip clear enable"]
pub type Pwm3TripCmp0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3_TRIP_RESET` reader - 18:18\\]
SW reset safety flag"]
pub type Pwm3TripResetR = crate::BitReader;
#[doc = "Field `PWM3_TRIP_RESET` writer - 18:18\\]
SW reset safety flag"]
pub type Pwm3TripResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3_OVER_ERR_TRIP` reader - 19:19\\]
SW over safety error trip"]
pub type Pwm3OverErrTripR = crate::BitReader;
#[doc = "Field `PWM3_OVER_ERR_TRIP` writer - 19:19\\]
SW over safety error trip"]
pub type Pwm3OverErrTripW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3_POS_ERR_TRIP` reader - 20:20\\]
SW position saftey error trip"]
pub type Pwm3PosErrTripR = crate::BitReader;
#[doc = "Field `PWM3_POS_ERR_TRIP` writer - 20:20\\]
SW position saftey error trip"]
pub type Pwm3PosErrTripW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3_TRIP_VEC` reader - 29:21\\]
Safety trip trigger cause vector"]
pub type Pwm3TripVecR = crate::FieldReader<u16>;
#[doc = "Field `PWM3_TRIP_VEC` writer - 29:21\\]
Safety trip trigger cause vector"]
pub type Pwm3TripVecW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PWM3_TRIP_S` reader - 30:30\\]
Safety trip status"]
pub type Pwm3TripSR = crate::BitReader;
#[doc = "Field `PWM3_TRIP_S` writer - 30:30\\]
Safety trip status"]
pub type Pwm3TripSW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
debounce counter , defines the number of core_clk required for the pulse not to get rejected"]
    #[inline(always)]
    pub fn pwm3_debounce_value(&self) -> Pwm3DebounceValueR {
        Pwm3DebounceValueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - 16:8\\]
SW mask for safety trip, one hot"]
    #[inline(always)]
    pub fn pwm3_trip_mask(&self) -> Pwm3TripMaskR {
        Pwm3TripMaskR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bit 17 - 17:17\\]
CMP0 reset safety trip clear enable"]
    #[inline(always)]
    pub fn pwm3_trip_cmp0_en(&self) -> Pwm3TripCmp0EnR {
        Pwm3TripCmp0EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
SW reset safety flag"]
    #[inline(always)]
    pub fn pwm3_trip_reset(&self) -> Pwm3TripResetR {
        Pwm3TripResetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
SW over safety error trip"]
    #[inline(always)]
    pub fn pwm3_over_err_trip(&self) -> Pwm3OverErrTripR {
        Pwm3OverErrTripR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
SW position saftey error trip"]
    #[inline(always)]
    pub fn pwm3_pos_err_trip(&self) -> Pwm3PosErrTripR {
        Pwm3PosErrTripR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:29 - 29:21\\]
Safety trip trigger cause vector"]
    #[inline(always)]
    pub fn pwm3_trip_vec(&self) -> Pwm3TripVecR {
        Pwm3TripVecR::new(((self.bits >> 21) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - 30:30\\]
Safety trip status"]
    #[inline(always)]
    pub fn pwm3_trip_s(&self) -> Pwm3TripSR {
        Pwm3TripSR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
debounce counter , defines the number of core_clk required for the pulse not to get rejected"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_debounce_value(&mut self) -> Pwm3DebounceValueW<Pr1Cfg_Slv_RegsPwm3Spec> {
        Pwm3DebounceValueW::new(self, 0)
    }
    #[doc = "Bits 8:16 - 16:8\\]
SW mask for safety trip, one hot"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_trip_mask(&mut self) -> Pwm3TripMaskW<Pr1Cfg_Slv_RegsPwm3Spec> {
        Pwm3TripMaskW::new(self, 8)
    }
    #[doc = "Bit 17 - 17:17\\]
CMP0 reset safety trip clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_trip_cmp0_en(&mut self) -> Pwm3TripCmp0EnW<Pr1Cfg_Slv_RegsPwm3Spec> {
        Pwm3TripCmp0EnW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
SW reset safety flag"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_trip_reset(&mut self) -> Pwm3TripResetW<Pr1Cfg_Slv_RegsPwm3Spec> {
        Pwm3TripResetW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
SW over safety error trip"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_over_err_trip(&mut self) -> Pwm3OverErrTripW<Pr1Cfg_Slv_RegsPwm3Spec> {
        Pwm3OverErrTripW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
SW position saftey error trip"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_pos_err_trip(&mut self) -> Pwm3PosErrTripW<Pr1Cfg_Slv_RegsPwm3Spec> {
        Pwm3PosErrTripW::new(self, 20)
    }
    #[doc = "Bits 21:29 - 29:21\\]
Safety trip trigger cause vector"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_trip_vec(&mut self) -> Pwm3TripVecW<Pr1Cfg_Slv_RegsPwm3Spec> {
        Pwm3TripVecW::new(self, 21)
    }
    #[doc = "Bit 30 - 30:30\\]
Safety trip status"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_trip_s(&mut self) -> Pwm3TripSW<Pr1Cfg_Slv_RegsPwm3Spec> {
        Pwm3TripSW::new(self, 30)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pwm3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPwm3Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPwm3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pwm3::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPwm3Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pwm3::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPwm3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pwm3 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPwm3Spec {
    const RESET_VALUE: u32 = 0;
}
