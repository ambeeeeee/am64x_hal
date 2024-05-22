#[doc = "Register `PR1_IEP1__SLV__REGS_pwm_reg` reader"]
pub type R = crate::R<Pr1Iep1_Slv_RegsPwmRegSpec>;
#[doc = "Register `PR1_IEP1__SLV__REGS_pwm_reg` writer"]
pub type W = crate::W<Pr1Iep1_Slv_RegsPwmRegSpec>;
#[doc = "Field `PWM0_RST_CNT_EN` reader - "]
pub type Pwm0RstCntEnR = crate::BitReader;
#[doc = "Field `PWM0_RST_CNT_EN` writer - "]
pub type Pwm0RstCntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM0_HIT` reader - "]
pub type Pwm0HitR = crate::BitReader;
#[doc = "Field `PWM0_HIT` writer - "]
pub type Pwm0HitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3_RST_CNT_EN` reader - "]
pub type Pwm3RstCntEnR = crate::BitReader;
#[doc = "Field `PWM3_RST_CNT_EN` writer - "]
pub type Pwm3RstCntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3_HIT` reader - "]
pub type Pwm3HitR = crate::BitReader;
#[doc = "Field `PWM3_HIT` writer - "]
pub type Pwm3HitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pwm0_rst_cnt_en(&self) -> Pwm0RstCntEnR {
        Pwm0RstCntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pwm0_hit(&self) -> Pwm0HitR {
        Pwm0HitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pwm3_rst_cnt_en(&self) -> Pwm3RstCntEnR {
        Pwm3RstCntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pwm3_hit(&self) -> Pwm3HitR {
        Pwm3HitR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pwm0_rst_cnt_en(&mut self) -> Pwm0RstCntEnW<Pr1Iep1_Slv_RegsPwmRegSpec> {
        Pwm0RstCntEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwm0_hit(&mut self) -> Pwm0HitW<Pr1Iep1_Slv_RegsPwmRegSpec> {
        Pwm0HitW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_rst_cnt_en(&mut self) -> Pwm3RstCntEnW<Pr1Iep1_Slv_RegsPwmRegSpec> {
        Pwm3RstCntEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_hit(&mut self) -> Pwm3HitW<Pr1Iep1_Slv_RegsPwmRegSpec> {
        Pwm3HitW::new(self, 3)
    }
}
#[doc = "PR1_IEP1__SLV__REGS_pwm_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_pwm_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_pwm_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep1_Slv_RegsPwmRegSpec;
impl crate::RegisterSpec for Pr1Iep1_Slv_RegsPwmRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep1__slv__regs_pwm_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep1_Slv_RegsPwmRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep1__slv__regs_pwm_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep1_Slv_RegsPwmRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP1__SLV__REGS_pwm_reg to value 0"]
impl crate::Resettable for Pr1Iep1_Slv_RegsPwmRegSpec {
    const RESET_VALUE: u32 = 0;
}
