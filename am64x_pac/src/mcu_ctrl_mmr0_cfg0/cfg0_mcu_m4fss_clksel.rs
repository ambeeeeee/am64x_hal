#[doc = "Register `CFG0_MCU_M4FSS_CLKSEL` reader"]
pub type R = crate::R<Cfg0McuM4fssClkselSpec>;
#[doc = "Register `CFG0_MCU_M4FSS_CLKSEL` writer"]
pub type W = crate::W<Cfg0McuM4fssClkselSpec>;
#[doc = "Field `MCU_M4FSS_CLKSEL_M4FSS_CLKSEL` reader - 0:0\\]
Selects the Clock Divider for M4FSS"]
pub type McuM4fssClkselM4fssClkselR = crate::BitReader;
#[doc = "Field `MCU_M4FSS_CLKSEL_M4FSS_CLKSEL` writer - 0:0\\]
Selects the Clock Divider for M4FSS"]
pub type McuM4fssClkselM4fssClkselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the Clock Divider for M4FSS"]
    #[inline(always)]
    pub fn mcu_m4fss_clksel_m4fss_clksel(&self) -> McuM4fssClkselM4fssClkselR {
        McuM4fssClkselM4fssClkselR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the Clock Divider for M4FSS"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss_clksel_m4fss_clksel(
        &mut self,
    ) -> McuM4fssClkselM4fssClkselW<Cfg0McuM4fssClkselSpec> {
        McuM4fssClkselM4fssClkselW::new(self, 0)
    }
}
#[doc = "CFG0_MCU_M4FSS_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuM4fssClkselSpec;
impl crate::RegisterSpec for Cfg0McuM4fssClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_m4fss_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0McuM4fssClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_m4fss_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0McuM4fssClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_M4FSS_CLKSEL to value 0"]
impl crate::Resettable for Cfg0McuM4fssClkselSpec {
    const RESET_VALUE: u32 = 0;
}
