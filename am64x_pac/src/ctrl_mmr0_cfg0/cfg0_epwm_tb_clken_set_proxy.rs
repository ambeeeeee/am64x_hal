#[doc = "Register `CFG0_EPWM_TB_CLKEN_SET_PROXY` reader"]
pub type R = crate::R<Cfg0EpwmTbClkenSetProxySpec>;
#[doc = "Register `CFG0_EPWM_TB_CLKEN_SET_PROXY` writer"]
pub type W = crate::W<Cfg0EpwmTbClkenSetProxySpec>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM0_TB_CLKEN_PROXY` reader - 0:0\\]
Writing One Activates Timebase Clock of EPWM0"]
pub type EpwmTbClkenSetEpwm0TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM0_TB_CLKEN_PROXY` writer - 0:0\\]
Writing One Activates Timebase Clock of EPWM0"]
pub type EpwmTbClkenSetEpwm0TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM1_TB_CLKEN_PROXY` reader - 1:1\\]
Writing One Activates Timebase Clock of EPWM1"]
pub type EpwmTbClkenSetEpwm1TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM1_TB_CLKEN_PROXY` writer - 1:1\\]
Writing One Activates Timebase Clock of EPWM1"]
pub type EpwmTbClkenSetEpwm1TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM2_TB_CLKEN_PROXY` reader - 2:2\\]
Writing One Activates Timebase Clock of EPWM2"]
pub type EpwmTbClkenSetEpwm2TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM2_TB_CLKEN_PROXY` writer - 2:2\\]
Writing One Activates Timebase Clock of EPWM2"]
pub type EpwmTbClkenSetEpwm2TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM3_TB_CLKEN_PROXY` reader - 3:3\\]
Writing One Activates Timebase Clock of EPWM3"]
pub type EpwmTbClkenSetEpwm3TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM3_TB_CLKEN_PROXY` writer - 3:3\\]
Writing One Activates Timebase Clock of EPWM3"]
pub type EpwmTbClkenSetEpwm3TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM4_TB_CLKEN_PROXY` reader - 4:4\\]
Writing One Activates Timebase Clock of EPWM4"]
pub type EpwmTbClkenSetEpwm4TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM4_TB_CLKEN_PROXY` writer - 4:4\\]
Writing One Activates Timebase Clock of EPWM4"]
pub type EpwmTbClkenSetEpwm4TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM5_TB_CLKEN_PROXY` reader - 5:5\\]
Writing One Activates Timebase Clock of EPWM5"]
pub type EpwmTbClkenSetEpwm5TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM5_TB_CLKEN_PROXY` writer - 5:5\\]
Writing One Activates Timebase Clock of EPWM5"]
pub type EpwmTbClkenSetEpwm5TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM6_TB_CLKEN_PROXY` reader - 6:6\\]
Writing One Activates Timebase Clock of EPWM6"]
pub type EpwmTbClkenSetEpwm6TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM6_TB_CLKEN_PROXY` writer - 6:6\\]
Writing One Activates Timebase Clock of EPWM6"]
pub type EpwmTbClkenSetEpwm6TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM7_TB_CLKEN_PROXY` reader - 7:7\\]
Writing One Activates Timebase Clock of EPWM7"]
pub type EpwmTbClkenSetEpwm7TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM7_TB_CLKEN_PROXY` writer - 7:7\\]
Writing One Activates Timebase Clock of EPWM7"]
pub type EpwmTbClkenSetEpwm7TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM8_TB_CLKEN_PROXY` reader - 8:8\\]
Writing One Activates Timebase Clock of EPWM8"]
pub type EpwmTbClkenSetEpwm8TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM8_TB_CLKEN_PROXY` writer - 8:8\\]
Writing One Activates Timebase Clock of EPWM8"]
pub type EpwmTbClkenSetEpwm8TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing One Activates Timebase Clock of EPWM0"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm0_tb_clken_proxy(&self) -> EpwmTbClkenSetEpwm0TbClkenProxyR {
        EpwmTbClkenSetEpwm0TbClkenProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing One Activates Timebase Clock of EPWM1"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm1_tb_clken_proxy(&self) -> EpwmTbClkenSetEpwm1TbClkenProxyR {
        EpwmTbClkenSetEpwm1TbClkenProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing One Activates Timebase Clock of EPWM2"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm2_tb_clken_proxy(&self) -> EpwmTbClkenSetEpwm2TbClkenProxyR {
        EpwmTbClkenSetEpwm2TbClkenProxyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing One Activates Timebase Clock of EPWM3"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm3_tb_clken_proxy(&self) -> EpwmTbClkenSetEpwm3TbClkenProxyR {
        EpwmTbClkenSetEpwm3TbClkenProxyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing One Activates Timebase Clock of EPWM4"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm4_tb_clken_proxy(&self) -> EpwmTbClkenSetEpwm4TbClkenProxyR {
        EpwmTbClkenSetEpwm4TbClkenProxyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing One Activates Timebase Clock of EPWM5"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm5_tb_clken_proxy(&self) -> EpwmTbClkenSetEpwm5TbClkenProxyR {
        EpwmTbClkenSetEpwm5TbClkenProxyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing One Activates Timebase Clock of EPWM6"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm6_tb_clken_proxy(&self) -> EpwmTbClkenSetEpwm6TbClkenProxyR {
        EpwmTbClkenSetEpwm6TbClkenProxyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing One Activates Timebase Clock of EPWM7"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm7_tb_clken_proxy(&self) -> EpwmTbClkenSetEpwm7TbClkenProxyR {
        EpwmTbClkenSetEpwm7TbClkenProxyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing One Activates Timebase Clock of EPWM8"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm8_tb_clken_proxy(&self) -> EpwmTbClkenSetEpwm8TbClkenProxyR {
        EpwmTbClkenSetEpwm8TbClkenProxyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing One Activates Timebase Clock of EPWM0"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm0_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenSetEpwm0TbClkenProxyW<Cfg0EpwmTbClkenSetProxySpec> {
        EpwmTbClkenSetEpwm0TbClkenProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing One Activates Timebase Clock of EPWM1"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm1_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenSetEpwm1TbClkenProxyW<Cfg0EpwmTbClkenSetProxySpec> {
        EpwmTbClkenSetEpwm1TbClkenProxyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing One Activates Timebase Clock of EPWM2"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm2_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenSetEpwm2TbClkenProxyW<Cfg0EpwmTbClkenSetProxySpec> {
        EpwmTbClkenSetEpwm2TbClkenProxyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing One Activates Timebase Clock of EPWM3"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm3_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenSetEpwm3TbClkenProxyW<Cfg0EpwmTbClkenSetProxySpec> {
        EpwmTbClkenSetEpwm3TbClkenProxyW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing One Activates Timebase Clock of EPWM4"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm4_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenSetEpwm4TbClkenProxyW<Cfg0EpwmTbClkenSetProxySpec> {
        EpwmTbClkenSetEpwm4TbClkenProxyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing One Activates Timebase Clock of EPWM5"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm5_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenSetEpwm5TbClkenProxyW<Cfg0EpwmTbClkenSetProxySpec> {
        EpwmTbClkenSetEpwm5TbClkenProxyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing One Activates Timebase Clock of EPWM6"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm6_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenSetEpwm6TbClkenProxyW<Cfg0EpwmTbClkenSetProxySpec> {
        EpwmTbClkenSetEpwm6TbClkenProxyW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing One Activates Timebase Clock of EPWM7"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm7_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenSetEpwm7TbClkenProxyW<Cfg0EpwmTbClkenSetProxySpec> {
        EpwmTbClkenSetEpwm7TbClkenProxyW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing One Activates Timebase Clock of EPWM8"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm8_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenSetEpwm8TbClkenProxyW<Cfg0EpwmTbClkenSetProxySpec> {
        EpwmTbClkenSetEpwm8TbClkenProxyW::new(self, 8)
    }
}
#[doc = "CFG0_EPWM_TB_CLKEN_SET_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm_tb_clken_set_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm_tb_clken_set_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0EpwmTbClkenSetProxySpec;
impl crate::RegisterSpec for Cfg0EpwmTbClkenSetProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_epwm_tb_clken_set_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0EpwmTbClkenSetProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_epwm_tb_clken_set_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0EpwmTbClkenSetProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EPWM_TB_CLKEN_SET_PROXY to value 0"]
impl crate::Resettable for Cfg0EpwmTbClkenSetProxySpec {
    const RESET_VALUE: u32 = 0;
}
