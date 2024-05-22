#[doc = "Register `PR1_CFG__SLV__REGS_sa_mx_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsSaMxRegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_sa_mx_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsSaMxRegSpec>;
#[doc = "Field `SA_MUX_SEL` reader - "]
pub type SaMuxSelR = crate::FieldReader;
#[doc = "Field `SA_MUX_SEL` writer - "]
pub type SaMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWM0_REMAP_EN` reader - "]
pub type Pwm0RemapEnR = crate::FieldReader;
#[doc = "Field `PWM0_REMAP_EN` writer - "]
pub type Pwm0RemapEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM3_REMAP_EN` reader - "]
pub type Pwm3RemapEnR = crate::FieldReader;
#[doc = "Field `PWM3_REMAP_EN` writer - "]
pub type Pwm3RemapEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM_EFC_EN` reader - "]
pub type PwmEfcEnR = crate::BitReader;
#[doc = "Field `PWM_EFC_EN` writer - "]
pub type PwmEfcEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sa_mux_sel(&self) -> SaMuxSelR {
        SaMuxSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pwm0_remap_en(&self) -> Pwm0RemapEnR {
        Pwm0RemapEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pwm3_remap_en(&self) -> Pwm3RemapEnR {
        Pwm3RemapEnR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pwm_efc_en(&self) -> PwmEfcEnR {
        PwmEfcEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn sa_mux_sel(&mut self) -> SaMuxSelW<Pr1Cfg_Slv_RegsSaMxRegSpec> {
        SaMuxSelW::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn pwm0_remap_en(&mut self) -> Pwm0RemapEnW<Pr1Cfg_Slv_RegsSaMxRegSpec> {
        Pwm0RemapEnW::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_remap_en(&mut self) -> Pwm3RemapEnW<Pr1Cfg_Slv_RegsSaMxRegSpec> {
        Pwm3RemapEnW::new(self, 10)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_efc_en(&mut self) -> PwmEfcEnW<Pr1Cfg_Slv_RegsSaMxRegSpec> {
        PwmEfcEnW::new(self, 16)
    }
}
#[doc = "PR1_CFG__SLV__REGS_sa_mx_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_sa_mx_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_sa_mx_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsSaMxRegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsSaMxRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_sa_mx_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsSaMxRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_sa_mx_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsSaMxRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_sa_mx_reg to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsSaMxRegSpec {
    const RESET_VALUE: u32 = 0;
}
