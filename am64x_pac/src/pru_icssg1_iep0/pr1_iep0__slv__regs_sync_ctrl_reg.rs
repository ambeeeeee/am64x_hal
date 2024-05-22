#[doc = "Register `PR1_IEP0__SLV__REGS_sync_ctrl_reg` reader"]
pub type R = crate::R<Pr1Iep0_Slv_RegsSyncCtrlRegSpec>;
#[doc = "Register `PR1_IEP0__SLV__REGS_sync_ctrl_reg` writer"]
pub type W = crate::W<Pr1Iep0_Slv_RegsSyncCtrlRegSpec>;
#[doc = "Field `SYNC_EN` reader - "]
pub type SyncEnR = crate::BitReader;
#[doc = "Field `SYNC_EN` writer - "]
pub type SyncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC0_EN` reader - "]
pub type Sync0EnR = crate::BitReader;
#[doc = "Field `SYNC0_EN` writer - "]
pub type Sync0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC1_EN` reader - "]
pub type Sync1EnR = crate::BitReader;
#[doc = "Field `SYNC1_EN` writer - "]
pub type Sync1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC0_ACK_EN` reader - "]
pub type Sync0AckEnR = crate::BitReader;
#[doc = "Field `SYNC0_ACK_EN` writer - "]
pub type Sync0AckEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC0_CYCLIC_EN` reader - "]
pub type Sync0CyclicEnR = crate::BitReader;
#[doc = "Field `SYNC0_CYCLIC_EN` writer - "]
pub type Sync0CyclicEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC1_ACK_EN` reader - "]
pub type Sync1AckEnR = crate::BitReader;
#[doc = "Field `SYNC1_ACK_EN` writer - "]
pub type Sync1AckEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC1_CYCLIC_EN` reader - "]
pub type Sync1CyclicEnR = crate::BitReader;
#[doc = "Field `SYNC1_CYCLIC_EN` writer - "]
pub type Sync1CyclicEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC1_IND_EN` reader - "]
pub type Sync1IndEnR = crate::BitReader;
#[doc = "Field `SYNC1_IND_EN` writer - "]
pub type Sync1IndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC0_OUT_NV_EN` reader - "]
pub type Sync0OutNvEnR = crate::BitReader;
#[doc = "Field `SYNC0_OUT_NV_EN` writer - "]
pub type Sync0OutNvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC1_OUT_NV_EN` reader - "]
pub type Sync1OutNvEnR = crate::BitReader;
#[doc = "Field `SYNC1_OUT_NV_EN` writer - "]
pub type Sync1OutNvEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sync_en(&self) -> SyncEnR {
        SyncEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sync0_en(&self) -> Sync0EnR {
        Sync0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sync1_en(&self) -> Sync1EnR {
        Sync1EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sync0_ack_en(&self) -> Sync0AckEnR {
        Sync0AckEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sync0_cyclic_en(&self) -> Sync0CyclicEnR {
        Sync0CyclicEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sync1_ack_en(&self) -> Sync1AckEnR {
        Sync1AckEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sync1_cyclic_en(&self) -> Sync1CyclicEnR {
        Sync1CyclicEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sync1_ind_en(&self) -> Sync1IndEnR {
        Sync1IndEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sync0_out_nv_en(&self) -> Sync0OutNvEnR {
        Sync0OutNvEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sync1_out_nv_en(&self) -> Sync1OutNvEnR {
        Sync1OutNvEnR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sync_en(&mut self) -> SyncEnW<Pr1Iep0_Slv_RegsSyncCtrlRegSpec> {
        SyncEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sync0_en(&mut self) -> Sync0EnW<Pr1Iep0_Slv_RegsSyncCtrlRegSpec> {
        Sync0EnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sync1_en(&mut self) -> Sync1EnW<Pr1Iep0_Slv_RegsSyncCtrlRegSpec> {
        Sync1EnW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sync0_ack_en(&mut self) -> Sync0AckEnW<Pr1Iep0_Slv_RegsSyncCtrlRegSpec> {
        Sync0AckEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn sync0_cyclic_en(&mut self) -> Sync0CyclicEnW<Pr1Iep0_Slv_RegsSyncCtrlRegSpec> {
        Sync0CyclicEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn sync1_ack_en(&mut self) -> Sync1AckEnW<Pr1Iep0_Slv_RegsSyncCtrlRegSpec> {
        Sync1AckEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn sync1_cyclic_en(&mut self) -> Sync1CyclicEnW<Pr1Iep0_Slv_RegsSyncCtrlRegSpec> {
        Sync1CyclicEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sync1_ind_en(&mut self) -> Sync1IndEnW<Pr1Iep0_Slv_RegsSyncCtrlRegSpec> {
        Sync1IndEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn sync0_out_nv_en(&mut self) -> Sync0OutNvEnW<Pr1Iep0_Slv_RegsSyncCtrlRegSpec> {
        Sync0OutNvEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn sync1_out_nv_en(&mut self) -> Sync1OutNvEnW<Pr1Iep0_Slv_RegsSyncCtrlRegSpec> {
        Sync1OutNvEnW::new(self, 10)
    }
}
#[doc = "PR1_IEP0__SLV__REGS_sync_ctrl_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep0__slv__regs_sync_ctrl_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep0__slv__regs_sync_ctrl_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep0_Slv_RegsSyncCtrlRegSpec;
impl crate::RegisterSpec for Pr1Iep0_Slv_RegsSyncCtrlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep0__slv__regs_sync_ctrl_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep0_Slv_RegsSyncCtrlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep0__slv__regs_sync_ctrl_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep0_Slv_RegsSyncCtrlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP0__SLV__REGS_sync_ctrl_reg to value 0"]
impl crate::Resettable for Pr1Iep0_Slv_RegsSyncCtrlRegSpec {
    const RESET_VALUE: u32 = 0;
}
