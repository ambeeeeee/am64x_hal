#[doc = "Register `BORG_ECC_AGGR__CFG__REGS_sec_enable_clr_reg0` reader"]
pub type R = crate::R<BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec>;
#[doc = "Register `BORG_ECC_AGGR__CFG__REGS_sec_enable_clr_reg0` writer"]
pub type W = crate::W<BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec>;
#[doc = "Field `PR1_DRAM0_ENABLE_CLR` reader - 0:0\\]
Interrupt Enable Clear Register for pr1_dram0_pend"]
pub type Pr1Dram0EnableClrR = crate::BitReader;
#[doc = "Field `PR1_DRAM0_ENABLE_CLR` writer - 0:0\\]
Interrupt Enable Clear Register for pr1_dram0_pend"]
pub type Pr1Dram0EnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_DRAM1_ENABLE_CLR` reader - 1:1\\]
Interrupt Enable Clear Register for pr1_dram1_pend"]
pub type Pr1Dram1EnableClrR = crate::BitReader;
#[doc = "Field `PR1_DRAM1_ENABLE_CLR` writer - 1:1\\]
Interrupt Enable Clear Register for pr1_dram1_pend"]
pub type Pr1Dram1EnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_PDSP0_IRAM_ENABLE_CLR` reader - 2:2\\]
Interrupt Enable Clear Register for pr1_pdsp0_iram_pend"]
pub type Pr1Pdsp0IramEnableClrR = crate::BitReader;
#[doc = "Field `PR1_PDSP0_IRAM_ENABLE_CLR` writer - 2:2\\]
Interrupt Enable Clear Register for pr1_pdsp0_iram_pend"]
pub type Pr1Pdsp0IramEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_PDSP1_IRAM_ENABLE_CLR` reader - 3:3\\]
Interrupt Enable Clear Register for pr1_pdsp1_iram_pend"]
pub type Pr1Pdsp1IramEnableClrR = crate::BitReader;
#[doc = "Field `PR1_PDSP1_IRAM_ENABLE_CLR` writer - 3:3\\]
Interrupt Enable Clear Register for pr1_pdsp1_iram_pend"]
pub type Pr1Pdsp1IramEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_RAM_ENABLE_CLR` reader - 4:4\\]
Interrupt Enable Clear Register for pr1_ram_pend"]
pub type Pr1RamEnableClrR = crate::BitReader;
#[doc = "Field `PR1_RAM_ENABLE_CLR` writer - 4:4\\]
Interrupt Enable Clear Register for pr1_ram_pend"]
pub type Pr1RamEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_RTU0_IRAM_ECC_ENABLE_CLR` reader - 5:5\\]
Interrupt Enable Clear Register for pr1_rtu0_iram_ecc_pend"]
pub type Pr1Rtu0IramEccEnableClrR = crate::BitReader;
#[doc = "Field `PR1_RTU0_IRAM_ECC_ENABLE_CLR` writer - 5:5\\]
Interrupt Enable Clear Register for pr1_rtu0_iram_ecc_pend"]
pub type Pr1Rtu0IramEccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_RTU1_IRAM_ECC_ENABLE_CLR` reader - 6:6\\]
Interrupt Enable Clear Register for pr1_rtu1_iram_ecc_pend"]
pub type Pr1Rtu1IramEccEnableClrR = crate::BitReader;
#[doc = "Field `PR1_RTU1_IRAM_ECC_ENABLE_CLR` writer - 6:6\\]
Interrupt Enable Clear Register for pr1_rtu1_iram_ecc_pend"]
pub type Pr1Rtu1IramEccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_PDSP_TX0_IRAM_ENABLE_CLR` reader - 7:7\\]
Interrupt Enable Clear Register for pr1_pdsp_tx0_iram_pend"]
pub type Pr1PdspTx0IramEnableClrR = crate::BitReader;
#[doc = "Field `PR1_PDSP_TX0_IRAM_ENABLE_CLR` writer - 7:7\\]
Interrupt Enable Clear Register for pr1_pdsp_tx0_iram_pend"]
pub type Pr1PdspTx0IramEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_PDSP_TX1_IRAM_ENABLE_CLR` reader - 8:8\\]
Interrupt Enable Clear Register for pr1_pdsp_tx1_iram_pend"]
pub type Pr1PdspTx1IramEnableClrR = crate::BitReader;
#[doc = "Field `PR1_PDSP_TX1_IRAM_ENABLE_CLR` writer - 8:8\\]
Interrupt Enable Clear Register for pr1_pdsp_tx1_iram_pend"]
pub type Pr1PdspTx1IramEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for pr1_dram0_pend"]
    #[inline(always)]
    pub fn pr1_dram0_enable_clr(&self) -> Pr1Dram0EnableClrR {
        Pr1Dram0EnableClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for pr1_dram1_pend"]
    #[inline(always)]
    pub fn pr1_dram1_enable_clr(&self) -> Pr1Dram1EnableClrR {
        Pr1Dram1EnableClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for pr1_pdsp0_iram_pend"]
    #[inline(always)]
    pub fn pr1_pdsp0_iram_enable_clr(&self) -> Pr1Pdsp0IramEnableClrR {
        Pr1Pdsp0IramEnableClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Clear Register for pr1_pdsp1_iram_pend"]
    #[inline(always)]
    pub fn pr1_pdsp1_iram_enable_clr(&self) -> Pr1Pdsp1IramEnableClrR {
        Pr1Pdsp1IramEnableClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Clear Register for pr1_ram_pend"]
    #[inline(always)]
    pub fn pr1_ram_enable_clr(&self) -> Pr1RamEnableClrR {
        Pr1RamEnableClrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Clear Register for pr1_rtu0_iram_ecc_pend"]
    #[inline(always)]
    pub fn pr1_rtu0_iram_ecc_enable_clr(&self) -> Pr1Rtu0IramEccEnableClrR {
        Pr1Rtu0IramEccEnableClrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Clear Register for pr1_rtu1_iram_ecc_pend"]
    #[inline(always)]
    pub fn pr1_rtu1_iram_ecc_enable_clr(&self) -> Pr1Rtu1IramEccEnableClrR {
        Pr1Rtu1IramEccEnableClrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Clear Register for pr1_pdsp_tx0_iram_pend"]
    #[inline(always)]
    pub fn pr1_pdsp_tx0_iram_enable_clr(&self) -> Pr1PdspTx0IramEnableClrR {
        Pr1PdspTx0IramEnableClrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Enable Clear Register for pr1_pdsp_tx1_iram_pend"]
    #[inline(always)]
    pub fn pr1_pdsp_tx1_iram_enable_clr(&self) -> Pr1PdspTx1IramEnableClrR {
        Pr1PdspTx1IramEnableClrR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for pr1_dram0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_dram0_enable_clr(
        &mut self,
    ) -> Pr1Dram0EnableClrW<BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec> {
        Pr1Dram0EnableClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for pr1_dram1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_dram1_enable_clr(
        &mut self,
    ) -> Pr1Dram1EnableClrW<BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec> {
        Pr1Dram1EnableClrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for pr1_pdsp0_iram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_pdsp0_iram_enable_clr(
        &mut self,
    ) -> Pr1Pdsp0IramEnableClrW<BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec> {
        Pr1Pdsp0IramEnableClrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Clear Register for pr1_pdsp1_iram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_pdsp1_iram_enable_clr(
        &mut self,
    ) -> Pr1Pdsp1IramEnableClrW<BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec> {
        Pr1Pdsp1IramEnableClrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Clear Register for pr1_ram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_ram_enable_clr(
        &mut self,
    ) -> Pr1RamEnableClrW<BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec> {
        Pr1RamEnableClrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Clear Register for pr1_rtu0_iram_ecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_rtu0_iram_ecc_enable_clr(
        &mut self,
    ) -> Pr1Rtu0IramEccEnableClrW<BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec> {
        Pr1Rtu0IramEccEnableClrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Clear Register for pr1_rtu1_iram_ecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_rtu1_iram_ecc_enable_clr(
        &mut self,
    ) -> Pr1Rtu1IramEccEnableClrW<BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec> {
        Pr1Rtu1IramEccEnableClrW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Clear Register for pr1_pdsp_tx0_iram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_pdsp_tx0_iram_enable_clr(
        &mut self,
    ) -> Pr1PdspTx0IramEnableClrW<BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec> {
        Pr1PdspTx0IramEnableClrW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Enable Clear Register for pr1_pdsp_tx1_iram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_pdsp_tx1_iram_enable_clr(
        &mut self,
    ) -> Pr1PdspTx1IramEnableClrW<BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec> {
        Pr1PdspTx1IramEnableClrW::new(self, 8)
    }
}
#[doc = "Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`borg_ecc_aggr__cfg__regs_sec_enable_clr_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`borg_ecc_aggr__cfg__regs_sec_enable_clr_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec;
impl crate::RegisterSpec for BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`borg_ecc_aggr__cfg__regs_sec_enable_clr_reg0::R`](R) reader structure"]
impl crate::Readable for BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`borg_ecc_aggr__cfg__regs_sec_enable_clr_reg0::W`](W) writer structure"]
impl crate::Writable for BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BORG_ECC_AGGR__CFG__REGS_sec_enable_clr_reg0 to value 0"]
impl crate::Resettable for BorgEccAggr_Cfg_RegsSecEnableClrReg0Spec {
    const RESET_VALUE: u32 = 0;
}
