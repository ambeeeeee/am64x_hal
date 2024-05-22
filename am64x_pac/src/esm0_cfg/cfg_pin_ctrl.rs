#[doc = "Register `CFG_PIN_CTRL` reader"]
pub type R = crate::R<CfgPinCtrlSpec>;
#[doc = "Register `CFG_PIN_CTRL` writer"]
pub type W = crate::W<CfgPinCtrlSpec>;
#[doc = "Field `KEY` reader - 3:0\\]
Pin Control Key"]
pub type KeyR = crate::FieldReader;
#[doc = "Field `KEY` writer - 3:0\\]
Pin Control Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PWM_EN` reader - 7:4\\]
PWM enable"]
pub type PwmEnR = crate::FieldReader;
#[doc = "Field `PWM_EN` writer - 7:4\\]
PWM enable"]
pub type PwmEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pin Control Key"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
PWM enable"]
    #[inline(always)]
    pub fn pwm_en(&self) -> PwmEnR {
        PwmEnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pin Control Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CfgPinCtrlSpec> {
        KeyW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
PWM enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_en(&mut self) -> PwmEnW<CfgPinCtrlSpec> {
        PwmEnW::new(self, 4)
    }
}
#[doc = "This register controls the error_pin_n output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pin_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pin_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPinCtrlSpec;
impl crate::RegisterSpec for CfgPinCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pin_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgPinCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pin_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgPinCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_PIN_CTRL to value 0"]
impl crate::Resettable for CfgPinCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
