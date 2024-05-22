#[doc = "Register `CFG0_MCU_M4FSS0_LBIST_SPARE0_PROXY` reader"]
pub type R = crate::R<Cfg0McuM4fss0LbistSpare0ProxySpec>;
#[doc = "Register `CFG0_MCU_M4FSS0_LBIST_SPARE0_PROXY` writer"]
pub type W = crate::W<Cfg0McuM4fss0LbistSpare0ProxySpec>;
#[doc = "Field `MCU_M4FSS0_LBIST_SPARE0_LBIST_SELFTEST_EN_PROXY` reader - 0:0\\]
LBIST isolation control"]
pub type McuM4fss0LbistSpare0LbistSelftestEnProxyR = crate::BitReader;
#[doc = "Field `MCU_M4FSS0_LBIST_SPARE0_LBIST_SELFTEST_EN_PROXY` writer - 0:0\\]
LBIST isolation control"]
pub type McuM4fss0LbistSpare0LbistSelftestEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_M4FSS0_LBIST_SPARE0_PBIST_SELFTEST_EN_PROXY` reader - 1:1\\]
PBIST isolation control"]
pub type McuM4fss0LbistSpare0PbistSelftestEnProxyR = crate::BitReader;
#[doc = "Field `MCU_M4FSS0_LBIST_SPARE0_PBIST_SELFTEST_EN_PROXY` writer - 1:1\\]
PBIST isolation control"]
pub type McuM4fss0LbistSpare0PbistSelftestEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_M4FSS0_LBIST_SPARE0_SPARE0_PROXY` reader - 31:2\\]
LBIST spare bits"]
pub type McuM4fss0LbistSpare0Spare0ProxyR = crate::FieldReader<u32>;
#[doc = "Field `MCU_M4FSS0_LBIST_SPARE0_SPARE0_PROXY` writer - 31:2\\]
LBIST spare bits"]
pub type McuM4fss0LbistSpare0Spare0ProxyW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
LBIST isolation control"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_spare0_lbist_selftest_en_proxy(
        &self,
    ) -> McuM4fss0LbistSpare0LbistSelftestEnProxyR {
        McuM4fss0LbistSpare0LbistSelftestEnProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
PBIST isolation control"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_spare0_pbist_selftest_en_proxy(
        &self,
    ) -> McuM4fss0LbistSpare0PbistSelftestEnProxyR {
        McuM4fss0LbistSpare0PbistSelftestEnProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
LBIST spare bits"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_spare0_spare0_proxy(&self) -> McuM4fss0LbistSpare0Spare0ProxyR {
        McuM4fss0LbistSpare0Spare0ProxyR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
LBIST isolation control"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_spare0_lbist_selftest_en_proxy(
        &mut self,
    ) -> McuM4fss0LbistSpare0LbistSelftestEnProxyW<Cfg0McuM4fss0LbistSpare0ProxySpec> {
        McuM4fss0LbistSpare0LbistSelftestEnProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
PBIST isolation control"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_spare0_pbist_selftest_en_proxy(
        &mut self,
    ) -> McuM4fss0LbistSpare0PbistSelftestEnProxyW<Cfg0McuM4fss0LbistSpare0ProxySpec> {
        McuM4fss0LbistSpare0PbistSelftestEnProxyW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
LBIST spare bits"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_spare0_spare0_proxy(
        &mut self,
    ) -> McuM4fss0LbistSpare0Spare0ProxyW<Cfg0McuM4fss0LbistSpare0ProxySpec> {
        McuM4fss0LbistSpare0Spare0ProxyW::new(self, 2)
    }
}
#[doc = "CFG0_MCU_M4FSS0_LBIST_SPARE0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_spare0_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_spare0_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuM4fss0LbistSpare0ProxySpec;
impl crate::RegisterSpec for Cfg0McuM4fss0LbistSpare0ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_m4fss0_lbist_spare0_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0McuM4fss0LbistSpare0ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_m4fss0_lbist_spare0_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0McuM4fss0LbistSpare0ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_M4FSS0_LBIST_SPARE0_PROXY to value 0"]
impl crate::Resettable for Cfg0McuM4fss0LbistSpare0ProxySpec {
    const RESET_VALUE: u32 = 0;
}
