#[doc = "Register `PR1_IEP1__SLV__REGS_wd_exp_cnt_reg` reader"]
pub type R = crate::R<Pr1Iep1_Slv_RegsWdExpCntRegSpec>;
#[doc = "Register `PR1_IEP1__SLV__REGS_wd_exp_cnt_reg` writer"]
pub type W = crate::W<Pr1Iep1_Slv_RegsWdExpCntRegSpec>;
#[doc = "Field `PDI_EXP_CNT` reader - "]
pub type PdiExpCntR = crate::FieldReader;
#[doc = "Field `PDI_EXP_CNT` writer - "]
pub type PdiExpCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PD_EXP_CNT` reader - "]
pub type PdExpCntR = crate::FieldReader;
#[doc = "Field `PD_EXP_CNT` writer - "]
pub type PdExpCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pdi_exp_cnt(&self) -> PdiExpCntR {
        PdiExpCntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pd_exp_cnt(&self) -> PdExpCntR {
        PdExpCntR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pdi_exp_cnt(&mut self) -> PdiExpCntW<Pr1Iep1_Slv_RegsWdExpCntRegSpec> {
        PdiExpCntW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn pd_exp_cnt(&mut self) -> PdExpCntW<Pr1Iep1_Slv_RegsWdExpCntRegSpec> {
        PdExpCntW::new(self, 8)
    }
}
#[doc = "PR1_IEP1__SLV__REGS_wd_exp_cnt_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_wd_exp_cnt_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_wd_exp_cnt_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep1_Slv_RegsWdExpCntRegSpec;
impl crate::RegisterSpec for Pr1Iep1_Slv_RegsWdExpCntRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep1__slv__regs_wd_exp_cnt_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep1_Slv_RegsWdExpCntRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep1__slv__regs_wd_exp_cnt_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep1_Slv_RegsWdExpCntRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP1__SLV__REGS_wd_exp_cnt_reg to value 0"]
impl crate::Resettable for Pr1Iep1_Slv_RegsWdExpCntRegSpec {
    const RESET_VALUE: u32 = 0;
}
