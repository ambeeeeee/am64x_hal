#[doc = "Register `PR1_CFG__SLV__REGS_spp_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsSppRegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_spp_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsSppRegSpec>;
#[doc = "Field `PRU1_PAD_HP_EN` reader - "]
pub type Pru1PadHpEnR = crate::BitReader;
#[doc = "Field `PRU1_PAD_HP_EN` writer - "]
pub type Pru1PadHpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFR_SHIFT_EN` reader - "]
pub type XfrShiftEnR = crate::BitReader;
#[doc = "Field `XFR_SHIFT_EN` writer - "]
pub type XfrShiftEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFR_BYTE_SHIFT_EN` reader - "]
pub type XfrByteShiftEnR = crate::BitReader;
#[doc = "Field `XFR_BYTE_SHIFT_EN` writer - "]
pub type XfrByteShiftEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTU_XFR_SHIFT_EN` reader - "]
pub type RtuXfrShiftEnR = crate::BitReader;
#[doc = "Field `RTU_XFR_SHIFT_EN` writer - "]
pub type RtuXfrShiftEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pru1_pad_hp_en(&self) -> Pru1PadHpEnR {
        Pru1PadHpEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xfr_shift_en(&self) -> XfrShiftEnR {
        XfrShiftEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xfr_byte_shift_en(&self) -> XfrByteShiftEnR {
        XfrByteShiftEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtu_xfr_shift_en(&self) -> RtuXfrShiftEnR {
        RtuXfrShiftEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_pad_hp_en(&mut self) -> Pru1PadHpEnW<Pr1Cfg_Slv_RegsSppRegSpec> {
        Pru1PadHpEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn xfr_shift_en(&mut self) -> XfrShiftEnW<Pr1Cfg_Slv_RegsSppRegSpec> {
        XfrShiftEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn xfr_byte_shift_en(&mut self) -> XfrByteShiftEnW<Pr1Cfg_Slv_RegsSppRegSpec> {
        XfrByteShiftEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rtu_xfr_shift_en(&mut self) -> RtuXfrShiftEnW<Pr1Cfg_Slv_RegsSppRegSpec> {
        RtuXfrShiftEnW::new(self, 3)
    }
}
#[doc = "PR1_CFG__SLV__REGS_spp_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_spp_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_spp_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsSppRegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsSppRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_spp_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsSppRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_spp_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsSppRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_spp_reg to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsSppRegSpec {
    const RESET_VALUE: u32 = 0;
}
