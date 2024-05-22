#[doc = "Register `PR1_IEP1__SLV__REGS_digio_ctrl_reg` reader"]
pub type R = crate::R<Pr1Iep1_Slv_RegsDigioCtrlRegSpec>;
#[doc = "Register `PR1_IEP1__SLV__REGS_digio_ctrl_reg` writer"]
pub type W = crate::W<Pr1Iep1_Slv_RegsDigioCtrlRegSpec>;
#[doc = "Field `OUTVALID_POL` reader - "]
pub type OutvalidPolR = crate::BitReader;
#[doc = "Field `OUTVALID_POL` writer - "]
pub type OutvalidPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTVALID_MODE` reader - "]
pub type OutvalidModeR = crate::BitReader;
#[doc = "Field `OUTVALID_MODE` writer - "]
pub type OutvalidModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIDI_MODE` reader - "]
pub type BidiModeR = crate::BitReader;
#[doc = "Field `BIDI_MODE` writer - "]
pub type BidiModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD_MODE` reader - "]
pub type WdModeR = crate::BitReader;
#[doc = "Field `WD_MODE` writer - "]
pub type WdModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_MODE` reader - "]
pub type InModeR = crate::FieldReader;
#[doc = "Field `IN_MODE` writer - "]
pub type InModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUT_MODE` reader - "]
pub type OutModeR = crate::FieldReader;
#[doc = "Field `OUT_MODE` writer - "]
pub type OutModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn outvalid_pol(&self) -> OutvalidPolR {
        OutvalidPolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn outvalid_mode(&self) -> OutvalidModeR {
        OutvalidModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bidi_mode(&self) -> BidiModeR {
        BidiModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wd_mode(&self) -> WdModeR {
        WdModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn in_mode(&self) -> InModeR {
        InModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn out_mode(&self) -> OutModeR {
        OutModeR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn outvalid_pol(&mut self) -> OutvalidPolW<Pr1Iep1_Slv_RegsDigioCtrlRegSpec> {
        OutvalidPolW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn outvalid_mode(&mut self) -> OutvalidModeW<Pr1Iep1_Slv_RegsDigioCtrlRegSpec> {
        OutvalidModeW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn bidi_mode(&mut self) -> BidiModeW<Pr1Iep1_Slv_RegsDigioCtrlRegSpec> {
        BidiModeW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn wd_mode(&mut self) -> WdModeW<Pr1Iep1_Slv_RegsDigioCtrlRegSpec> {
        WdModeW::new(self, 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn in_mode(&mut self) -> InModeW<Pr1Iep1_Slv_RegsDigioCtrlRegSpec> {
        InModeW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn out_mode(&mut self) -> OutModeW<Pr1Iep1_Slv_RegsDigioCtrlRegSpec> {
        OutModeW::new(self, 6)
    }
}
#[doc = "PR1_IEP1__SLV__REGS_digio_ctrl_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_digio_ctrl_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_digio_ctrl_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep1_Slv_RegsDigioCtrlRegSpec;
impl crate::RegisterSpec for Pr1Iep1_Slv_RegsDigioCtrlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep1__slv__regs_digio_ctrl_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep1_Slv_RegsDigioCtrlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep1__slv__regs_digio_ctrl_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep1_Slv_RegsDigioCtrlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP1__SLV__REGS_digio_ctrl_reg to value 0x04"]
impl crate::Resettable for Pr1Iep1_Slv_RegsDigioCtrlRegSpec {
    const RESET_VALUE: u32 = 0x04;
}
