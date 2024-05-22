#[doc = "Register `CFG0_EPWM_TB_CLKEN_CLR_PROXY` reader"]
pub type R = crate::R<Cfg0EpwmTbClkenClrProxySpec>;
#[doc = "Register `CFG0_EPWM_TB_CLKEN_CLR_PROXY` writer"]
pub type W = crate::W<Cfg0EpwmTbClkenClrProxySpec>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM0_TB_CLKEN_PROXY` reader - 0:0\\]
Writing One Deactivates Timebase Clock of EPWM0"]
pub type EpwmTbClkenClrEpwm0TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM0_TB_CLKEN_PROXY` writer - 0:0\\]
Writing One Deactivates Timebase Clock of EPWM0"]
pub type EpwmTbClkenClrEpwm0TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM1_TB_CLKEN_PROXY` reader - 1:1\\]
Writing One Deactivates Timebase Clock of EPWM1"]
pub type EpwmTbClkenClrEpwm1TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM1_TB_CLKEN_PROXY` writer - 1:1\\]
Writing One Deactivates Timebase Clock of EPWM1"]
pub type EpwmTbClkenClrEpwm1TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM2_TB_CLKEN_PROXY` reader - 2:2\\]
Writing One Deactivates Timebase Clock of EPWM2"]
pub type EpwmTbClkenClrEpwm2TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM2_TB_CLKEN_PROXY` writer - 2:2\\]
Writing One Deactivates Timebase Clock of EPWM2"]
pub type EpwmTbClkenClrEpwm2TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM3_TB_CLKEN_PROXY` reader - 3:3\\]
Writing One Deactivates Timebase Clock of EPWM3"]
pub type EpwmTbClkenClrEpwm3TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM3_TB_CLKEN_PROXY` writer - 3:3\\]
Writing One Deactivates Timebase Clock of EPWM3"]
pub type EpwmTbClkenClrEpwm3TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM4_TB_CLKEN_PROXY` reader - 4:4\\]
Writing One Deactivates Timebase Clock of EPWM4"]
pub type EpwmTbClkenClrEpwm4TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM4_TB_CLKEN_PROXY` writer - 4:4\\]
Writing One Deactivates Timebase Clock of EPWM4"]
pub type EpwmTbClkenClrEpwm4TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM5_TB_CLKEN_PROXY` reader - 5:5\\]
Writing One Deactivates Timebase Clock of EPWM5"]
pub type EpwmTbClkenClrEpwm5TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM5_TB_CLKEN_PROXY` writer - 5:5\\]
Writing One Deactivates Timebase Clock of EPWM5"]
pub type EpwmTbClkenClrEpwm5TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM6_TB_CLKEN_PROXY` reader - 6:6\\]
Writing One Deactivates Timebase Clock of EPWM6"]
pub type EpwmTbClkenClrEpwm6TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM6_TB_CLKEN_PROXY` writer - 6:6\\]
Writing One Deactivates Timebase Clock of EPWM6"]
pub type EpwmTbClkenClrEpwm6TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM7_TB_CLKEN_PROXY` reader - 7:7\\]
Writing One Deactivates Timebase Clock of EPWM7"]
pub type EpwmTbClkenClrEpwm7TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM7_TB_CLKEN_PROXY` writer - 7:7\\]
Writing One Deactivates Timebase Clock of EPWM7"]
pub type EpwmTbClkenClrEpwm7TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM8_TB_CLKEN_PROXY` reader - 8:8\\]
Writing One Deactivates Timebase Clock of EPWM8"]
pub type EpwmTbClkenClrEpwm8TbClkenProxyR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM8_TB_CLKEN_PROXY` writer - 8:8\\]
Writing One Deactivates Timebase Clock of EPWM8"]
pub type EpwmTbClkenClrEpwm8TbClkenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing One Deactivates Timebase Clock of EPWM0"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm0_tb_clken_proxy(&self) -> EpwmTbClkenClrEpwm0TbClkenProxyR {
        EpwmTbClkenClrEpwm0TbClkenProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing One Deactivates Timebase Clock of EPWM1"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm1_tb_clken_proxy(&self) -> EpwmTbClkenClrEpwm1TbClkenProxyR {
        EpwmTbClkenClrEpwm1TbClkenProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing One Deactivates Timebase Clock of EPWM2"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm2_tb_clken_proxy(&self) -> EpwmTbClkenClrEpwm2TbClkenProxyR {
        EpwmTbClkenClrEpwm2TbClkenProxyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing One Deactivates Timebase Clock of EPWM3"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm3_tb_clken_proxy(&self) -> EpwmTbClkenClrEpwm3TbClkenProxyR {
        EpwmTbClkenClrEpwm3TbClkenProxyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing One Deactivates Timebase Clock of EPWM4"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm4_tb_clken_proxy(&self) -> EpwmTbClkenClrEpwm4TbClkenProxyR {
        EpwmTbClkenClrEpwm4TbClkenProxyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing One Deactivates Timebase Clock of EPWM5"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm5_tb_clken_proxy(&self) -> EpwmTbClkenClrEpwm5TbClkenProxyR {
        EpwmTbClkenClrEpwm5TbClkenProxyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing One Deactivates Timebase Clock of EPWM6"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm6_tb_clken_proxy(&self) -> EpwmTbClkenClrEpwm6TbClkenProxyR {
        EpwmTbClkenClrEpwm6TbClkenProxyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing One Deactivates Timebase Clock of EPWM7"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm7_tb_clken_proxy(&self) -> EpwmTbClkenClrEpwm7TbClkenProxyR {
        EpwmTbClkenClrEpwm7TbClkenProxyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing One Deactivates Timebase Clock of EPWM8"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm8_tb_clken_proxy(&self) -> EpwmTbClkenClrEpwm8TbClkenProxyR {
        EpwmTbClkenClrEpwm8TbClkenProxyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing One Deactivates Timebase Clock of EPWM0"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm0_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenClrEpwm0TbClkenProxyW<Cfg0EpwmTbClkenClrProxySpec> {
        EpwmTbClkenClrEpwm0TbClkenProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing One Deactivates Timebase Clock of EPWM1"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm1_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenClrEpwm1TbClkenProxyW<Cfg0EpwmTbClkenClrProxySpec> {
        EpwmTbClkenClrEpwm1TbClkenProxyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing One Deactivates Timebase Clock of EPWM2"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm2_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenClrEpwm2TbClkenProxyW<Cfg0EpwmTbClkenClrProxySpec> {
        EpwmTbClkenClrEpwm2TbClkenProxyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing One Deactivates Timebase Clock of EPWM3"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm3_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenClrEpwm3TbClkenProxyW<Cfg0EpwmTbClkenClrProxySpec> {
        EpwmTbClkenClrEpwm3TbClkenProxyW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing One Deactivates Timebase Clock of EPWM4"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm4_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenClrEpwm4TbClkenProxyW<Cfg0EpwmTbClkenClrProxySpec> {
        EpwmTbClkenClrEpwm4TbClkenProxyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing One Deactivates Timebase Clock of EPWM5"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm5_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenClrEpwm5TbClkenProxyW<Cfg0EpwmTbClkenClrProxySpec> {
        EpwmTbClkenClrEpwm5TbClkenProxyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing One Deactivates Timebase Clock of EPWM6"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm6_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenClrEpwm6TbClkenProxyW<Cfg0EpwmTbClkenClrProxySpec> {
        EpwmTbClkenClrEpwm6TbClkenProxyW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing One Deactivates Timebase Clock of EPWM7"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm7_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenClrEpwm7TbClkenProxyW<Cfg0EpwmTbClkenClrProxySpec> {
        EpwmTbClkenClrEpwm7TbClkenProxyW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing One Deactivates Timebase Clock of EPWM8"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm8_tb_clken_proxy(
        &mut self,
    ) -> EpwmTbClkenClrEpwm8TbClkenProxyW<Cfg0EpwmTbClkenClrProxySpec> {
        EpwmTbClkenClrEpwm8TbClkenProxyW::new(self, 8)
    }
}
#[doc = "CFG0_EPWM_TB_CLKEN_CLR_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm_tb_clken_clr_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm_tb_clken_clr_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0EpwmTbClkenClrProxySpec;
impl crate::RegisterSpec for Cfg0EpwmTbClkenClrProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_epwm_tb_clken_clr_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0EpwmTbClkenClrProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_epwm_tb_clken_clr_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0EpwmTbClkenClrProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EPWM_TB_CLKEN_CLR_PROXY to value 0"]
impl crate::Resettable for Cfg0EpwmTbClkenClrProxySpec {
    const RESET_VALUE: u32 = 0;
}
