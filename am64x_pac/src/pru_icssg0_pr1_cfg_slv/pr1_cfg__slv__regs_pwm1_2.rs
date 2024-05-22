#[doc = "Register `PR1_CFG__SLV__REGS_pwm1_2` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPwm1_2Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pwm1_2` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPwm1_2Spec>;
#[doc = "Field `PWM1_2_POS_INIT` reader - 1:0\\]
Initial pos state 00 z 01 L 10 H"]
pub type Pwm1_2PosInitR = crate::FieldReader;
#[doc = "Field `PWM1_2_POS_INIT` writer - 1:0\\]
Initial pos state 00 z 01 L 10 H"]
pub type Pwm1_2PosInitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM1_2_NEG_INIT` reader - 3:2\\]
Initial neg state 00 z 01 L 10 H"]
pub type Pwm1_2NegInitR = crate::FieldReader;
#[doc = "Field `PWM1_2_NEG_INIT` writer - 3:2\\]
Initial neg state 00 z 01 L 10 H"]
pub type Pwm1_2NegInitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM1_2_POS_TRIP` reader - 5:4\\]
Safety Trip pos state 00 z 01 L 10 H"]
pub type Pwm1_2PosTripR = crate::FieldReader;
#[doc = "Field `PWM1_2_POS_TRIP` writer - 5:4\\]
Safety Trip pos state 00 z 01 L 10 H"]
pub type Pwm1_2PosTripW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM1_2_NEG_TRIP` reader - 7:6\\]
Safety Trip neg state 00 z 01 L 10 H"]
pub type Pwm1_2NegTripR = crate::FieldReader;
#[doc = "Field `PWM1_2_NEG_TRIP` writer - 7:6\\]
Safety Trip neg state 00 z 01 L 10 H"]
pub type Pwm1_2NegTripW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM1_2_POS_ACT` reader - 9:8\\]
Active pos state 00 toggle 01 L 10 H"]
pub type Pwm1_2PosActR = crate::FieldReader;
#[doc = "Field `PWM1_2_POS_ACT` writer - 9:8\\]
Active pos state 00 toggle 01 L 10 H"]
pub type Pwm1_2PosActW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM1_2_NEG_ACT` reader - 11:10\\]
Active neg state 00 toggle 01 L 10 H"]
pub type Pwm1_2NegActR = crate::FieldReader;
#[doc = "Field `PWM1_2_NEG_ACT` writer - 11:10\\]
Active neg state 00 toggle 01 L 10 H"]
pub type Pwm1_2NegActW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Initial pos state 00 z 01 L 10 H"]
    #[inline(always)]
    pub fn pwm1_2_pos_init(&self) -> Pwm1_2PosInitR {
        Pwm1_2PosInitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Initial neg state 00 z 01 L 10 H"]
    #[inline(always)]
    pub fn pwm1_2_neg_init(&self) -> Pwm1_2NegInitR {
        Pwm1_2NegInitR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Safety Trip pos state 00 z 01 L 10 H"]
    #[inline(always)]
    pub fn pwm1_2_pos_trip(&self) -> Pwm1_2PosTripR {
        Pwm1_2PosTripR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Safety Trip neg state 00 z 01 L 10 H"]
    #[inline(always)]
    pub fn pwm1_2_neg_trip(&self) -> Pwm1_2NegTripR {
        Pwm1_2NegTripR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Active pos state 00 toggle 01 L 10 H"]
    #[inline(always)]
    pub fn pwm1_2_pos_act(&self) -> Pwm1_2PosActR {
        Pwm1_2PosActR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Active neg state 00 toggle 01 L 10 H"]
    #[inline(always)]
    pub fn pwm1_2_neg_act(&self) -> Pwm1_2NegActR {
        Pwm1_2NegActR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Initial pos state 00 z 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_2_pos_init(&mut self) -> Pwm1_2PosInitW<Pr1Cfg_Slv_RegsPwm1_2Spec> {
        Pwm1_2PosInitW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Initial neg state 00 z 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_2_neg_init(&mut self) -> Pwm1_2NegInitW<Pr1Cfg_Slv_RegsPwm1_2Spec> {
        Pwm1_2NegInitW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Safety Trip pos state 00 z 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_2_pos_trip(&mut self) -> Pwm1_2PosTripW<Pr1Cfg_Slv_RegsPwm1_2Spec> {
        Pwm1_2PosTripW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Safety Trip neg state 00 z 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_2_neg_trip(&mut self) -> Pwm1_2NegTripW<Pr1Cfg_Slv_RegsPwm1_2Spec> {
        Pwm1_2NegTripW::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Active pos state 00 toggle 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_2_pos_act(&mut self) -> Pwm1_2PosActW<Pr1Cfg_Slv_RegsPwm1_2Spec> {
        Pwm1_2PosActW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Active neg state 00 toggle 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_2_neg_act(&mut self) -> Pwm1_2NegActW<Pr1Cfg_Slv_RegsPwm1_2Spec> {
        Pwm1_2NegActW::new(self, 10)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pwm1_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm1_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm1_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPwm1_2Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPwm1_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pwm1_2::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPwm1_2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pwm1_2::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPwm1_2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pwm1_2 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPwm1_2Spec {
    const RESET_VALUE: u32 = 0;
}
