#[doc = "Register `REGS_ded_enable_set_reg0` reader"]
pub type R = crate::R<RegsDedEnableSetReg0Spec>;
#[doc = "Register `REGS_ded_enable_set_reg0` writer"]
pub type W = crate::W<RegsDedEnableSetReg0Spec>;
#[doc = "Field `ICB_RAMECC_ENABLE_SET` reader - 0:0\\]
Interrupt Enable Set Register for icb_ramecc_pend"]
pub type IcbRameccEnableSetR = crate::BitReader;
#[doc = "Field `ICB_RAMECC_ENABLE_SET` writer - 0:0\\]
Interrupt Enable Set Register for icb_ramecc_pend"]
pub type IcbRameccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITE_RAMECC_ENABLE_SET` reader - 1:1\\]
Interrupt Enable Set Register for ite_ramecc_pend"]
pub type IteRameccEnableSetR = crate::BitReader;
#[doc = "Field `ITE_RAMECC_ENABLE_SET` writer - 1:1\\]
Interrupt Enable Set Register for ite_ramecc_pend"]
pub type IteRameccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPI_RAMECC_ENABLE_SET` reader - 2:2\\]
Interrupt Enable Set Register for lpi_ramecc_pend"]
pub type LpiRameccEnableSetR = crate::BitReader;
#[doc = "Field `LPI_RAMECC_ENABLE_SET` writer - 2:2\\]
Interrupt Enable Set Register for lpi_ramecc_pend"]
pub type LpiRameccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for icb_ramecc_pend"]
    #[inline(always)]
    pub fn icb_ramecc_enable_set(&self) -> IcbRameccEnableSetR {
        IcbRameccEnableSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for ite_ramecc_pend"]
    #[inline(always)]
    pub fn ite_ramecc_enable_set(&self) -> IteRameccEnableSetR {
        IteRameccEnableSetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Set Register for lpi_ramecc_pend"]
    #[inline(always)]
    pub fn lpi_ramecc_enable_set(&self) -> LpiRameccEnableSetR {
        LpiRameccEnableSetR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for icb_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn icb_ramecc_enable_set(&mut self) -> IcbRameccEnableSetW<RegsDedEnableSetReg0Spec> {
        IcbRameccEnableSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for ite_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn ite_ramecc_enable_set(&mut self) -> IteRameccEnableSetW<RegsDedEnableSetReg0Spec> {
        IteRameccEnableSetW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Set Register for lpi_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_ramecc_enable_set(&mut self) -> LpiRameccEnableSetW<RegsDedEnableSetReg0Spec> {
        LpiRameccEnableSetW::new(self, 2)
    }
}
#[doc = "Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_ded_enable_set_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_ded_enable_set_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsDedEnableSetReg0Spec;
impl crate::RegisterSpec for RegsDedEnableSetReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_ded_enable_set_reg0::R`](R) reader structure"]
impl crate::Readable for RegsDedEnableSetReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`regs_ded_enable_set_reg0::W`](W) writer structure"]
impl crate::Writable for RegsDedEnableSetReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_ded_enable_set_reg0 to value 0"]
impl crate::Resettable for RegsDedEnableSetReg0Spec {
    const RESET_VALUE: u32 = 0;
}
