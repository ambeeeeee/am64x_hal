#[doc = "Register `PR1_CFG__SLV__REGS_iepclk_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsIepclkRegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_iepclk_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsIepclkRegSpec>;
#[doc = "Field `IEP_OCP_CLK_EN` reader - "]
pub type IepOcpClkEnR = crate::BitReader;
#[doc = "Field `IEP_OCP_CLK_EN` writer - "]
pub type IepOcpClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEP1_SLV_EN` reader - "]
pub type Iep1SlvEnR = crate::BitReader;
#[doc = "Field `IEP1_SLV_EN` writer - "]
pub type Iep1SlvEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn iep_ocp_clk_en(&self) -> IepOcpClkEnR {
        IepOcpClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn iep1_slv_en(&self) -> Iep1SlvEnR {
        Iep1SlvEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iep_ocp_clk_en(&mut self) -> IepOcpClkEnW<Pr1Cfg_Slv_RegsIepclkRegSpec> {
        IepOcpClkEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iep1_slv_en(&mut self) -> Iep1SlvEnW<Pr1Cfg_Slv_RegsIepclkRegSpec> {
        Iep1SlvEnW::new(self, 1)
    }
}
#[doc = "PR1_CFG__SLV__REGS_iepclk_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_iepclk_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_iepclk_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsIepclkRegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsIepclkRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_iepclk_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsIepclkRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_iepclk_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsIepclkRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_iepclk_reg to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsIepclkRegSpec {
    const RESET_VALUE: u32 = 0;
}
