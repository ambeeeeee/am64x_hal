#[doc = "Register `CFG0_EPWM_TB_CLKEN_PROXY` reader"]
pub type R = crate::R<Cfg0EpwmTbClkenProxySpec>;
#[doc = "Register `CFG0_EPWM_TB_CLKEN_PROXY` writer"]
pub type W = crate::W<Cfg0EpwmTbClkenProxySpec>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM0_TB_CLKEN_PROXY` reader - 0:0\\]
Activates Timebase Clock of EPWM0 When Set"]
pub type EpwmTbClkenEpwm0TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM0_TB_CLKEN_PROXY` writer - 0:0\\]
Activates Timebase Clock of EPWM0 When Set"]
pub type EpwmTbClkenEpwm0TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM1_TB_CLKEN_PROXY` reader - 1:1\\]
Activates Timebase Clock of EPWM1 When Set"]
pub type EpwmTbClkenEpwm1TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM1_TB_CLKEN_PROXY` writer - 1:1\\]
Activates Timebase Clock of EPWM1 When Set"]
pub type EpwmTbClkenEpwm1TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM2_TB_CLKEN_PROXY` reader - 2:2\\]
Activates Timebase Clock of EPWM2 When Set"]
pub type EpwmTbClkenEpwm2TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM2_TB_CLKEN_PROXY` writer - 2:2\\]
Activates Timebase Clock of EPWM2 When Set"]
pub type EpwmTbClkenEpwm2TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM3_TB_CLKEN_PROXY` reader - 3:3\\]
Activates Timebase Clock of EPWM3 When Set"]
pub type EpwmTbClkenEpwm3TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM3_TB_CLKEN_PROXY` writer - 3:3\\]
Activates Timebase Clock of EPWM3 When Set"]
pub type EpwmTbClkenEpwm3TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM4_TB_CLKEN_PROXY` reader - 4:4\\]
Activates Timebase Clock of EPWM4 When Set"]
pub type EpwmTbClkenEpwm4TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM4_TB_CLKEN_PROXY` writer - 4:4\\]
Activates Timebase Clock of EPWM4 When Set"]
pub type EpwmTbClkenEpwm4TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM5_TB_CLKEN_PROXY` reader - 5:5\\]
Activates Timebase Clock of EPWM5 When Set"]
pub type EpwmTbClkenEpwm5TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM5_TB_CLKEN_PROXY` writer - 5:5\\]
Activates Timebase Clock of EPWM5 When Set"]
pub type EpwmTbClkenEpwm5TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM6_TB_CLKEN_PROXY` reader - 6:6\\]
Activates Timebase Clock of EPWM6 When Set"]
pub type EpwmTbClkenEpwm6TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM6_TB_CLKEN_PROXY` writer - 6:6\\]
Activates Timebase Clock of EPWM6 When Set"]
pub type EpwmTbClkenEpwm6TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM7_TB_CLKEN_PROXY` reader - 7:7\\]
Activates Timebase Clock of EPWM7 When Set"]
pub type EpwmTbClkenEpwm7TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM7_TB_CLKEN_PROXY` writer - 7:7\\]
Activates Timebase Clock of EPWM7 When Set"]
pub type EpwmTbClkenEpwm7TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM8_TB_CLKEN_PROXY` reader - 8:8\\]
Activates Timebase Clock of EPWM8 When Set"]
pub type EpwmTbClkenEpwm8TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM8_TB_CLKEN_PROXY` writer - 8:8\\]
Activates Timebase Clock of EPWM8 When Set"]
pub type EpwmTbClkenEpwm8TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Activates Timebase Clock of EPWM0 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm0_tb_clken_proxy(&self) -> EpwmTbClkenEpwm0TbClkenProxyR {
        EpwmTbClkenEpwm0TbClkenProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Activates Timebase Clock of EPWM1 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm1_tb_clken_proxy(&self) -> EpwmTbClkenEpwm1TbClkenProxyR {
        EpwmTbClkenEpwm1TbClkenProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Activates Timebase Clock of EPWM2 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm2_tb_clken_proxy(&self) -> EpwmTbClkenEpwm2TbClkenProxyR {
        EpwmTbClkenEpwm2TbClkenProxyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Activates Timebase Clock of EPWM3 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm3_tb_clken_proxy(&self) -> EpwmTbClkenEpwm3TbClkenProxyR {
        EpwmTbClkenEpwm3TbClkenProxyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Activates Timebase Clock of EPWM4 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm4_tb_clken_proxy(&self) -> EpwmTbClkenEpwm4TbClkenProxyR {
        EpwmTbClkenEpwm4TbClkenProxyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Activates Timebase Clock of EPWM5 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm5_tb_clken_proxy(&self) -> EpwmTbClkenEpwm5TbClkenProxyR {
        EpwmTbClkenEpwm5TbClkenProxyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Activates Timebase Clock of EPWM6 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm6_tb_clken_proxy(&self) -> EpwmTbClkenEpwm6TbClkenProxyR {
        EpwmTbClkenEpwm6TbClkenProxyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Activates Timebase Clock of EPWM7 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm7_tb_clken_proxy(&self) -> EpwmTbClkenEpwm7TbClkenProxyR {
        EpwmTbClkenEpwm7TbClkenProxyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Activates Timebase Clock of EPWM8 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm8_tb_clken_proxy(&self) -> EpwmTbClkenEpwm8TbClkenProxyR {
        EpwmTbClkenEpwm8TbClkenProxyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Activates Timebase Clock of EPWM0 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm0_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenEpwm0TbClkenProxyW<Cfg0EpwmTbClkenProxySpec> {
        EpwmTbClkenEpwm0TbClkenProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Activates Timebase Clock of EPWM1 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm1_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenEpwm1TbClkenProxyW<Cfg0EpwmTbClkenProxySpec> {
        EpwmTbClkenEpwm1TbClkenProxyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Activates Timebase Clock of EPWM2 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm2_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenEpwm2TbClkenProxyW<Cfg0EpwmTbClkenProxySpec> {
        EpwmTbClkenEpwm2TbClkenProxyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Activates Timebase Clock of EPWM3 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm3_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenEpwm3TbClkenProxyW<Cfg0EpwmTbClkenProxySpec> {
        EpwmTbClkenEpwm3TbClkenProxyW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Activates Timebase Clock of EPWM4 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm4_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenEpwm4TbClkenProxyW<Cfg0EpwmTbClkenProxySpec> {
        EpwmTbClkenEpwm4TbClkenProxyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Activates Timebase Clock of EPWM5 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm5_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenEpwm5TbClkenProxyW<Cfg0EpwmTbClkenProxySpec> {
        EpwmTbClkenEpwm5TbClkenProxyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Activates Timebase Clock of EPWM6 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm6_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenEpwm6TbClkenProxyW<Cfg0EpwmTbClkenProxySpec> {
        EpwmTbClkenEpwm6TbClkenProxyW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Activates Timebase Clock of EPWM7 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm7_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenEpwm7TbClkenProxyW<Cfg0EpwmTbClkenProxySpec> {
        EpwmTbClkenEpwm7TbClkenProxyW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Activates Timebase Clock of EPWM8 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm8_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenEpwm8TbClkenProxyW<Cfg0EpwmTbClkenProxySpec> {
        EpwmTbClkenEpwm8TbClkenProxyW::new(self, 8)
    }
}
#[doc = "CFG0_EPWM_TB_CLKEN_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm_tb_clken_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm_tb_clken_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0EpwmTbClkenProxySpec;
impl crate::RegisterSpec for Cfg0EpwmTbClkenProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_epwm_tb_clken_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0EpwmTbClkenProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_epwm_tb_clken_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0EpwmTbClkenProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EPWM_TB_CLKEN_PROXY to value 0"]
impl crate::Resettable for Cfg0EpwmTbClkenProxySpec {
    const RESET_VALUE: u32 = 0;
}
