#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_queue_cnt41` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt41Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_queue_cnt41` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt41Spec>;
#[doc = "Field `QUEUE_CNT_ENTRIES_41` reader - 15:0\\]
Queue Entry Count41"]
pub type QueueCntEntries41R = crate::FieldReader<u16>;
#[doc = "Field `QUEUE_CNT_ENTRIES_41` writer - 15:0\\]
Queue Entry Count41"]
pub type QueueCntEntries41W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Queue Entry Count41"]
    #[inline(always)]
    pub fn queue_cnt_entries_41(&self) -> QueueCntEntries41R {
        QueueCntEntries41R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Queue Entry Count41"]
    #[inline(always)]
    #[must_use]
    pub fn queue_cnt_entries_41(
        &mut self,
    ) -> QueueCntEntries41W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt41Spec> {
        QueueCntEntries41W::new(self, 0)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_queue_cnt41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_queue_cnt41::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_queue_cnt41::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt41Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_queue_cnt41::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt41Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_queue_cnt41::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt41Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_queue_cnt41 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueueCnt41Spec {
    const RESET_VALUE: u32 = 0;
}
