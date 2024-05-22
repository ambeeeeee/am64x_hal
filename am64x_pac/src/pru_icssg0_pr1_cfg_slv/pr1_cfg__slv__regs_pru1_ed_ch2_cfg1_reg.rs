#[doc = "Register `PR1_CFG__SLV__REGS_pru1_ed_ch2_cfg1_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1EdCh2Cfg1RegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_ed_ch2_cfg1_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1EdCh2Cfg1RegSpec>;
#[doc = "Field `PRU1_ED_TST_DELAY_COUNTER2` reader - "]
pub type Pru1EdTstDelayCounter2R = crate::FieldReader<u16>;
#[doc = "Field `PRU1_ED_TST_DELAY_COUNTER2` writer - "]
pub type Pru1EdTstDelayCounter2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PRU1_ED_RX_EN_COUNTER2` reader - "]
pub type Pru1EdRxEnCounter2R = crate::FieldReader<u16>;
#[doc = "Field `PRU1_ED_RX_EN_COUNTER2` writer - "]
pub type Pru1EdRxEnCounter2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pru1_ed_tst_delay_counter2(&self) -> Pru1EdTstDelayCounter2R {
        Pru1EdTstDelayCounter2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn pru1_ed_rx_en_counter2(&self) -> Pru1EdRxEnCounter2R {
        Pru1EdRxEnCounter2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_tst_delay_counter2(
        &mut self,
    ) -> Pru1EdTstDelayCounter2W<Pr1Cfg_Slv_RegsPru1EdCh2Cfg1RegSpec> {
        Pru1EdTstDelayCounter2W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_rx_en_counter2(
        &mut self,
    ) -> Pru1EdRxEnCounter2W<Pr1Cfg_Slv_RegsPru1EdCh2Cfg1RegSpec> {
        Pru1EdRxEnCounter2W::new(self, 16)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch2_cfg1_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_ed_ch2_cfg1_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_ed_ch2_cfg1_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1EdCh2Cfg1RegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1EdCh2Cfg1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_ed_ch2_cfg1_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1EdCh2Cfg1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_ed_ch2_cfg1_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1EdCh2Cfg1RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_ed_ch2_cfg1_reg to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1EdCh2Cfg1RegSpec {
    const RESET_VALUE: u32 = 0;
}
