#[doc = "Register `PR1_IEP0__SLV__REGS_digio_exp_reg` reader"]
pub type R = crate::R<Pr1Iep0_Slv_RegsDigioExpRegSpec>;
#[doc = "Register `PR1_IEP0__SLV__REGS_digio_exp_reg` writer"]
pub type W = crate::W<Pr1Iep0_Slv_RegsDigioExpRegSpec>;
#[doc = "Field `SW_DATA_OUT_UP` reader - "]
pub type SwDataOutUpR = crate::BitReader;
#[doc = "Field `SW_DATA_OUT_UP` writer - "]
pub type SwDataOutUpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTVALID_OVR_EN` reader - "]
pub type OutvalidOvrEnR = crate::BitReader;
#[doc = "Field `OUTVALID_OVR_EN` writer - "]
pub type OutvalidOvrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_OUTVALID` reader - "]
pub type SwOutvalidR = crate::BitReader;
#[doc = "Field `SW_OUTVALID` writer - "]
pub type SwOutvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTVALID_DLY` reader - "]
pub type OutvalidDlyR = crate::FieldReader;
#[doc = "Field `OUTVALID_DLY` writer - "]
pub type OutvalidDlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SOF_DLY` reader - "]
pub type SofDlyR = crate::FieldReader;
#[doc = "Field `SOF_DLY` writer - "]
pub type SofDlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SOF_SEL` reader - "]
pub type SofSelR = crate::BitReader;
#[doc = "Field `SOF_SEL` writer - "]
pub type SofSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOF_SEL` reader - "]
pub type EofSelR = crate::BitReader;
#[doc = "Field `EOF_SEL` writer - "]
pub type EofSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_data_out_up(&self) -> SwDataOutUpR {
        SwDataOutUpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn outvalid_ovr_en(&self) -> OutvalidOvrEnR {
        OutvalidOvrEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sw_outvalid(&self) -> SwOutvalidR {
        SwOutvalidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn outvalid_dly(&self) -> OutvalidDlyR {
        OutvalidDlyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn sof_dly(&self) -> SofDlyR {
        SofDlyR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sof_sel(&self) -> SofSelR {
        SofSelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn eof_sel(&self) -> EofSelR {
        EofSelR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sw_data_out_up(&mut self) -> SwDataOutUpW<Pr1Iep0_Slv_RegsDigioExpRegSpec> {
        SwDataOutUpW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn outvalid_ovr_en(&mut self) -> OutvalidOvrEnW<Pr1Iep0_Slv_RegsDigioExpRegSpec> {
        OutvalidOvrEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sw_outvalid(&mut self) -> SwOutvalidW<Pr1Iep0_Slv_RegsDigioExpRegSpec> {
        SwOutvalidW::new(self, 2)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn outvalid_dly(&mut self) -> OutvalidDlyW<Pr1Iep0_Slv_RegsDigioExpRegSpec> {
        OutvalidDlyW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn sof_dly(&mut self) -> SofDlyW<Pr1Iep0_Slv_RegsDigioExpRegSpec> {
        SofDlyW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn sof_sel(&mut self) -> SofSelW<Pr1Iep0_Slv_RegsDigioExpRegSpec> {
        SofSelW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn eof_sel(&mut self) -> EofSelW<Pr1Iep0_Slv_RegsDigioExpRegSpec> {
        EofSelW::new(self, 13)
    }
}
#[doc = "PR1_IEP0__SLV__REGS_digio_exp_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep0__slv__regs_digio_exp_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep0__slv__regs_digio_exp_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep0_Slv_RegsDigioExpRegSpec;
impl crate::RegisterSpec for Pr1Iep0_Slv_RegsDigioExpRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep0__slv__regs_digio_exp_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep0_Slv_RegsDigioExpRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep0__slv__regs_digio_exp_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep0_Slv_RegsDigioExpRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP0__SLV__REGS_digio_exp_reg to value 0x20"]
impl crate::Resettable for Pr1Iep0_Slv_RegsDigioExpRegSpec {
    const RESET_VALUE: u32 = 0x20;
}
