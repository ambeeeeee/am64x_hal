#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_ipg1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_ipg1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg1Spec>;
#[doc = "Field `TX_IPG1` reader - 15:0\\]
Transmit IPG"]
pub type TxIpg1R = crate::FieldReader<u16>;
#[doc = "Field `TX_IPG1` writer - 15:0\\]
Transmit IPG"]
pub type TxIpg1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Transmit IPG"]
    #[inline(always)]
    pub fn tx_ipg1(&self) -> TxIpg1R {
        TxIpg1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Transmit IPG"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ipg1(&mut self) -> TxIpg1W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg1Spec> {
        TxIpg1W::new(self, 0)
    }
}
#[doc = "MIITXIPG1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_ipg1 to value 0x40"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg1Spec {
    const RESET_VALUE: u32 = 0x40;
}
