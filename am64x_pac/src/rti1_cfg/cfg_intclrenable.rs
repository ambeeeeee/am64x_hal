#[doc = "Register `CFG_INTCLRENABLE` reader"]
pub type R = crate::R<CfgIntclrenableSpec>;
#[doc = "Register `CFG_INTCLRENABLE` writer"]
pub type W = crate::W<CfgIntclrenableSpec>;
#[doc = "Field `INTCLRENABLE0` reader - 3:0\\]
Enables the auto-clear functionality on the compare 0 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 0 interrupt is disabled. Any other value = Auto-clear for compare 0 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 0 interrupt. Any other value = Enables the auto-clear functionality on the compare 0 interrupt. Note: Hook-up of Compare Interrupt to a device pin The RTI module generates up to 4 compare interrupts. The connection between one or more of these compare interrupt(s) to a device pin is completely device-dependent. Refer to the device datasheet to identify the actual pin(s) that connects to the compare interrupt(s)."]
pub type Intclrenable0R = crate::FieldReader;
#[doc = "Field `INTCLRENABLE0` writer - 3:0\\]
Enables the auto-clear functionality on the compare 0 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 0 interrupt is disabled. Any other value = Auto-clear for compare 0 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 0 interrupt. Any other value = Enables the auto-clear functionality on the compare 0 interrupt. Note: Hook-up of Compare Interrupt to a device pin The RTI module generates up to 4 compare interrupts. The connection between one or more of these compare interrupt(s) to a device pin is completely device-dependent. Refer to the device datasheet to identify the actual pin(s) that connects to the compare interrupt(s)."]
pub type Intclrenable0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTCLRENABLE1` reader - 11:8\\]
Enables the auto-clear functionality on the compare 1 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 1 interrupt is disabled. Any other value = Auto-clear for compare 1 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 1 interrupt. Any other value = Enables the auto-clear functionality on the compare 1 interrupt."]
pub type Intclrenable1R = crate::FieldReader;
#[doc = "Field `INTCLRENABLE1` writer - 11:8\\]
Enables the auto-clear functionality on the compare 1 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 1 interrupt is disabled. Any other value = Auto-clear for compare 1 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 1 interrupt. Any other value = Enables the auto-clear functionality on the compare 1 interrupt."]
pub type Intclrenable1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTCLRENABLE2` reader - 19:16\\]
Enables the auto-clear functionality on the compare 2 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 2 interrupt is disabled. Any other value = Auto-clear for compare 2 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 2 interrupt. Any other value = Enables the auto-clear functionality on the compare 2 interrupt."]
pub type Intclrenable2R = crate::FieldReader;
#[doc = "Field `INTCLRENABLE2` writer - 19:16\\]
Enables the auto-clear functionality on the compare 2 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 2 interrupt is disabled. Any other value = Auto-clear for compare 2 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 2 interrupt. Any other value = Enables the auto-clear functionality on the compare 2 interrupt."]
pub type Intclrenable2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTCLRENABLE3` reader - 27:24\\]
Enables the auto-clear functionality on the compare 3 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 3 interrupt is disabled. Any other value = Auto-clear for compare 3 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 3 interrupt. Any other value = Enables the auto-clear functionality on the compare 3 interrupt."]
pub type Intclrenable3R = crate::FieldReader;
#[doc = "Field `INTCLRENABLE3` writer - 27:24\\]
Enables the auto-clear functionality on the compare 3 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 3 interrupt is disabled. Any other value = Auto-clear for compare 3 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 3 interrupt. Any other value = Enables the auto-clear functionality on the compare 3 interrupt."]
pub type Intclrenable3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Enables the auto-clear functionality on the compare 0 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 0 interrupt is disabled. Any other value = Auto-clear for compare 0 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 0 interrupt. Any other value = Enables the auto-clear functionality on the compare 0 interrupt. Note: Hook-up of Compare Interrupt to a device pin The RTI module generates up to 4 compare interrupts. The connection between one or more of these compare interrupt(s) to a device pin is completely device-dependent. Refer to the device datasheet to identify the actual pin(s) that connects to the compare interrupt(s)."]
    #[inline(always)]
    pub fn intclrenable0(&self) -> Intclrenable0R {
        Intclrenable0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Enables the auto-clear functionality on the compare 1 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 1 interrupt is disabled. Any other value = Auto-clear for compare 1 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 1 interrupt. Any other value = Enables the auto-clear functionality on the compare 1 interrupt."]
    #[inline(always)]
    pub fn intclrenable1(&self) -> Intclrenable1R {
        Intclrenable1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Enables the auto-clear functionality on the compare 2 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 2 interrupt is disabled. Any other value = Auto-clear for compare 2 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 2 interrupt. Any other value = Enables the auto-clear functionality on the compare 2 interrupt."]
    #[inline(always)]
    pub fn intclrenable2(&self) -> Intclrenable2R {
        Intclrenable2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Enables the auto-clear functionality on the compare 3 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 3 interrupt is disabled. Any other value = Auto-clear for compare 3 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 3 interrupt. Any other value = Enables the auto-clear functionality on the compare 3 interrupt."]
    #[inline(always)]
    pub fn intclrenable3(&self) -> Intclrenable3R {
        Intclrenable3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Enables the auto-clear functionality on the compare 0 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 0 interrupt is disabled. Any other value = Auto-clear for compare 0 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 0 interrupt. Any other value = Enables the auto-clear functionality on the compare 0 interrupt. Note: Hook-up of Compare Interrupt to a device pin The RTI module generates up to 4 compare interrupts. The connection between one or more of these compare interrupt(s) to a device pin is completely device-dependent. Refer to the device datasheet to identify the actual pin(s) that connects to the compare interrupt(s)."]
    #[inline(always)]
    #[must_use]
    pub fn intclrenable0(&mut self) -> Intclrenable0W<CfgIntclrenableSpec> {
        Intclrenable0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Enables the auto-clear functionality on the compare 1 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 1 interrupt is disabled. Any other value = Auto-clear for compare 1 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 1 interrupt. Any other value = Enables the auto-clear functionality on the compare 1 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn intclrenable1(&mut self) -> Intclrenable1W<CfgIntclrenableSpec> {
        Intclrenable1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Enables the auto-clear functionality on the compare 2 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 2 interrupt is disabled. Any other value = Auto-clear for compare 2 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 2 interrupt. Any other value = Enables the auto-clear functionality on the compare 2 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn intclrenable2(&mut self) -> Intclrenable2W<CfgIntclrenableSpec> {
        Intclrenable2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Enables the auto-clear functionality on the compare 3 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 3 interrupt is disabled. Any other value = Auto-clear for compare 3 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 3 interrupt. Any other value = Enables the auto-clear functionality on the compare 3 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn intclrenable3(&mut self) -> Intclrenable3W<CfgIntclrenableSpec> {
        Intclrenable3W::new(self, 24)
    }
}
#[doc = "CFG_INTCLRENABLE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_intclrenable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_intclrenable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgIntclrenableSpec;
impl crate::RegisterSpec for CfgIntclrenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_intclrenable::R`](R) reader structure"]
impl crate::Readable for CfgIntclrenableSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_intclrenable::W`](W) writer structure"]
impl crate::Writable for CfgIntclrenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_INTCLRENABLE to value 0x0505_0505"]
impl crate::Resettable for CfgIntclrenableSpec {
    const RESET_VALUE: u32 = 0x0505_0505;
}
