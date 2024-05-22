#[doc = "Register `PR1_CFG__SLV__REGS_pid_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPidRegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pid_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPidRegSpec>;
#[doc = "Field `ICSS_IDVER` reader - 31:0\\]
Module ID field"]
pub type IcssIdverR = crate::FieldReader<u32>;
#[doc = "Field `ICSS_IDVER` writer - 31:0\\]
Module ID field"]
pub type IcssIdverW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Module ID field"]
    #[inline(always)]
    pub fn icss_idver(&self) -> IcssIdverR {
        IcssIdverR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Module ID field"]
    #[inline(always)]
    #[must_use]
    pub fn icss_idver(&mut self) -> IcssIdverW<Pr1Cfg_Slv_RegsPidRegSpec> {
        IcssIdverW::new(self, 0)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pid_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pid_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pid_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPidRegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPidRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pid_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPidRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pid_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPidRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pid_reg to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPidRegSpec {
    const RESET_VALUE: u32 = 0;
}
