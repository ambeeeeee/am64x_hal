#[doc = "Register `PR1_CFG__SLV__REGS_pwm2_0` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPwm2_0Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pwm2_0` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPwm2_0Spec>;
#[doc = "Field `PWM2_0_POS_INIT` reader - 1:0\\]
Initial pos state 00 z 01 L 10 H"]
pub type Pwm2_0PosInitR = crate::FieldReader;
#[doc = "Field `PWM2_0_POS_INIT` writer - 1:0\\]
Initial pos state 00 z 01 L 10 H"]
pub type Pwm2_0PosInitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM2_0_NEG_INIT` reader - 3:2\\]
Initial neg state 00 z 01 L 10 H"]
pub type Pwm2_0NegInitR = crate::FieldReader;
#[doc = "Field `PWM2_0_NEG_INIT` writer - 3:2\\]
Initial neg state 00 z 01 L 10 H"]
pub type Pwm2_0NegInitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM2_0_POS_TRIP` reader - 5:4\\]
Safety Trip pos state 00 z 01 L 10 H"]
pub type Pwm2_0PosTripR = crate::FieldReader;
#[doc = "Field `PWM2_0_POS_TRIP` writer - 5:4\\]
Safety Trip pos state 00 z 01 L 10 H"]
pub type Pwm2_0PosTripW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM2_0_NEG_TRIP` reader - 7:6\\]
Safety Trip neg state 00 z 01 L 10 H"]
pub type Pwm2_0NegTripR = crate::FieldReader;
#[doc = "Field `PWM2_0_NEG_TRIP` writer - 7:6\\]
Safety Trip neg state 00 z 01 L 10 H"]
pub type Pwm2_0NegTripW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM2_0_POS_ACT` reader - 9:8\\]
Active pos state 00 toggle 01 L 10 H"]
pub type Pwm2_0PosActR = crate::FieldReader;
#[doc = "Field `PWM2_0_POS_ACT` writer - 9:8\\]
Active pos state 00 toggle 01 L 10 H"]
pub type Pwm2_0PosActW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM2_0_NEG_ACT` reader - 11:10\\]
Active neg state 00 toggle 01 L 10 H"]
pub type Pwm2_0NegActR = crate::FieldReader;
#[doc = "Field `PWM2_0_NEG_ACT` writer - 11:10\\]
Active neg state 00 toggle 01 L 10 H"]
pub type Pwm2_0NegActW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Initial pos state 00 z 01 L 10 H"]
    #[inline(always)]
    pub fn pwm2_0_pos_init(&self) -> Pwm2_0PosInitR {
        Pwm2_0PosInitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Initial neg state 00 z 01 L 10 H"]
    #[inline(always)]
    pub fn pwm2_0_neg_init(&self) -> Pwm2_0NegInitR {
        Pwm2_0NegInitR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Safety Trip pos state 00 z 01 L 10 H"]
    #[inline(always)]
    pub fn pwm2_0_pos_trip(&self) -> Pwm2_0PosTripR {
        Pwm2_0PosTripR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Safety Trip neg state 00 z 01 L 10 H"]
    #[inline(always)]
    pub fn pwm2_0_neg_trip(&self) -> Pwm2_0NegTripR {
        Pwm2_0NegTripR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Active pos state 00 toggle 01 L 10 H"]
    #[inline(always)]
    pub fn pwm2_0_pos_act(&self) -> Pwm2_0PosActR {
        Pwm2_0PosActR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Active neg state 00 toggle 01 L 10 H"]
    #[inline(always)]
    pub fn pwm2_0_neg_act(&self) -> Pwm2_0NegActR {
        Pwm2_0NegActR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Initial pos state 00 z 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm2_0_pos_init(&mut self) -> Pwm2_0PosInitW<Pr1Cfg_Slv_RegsPwm2_0Spec> {
        Pwm2_0PosInitW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Initial neg state 00 z 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm2_0_neg_init(&mut self) -> Pwm2_0NegInitW<Pr1Cfg_Slv_RegsPwm2_0Spec> {
        Pwm2_0NegInitW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Safety Trip pos state 00 z 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm2_0_pos_trip(&mut self) -> Pwm2_0PosTripW<Pr1Cfg_Slv_RegsPwm2_0Spec> {
        Pwm2_0PosTripW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Safety Trip neg state 00 z 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm2_0_neg_trip(&mut self) -> Pwm2_0NegTripW<Pr1Cfg_Slv_RegsPwm2_0Spec> {
        Pwm2_0NegTripW::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Active pos state 00 toggle 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm2_0_pos_act(&mut self) -> Pwm2_0PosActW<Pr1Cfg_Slv_RegsPwm2_0Spec> {
        Pwm2_0PosActW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Active neg state 00 toggle 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm2_0_neg_act(&mut self) -> Pwm2_0NegActW<Pr1Cfg_Slv_RegsPwm2_0Spec> {
        Pwm2_0NegActW::new(self, 10)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pwm2_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm2_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm2_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPwm2_0Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPwm2_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pwm2_0::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPwm2_0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pwm2_0::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPwm2_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pwm2_0 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPwm2_0Spec {
    const RESET_VALUE: u32 = 0;
}
