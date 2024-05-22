#[doc = "Register `CFG0_MCU_M4FSS0_LBIST_SPARE0` reader"]
pub type R = crate::R<Cfg0McuM4fss0LbistSpare0Spec>;
#[doc = "Register `CFG0_MCU_M4FSS0_LBIST_SPARE0` writer"]
pub type W = crate::W<Cfg0McuM4fss0LbistSpare0Spec>;
#[doc = "Field `MCU_M4FSS0_LBIST_SPARE0_LBIST_SELFTEST_EN` reader - 0:0\\]
LBIST isolation control"]
pub type McuM4fss0LbistSpare0LbistSelftestEnR = crate::BitReader;
#[doc = "Field `MCU_M4FSS0_LBIST_SPARE0_LBIST_SELFTEST_EN` writer - 0:0\\]
LBIST isolation control"]
pub type McuM4fss0LbistSpare0LbistSelftestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_M4FSS0_LBIST_SPARE0_PBIST_SELFTEST_EN` reader - 1:1\\]
PBIST isolation control"]
pub type McuM4fss0LbistSpare0PbistSelftestEnR = crate::BitReader;
#[doc = "Field `MCU_M4FSS0_LBIST_SPARE0_PBIST_SELFTEST_EN` writer - 1:1\\]
PBIST isolation control"]
pub type McuM4fss0LbistSpare0PbistSelftestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_M4FSS0_LBIST_SPARE0_SPARE0` reader - 31:2\\]
LBIST spare bits"]
pub type McuM4fss0LbistSpare0Spare0R = crate::FieldReader<u32>;
#[doc = "Field `MCU_M4FSS0_LBIST_SPARE0_SPARE0` writer - 31:2\\]
LBIST spare bits"]
pub type McuM4fss0LbistSpare0Spare0W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
LBIST isolation control"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_spare0_lbist_selftest_en(
        &self,
    ) -> McuM4fss0LbistSpare0LbistSelftestEnR {
        McuM4fss0LbistSpare0LbistSelftestEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
PBIST isolation control"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_spare0_pbist_selftest_en(
        &self,
    ) -> McuM4fss0LbistSpare0PbistSelftestEnR {
        McuM4fss0LbistSpare0PbistSelftestEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
LBIST spare bits"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_spare0_spare0(&self) -> McuM4fss0LbistSpare0Spare0R {
        McuM4fss0LbistSpare0Spare0R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
LBIST isolation control"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_spare0_lbist_selftest_en(
        &mut self,
    ) -> McuM4fss0LbistSpare0LbistSelftestEnW<Cfg0McuM4fss0LbistSpare0Spec> {
        McuM4fss0LbistSpare0LbistSelftestEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
PBIST isolation control"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_spare0_pbist_selftest_en(
        &mut self,
    ) -> McuM4fss0LbistSpare0PbistSelftestEnW<Cfg0McuM4fss0LbistSpare0Spec> {
        McuM4fss0LbistSpare0PbistSelftestEnW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
LBIST spare bits"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_spare0_spare0(
        &mut self,
    ) -> McuM4fss0LbistSpare0Spare0W<Cfg0McuM4fss0LbistSpare0Spec> {
        McuM4fss0LbistSpare0Spare0W::new(self, 2)
    }
}
#[doc = "CFG0_MCU_M4FSS0_LBIST_SPARE0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_spare0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_spare0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuM4fss0LbistSpare0Spec;
impl crate::RegisterSpec for Cfg0McuM4fss0LbistSpare0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_m4fss0_lbist_spare0::R`](R) reader structure"]
impl crate::Readable for Cfg0McuM4fss0LbistSpare0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_m4fss0_lbist_spare0::W`](W) writer structure"]
impl crate::Writable for Cfg0McuM4fss0LbistSpare0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_M4FSS0_LBIST_SPARE0 to value 0"]
impl crate::Resettable for Cfg0McuM4fss0LbistSpare0Spec {
    const RESET_VALUE: u32 = 0;
}
