#[doc = "Register `BORG_ECC_AGGR__CFG__REGS_ded_status_reg0` reader"]
pub type R = crate::R<BorgEccAggr_Cfg_RegsDedStatusReg0Spec>;
#[doc = "Register `BORG_ECC_AGGR__CFG__REGS_ded_status_reg0` writer"]
pub type W = crate::W<BorgEccAggr_Cfg_RegsDedStatusReg0Spec>;
#[doc = "Field `PR1_DRAM0_PEND` reader - 0:0\\]
Interrupt Pending Status for pr1_dram0_pend"]
pub type Pr1Dram0PendR = crate::BitReader;
#[doc = "Field `PR1_DRAM0_PEND` writer - 0:0\\]
Interrupt Pending Status for pr1_dram0_pend"]
pub type Pr1Dram0PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_DRAM1_PEND` reader - 1:1\\]
Interrupt Pending Status for pr1_dram1_pend"]
pub type Pr1Dram1PendR = crate::BitReader;
#[doc = "Field `PR1_DRAM1_PEND` writer - 1:1\\]
Interrupt Pending Status for pr1_dram1_pend"]
pub type Pr1Dram1PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_PDSP0_IRAM_PEND` reader - 2:2\\]
Interrupt Pending Status for pr1_pdsp0_iram_pend"]
pub type Pr1Pdsp0IramPendR = crate::BitReader;
#[doc = "Field `PR1_PDSP0_IRAM_PEND` writer - 2:2\\]
Interrupt Pending Status for pr1_pdsp0_iram_pend"]
pub type Pr1Pdsp0IramPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_PDSP1_IRAM_PEND` reader - 3:3\\]
Interrupt Pending Status for pr1_pdsp1_iram_pend"]
pub type Pr1Pdsp1IramPendR = crate::BitReader;
#[doc = "Field `PR1_PDSP1_IRAM_PEND` writer - 3:3\\]
Interrupt Pending Status for pr1_pdsp1_iram_pend"]
pub type Pr1Pdsp1IramPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_RAM_PEND` reader - 4:4\\]
Interrupt Pending Status for pr1_ram_pend"]
pub type Pr1RamPendR = crate::BitReader;
#[doc = "Field `PR1_RAM_PEND` writer - 4:4\\]
Interrupt Pending Status for pr1_ram_pend"]
pub type Pr1RamPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_RTU0_IRAM_ECC_PEND` reader - 5:5\\]
Interrupt Pending Status for pr1_rtu0_iram_ecc_pend"]
pub type Pr1Rtu0IramEccPendR = crate::BitReader;
#[doc = "Field `PR1_RTU0_IRAM_ECC_PEND` writer - 5:5\\]
Interrupt Pending Status for pr1_rtu0_iram_ecc_pend"]
pub type Pr1Rtu0IramEccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_RTU1_IRAM_ECC_PEND` reader - 6:6\\]
Interrupt Pending Status for pr1_rtu1_iram_ecc_pend"]
pub type Pr1Rtu1IramEccPendR = crate::BitReader;
#[doc = "Field `PR1_RTU1_IRAM_ECC_PEND` writer - 6:6\\]
Interrupt Pending Status for pr1_rtu1_iram_ecc_pend"]
pub type Pr1Rtu1IramEccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_PDSP_TX0_IRAM_PEND` reader - 7:7\\]
Interrupt Pending Status for pr1_pdsp_tx0_iram_pend"]
pub type Pr1PdspTx0IramPendR = crate::BitReader;
#[doc = "Field `PR1_PDSP_TX0_IRAM_PEND` writer - 7:7\\]
Interrupt Pending Status for pr1_pdsp_tx0_iram_pend"]
pub type Pr1PdspTx0IramPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_PDSP_TX1_IRAM_PEND` reader - 8:8\\]
Interrupt Pending Status for pr1_pdsp_tx1_iram_pend"]
pub type Pr1PdspTx1IramPendR = crate::BitReader;
#[doc = "Field `PR1_PDSP_TX1_IRAM_PEND` writer - 8:8\\]
Interrupt Pending Status for pr1_pdsp_tx1_iram_pend"]
pub type Pr1PdspTx1IramPendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for pr1_dram0_pend"]
    #[inline(always)]
    pub fn pr1_dram0_pend(&self) -> Pr1Dram0PendR {
        Pr1Dram0PendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for pr1_dram1_pend"]
    #[inline(always)]
    pub fn pr1_dram1_pend(&self) -> Pr1Dram1PendR {
        Pr1Dram1PendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for pr1_pdsp0_iram_pend"]
    #[inline(always)]
    pub fn pr1_pdsp0_iram_pend(&self) -> Pr1Pdsp0IramPendR {
        Pr1Pdsp0IramPendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for pr1_pdsp1_iram_pend"]
    #[inline(always)]
    pub fn pr1_pdsp1_iram_pend(&self) -> Pr1Pdsp1IramPendR {
        Pr1Pdsp1IramPendR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Pending Status for pr1_ram_pend"]
    #[inline(always)]
    pub fn pr1_ram_pend(&self) -> Pr1RamPendR {
        Pr1RamPendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Pending Status for pr1_rtu0_iram_ecc_pend"]
    #[inline(always)]
    pub fn pr1_rtu0_iram_ecc_pend(&self) -> Pr1Rtu0IramEccPendR {
        Pr1Rtu0IramEccPendR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Pending Status for pr1_rtu1_iram_ecc_pend"]
    #[inline(always)]
    pub fn pr1_rtu1_iram_ecc_pend(&self) -> Pr1Rtu1IramEccPendR {
        Pr1Rtu1IramEccPendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Pending Status for pr1_pdsp_tx0_iram_pend"]
    #[inline(always)]
    pub fn pr1_pdsp_tx0_iram_pend(&self) -> Pr1PdspTx0IramPendR {
        Pr1PdspTx0IramPendR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Pending Status for pr1_pdsp_tx1_iram_pend"]
    #[inline(always)]
    pub fn pr1_pdsp_tx1_iram_pend(&self) -> Pr1PdspTx1IramPendR {
        Pr1PdspTx1IramPendR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for pr1_dram0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_dram0_pend(&mut self) -> Pr1Dram0PendW<BorgEccAggr_Cfg_RegsDedStatusReg0Spec> {
        Pr1Dram0PendW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for pr1_dram1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_dram1_pend(&mut self) -> Pr1Dram1PendW<BorgEccAggr_Cfg_RegsDedStatusReg0Spec> {
        Pr1Dram1PendW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for pr1_pdsp0_iram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_pdsp0_iram_pend(
        &mut self,
    ) -> Pr1Pdsp0IramPendW<BorgEccAggr_Cfg_RegsDedStatusReg0Spec> {
        Pr1Pdsp0IramPendW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for pr1_pdsp1_iram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_pdsp1_iram_pend(
        &mut self,
    ) -> Pr1Pdsp1IramPendW<BorgEccAggr_Cfg_RegsDedStatusReg0Spec> {
        Pr1Pdsp1IramPendW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Pending Status for pr1_ram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_ram_pend(&mut self) -> Pr1RamPendW<BorgEccAggr_Cfg_RegsDedStatusReg0Spec> {
        Pr1RamPendW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Pending Status for pr1_rtu0_iram_ecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_rtu0_iram_ecc_pend(
        &mut self,
    ) -> Pr1Rtu0IramEccPendW<BorgEccAggr_Cfg_RegsDedStatusReg0Spec> {
        Pr1Rtu0IramEccPendW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Pending Status for pr1_rtu1_iram_ecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_rtu1_iram_ecc_pend(
        &mut self,
    ) -> Pr1Rtu1IramEccPendW<BorgEccAggr_Cfg_RegsDedStatusReg0Spec> {
        Pr1Rtu1IramEccPendW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Pending Status for pr1_pdsp_tx0_iram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_pdsp_tx0_iram_pend(
        &mut self,
    ) -> Pr1PdspTx0IramPendW<BorgEccAggr_Cfg_RegsDedStatusReg0Spec> {
        Pr1PdspTx0IramPendW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Pending Status for pr1_pdsp_tx1_iram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_pdsp_tx1_iram_pend(
        &mut self,
    ) -> Pr1PdspTx1IramPendW<BorgEccAggr_Cfg_RegsDedStatusReg0Spec> {
        Pr1PdspTx1IramPendW::new(self, 8)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`borg_ecc_aggr__cfg__regs_ded_status_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`borg_ecc_aggr__cfg__regs_ded_status_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BorgEccAggr_Cfg_RegsDedStatusReg0Spec;
impl crate::RegisterSpec for BorgEccAggr_Cfg_RegsDedStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`borg_ecc_aggr__cfg__regs_ded_status_reg0::R`](R) reader structure"]
impl crate::Readable for BorgEccAggr_Cfg_RegsDedStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`borg_ecc_aggr__cfg__regs_ded_status_reg0::W`](W) writer structure"]
impl crate::Writable for BorgEccAggr_Cfg_RegsDedStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BORG_ECC_AGGR__CFG__REGS_ded_status_reg0 to value 0"]
impl crate::Resettable for BorgEccAggr_Cfg_RegsDedStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}
