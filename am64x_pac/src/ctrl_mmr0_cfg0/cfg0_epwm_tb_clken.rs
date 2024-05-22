#[doc = "Register `CFG0_EPWM_TB_CLKEN` reader"]
pub type R = crate::R<Cfg0EpwmTbClkenSpec>;
#[doc = "Register `CFG0_EPWM_TB_CLKEN` writer"]
pub type W = crate::W<Cfg0EpwmTbClkenSpec>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM0_TB_CLKEN` reader - 0:0\\]
Activates Timebase Clock of EPWM0 When Set"]
pub type EpwmTbClkenEpwm0TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM0_TB_CLKEN` writer - 0:0\\]
Activates Timebase Clock of EPWM0 When Set"]
pub type EpwmTbClkenEpwm0TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM1_TB_CLKEN` reader - 1:1\\]
Activates Timebase Clock of EPWM1 When Set"]
pub type EpwmTbClkenEpwm1TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM1_TB_CLKEN` writer - 1:1\\]
Activates Timebase Clock of EPWM1 When Set"]
pub type EpwmTbClkenEpwm1TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM2_TB_CLKEN` reader - 2:2\\]
Activates Timebase Clock of EPWM2 When Set"]
pub type EpwmTbClkenEpwm2TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM2_TB_CLKEN` writer - 2:2\\]
Activates Timebase Clock of EPWM2 When Set"]
pub type EpwmTbClkenEpwm2TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM3_TB_CLKEN` reader - 3:3\\]
Activates Timebase Clock of EPWM3 When Set"]
pub type EpwmTbClkenEpwm3TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM3_TB_CLKEN` writer - 3:3\\]
Activates Timebase Clock of EPWM3 When Set"]
pub type EpwmTbClkenEpwm3TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM4_TB_CLKEN` reader - 4:4\\]
Activates Timebase Clock of EPWM4 When Set"]
pub type EpwmTbClkenEpwm4TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM4_TB_CLKEN` writer - 4:4\\]
Activates Timebase Clock of EPWM4 When Set"]
pub type EpwmTbClkenEpwm4TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM5_TB_CLKEN` reader - 5:5\\]
Activates Timebase Clock of EPWM5 When Set"]
pub type EpwmTbClkenEpwm5TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM5_TB_CLKEN` writer - 5:5\\]
Activates Timebase Clock of EPWM5 When Set"]
pub type EpwmTbClkenEpwm5TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM6_TB_CLKEN` reader - 6:6\\]
Activates Timebase Clock of EPWM6 When Set"]
pub type EpwmTbClkenEpwm6TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM6_TB_CLKEN` writer - 6:6\\]
Activates Timebase Clock of EPWM6 When Set"]
pub type EpwmTbClkenEpwm6TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM7_TB_CLKEN` reader - 7:7\\]
Activates Timebase Clock of EPWM7 When Set"]
pub type EpwmTbClkenEpwm7TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM7_TB_CLKEN` writer - 7:7\\]
Activates Timebase Clock of EPWM7 When Set"]
pub type EpwmTbClkenEpwm7TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_EPWM8_TB_CLKEN` reader - 8:8\\]
Activates Timebase Clock of EPWM8 When Set"]
pub type EpwmTbClkenEpwm8TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_EPWM8_TB_CLKEN` writer - 8:8\\]
Activates Timebase Clock of EPWM8 When Set"]
pub type EpwmTbClkenEpwm8TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Activates Timebase Clock of EPWM0 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm0_tb_clken(&self) -> EpwmTbClkenEpwm0TbClkenR {
        EpwmTbClkenEpwm0TbClkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Activates Timebase Clock of EPWM1 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm1_tb_clken(&self) -> EpwmTbClkenEpwm1TbClkenR {
        EpwmTbClkenEpwm1TbClkenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Activates Timebase Clock of EPWM2 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm2_tb_clken(&self) -> EpwmTbClkenEpwm2TbClkenR {
        EpwmTbClkenEpwm2TbClkenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Activates Timebase Clock of EPWM3 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm3_tb_clken(&self) -> EpwmTbClkenEpwm3TbClkenR {
        EpwmTbClkenEpwm3TbClkenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Activates Timebase Clock of EPWM4 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm4_tb_clken(&self) -> EpwmTbClkenEpwm4TbClkenR {
        EpwmTbClkenEpwm4TbClkenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Activates Timebase Clock of EPWM5 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm5_tb_clken(&self) -> EpwmTbClkenEpwm5TbClkenR {
        EpwmTbClkenEpwm5TbClkenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Activates Timebase Clock of EPWM6 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm6_tb_clken(&self) -> EpwmTbClkenEpwm6TbClkenR {
        EpwmTbClkenEpwm6TbClkenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Activates Timebase Clock of EPWM7 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm7_tb_clken(&self) -> EpwmTbClkenEpwm7TbClkenR {
        EpwmTbClkenEpwm7TbClkenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Activates Timebase Clock of EPWM8 When Set"]
    #[inline(always)]
    pub fn epwm_tb_clken_epwm8_tb_clken(&self) -> EpwmTbClkenEpwm8TbClkenR {
        EpwmTbClkenEpwm8TbClkenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Activates Timebase Clock of EPWM0 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm0_tb_clken(
        &mut self,
    ) -> EpwmTbClkenEpwm0TbClkenW<Cfg0EpwmTbClkenSpec> {
        EpwmTbClkenEpwm0TbClkenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Activates Timebase Clock of EPWM1 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm1_tb_clken(
        &mut self,
    ) -> EpwmTbClkenEpwm1TbClkenW<Cfg0EpwmTbClkenSpec> {
        EpwmTbClkenEpwm1TbClkenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Activates Timebase Clock of EPWM2 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm2_tb_clken(
        &mut self,
    ) -> EpwmTbClkenEpwm2TbClkenW<Cfg0EpwmTbClkenSpec> {
        EpwmTbClkenEpwm2TbClkenW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Activates Timebase Clock of EPWM3 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm3_tb_clken(
        &mut self,
    ) -> EpwmTbClkenEpwm3TbClkenW<Cfg0EpwmTbClkenSpec> {
        EpwmTbClkenEpwm3TbClkenW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Activates Timebase Clock of EPWM4 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm4_tb_clken(
        &mut self,
    ) -> EpwmTbClkenEpwm4TbClkenW<Cfg0EpwmTbClkenSpec> {
        EpwmTbClkenEpwm4TbClkenW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Activates Timebase Clock of EPWM5 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm5_tb_clken(
        &mut self,
    ) -> EpwmTbClkenEpwm5TbClkenW<Cfg0EpwmTbClkenSpec> {
        EpwmTbClkenEpwm5TbClkenW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Activates Timebase Clock of EPWM6 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm6_tb_clken(
        &mut self,
    ) -> EpwmTbClkenEpwm6TbClkenW<Cfg0EpwmTbClkenSpec> {
        EpwmTbClkenEpwm6TbClkenW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Activates Timebase Clock of EPWM7 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm7_tb_clken(
        &mut self,
    ) -> EpwmTbClkenEpwm7TbClkenW<Cfg0EpwmTbClkenSpec> {
        EpwmTbClkenEpwm7TbClkenW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Activates Timebase Clock of EPWM8 When Set"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_epwm8_tb_clken(
        &mut self,
    ) -> EpwmTbClkenEpwm8TbClkenW<Cfg0EpwmTbClkenSpec> {
        EpwmTbClkenEpwm8TbClkenW::new(self, 8)
    }
}
#[doc = "CFG0_EPWM_TB_CLKEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm_tb_clken::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm_tb_clken::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0EpwmTbClkenSpec;
impl crate::RegisterSpec for Cfg0EpwmTbClkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_epwm_tb_clken::R`](R) reader structure"]
impl crate::Readable for Cfg0EpwmTbClkenSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_epwm_tb_clken::W`](W) writer structure"]
impl crate::Writable for Cfg0EpwmTbClkenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EPWM_TB_CLKEN to value 0"]
impl crate::Resettable for Cfg0EpwmTbClkenSpec {
    const RESET_VALUE: u32 = 0;
}
