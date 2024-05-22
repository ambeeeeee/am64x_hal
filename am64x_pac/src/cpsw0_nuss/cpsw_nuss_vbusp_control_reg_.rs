#[doc = "Register `CPSW_NUSS_VBUSP_CONTROL_REG_` reader"]
pub type R = crate::R<CpswNussVbuspControlReg_Spec>;
#[doc = "Register `CPSW_NUSS_VBUSP_CONTROL_REG_` writer"]
pub type W = crate::W<CpswNussVbuspControlReg_Spec>;
#[doc = "Field `MR_AN_ENABLE` reader - 0:0\\]
Auto-negotiation enable"]
pub type MrAnEnableR = crate::BitReader;
#[doc = "Field `MR_AN_ENABLE` writer - 0:0\\]
Auto-negotiation enable"]
pub type MrAnEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR_AN_RESTART` reader - 1:1\\]
Auto-negotiation restart"]
pub type MrAnRestartR = crate::BitReader;
#[doc = "Field `MR_AN_RESTART` writer - 1:1\\]
Auto-negotiation restart"]
pub type MrAnRestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAST_LINK_TIMER` reader - 2:2\\]
Fast link timer"]
pub type FastLinkTimerR = crate::BitReader;
#[doc = "Field `FAST_LINK_TIMER` writer - 2:2\\]
Fast link timer"]
pub type FastLinkTimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR_NP_LOADED` reader - 3:3\\]
Next page loaded"]
pub type MrNpLoadedR = crate::BitReader;
#[doc = "Field `MR_NP_LOADED` writer - 3:3\\]
Next page loaded"]
pub type MrNpLoadedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK` reader - 4:4\\]
Loopback mode"]
pub type LoopbackR = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - 4:4\\]
Loopback mode"]
pub type LoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER` reader - 5:5\\]
Master mode"]
pub type MasterR = crate::BitReader;
#[doc = "Field `MASTER` writer - 5:5\\]
Master mode"]
pub type MasterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST_PATTERN_EN` reader - 6:6\\]
Test pattern enable"]
pub type TestPatternEnR = crate::BitReader;
#[doc = "Field `TEST_PATTERN_EN` writer - 6:6\\]
Test pattern enable"]
pub type TestPatternEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Auto-negotiation enable"]
    #[inline(always)]
    pub fn mr_an_enable(&self) -> MrAnEnableR {
        MrAnEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Auto-negotiation restart"]
    #[inline(always)]
    pub fn mr_an_restart(&self) -> MrAnRestartR {
        MrAnRestartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Fast link timer"]
    #[inline(always)]
    pub fn fast_link_timer(&self) -> FastLinkTimerR {
        FastLinkTimerR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Next page loaded"]
    #[inline(always)]
    pub fn mr_np_loaded(&self) -> MrNpLoadedR {
        MrNpLoadedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Loopback mode"]
    #[inline(always)]
    pub fn loopback(&self) -> LoopbackR {
        LoopbackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Master mode"]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Test pattern enable"]
    #[inline(always)]
    pub fn test_pattern_en(&self) -> TestPatternEnR {
        TestPatternEnR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Auto-negotiation enable"]
    #[inline(always)]
    #[must_use]
    pub fn mr_an_enable(&mut self) -> MrAnEnableW<CpswNussVbuspControlReg_Spec> {
        MrAnEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Auto-negotiation restart"]
    #[inline(always)]
    #[must_use]
    pub fn mr_an_restart(&mut self) -> MrAnRestartW<CpswNussVbuspControlReg_Spec> {
        MrAnRestartW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Fast link timer"]
    #[inline(always)]
    #[must_use]
    pub fn fast_link_timer(&mut self) -> FastLinkTimerW<CpswNussVbuspControlReg_Spec> {
        FastLinkTimerW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Next page loaded"]
    #[inline(always)]
    #[must_use]
    pub fn mr_np_loaded(&mut self) -> MrNpLoadedW<CpswNussVbuspControlReg_Spec> {
        MrNpLoadedW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Loopback mode"]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LoopbackW<CpswNussVbuspControlReg_Spec> {
        LoopbackW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Master mode"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MasterW<CpswNussVbuspControlReg_Spec> {
        MasterW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Test pattern enable"]
    #[inline(always)]
    #[must_use]
    pub fn test_pattern_en(&mut self) -> TestPatternEnW<CpswNussVbuspControlReg_Spec> {
        TestPatternEnW::new(self, 6)
    }
}
#[doc = "SGMII Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_control_reg_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_control_reg_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspControlReg_Spec;
impl crate::RegisterSpec for CpswNussVbuspControlReg_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_control_reg_::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspControlReg_Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_control_reg_::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspControlReg_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_CONTROL_REG_ to value 0"]
impl crate::Resettable for CpswNussVbuspControlReg_Spec {
    const RESET_VALUE: u32 = 0;
}
