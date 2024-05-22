#[doc = "Register `ECCREGS_sec_enable_clr_reg0` reader"]
pub type R = crate::R<EccregsSecEnableClrReg0Spec>;
#[doc = "Register `ECCREGS_sec_enable_clr_reg0` writer"]
pub type W = crate::W<EccregsSecEnableClrReg0Spec>;
#[doc = "Field `RAM0_ENABLE_CLR` reader - 0:0\\]
Interrupt Enable Clear Register for ram0_pend"]
pub type Ram0EnableClrR = crate::BitReader;
#[doc = "Field `RAM0_ENABLE_CLR` writer - 0:0\\]
Interrupt Enable Clear Register for ram0_pend"]
pub type Ram0EnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1_ENABLE_CLR` reader - 1:1\\]
Interrupt Enable Clear Register for ram1_pend"]
pub type Ram1EnableClrR = crate::BitReader;
#[doc = "Field `RAM1_ENABLE_CLR` writer - 1:1\\]
Interrupt Enable Clear Register for ram1_pend"]
pub type Ram1EnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for ram0_pend"]
    #[inline(always)]
    pub fn ram0_enable_clr(&self) -> Ram0EnableClrR {
        Ram0EnableClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for ram1_pend"]
    #[inline(always)]
    pub fn ram1_enable_clr(&self) -> Ram1EnableClrR {
        Ram1EnableClrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for ram0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn ram0_enable_clr(&mut self) -> Ram0EnableClrW<EccregsSecEnableClrReg0Spec> {
        Ram0EnableClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for ram1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn ram1_enable_clr(&mut self) -> Ram1EnableClrW<EccregsSecEnableClrReg0Spec> {
        Ram1EnableClrW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_sec_enable_clr_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_sec_enable_clr_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccregsSecEnableClrReg0Spec;
impl crate::RegisterSpec for EccregsSecEnableClrReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccregs_sec_enable_clr_reg0::R`](R) reader structure"]
impl crate::Readable for EccregsSecEnableClrReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`eccregs_sec_enable_clr_reg0::W`](W) writer structure"]
impl crate::Writable for EccregsSecEnableClrReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCREGS_sec_enable_clr_reg0 to value 0"]
impl crate::Resettable for EccregsSecEnableClrReg0Spec {
    const RESET_VALUE: u32 = 0;
}
