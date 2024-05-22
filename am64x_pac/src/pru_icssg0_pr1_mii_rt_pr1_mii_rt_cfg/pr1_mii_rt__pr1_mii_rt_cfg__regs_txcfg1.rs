#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_txcfg1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_txcfg1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec>;
#[doc = "Field `TX_ENABLE1` reader - "]
pub type TxEnable1R = crate::BitReader;
#[doc = "Field `TX_ENABLE1` writer - "]
pub type TxEnable1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_AUTO_PREAMBLE1` reader - "]
pub type TxAutoPreamble1R = crate::BitReader;
#[doc = "Field `TX_AUTO_PREAMBLE1` writer - "]
pub type TxAutoPreamble1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EN_MODE1` reader - "]
pub type TxEnMode1R = crate::BitReader;
#[doc = "Field `TX_EN_MODE1` writer - "]
pub type TxEnMode1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_BYTE_SWAP1` reader - "]
pub type TxByteSwap1R = crate::BitReader;
#[doc = "Field `TX_BYTE_SWAP1` writer - "]
pub type TxByteSwap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_MUX_SEL1` reader - "]
pub type TxMuxSel1R = crate::BitReader;
#[doc = "Field `TX_MUX_SEL1` writer - "]
pub type TxMuxSel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE_TX_AUTO_SEQUENCE1` reader - "]
pub type PreTxAutoSequence1R = crate::BitReader;
#[doc = "Field `PRE_TX_AUTO_SEQUENCE1` writer - "]
pub type PreTxAutoSequence1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE_TX_AUTO_ESC_ERR1` reader - "]
pub type PreTxAutoEscErr1R = crate::BitReader;
#[doc = "Field `PRE_TX_AUTO_ESC_ERR1` writer - "]
pub type PreTxAutoEscErr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_32_MODE_EN1` reader - "]
pub type Tx32ModeEn1R = crate::BitReader;
#[doc = "Field `TX_32_MODE_EN1` writer - "]
pub type Tx32ModeEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_IPG_WIRE_CLK_EN1` reader - "]
pub type TxIpgWireClkEn1R = crate::BitReader;
#[doc = "Field `TX_IPG_WIRE_CLK_EN1` writer - "]
pub type TxIpgWireClkEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_START_DELAY1` reader - "]
pub type TxStartDelay1R = crate::FieldReader<u16>;
#[doc = "Field `TX_START_DELAY1` writer - "]
pub type TxStartDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TX_CLK_DELAY1` reader - "]
pub type TxClkDelay1R = crate::FieldReader;
#[doc = "Field `TX_CLK_DELAY1` writer - "]
pub type TxClkDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_enable1(&self) -> TxEnable1R {
        TxEnable1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_auto_preamble1(&self) -> TxAutoPreamble1R {
        TxAutoPreamble1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_en_mode1(&self) -> TxEnMode1R {
        TxEnMode1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_byte_swap1(&self) -> TxByteSwap1R {
        TxByteSwap1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_mux_sel1(&self) -> TxMuxSel1R {
        TxMuxSel1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pre_tx_auto_sequence1(&self) -> PreTxAutoSequence1R {
        PreTxAutoSequence1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pre_tx_auto_esc_err1(&self) -> PreTxAutoEscErr1R {
        PreTxAutoEscErr1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tx_32_mode_en1(&self) -> Tx32ModeEn1R {
        Tx32ModeEn1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_ipg_wire_clk_en1(&self) -> TxIpgWireClkEn1R {
        TxIpgWireClkEn1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tx_start_delay1(&self) -> TxStartDelay1R {
        TxStartDelay1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn tx_clk_delay1(&self) -> TxClkDelay1R {
        TxClkDelay1R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tx_enable1(&mut self) -> TxEnable1W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec> {
        TxEnable1W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tx_auto_preamble1(&mut self) -> TxAutoPreamble1W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec> {
        TxAutoPreamble1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tx_en_mode1(&mut self) -> TxEnMode1W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec> {
        TxEnMode1W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_swap1(&mut self) -> TxByteSwap1W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec> {
        TxByteSwap1W::new(self, 3)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tx_mux_sel1(&mut self) -> TxMuxSel1W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec> {
        TxMuxSel1W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pre_tx_auto_sequence1(
        &mut self,
    ) -> PreTxAutoSequence1W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec> {
        PreTxAutoSequence1W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pre_tx_auto_esc_err1(
        &mut self,
    ) -> PreTxAutoEscErr1W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec> {
        PreTxAutoEscErr1W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn tx_32_mode_en1(&mut self) -> Tx32ModeEn1W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec> {
        Tx32ModeEn1W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ipg_wire_clk_en1(&mut self) -> TxIpgWireClkEn1W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec> {
        TxIpgWireClkEn1W::new(self, 12)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn tx_start_delay1(&mut self) -> TxStartDelay1W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec> {
        TxStartDelay1W::new(self, 16)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn tx_clk_delay1(&mut self) -> TxClkDelay1W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec> {
        TxClkDelay1W::new(self, 28)
    }
}
#[doc = "MIITXCFG1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_CFG__REGS_txcfg1 to value 0x0064_0000"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec {
    const RESET_VALUE: u32 = 0x0064_0000;
}
