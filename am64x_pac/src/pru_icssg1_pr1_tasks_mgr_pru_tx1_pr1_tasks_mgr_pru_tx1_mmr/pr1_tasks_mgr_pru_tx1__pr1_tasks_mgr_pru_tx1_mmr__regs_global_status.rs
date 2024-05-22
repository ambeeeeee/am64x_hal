#[doc = "Register `PR1_TASKS_MGR_PRU_TX1__PR1_TASKS_MGR_PRU_TX1_MMR__REGS_global_status` reader"]
pub type R = crate::R<Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec>;
#[doc = "Register `PR1_TASKS_MGR_PRU_TX1__PR1_TASKS_MGR_PRU_TX1_MMR__REGS_global_status` writer"]
pub type W = crate::W<Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec>;
#[doc = "Field `TS1_STATE` reader - 3:0\\]
Task1 State"]
pub type Ts1StateR = crate::FieldReader;
#[doc = "Field `TS1_STATE` writer - 3:0\\]
Task1 State"]
pub type Ts1StateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TS2_STATE` reader - 7:4\\]
Task2 State"]
pub type Ts2StateR = crate::FieldReader;
#[doc = "Field `TS2_STATE` writer - 7:4\\]
Task2 State"]
pub type Ts2StateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TS1_SUB_PEND_0` reader - 8:8\\]
Task1 Sub0 Pend State"]
pub type Ts1SubPend0R = crate::BitReader;
#[doc = "Field `TS1_SUB_PEND_0` writer - 8:8\\]
Task1 Sub0 Pend State"]
pub type Ts1SubPend0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_SUB_PEND_1` reader - 9:9\\]
Task1 Sub1 Pend State"]
pub type Ts1SubPend1R = crate::BitReader;
#[doc = "Field `TS1_SUB_PEND_1` writer - 9:9\\]
Task1 Sub1 Pend State"]
pub type Ts1SubPend1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_SUB_PEND_2` reader - 10:10\\]
Task1 Sub2 Pend State"]
pub type Ts1SubPend2R = crate::BitReader;
#[doc = "Field `TS1_SUB_PEND_2` writer - 10:10\\]
Task1 Sub2 Pend State"]
pub type Ts1SubPend2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_SUB_PEND_3` reader - 11:11\\]
Task1 Sub3 Pend State"]
pub type Ts1SubPend3R = crate::BitReader;
#[doc = "Field `TS1_SUB_PEND_3` writer - 11:11\\]
Task1 Sub3 Pend State"]
pub type Ts1SubPend3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_SUB_PEND_4` reader - 12:12\\]
Task1 Sub4 Pend State"]
pub type Ts1SubPend4R = crate::BitReader;
#[doc = "Field `TS1_SUB_PEND_4` writer - 12:12\\]
Task1 Sub4 Pend State"]
pub type Ts1SubPend4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS2_SUB_PEND_0` reader - 13:13\\]
Task2 Sub0 Pend State"]
pub type Ts2SubPend0R = crate::BitReader;
#[doc = "Field `TS2_SUB_PEND_0` writer - 13:13\\]
Task2 Sub0 Pend State"]
pub type Ts2SubPend0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS2_SUB_PEND_1` reader - 14:14\\]
Task2 Sub1 Pend State"]
pub type Ts2SubPend1R = crate::BitReader;
#[doc = "Field `TS2_SUB_PEND_1` writer - 14:14\\]
Task2 Sub1 Pend State"]
pub type Ts2SubPend1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS2_SUB_PEND_2` reader - 15:15\\]
Task2 Sub2 Pend State"]
pub type Ts2SubPend2R = crate::BitReader;
#[doc = "Field `TS2_SUB_PEND_2` writer - 15:15\\]
Task2 Sub2 Pend State"]
pub type Ts2SubPend2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS2_SUB_PEND_3` reader - 16:16\\]
Task2 Sub3 Pend State"]
pub type Ts2SubPend3R = crate::BitReader;
#[doc = "Field `TS2_SUB_PEND_3` writer - 16:16\\]
Task2 Sub3 Pend State"]
pub type Ts2SubPend3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS2_SUB_PEND_4` reader - 17:17\\]
Task2 Sub4 Pend State"]
pub type Ts2SubPend4R = crate::BitReader;
#[doc = "Field `TS2_SUB_PEND_4` writer - 17:17\\]
Task2 Sub4 Pend State"]
pub type Ts2SubPend4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Task1 State"]
    #[inline(always)]
    pub fn ts1_state(&self) -> Ts1StateR {
        Ts1StateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Task2 State"]
    #[inline(always)]
    pub fn ts2_state(&self) -> Ts2StateR {
        Ts2StateR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Task1 Sub0 Pend State"]
    #[inline(always)]
    pub fn ts1_sub_pend_0(&self) -> Ts1SubPend0R {
        Ts1SubPend0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Task1 Sub1 Pend State"]
    #[inline(always)]
    pub fn ts1_sub_pend_1(&self) -> Ts1SubPend1R {
        Ts1SubPend1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Task1 Sub2 Pend State"]
    #[inline(always)]
    pub fn ts1_sub_pend_2(&self) -> Ts1SubPend2R {
        Ts1SubPend2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Task1 Sub3 Pend State"]
    #[inline(always)]
    pub fn ts1_sub_pend_3(&self) -> Ts1SubPend3R {
        Ts1SubPend3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Task1 Sub4 Pend State"]
    #[inline(always)]
    pub fn ts1_sub_pend_4(&self) -> Ts1SubPend4R {
        Ts1SubPend4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Task2 Sub0 Pend State"]
    #[inline(always)]
    pub fn ts2_sub_pend_0(&self) -> Ts2SubPend0R {
        Ts2SubPend0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Task2 Sub1 Pend State"]
    #[inline(always)]
    pub fn ts2_sub_pend_1(&self) -> Ts2SubPend1R {
        Ts2SubPend1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Task2 Sub2 Pend State"]
    #[inline(always)]
    pub fn ts2_sub_pend_2(&self) -> Ts2SubPend2R {
        Ts2SubPend2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Task2 Sub3 Pend State"]
    #[inline(always)]
    pub fn ts2_sub_pend_3(&self) -> Ts2SubPend3R {
        Ts2SubPend3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Task2 Sub4 Pend State"]
    #[inline(always)]
    pub fn ts2_sub_pend_4(&self) -> Ts2SubPend4R {
        Ts2SubPend4R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Task1 State"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_state(
        &mut self,
    ) -> Ts1StateW<Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec> {
        Ts1StateW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Task2 State"]
    #[inline(always)]
    #[must_use]
    pub fn ts2_state(
        &mut self,
    ) -> Ts2StateW<Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec> {
        Ts2StateW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Task1 Sub0 Pend State"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_sub_pend_0(
        &mut self,
    ) -> Ts1SubPend0W<Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec> {
        Ts1SubPend0W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Task1 Sub1 Pend State"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_sub_pend_1(
        &mut self,
    ) -> Ts1SubPend1W<Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec> {
        Ts1SubPend1W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Task1 Sub2 Pend State"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_sub_pend_2(
        &mut self,
    ) -> Ts1SubPend2W<Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec> {
        Ts1SubPend2W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Task1 Sub3 Pend State"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_sub_pend_3(
        &mut self,
    ) -> Ts1SubPend3W<Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec> {
        Ts1SubPend3W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Task1 Sub4 Pend State"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_sub_pend_4(
        &mut self,
    ) -> Ts1SubPend4W<Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec> {
        Ts1SubPend4W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Task2 Sub0 Pend State"]
    #[inline(always)]
    #[must_use]
    pub fn ts2_sub_pend_0(
        &mut self,
    ) -> Ts2SubPend0W<Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec> {
        Ts2SubPend0W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Task2 Sub1 Pend State"]
    #[inline(always)]
    #[must_use]
    pub fn ts2_sub_pend_1(
        &mut self,
    ) -> Ts2SubPend1W<Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec> {
        Ts2SubPend1W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Task2 Sub2 Pend State"]
    #[inline(always)]
    #[must_use]
    pub fn ts2_sub_pend_2(
        &mut self,
    ) -> Ts2SubPend2W<Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec> {
        Ts2SubPend2W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Task2 Sub3 Pend State"]
    #[inline(always)]
    #[must_use]
    pub fn ts2_sub_pend_3(
        &mut self,
    ) -> Ts2SubPend3W<Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec> {
        Ts2SubPend3W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Task2 Sub4 Pend State"]
    #[inline(always)]
    #[must_use]
    pub fn ts2_sub_pend_4(
        &mut self,
    ) -> Ts2SubPend4W<Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec> {
        Ts2SubPend4W::new(self, 17)
    }
}
#[doc = "Global Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_pru_tx1__pr1_tasks_mgr_pru_tx1_mmr__regs_global_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_pru_tx1__pr1_tasks_mgr_pru_tx1_mmr__regs_global_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec;
impl crate::RegisterSpec for Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_tasks_mgr_pru_tx1__pr1_tasks_mgr_pru_tx1_mmr__regs_global_status::R`](R) reader structure"]
impl crate::Readable for Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_tasks_mgr_pru_tx1__pr1_tasks_mgr_pru_tx1_mmr__regs_global_status::W`](W) writer structure"]
impl crate::Writable for Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_TASKS_MGR_PRU_TX1__PR1_TASKS_MGR_PRU_TX1_MMR__REGS_global_status to value 0"]
impl crate::Resettable for Pr1TasksMgrPruTx1_Pr1TasksMgrPruTx1Mmr_RegsGlobalStatusSpec {
    const RESET_VALUE: u32 = 0;
}
