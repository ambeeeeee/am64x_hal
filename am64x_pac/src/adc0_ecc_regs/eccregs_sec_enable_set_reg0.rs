#[doc = "Register `ECCREGS_sec_enable_set_reg0` reader"]
pub type R = crate::R<EccregsSecEnableSetReg0Spec>;
#[doc = "Register `ECCREGS_sec_enable_set_reg0` writer"]
pub type W = crate::W<EccregsSecEnableSetReg0Spec>;
#[doc = "Field `RAM0_ENABLE_SET` reader - 0:0\\]
Interrupt Enable Set Register for ram0_pend"]
pub type Ram0EnableSetR = crate::BitReader;
#[doc = "Field `RAM0_ENABLE_SET` writer - 0:0\\]
Interrupt Enable Set Register for ram0_pend"]
pub type Ram0EnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1_ENABLE_SET` reader - 1:1\\]
Interrupt Enable Set Register for ram1_pend"]
pub type Ram1EnableSetR = crate::BitReader;
#[doc = "Field `RAM1_ENABLE_SET` writer - 1:1\\]
Interrupt Enable Set Register for ram1_pend"]
pub type Ram1EnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for ram0_pend"]
    #[inline(always)]
    pub fn ram0_enable_set(&self) -> Ram0EnableSetR {
        Ram0EnableSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for ram1_pend"]
    #[inline(always)]
    pub fn ram1_enable_set(&self) -> Ram1EnableSetR {
        Ram1EnableSetR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for ram0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn ram0_enable_set(&mut self) -> Ram0EnableSetW<EccregsSecEnableSetReg0Spec> {
        Ram0EnableSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for ram1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn ram1_enable_set(&mut self) -> Ram1EnableSetW<EccregsSecEnableSetReg0Spec> {
        Ram1EnableSetW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_sec_enable_set_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_sec_enable_set_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccregsSecEnableSetReg0Spec;
impl crate::RegisterSpec for EccregsSecEnableSetReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccregs_sec_enable_set_reg0::R`](R) reader structure"]
impl crate::Readable for EccregsSecEnableSetReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`eccregs_sec_enable_set_reg0::W`](W) writer structure"]
impl crate::Writable for EccregsSecEnableSetReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCREGS_sec_enable_set_reg0 to value 0"]
impl crate::Resettable for EccregsSecEnableSetReg0Spec {
    const RESET_VALUE: u32 = 0;
}
