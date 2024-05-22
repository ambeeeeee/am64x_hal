#[doc = "Register `BORG_ECC_AGGR__CFG__REGS_ded_enable_set_reg0` reader"]
pub type R = crate::R<BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec>;
#[doc = "Register `BORG_ECC_AGGR__CFG__REGS_ded_enable_set_reg0` writer"]
pub type W = crate::W<BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec>;
#[doc = "Field `PR1_DRAM0_ENABLE_SET` reader - 0:0\\]
Interrupt Enable Set Register for pr1_dram0_pend"]
pub type Pr1Dram0EnableSetR = crate::BitReader;
#[doc = "Field `PR1_DRAM0_ENABLE_SET` writer - 0:0\\]
Interrupt Enable Set Register for pr1_dram0_pend"]
pub type Pr1Dram0EnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_DRAM1_ENABLE_SET` reader - 1:1\\]
Interrupt Enable Set Register for pr1_dram1_pend"]
pub type Pr1Dram1EnableSetR = crate::BitReader;
#[doc = "Field `PR1_DRAM1_ENABLE_SET` writer - 1:1\\]
Interrupt Enable Set Register for pr1_dram1_pend"]
pub type Pr1Dram1EnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_PDSP0_IRAM_ENABLE_SET` reader - 2:2\\]
Interrupt Enable Set Register for pr1_pdsp0_iram_pend"]
pub type Pr1Pdsp0IramEnableSetR = crate::BitReader;
#[doc = "Field `PR1_PDSP0_IRAM_ENABLE_SET` writer - 2:2\\]
Interrupt Enable Set Register for pr1_pdsp0_iram_pend"]
pub type Pr1Pdsp0IramEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_PDSP1_IRAM_ENABLE_SET` reader - 3:3\\]
Interrupt Enable Set Register for pr1_pdsp1_iram_pend"]
pub type Pr1Pdsp1IramEnableSetR = crate::BitReader;
#[doc = "Field `PR1_PDSP1_IRAM_ENABLE_SET` writer - 3:3\\]
Interrupt Enable Set Register for pr1_pdsp1_iram_pend"]
pub type Pr1Pdsp1IramEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_RAM_ENABLE_SET` reader - 4:4\\]
Interrupt Enable Set Register for pr1_ram_pend"]
pub type Pr1RamEnableSetR = crate::BitReader;
#[doc = "Field `PR1_RAM_ENABLE_SET` writer - 4:4\\]
Interrupt Enable Set Register for pr1_ram_pend"]
pub type Pr1RamEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_RTU0_IRAM_ECC_ENABLE_SET` reader - 5:5\\]
Interrupt Enable Set Register for pr1_rtu0_iram_ecc_pend"]
pub type Pr1Rtu0IramEccEnableSetR = crate::BitReader;
#[doc = "Field `PR1_RTU0_IRAM_ECC_ENABLE_SET` writer - 5:5\\]
Interrupt Enable Set Register for pr1_rtu0_iram_ecc_pend"]
pub type Pr1Rtu0IramEccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_RTU1_IRAM_ECC_ENABLE_SET` reader - 6:6\\]
Interrupt Enable Set Register for pr1_rtu1_iram_ecc_pend"]
pub type Pr1Rtu1IramEccEnableSetR = crate::BitReader;
#[doc = "Field `PR1_RTU1_IRAM_ECC_ENABLE_SET` writer - 6:6\\]
Interrupt Enable Set Register for pr1_rtu1_iram_ecc_pend"]
pub type Pr1Rtu1IramEccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_PDSP_TX0_IRAM_ENABLE_SET` reader - 7:7\\]
Interrupt Enable Set Register for pr1_pdsp_tx0_iram_pend"]
pub type Pr1PdspTx0IramEnableSetR = crate::BitReader;
#[doc = "Field `PR1_PDSP_TX0_IRAM_ENABLE_SET` writer - 7:7\\]
Interrupt Enable Set Register for pr1_pdsp_tx0_iram_pend"]
pub type Pr1PdspTx0IramEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_PDSP_TX1_IRAM_ENABLE_SET` reader - 8:8\\]
Interrupt Enable Set Register for pr1_pdsp_tx1_iram_pend"]
pub type Pr1PdspTx1IramEnableSetR = crate::BitReader;
#[doc = "Field `PR1_PDSP_TX1_IRAM_ENABLE_SET` writer - 8:8\\]
Interrupt Enable Set Register for pr1_pdsp_tx1_iram_pend"]
pub type Pr1PdspTx1IramEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for pr1_dram0_pend"]
    #[inline(always)]
    pub fn pr1_dram0_enable_set(&self) -> Pr1Dram0EnableSetR {
        Pr1Dram0EnableSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for pr1_dram1_pend"]
    #[inline(always)]
    pub fn pr1_dram1_enable_set(&self) -> Pr1Dram1EnableSetR {
        Pr1Dram1EnableSetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Set Register for pr1_pdsp0_iram_pend"]
    #[inline(always)]
    pub fn pr1_pdsp0_iram_enable_set(&self) -> Pr1Pdsp0IramEnableSetR {
        Pr1Pdsp0IramEnableSetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Set Register for pr1_pdsp1_iram_pend"]
    #[inline(always)]
    pub fn pr1_pdsp1_iram_enable_set(&self) -> Pr1Pdsp1IramEnableSetR {
        Pr1Pdsp1IramEnableSetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Set Register for pr1_ram_pend"]
    #[inline(always)]
    pub fn pr1_ram_enable_set(&self) -> Pr1RamEnableSetR {
        Pr1RamEnableSetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Set Register for pr1_rtu0_iram_ecc_pend"]
    #[inline(always)]
    pub fn pr1_rtu0_iram_ecc_enable_set(&self) -> Pr1Rtu0IramEccEnableSetR {
        Pr1Rtu0IramEccEnableSetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Set Register for pr1_rtu1_iram_ecc_pend"]
    #[inline(always)]
    pub fn pr1_rtu1_iram_ecc_enable_set(&self) -> Pr1Rtu1IramEccEnableSetR {
        Pr1Rtu1IramEccEnableSetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Set Register for pr1_pdsp_tx0_iram_pend"]
    #[inline(always)]
    pub fn pr1_pdsp_tx0_iram_enable_set(&self) -> Pr1PdspTx0IramEnableSetR {
        Pr1PdspTx0IramEnableSetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Enable Set Register for pr1_pdsp_tx1_iram_pend"]
    #[inline(always)]
    pub fn pr1_pdsp_tx1_iram_enable_set(&self) -> Pr1PdspTx1IramEnableSetR {
        Pr1PdspTx1IramEnableSetR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for pr1_dram0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_dram0_enable_set(
        &mut self,
    ) -> Pr1Dram0EnableSetW<BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec> {
        Pr1Dram0EnableSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for pr1_dram1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_dram1_enable_set(
        &mut self,
    ) -> Pr1Dram1EnableSetW<BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec> {
        Pr1Dram1EnableSetW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Set Register for pr1_pdsp0_iram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_pdsp0_iram_enable_set(
        &mut self,
    ) -> Pr1Pdsp0IramEnableSetW<BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec> {
        Pr1Pdsp0IramEnableSetW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Set Register for pr1_pdsp1_iram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_pdsp1_iram_enable_set(
        &mut self,
    ) -> Pr1Pdsp1IramEnableSetW<BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec> {
        Pr1Pdsp1IramEnableSetW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Set Register for pr1_ram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_ram_enable_set(
        &mut self,
    ) -> Pr1RamEnableSetW<BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec> {
        Pr1RamEnableSetW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Set Register for pr1_rtu0_iram_ecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_rtu0_iram_ecc_enable_set(
        &mut self,
    ) -> Pr1Rtu0IramEccEnableSetW<BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec> {
        Pr1Rtu0IramEccEnableSetW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Set Register for pr1_rtu1_iram_ecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_rtu1_iram_ecc_enable_set(
        &mut self,
    ) -> Pr1Rtu1IramEccEnableSetW<BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec> {
        Pr1Rtu1IramEccEnableSetW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Set Register for pr1_pdsp_tx0_iram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_pdsp_tx0_iram_enable_set(
        &mut self,
    ) -> Pr1PdspTx0IramEnableSetW<BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec> {
        Pr1PdspTx0IramEnableSetW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Enable Set Register for pr1_pdsp_tx1_iram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_pdsp_tx1_iram_enable_set(
        &mut self,
    ) -> Pr1PdspTx1IramEnableSetW<BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec> {
        Pr1PdspTx1IramEnableSetW::new(self, 8)
    }
}
#[doc = "Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`borg_ecc_aggr__cfg__regs_ded_enable_set_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`borg_ecc_aggr__cfg__regs_ded_enable_set_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec;
impl crate::RegisterSpec for BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`borg_ecc_aggr__cfg__regs_ded_enable_set_reg0::R`](R) reader structure"]
impl crate::Readable for BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`borg_ecc_aggr__cfg__regs_ded_enable_set_reg0::W`](W) writer structure"]
impl crate::Writable for BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BORG_ECC_AGGR__CFG__REGS_ded_enable_set_reg0 to value 0"]
impl crate::Resettable for BorgEccAggr_Cfg_RegsDedEnableSetReg0Spec {
    const RESET_VALUE: u32 = 0;
}
