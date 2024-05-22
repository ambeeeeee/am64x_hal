#[doc = "Register `CFG0_EPWM_TB_CLKEN_CLR` reader"]
pub type R = crate::R<Cfg0EpwmTbClkenClrSpec>;
#[doc = "Register `CFG0_EPWM_TB_CLKEN_CLR` writer"]
pub type W = crate::W<Cfg0EpwmTbClkenClrSpec>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM0_TB_CLKEN` reader - 0:0\\]
Writing One Deactivates Timebase Clock of EPWM0"]
pub type EpwmTbClkenClrEpwm0TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM0_TB_CLKEN` writer - 0:0\\]
Writing One Deactivates Timebase Clock of EPWM0"]
pub type EpwmTbClkenClrEpwm0TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM1_TB_CLKEN` reader - 1:1\\]
Writing One Deactivates Timebase Clock of EPWM1"]
pub type EpwmTbClkenClrEpwm1TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM1_TB_CLKEN` writer - 1:1\\]
Writing One Deactivates Timebase Clock of EPWM1"]
pub type EpwmTbClkenClrEpwm1TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM2_TB_CLKEN` reader - 2:2\\]
Writing One Deactivates Timebase Clock of EPWM2"]
pub type EpwmTbClkenClrEpwm2TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM2_TB_CLKEN` writer - 2:2\\]
Writing One Deactivates Timebase Clock of EPWM2"]
pub type EpwmTbClkenClrEpwm2TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM3_TB_CLKEN` reader - 3:3\\]
Writing One Deactivates Timebase Clock of EPWM3"]
pub type EpwmTbClkenClrEpwm3TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM3_TB_CLKEN` writer - 3:3\\]
Writing One Deactivates Timebase Clock of EPWM3"]
pub type EpwmTbClkenClrEpwm3TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM4_TB_CLKEN` reader - 4:4\\]
Writing One Deactivates Timebase Clock of EPWM4"]
pub type EpwmTbClkenClrEpwm4TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM4_TB_CLKEN` writer - 4:4\\]
Writing One Deactivates Timebase Clock of EPWM4"]
pub type EpwmTbClkenClrEpwm4TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM5_TB_CLKEN` reader - 5:5\\]
Writing One Deactivates Timebase Clock of EPWM5"]
pub type EpwmTbClkenClrEpwm5TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM5_TB_CLKEN` writer - 5:5\\]
Writing One Deactivates Timebase Clock of EPWM5"]
pub type EpwmTbClkenClrEpwm5TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM6_TB_CLKEN` reader - 6:6\\]
Writing One Deactivates Timebase Clock of EPWM6"]
pub type EpwmTbClkenClrEpwm6TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM6_TB_CLKEN` writer - 6:6\\]
Writing One Deactivates Timebase Clock of EPWM6"]
pub type EpwmTbClkenClrEpwm6TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM7_TB_CLKEN` reader - 7:7\\]
Writing One Deactivates Timebase Clock of EPWM7"]
pub type EpwmTbClkenClrEpwm7TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM7_TB_CLKEN` writer - 7:7\\]
Writing One Deactivates Timebase Clock of EPWM7"]
pub type EpwmTbClkenClrEpwm7TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM8_TB_CLKEN` reader - 8:8\\]
Writing One Deactivates Timebase Clock of EPWM8"]
pub type EpwmTbClkenClrEpwm8TbClkenR = crate::BitReader;
#[doc = "Field `EPWM_TB_CLKEN_CLR_EPWM8_TB_CLKEN` writer - 8:8\\]
Writing One Deactivates Timebase Clock of EPWM8"]
pub type EpwmTbClkenClrEpwm8TbClkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing One Deactivates Timebase Clock of EPWM0"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm0_tb_clken(&self) -> EpwmTbClkenClrEpwm0TbClkenR {
        EpwmTbClkenClrEpwm0TbClkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing One Deactivates Timebase Clock of EPWM1"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm1_tb_clken(&self) -> EpwmTbClkenClrEpwm1TbClkenR {
        EpwmTbClkenClrEpwm1TbClkenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing One Deactivates Timebase Clock of EPWM2"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm2_tb_clken(&self) -> EpwmTbClkenClrEpwm2TbClkenR {
        EpwmTbClkenClrEpwm2TbClkenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing One Deactivates Timebase Clock of EPWM3"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm3_tb_clken(&self) -> EpwmTbClkenClrEpwm3TbClkenR {
        EpwmTbClkenClrEpwm3TbClkenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing One Deactivates Timebase Clock of EPWM4"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm4_tb_clken(&self) -> EpwmTbClkenClrEpwm4TbClkenR {
        EpwmTbClkenClrEpwm4TbClkenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing One Deactivates Timebase Clock of EPWM5"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm5_tb_clken(&self) -> EpwmTbClkenClrEpwm5TbClkenR {
        EpwmTbClkenClrEpwm5TbClkenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing One Deactivates Timebase Clock of EPWM6"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm6_tb_clken(&self) -> EpwmTbClkenClrEpwm6TbClkenR {
        EpwmTbClkenClrEpwm6TbClkenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing One Deactivates Timebase Clock of EPWM7"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm7_tb_clken(&self) -> EpwmTbClkenClrEpwm7TbClkenR {
        EpwmTbClkenClrEpwm7TbClkenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing One Deactivates Timebase Clock of EPWM8"]
    #[inline(always)]
    pub fn epwm_tb_clken_clr_epwm8_tb_clken(&self) -> EpwmTbClkenClrEpwm8TbClkenR {
        EpwmTbClkenClrEpwm8TbClkenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing One Deactivates Timebase Clock of EPWM0"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm0_tb_clken(
        &mut self,
    ) -> EpwmTbClkenClrEpwm0TbClkenW<Cfg0EpwmTbClkenClrSpec> {
        EpwmTbClkenClrEpwm0TbClkenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing One Deactivates Timebase Clock of EPWM1"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm1_tb_clken(
        &mut self,
    ) -> EpwmTbClkenClrEpwm1TbClkenW<Cfg0EpwmTbClkenClrSpec> {
        EpwmTbClkenClrEpwm1TbClkenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing One Deactivates Timebase Clock of EPWM2"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm2_tb_clken(
        &mut self,
    ) -> EpwmTbClkenClrEpwm2TbClkenW<Cfg0EpwmTbClkenClrSpec> {
        EpwmTbClkenClrEpwm2TbClkenW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing One Deactivates Timebase Clock of EPWM3"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm3_tb_clken(
        &mut self,
    ) -> EpwmTbClkenClrEpwm3TbClkenW<Cfg0EpwmTbClkenClrSpec> {
        EpwmTbClkenClrEpwm3TbClkenW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing One Deactivates Timebase Clock of EPWM4"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm4_tb_clken(
        &mut self,
    ) -> EpwmTbClkenClrEpwm4TbClkenW<Cfg0EpwmTbClkenClrSpec> {
        EpwmTbClkenClrEpwm4TbClkenW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing One Deactivates Timebase Clock of EPWM5"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm5_tb_clken(
        &mut self,
    ) -> EpwmTbClkenClrEpwm5TbClkenW<Cfg0EpwmTbClkenClrSpec> {
        EpwmTbClkenClrEpwm5TbClkenW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing One Deactivates Timebase Clock of EPWM6"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm6_tb_clken(
        &mut self,
    ) -> EpwmTbClkenClrEpwm6TbClkenW<Cfg0EpwmTbClkenClrSpec> {
        EpwmTbClkenClrEpwm6TbClkenW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing One Deactivates Timebase Clock of EPWM7"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm7_tb_clken(
        &mut self,
    ) -> EpwmTbClkenClrEpwm7TbClkenW<Cfg0EpwmTbClkenClrSpec> {
        EpwmTbClkenClrEpwm7TbClkenW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing One Deactivates Timebase Clock of EPWM8"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_tb_clken_clr_epwm8_tb_clken(
        &mut self,
    ) -> EpwmTbClkenClrEpwm8TbClkenW<Cfg0EpwmTbClkenClrSpec> {
        EpwmTbClkenClrEpwm8TbClkenW::new(self, 8)
    }
}
#[doc = "CFG0_EPWM_TB_CLKEN_CLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm_tb_clken_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm_tb_clken_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0EpwmTbClkenClrSpec;
impl crate::RegisterSpec for Cfg0EpwmTbClkenClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_epwm_tb_clken_clr::R`](R) reader structure"]
impl crate::Readable for Cfg0EpwmTbClkenClrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_epwm_tb_clken_clr::W`](W) writer structure"]
impl crate::Writable for Cfg0EpwmTbClkenClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EPWM_TB_CLKEN_CLR to value 0"]
impl crate::Resettable for Cfg0EpwmTbClkenClrSpec {
    const RESET_VALUE: u32 = 0;
}
