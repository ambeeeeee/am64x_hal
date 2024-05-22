#[doc = "Register `REGS_ded_enable_set_reg0` reader"]
pub type R = crate::R<RegsDedEnableSetReg0Spec>;
#[doc = "Register `REGS_ded_enable_set_reg0` writer"]
pub type W = crate::W<RegsDedEnableSetReg0Spec>;
#[doc = "Field `TPCF0_RAMECC_ENABLE_SET` reader - 0:0\\]
Interrupt Enable Set Register for tpcf0_ramecc_pend"]
pub type Tpcf0RameccEnableSetR = crate::BitReader;
#[doc = "Field `TPCF0_RAMECC_ENABLE_SET` writer - 0:0\\]
Interrupt Enable Set Register for tpcf0_ramecc_pend"]
pub type Tpcf0RameccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPCF1_RAMECC_ENABLE_SET` reader - 1:1\\]
Interrupt Enable Set Register for tpcf1_ramecc_pend"]
pub type Tpcf1RameccEnableSetR = crate::BitReader;
#[doc = "Field `TPCF1_RAMECC_ENABLE_SET` writer - 1:1\\]
Interrupt Enable Set Register for tpcf1_ramecc_pend"]
pub type Tpcf1RameccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPCF0_RAMECC_ENABLE_SET` reader - 2:2\\]
Interrupt Enable Set Register for rpcf0_ramecc_pend"]
pub type Rpcf0RameccEnableSetR = crate::BitReader;
#[doc = "Field `RPCF0_RAMECC_ENABLE_SET` writer - 2:2\\]
Interrupt Enable Set Register for rpcf0_ramecc_pend"]
pub type Rpcf0RameccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPCF1_RAMECC_ENABLE_SET` reader - 3:3\\]
Interrupt Enable Set Register for rpcf1_ramecc_pend"]
pub type Rpcf1RameccEnableSetR = crate::BitReader;
#[doc = "Field `RPCF1_RAMECC_ENABLE_SET` writer - 3:3\\]
Interrupt Enable Set Register for rpcf1_ramecc_pend"]
pub type Rpcf1RameccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for tpcf0_ramecc_pend"]
    #[inline(always)]
    pub fn tpcf0_ramecc_enable_set(&self) -> Tpcf0RameccEnableSetR {
        Tpcf0RameccEnableSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for tpcf1_ramecc_pend"]
    #[inline(always)]
    pub fn tpcf1_ramecc_enable_set(&self) -> Tpcf1RameccEnableSetR {
        Tpcf1RameccEnableSetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Set Register for rpcf0_ramecc_pend"]
    #[inline(always)]
    pub fn rpcf0_ramecc_enable_set(&self) -> Rpcf0RameccEnableSetR {
        Rpcf0RameccEnableSetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Set Register for rpcf1_ramecc_pend"]
    #[inline(always)]
    pub fn rpcf1_ramecc_enable_set(&self) -> Rpcf1RameccEnableSetR {
        Rpcf1RameccEnableSetR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for tpcf0_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn tpcf0_ramecc_enable_set(&mut self) -> Tpcf0RameccEnableSetW<RegsDedEnableSetReg0Spec> {
        Tpcf0RameccEnableSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for tpcf1_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn tpcf1_ramecc_enable_set(&mut self) -> Tpcf1RameccEnableSetW<RegsDedEnableSetReg0Spec> {
        Tpcf1RameccEnableSetW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Set Register for rpcf0_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn rpcf0_ramecc_enable_set(&mut self) -> Rpcf0RameccEnableSetW<RegsDedEnableSetReg0Spec> {
        Rpcf0RameccEnableSetW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Set Register for rpcf1_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn rpcf1_ramecc_enable_set(&mut self) -> Rpcf1RameccEnableSetW<RegsDedEnableSetReg0Spec> {
        Rpcf1RameccEnableSetW::new(self, 3)
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
