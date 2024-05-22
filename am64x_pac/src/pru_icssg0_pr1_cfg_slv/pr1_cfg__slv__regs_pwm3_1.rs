#[doc = "Register `PR1_CFG__SLV__REGS_pwm3_1` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPwm3_1Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pwm3_1` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPwm3_1Spec>;
#[doc = "Field `PWM3_1_POS_INIT` reader - 1:0\\]
Initial pos state 00 z 01 L 10 H"]
pub type Pwm3_1PosInitR = crate::FieldReader;
#[doc = "Field `PWM3_1_POS_INIT` writer - 1:0\\]
Initial pos state 00 z 01 L 10 H"]
pub type Pwm3_1PosInitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM3_1_NEG_INIT` reader - 3:2\\]
Initial neg state 00 z 01 L 10 H"]
pub type Pwm3_1NegInitR = crate::FieldReader;
#[doc = "Field `PWM3_1_NEG_INIT` writer - 3:2\\]
Initial neg state 00 z 01 L 10 H"]
pub type Pwm3_1NegInitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM3_1_POS_TRIP` reader - 5:4\\]
Safety Trip pos state 00 z 01 L 10 H"]
pub type Pwm3_1PosTripR = crate::FieldReader;
#[doc = "Field `PWM3_1_POS_TRIP` writer - 5:4\\]
Safety Trip pos state 00 z 01 L 10 H"]
pub type Pwm3_1PosTripW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM3_1_NEG_TRIP` reader - 7:6\\]
Safety Trip neg state 00 z 01 L 10 H"]
pub type Pwm3_1NegTripR = crate::FieldReader;
#[doc = "Field `PWM3_1_NEG_TRIP` writer - 7:6\\]
Safety Trip neg state 00 z 01 L 10 H"]
pub type Pwm3_1NegTripW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM3_1_POS_ACT` reader - 9:8\\]
Active pos state 00 toggle 01 L 10 H"]
pub type Pwm3_1PosActR = crate::FieldReader;
#[doc = "Field `PWM3_1_POS_ACT` writer - 9:8\\]
Active pos state 00 toggle 01 L 10 H"]
pub type Pwm3_1PosActW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM3_1_NEG_ACT` reader - 11:10\\]
Active neg state 00 toggle 01 L 10 H"]
pub type Pwm3_1NegActR = crate::FieldReader;
#[doc = "Field `PWM3_1_NEG_ACT` writer - 11:10\\]
Active neg state 00 toggle 01 L 10 H"]
pub type Pwm3_1NegActW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Initial pos state 00 z 01 L 10 H"]
    #[inline(always)]
    pub fn pwm3_1_pos_init(&self) -> Pwm3_1PosInitR {
        Pwm3_1PosInitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Initial neg state 00 z 01 L 10 H"]
    #[inline(always)]
    pub fn pwm3_1_neg_init(&self) -> Pwm3_1NegInitR {
        Pwm3_1NegInitR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Safety Trip pos state 00 z 01 L 10 H"]
    #[inline(always)]
    pub fn pwm3_1_pos_trip(&self) -> Pwm3_1PosTripR {
        Pwm3_1PosTripR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Safety Trip neg state 00 z 01 L 10 H"]
    #[inline(always)]
    pub fn pwm3_1_neg_trip(&self) -> Pwm3_1NegTripR {
        Pwm3_1NegTripR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Active pos state 00 toggle 01 L 10 H"]
    #[inline(always)]
    pub fn pwm3_1_pos_act(&self) -> Pwm3_1PosActR {
        Pwm3_1PosActR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Active neg state 00 toggle 01 L 10 H"]
    #[inline(always)]
    pub fn pwm3_1_neg_act(&self) -> Pwm3_1NegActR {
        Pwm3_1NegActR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Initial pos state 00 z 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_1_pos_init(&mut self) -> Pwm3_1PosInitW<Pr1Cfg_Slv_RegsPwm3_1Spec> {
        Pwm3_1PosInitW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Initial neg state 00 z 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_1_neg_init(&mut self) -> Pwm3_1NegInitW<Pr1Cfg_Slv_RegsPwm3_1Spec> {
        Pwm3_1NegInitW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Safety Trip pos state 00 z 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_1_pos_trip(&mut self) -> Pwm3_1PosTripW<Pr1Cfg_Slv_RegsPwm3_1Spec> {
        Pwm3_1PosTripW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Safety Trip neg state 00 z 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_1_neg_trip(&mut self) -> Pwm3_1NegTripW<Pr1Cfg_Slv_RegsPwm3_1Spec> {
        Pwm3_1NegTripW::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Active pos state 00 toggle 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_1_pos_act(&mut self) -> Pwm3_1PosActW<Pr1Cfg_Slv_RegsPwm3_1Spec> {
        Pwm3_1PosActW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Active neg state 00 toggle 01 L 10 H"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_1_neg_act(&mut self) -> Pwm3_1NegActW<Pr1Cfg_Slv_RegsPwm3_1Spec> {
        Pwm3_1NegActW::new(self, 10)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pwm3_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm3_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm3_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPwm3_1Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPwm3_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pwm3_1::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPwm3_1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pwm3_1::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPwm3_1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pwm3_1 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPwm3_1Spec {
    const RESET_VALUE: u32 = 0;
}
