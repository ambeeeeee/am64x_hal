#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_queue_cnt25` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt25Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_queue_cnt25` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt25Spec>;
#[doc = "Field `QUEUE_CNT_ENTRIES_25` reader - 15:0\\]
Queue Entry Count25"]
pub type QueueCntEntries25R = crate::FieldReader<u16>;
#[doc = "Field `QUEUE_CNT_ENTRIES_25` writer - 15:0\\]
Queue Entry Count25"]
pub type QueueCntEntries25W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Queue Entry Count25"]
    #[inline(always)]
    pub fn queue_cnt_entries_25(&self) -> QueueCntEntries25R {
        QueueCntEntries25R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Queue Entry Count25"]
    #[inline(always)]
    #[must_use]
    pub fn queue_cnt_entries_25(
        &mut self,
    ) -> QueueCntEntries25W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt25Spec> {
        QueueCntEntries25W::new(self, 0)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_queue_cnt25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_queue_cnt25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_queue_cnt25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt25Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_queue_cnt25::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt25Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_queue_cnt25::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_queue_cnt25 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt25Spec {
    const RESET_VALUE: u32 = 0;
}