#[doc = "Register `CFG0_EPWM_TB_CLKEN_SET` reader"]
pub type R = crate::R<Cfg0EpwmTbClkenSetSpec>;
#[doc = "Register `CFG0_EPWM_TB_CLKEN_SET` writer"]
pub type W = crate::W<Cfg0EpwmTbClkenSetSpec>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM0_TB_CLKEN` reader - 0:0\\]
Writing One Activates Timebase Clock of EPWM0"]
pub type EpwmTbClkenSetEpwm0TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM0_TB_CLKEN` writer - 0:0\\]
Writing One Activates Timebase Clock of EPWM0"]
pub type EpwmTbClkenSetEpwm0TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM1_TB_CLKEN` reader - 1:1\\]
Writing One Activates Timebase Clock of EPWM1"]
pub type EpwmTbClkenSetEpwm1TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM1_TB_CLKEN` writer - 1:1\\]
Writing One Activates Timebase Clock of EPWM1"]
pub type EpwmTbClkenSetEpwm1TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM2_TB_CLKEN` reader - 2:2\\]
Writing One Activates Timebase Clock of EPWM2"]
pub type EpwmTbClkenSetEpwm2TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM2_TB_CLKEN` writer - 2:2\\]
Writing One Activates Timebase Clock of EPWM2"]
pub type EpwmTbClkenSetEpwm2TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM3_TB_CLKEN` reader - 3:3\\]
Writing One Activates Timebase Clock of EPWM3"]
pub type EpwmTbClkenSetEpwm3TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM3_TB_CLKEN` writer - 3:3\\]
Writing One Activates Timebase Clock of EPWM3"]
pub type EpwmTbClkenSetEpwm3TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM4_TB_CLKEN` reader - 4:4\\]
Writing One Activates Timebase Clock of EPWM4"]
pub type EpwmTbClkenSetEpwm4TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM4_TB_CLKEN` writer - 4:4\\]
Writing One Activates Timebase Clock of EPWM4"]
pub type EpwmTbClkenSetEpwm4TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM5_TB_CLKEN` reader - 5:5\\]
Writing One Activates Timebase Clock of EPWM5"]
pub type EpwmTbClkenSetEpwm5TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM5_TB_CLKEN` writer - 5:5\\]
Writing One Activates Timebase Clock of EPWM5"]
pub type EpwmTbClkenSetEpwm5TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM6_TB_CLKEN` reader - 6:6\\]
Writing One Activates Timebase Clock of EPWM6"]
pub type EpwmTbClkenSetEpwm6TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM6_TB_CLKEN` writer - 6:6\\]
Writing One Activates Timebase Clock of EPWM6"]
pub type EpwmTbClkenSetEpwm6TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM7_TB_CLKEN` reader - 7:7\\]
Writing One Activates Timebase Clock of EPWM7"]
pub type EpwmTbClkenSetEpwm7TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM7_TB_CLKEN` writer - 7:7\\]
Writing One Activates Timebase Clock of EPWM7"]
pub type EpwmTbClkenSetEpwm7TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM8_TB_CLKEN` reader - 8:8\\]
Writing One Activates Timebase Clock of EPWM8"]
pub type EpwmTbClkenSetEpwm8TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_SET_EPWM8_TB_CLKEN` writer - 8:8\\]
Writing One Activates Timebase Clock of EPWM8"]
pub type EpwmTbClkenSetEpwm8TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing One Activates Timebase Clock of EPWM0"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm0_tb_clken(&self) -> EpwmTbClkenSetEpwm0TbClkenR {
        EpwmTbClkenSetEpwm0TbClkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing One Activates Timebase Clock of EPWM1"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm1_tb_clken(&self) -> EpwmTbClkenSetEpwm1TbClkenR {
        EpwmTbClkenSetEpwm1TbClkenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing One Activates Timebase Clock of EPWM2"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm2_tb_clken(&self) -> EpwmTbClkenSetEpwm2TbClkenR {
        EpwmTbClkenSetEpwm2TbClkenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing One Activates Timebase Clock of EPWM3"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm3_tb_clken(&self) -> EpwmTbClkenSetEpwm3TbClkenR {
        EpwmTbClkenSetEpwm3TbClkenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing One Activates Timebase Clock of EPWM4"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm4_tb_clken(&self) -> EpwmTbClkenSetEpwm4TbClkenR {
        EpwmTbClkenSetEpwm4TbClkenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing One Activates Timebase Clock of EPWM5"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm5_tb_clken(&self) -> EpwmTbClkenSetEpwm5TbClkenR {
        EpwmTbClkenSetEpwm5TbClkenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing One Activates Timebase Clock of EPWM6"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm6_tb_clken(&self) -> EpwmTbClkenSetEpwm6TbClkenR {
        EpwmTbClkenSetEpwm6TbClkenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing One Activates Timebase Clock of EPWM7"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm7_tb_clken(&self) -> EpwmTbClkenSetEpwm7TbClkenR {
        EpwmTbClkenSetEpwm7TbClkenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing One Activates Timebase Clock of EPWM8"]
    #[inline(always)]
    pub fn epwm_tb_clken_set_epwm8_tb_clken(&self) -> EpwmTbClkenSetEpwm8TbClkenR {
        EpwmTbClkenSetEpwm8TbClkenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing One Activates Timebase Clock of EPWM0"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm0_tb_clken(
        &mut self,
    ) -> EpwmTbClkenSetEpwm0TbClkenW<Cfg0EpwmTbClkenSetSpec> {
        EpwmTbClkenSetEpwm0TbClkenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing One Activates Timebase Clock of EPWM1"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm1_tb_clken(
        &mut self,
    ) -> EpwmTbClkenSetEpwm1TbClkenW<Cfg0EpwmTbClkenSetSpec> {
        EpwmTbClkenSetEpwm1TbClkenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing One Activates Timebase Clock of EPWM2"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm2_tb_clken(
        &mut self,
    ) -> EpwmTbClkenSetEpwm2TbClkenW<Cfg0EpwmTbClkenSetSpec> {
        EpwmTbClkenSetEpwm2TbClkenW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing One Activates Timebase Clock of EPWM3"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm3_tb_clken(
        &mut self,
    ) -> EpwmTbClkenSetEpwm3TbClkenW<Cfg0EpwmTbClkenSetSpec> {
        EpwmTbClkenSetEpwm3TbClkenW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing One Activates Timebase Clock of EPWM4"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm4_tb_clken(
        &mut self,
    ) -> EpwmTbClkenSetEpwm4TbClkenW<Cfg0EpwmTbClkenSetSpec> {
        EpwmTbClkenSetEpwm4TbClkenW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing One Activates Timebase Clock of EPWM5"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm5_tb_clken(
        &mut self,
    ) -> EpwmTbClkenSetEpwm5TbClkenW<Cfg0EpwmTbClkenSetSpec> {
        EpwmTbClkenSetEpwm5TbClkenW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing One Activates Timebase Clock of EPWM6"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm6_tb_clken(
        &mut self,
    ) -> EpwmTbClkenSetEpwm6TbClkenW<Cfg0EpwmTbClkenSetSpec> {
        EpwmTbClkenSetEpwm6TbClkenW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing One Activates Timebase Clock of EPWM7"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm7_tb_clken(
        &mut self,
    ) -> EpwmTbClkenSetEpwm7TbClkenW<Cfg0EpwmTbClkenSetSpec> {
        EpwmTbClkenSetEpwm7TbClkenW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing One Activates Timebase Clock of EPWM8"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_set_epwm8_tb_clken(
        &mut self,
    ) -> EpwmTbClkenSetEpwm8TbClkenW<Cfg0EpwmTbClkenSetSpec> {
        EpwmTbClkenSetEpwm8TbClkenW::new(self, 8)
    }
}
#[doc = "CFG0_EPWM_TB_CLKEN_SET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm_tb_clken_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm_tb_clken_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0EpwmTbClkenSetSpec;
impl crate::RegisterSpec for Cfg0EpwmTbClkenSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_epwm_tb_clken_set::R`](R) reader structure"]
impl crate::Readable for Cfg0EpwmTbClkenSetSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_epwm_tb_clken_set::W`](W) writer structure"]
impl crate::Writable for Cfg0EpwmTbClkenSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EPWM_TB_CLKEN_SET to value 0"]
impl crate::Resettable for Cfg0EpwmTbClkenSetSpec {
    const RESET_VALUE: u32 = 0;
}
