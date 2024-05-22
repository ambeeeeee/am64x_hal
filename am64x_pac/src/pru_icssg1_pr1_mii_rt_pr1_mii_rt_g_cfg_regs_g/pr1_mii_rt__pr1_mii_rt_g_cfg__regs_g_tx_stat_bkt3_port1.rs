#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_stat_bkt3_port1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatBkt3Port1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_stat_bkt3_port1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatBkt3Port1Spec>;
#[doc = "Field `TX_STAT_BKT3` reader - 15:0\\]
TX Bucket3 Inc if &lt;= than Bucket3 Byte Size and if > than Bucket2 Byte Size"]
pub type TxStatBkt3R = crate::FieldReader<u16>;
#[doc = "Field `TX_STAT_BKT3` writer - 15:0\\]
TX Bucket3 Inc if &lt;= than Bucket3 Byte Size and if > than Bucket2 Byte Size"]
pub type TxStatBkt3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
TX Bucket3 Inc if &lt;= than Bucket3 Byte Size and if > than Bucket2 Byte Size"]
    #[inline(always)]
    pub fn tx_stat_bkt3(&self) -> TxStatBkt3R {
        TxStatBkt3R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
TX Bucket3 Inc if &lt;= than Bucket3 Byte Size and if > than Bucket2 Byte Size"]
    #[inline(always)]
    #[must_use]
    pub fn tx_stat_bkt3(&mut self) -> TxStatBkt3W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatBkt3Port1Spec> {
        TxStatBkt3W::new(self, 0)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_stat_bkt3_port1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_stat_bkt3_port1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_stat_bkt3_port1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatBkt3Port1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatBkt3Port1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_stat_bkt3_port1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatBkt3Port1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_stat_bkt3_port1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatBkt3Port1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_stat_bkt3_port1 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatBkt3Port1Spec {
    const RESET_VALUE: u32 = 0;
}
