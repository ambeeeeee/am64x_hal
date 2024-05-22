#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_queue23` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueue23Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_queue23` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueue23Spec>;
#[doc = "Field `QUEUE_H_PTR23` reader - 15:0\\]
Queue 23"]
pub type QueueHPtr23R = crate::FieldReader<u16>;
#[doc = "Field `QUEUE_H_PTR23` writer - 15:0\\]
Queue 23"]
pub type QueueHPtr23W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Queue 23"]
    #[inline(always)]
    pub fn queue_h_ptr23(&self) -> QueueHPtr23R {
        QueueHPtr23R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Queue 23"]
    #[inline(always)]
    #[must_use]
    pub fn queue_h_ptr23(&mut self) -> QueueHPtr23W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueue23Spec> {
        QueueHPtr23W::new(self, 0)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_queue23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_queue23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_queue23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueue23Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueue23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_queue23::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueue23Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_queue23::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueue23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_queue23 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGQueue23Spec {
    const RESET_VALUE: u32 = 0;
}