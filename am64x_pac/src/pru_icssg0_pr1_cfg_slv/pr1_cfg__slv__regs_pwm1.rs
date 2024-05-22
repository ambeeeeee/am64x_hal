#[doc = "Register `PR1_CFG__SLV__REGS_pwm1` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPwm1Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pwm1` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPwm1Spec>;
#[doc = "Field `PWM1_DEBOUNCE_VALUE` reader - 7:0\\]
debounce counter , defines the number of core_clk required for the pulse not to get rejected"]
pub type Pwm1DebounceValueR = crate::FieldReader;
#[doc = "Field `PWM1_DEBOUNCE_VALUE` writer - 7:0\\]
debounce counter , defines the number of core_clk required for the pulse not to get rejected"]
pub type Pwm1DebounceValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWM1_TRIP_MASK` reader - 16:8\\]
SW mask for safety trip, one hot"]
pub type Pwm1TripMaskR = crate::FieldReader<u16>;
#[doc = "Field `PWM1_TRIP_MASK` writer - 16:8\\]
SW mask for safety trip, one hot"]
pub type Pwm1TripMaskW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PWM1_TRIP_CMP0_EN` reader - 17:17\\]
CMP0 reset safety trip clear enable"]
pub type Pwm1TripCmp0EnR = crate::BitReader;
#[doc = "Field `PWM1_TRIP_CMP0_EN` writer - 17:17\\]
CMP0 reset safety trip clear enable"]
pub type Pwm1TripCmp0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1_TRIP_RESET` reader - 18:18\\]
SW reset safety flag"]
pub type Pwm1TripResetR = crate::BitReader;
#[doc = "Field `PWM1_TRIP_RESET` writer - 18:18\\]
SW reset safety flag"]
pub type Pwm1TripResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1_OVER_ERR_TRIP` reader - 19:19\\]
SW over safety error trip"]
pub type Pwm1OverErrTripR = crate::BitReader;
#[doc = "Field `PWM1_OVER_ERR_TRIP` writer - 19:19\\]
SW over safety error trip"]
pub type Pwm1OverErrTripW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1_POS_ERR_TRIP` reader - 20:20\\]
SW position saftey error trip"]
pub type Pwm1PosErrTripR = crate::BitReader;
#[doc = "Field `PWM1_POS_ERR_TRIP` writer - 20:20\\]
SW position saftey error trip"]
pub type Pwm1PosErrTripW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1_TRIP_VEC` reader - 29:21\\]
Safety trip trigger cause vector"]
pub type Pwm1TripVecR = crate::FieldReader<u16>;
#[doc = "Field `PWM1_TRIP_VEC` writer - 29:21\\]
Safety trip trigger cause vector"]
pub type Pwm1TripVecW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PWM1_TRIP_S` reader - 30:30\\]
Safety trip status"]
pub type Pwm1TripSR = crate::BitReader;
#[doc = "Field `PWM1_TRIP_S` writer - 30:30\\]
Safety trip status"]
pub type Pwm1TripSW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
debounce counter , defines the number of core_clk required for the pulse not to get rejected"]
    #[inline(always)]
    pub fn pwm1_debounce_value(&self) -> Pwm1DebounceValueR {
        Pwm1DebounceValueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - 16:8\\]
SW mask for safety trip, one hot"]
    #[inline(always)]
    pub fn pwm1_trip_mask(&self) -> Pwm1TripMaskR {
        Pwm1TripMaskR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bit 17 - 17:17\\]
CMP0 reset safety trip clear enable"]
    #[inline(always)]
    pub fn pwm1_trip_cmp0_en(&self) -> Pwm1TripCmp0EnR {
        Pwm1TripCmp0EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
SW reset safety flag"]
    #[inline(always)]
    pub fn pwm1_trip_reset(&self) -> Pwm1TripResetR {
        Pwm1TripResetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
SW over safety error trip"]
    #[inline(always)]
    pub fn pwm1_over_err_trip(&self) -> Pwm1OverErrTripR {
        Pwm1OverErrTripR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
SW position saftey error trip"]
    #[inline(always)]
    pub fn pwm1_pos_err_trip(&self) -> Pwm1PosErrTripR {
        Pwm1PosErrTripR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:29 - 29:21\\]
Safety trip trigger cause vector"]
    #[inline(always)]
    pub fn pwm1_trip_vec(&self) -> Pwm1TripVecR {
        Pwm1TripVecR::new(((self.bits >> 21) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - 30:30\\]
Safety trip status"]
    #[inline(always)]
    pub fn pwm1_trip_s(&self) -> Pwm1TripSR {
        Pwm1TripSR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
debounce counter , defines the number of core_clk required for the pulse not to get rejected"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_debounce_value(&mut self) -> Pwm1DebounceValueW<Pr1Cfg_Slv_RegsPwm1Spec> {
        Pwm1DebounceValueW::new(self, 0)
    }
    #[doc = "Bits 8:16 - 16:8\\]
SW mask for safety trip, one hot"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_trip_mask(&mut self) -> Pwm1TripMaskW<Pr1Cfg_Slv_RegsPwm1Spec> {
        Pwm1TripMaskW::new(self, 8)
    }
    #[doc = "Bit 17 - 17:17\\]
CMP0 reset safety trip clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_trip_cmp0_en(&mut self) -> Pwm1TripCmp0EnW<Pr1Cfg_Slv_RegsPwm1Spec> {
        Pwm1TripCmp0EnW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
SW reset safety flag"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_trip_reset(&mut self) -> Pwm1TripResetW<Pr1Cfg_Slv_RegsPwm1Spec> {
        Pwm1TripResetW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
SW over safety error trip"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_over_err_trip(&mut self) -> Pwm1OverErrTripW<Pr1Cfg_Slv_RegsPwm1Spec> {
        Pwm1OverErrTripW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
SW position saftey error trip"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_pos_err_trip(&mut self) -> Pwm1PosErrTripW<Pr1Cfg_Slv_RegsPwm1Spec> {
        Pwm1PosErrTripW::new(self, 20)
    }
    #[doc = "Bits 21:29 - 29:21\\]
Safety trip trigger cause vector"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_trip_vec(&mut self) -> Pwm1TripVecW<Pr1Cfg_Slv_RegsPwm1Spec> {
        Pwm1TripVecW::new(self, 21)
    }
    #[doc = "Bit 30 - 30:30\\]
Safety trip status"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_trip_s(&mut self) -> Pwm1TripSW<Pr1Cfg_Slv_RegsPwm1Spec> {
        Pwm1TripSW::new(self, 30)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pwm1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPwm1Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPwm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pwm1::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPwm1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pwm1::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPwm1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pwm1 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPwm1Spec {
    const RESET_VALUE: u32 = 0;
}
