#[doc = "Register `PR1_TASKS_MGR_PRU_TX0__PR1_TASKS_MGR_PRU_TX0_MMR__REGS_rx_cfg` reader"]
pub type R = crate::R<Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsRxCfgSpec>;
#[doc = "Register `PR1_TASKS_MGR_PRU_TX0__PR1_TASKS_MGR_PRU_TX0_MMR__REGS_rx_cfg` writer"]
pub type W = crate::W<Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsRxCfgSpec>;
#[doc = "Field `BK1_SIZE` reader - 4:0\\]
RX BK1 Size , The first 1 to 32 Bytes trigger"]
pub type Bk1SizeR = crate::FieldReader;
#[doc = "Field `BK1_SIZE` writer - 4:0\\]
RX BK1 Size , The first 1 to 32 Bytes trigger"]
pub type Bk1SizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BK2_SIZE` reader - 9:5\\]
RX BK2 Size, The Second 1 to 32 Bytes trigger"]
pub type Bk2SizeR = crate::FieldReader;
#[doc = "Field `BK2_SIZE` writer - 9:5\\]
RX BK2 Size, The Second 1 to 32 Bytes trigger"]
pub type Bk2SizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BKN_SIZE` reader - 14:10\\]
RX BKN Size, After BK1 and BK@, then after another 1 to 32 Bytes"]
pub type BknSizeR = crate::FieldReader;
#[doc = "Field `BKN_SIZE` writer - 14:10\\]
RX BKN Size, After BK1 and BK@, then after another 1 to 32 Bytes"]
pub type BknSizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
RX BK1 Size , The first 1 to 32 Bytes trigger"]
    #[inline(always)]
    pub fn bk1_size(&self) -> Bk1SizeR {
        Bk1SizeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 9:5\\]
RX BK2 Size, The Second 1 to 32 Bytes trigger"]
    #[inline(always)]
    pub fn bk2_size(&self) -> Bk2SizeR {
        Bk2SizeR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 14:10\\]
RX BKN Size, After BK1 and BK@, then after another 1 to 32 Bytes"]
    #[inline(always)]
    pub fn bkn_size(&self) -> BknSizeR {
        BknSizeR::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
RX BK1 Size , The first 1 to 32 Bytes trigger"]
    #[inline(always)]
    #[must_use]
    pub fn bk1_size(&mut self) -> Bk1SizeW<Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsRxCfgSpec> {
        Bk1SizeW::new(self, 0)
    }
    #[doc = "Bits 5:9 - 9:5\\]
RX BK2 Size, The Second 1 to 32 Bytes trigger"]
    #[inline(always)]
    #[must_use]
    pub fn bk2_size(&mut self) -> Bk2SizeW<Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsRxCfgSpec> {
        Bk2SizeW::new(self, 5)
    }
    #[doc = "Bits 10:14 - 14:10\\]
RX BKN Size, After BK1 and BK@, then after another 1 to 32 Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn bkn_size(&mut self) -> BknSizeW<Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsRxCfgSpec> {
        BknSizeW::new(self, 10)
    }
}
#[doc = "RX Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_pru_tx0__pr1_tasks_mgr_pru_tx0_mmr__regs_rx_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_pru_tx0__pr1_tasks_mgr_pru_tx0_mmr__regs_rx_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsRxCfgSpec;
impl crate::RegisterSpec for Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsRxCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_tasks_mgr_pru_tx0__pr1_tasks_mgr_pru_tx0_mmr__regs_rx_cfg::R`](R) reader structure"]
impl crate::Readable for Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsRxCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_tasks_mgr_pru_tx0__pr1_tasks_mgr_pru_tx0_mmr__regs_rx_cfg::W`](W) writer structure"]
impl crate::Writable for Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsRxCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_TASKS_MGR_PRU_TX0__PR1_TASKS_MGR_PRU_TX0_MMR__REGS_rx_cfg to value 0xc631"]
impl crate::Resettable for Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsRxCfgSpec {
    const RESET_VALUE: u32 = 0xc631;
}
