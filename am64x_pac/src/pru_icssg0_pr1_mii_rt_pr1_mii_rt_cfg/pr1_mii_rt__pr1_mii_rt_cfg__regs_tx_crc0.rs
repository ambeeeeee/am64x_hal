#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_crc0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_crc0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc0Spec>;
#[doc = "Field `TX_CRC0` reader - 31:0\\]
Transmit CRC for last packet"]
pub type TxCrc0R = crate::FieldReader<u32>;
#[doc = "Field `TX_CRC0` writer - 31:0\\]
Transmit CRC for last packet"]
pub type TxCrc0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Transmit CRC for last packet"]
    #[inline(always)]
    pub fn tx_crc0(&self) -> TxCrc0R {
        TxCrc0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Transmit CRC for last packet"]
    #[inline(always)]
    #[must_use]
    pub fn tx_crc0(&mut self) -> TxCrc0W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc0Spec> {
        TxCrc0W::new(self, 0)
    }
}
#[doc = "MIITXCRC0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_crc0 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc0Spec {
    const RESET_VALUE: u32 = 0;
}
