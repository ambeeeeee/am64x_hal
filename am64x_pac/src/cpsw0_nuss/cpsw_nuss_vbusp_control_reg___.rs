#[doc = "Register `CPSW_NUSS_VBUSP_CONTROL_REG___` reader"]
pub type R = crate::R<CpswNussVbuspControlReg_Spec>;
#[doc = "Register `CPSW_NUSS_VBUSP_CONTROL_REG___` writer"]
pub type W = crate::W<CpswNussVbuspControlReg_Spec>;
#[doc = "Field `S_CN_SWITCH` reader - 0:0\\]
VLAN Aware Mode"]
pub type SCnSwitchR = crate::BitReader;
#[doc = "Field `S_CN_SWITCH` writer - 0:0\\]
VLAN Aware Mode"]
pub type SCnSwitchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLAN_AWARE` reader - 1:1\\]
VLAN Aware Mode"]
pub type VlanAwareR = crate::BitReader;
#[doc = "Field `VLAN_AWARE` writer - 1:1\\]
VLAN Aware Mode"]
pub type VlanAwareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P0_ENABLE` reader - 2:2\\]
Port 0 Enable"]
pub type P0EnableR = crate::BitReader;
#[doc = "Field `P0_ENABLE` writer - 2:2\\]
Port 0 Enable"]
pub type P0EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P0_PASS_PRI_TAGGED` reader - 3:3\\]
Port 0 Pass Priority Tagged"]
pub type P0PassPriTaggedR = crate::BitReader;
#[doc = "Field `P0_PASS_PRI_TAGGED` writer - 3:3\\]
Port 0 Pass Priority Tagged"]
pub type P0PassPriTaggedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1_PASS_PRI_TAGGED` reader - 4:4\\]
Port 1 Pass Priority Tagged"]
pub type P1PassPriTaggedR = crate::BitReader;
#[doc = "Field `P1_PASS_PRI_TAGGED` writer - 4:4\\]
Port 1 Pass Priority Tagged"]
pub type P1PassPriTaggedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_PASS_PRI_TAGGED` reader - 5:5\\]
Port 2 Pass Priority Tagged"]
pub type P2PassPriTaggedR = crate::BitReader;
#[doc = "Field `P2_PASS_PRI_TAGGED` writer - 5:5\\]
Port 2 Pass Priority Tagged"]
pub type P2PassPriTaggedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3_PASS_PRI_TAGGED` reader - 6:6\\]
Port 3 Pass Priority Tagged"]
pub type P3PassPriTaggedR = crate::BitReader;
#[doc = "Field `P3_PASS_PRI_TAGGED` writer - 6:6\\]
Port 3 Pass Priority Tagged"]
pub type P3PassPriTaggedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4_PASS_PRI_TAGGED` reader - 7:7\\]
Port 4 Pass Priority Tagged"]
pub type P4PassPriTaggedR = crate::BitReader;
#[doc = "Field `P4_PASS_PRI_TAGGED` writer - 7:7\\]
Port 4 Pass Priority Tagged"]
pub type P4PassPriTaggedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5_PASS_PRI_TAGGED` reader - 8:8\\]
Port 5 Pass Priority Tagged"]
pub type P5PassPriTaggedR = crate::BitReader;
#[doc = "Field `P5_PASS_PRI_TAGGED` writer - 8:8\\]
Port 5 Pass Priority Tagged"]
pub type P5PassPriTaggedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6_PASS_PRI_TAGGED` reader - 9:9\\]
Port 6 Pass Priority Tagged"]
pub type P6PassPriTaggedR = crate::BitReader;
#[doc = "Field `P6_PASS_PRI_TAGGED` writer - 9:9\\]
Port 6 Pass Priority Tagged"]
pub type P6PassPriTaggedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7_PASS_PRI_TAGGED` reader - 10:10\\]
Port 7 Pass Priority Tagged"]
pub type P7PassPriTaggedR = crate::BitReader;
#[doc = "Field `P7_PASS_PRI_TAGGED` writer - 10:10\\]
Port 7 Pass Priority Tagged"]
pub type P7PassPriTaggedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8_PASS_PRI_TAGGED` reader - 11:11\\]
Port 8 Pass Priority Tagged"]
pub type P8PassPriTaggedR = crate::BitReader;
#[doc = "Field `P8_PASS_PRI_TAGGED` writer - 11:11\\]
Port 8 Pass Priority Tagged"]
pub type P8PassPriTaggedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P0_TX_CRC_TYPE` reader - 12:12\\]
Port 0 Transmit CRC Type"]
pub type P0TxCrcTypeR = crate::BitReader;
#[doc = "Field `P0_TX_CRC_TYPE` writer - 12:12\\]
Port 0 Transmit CRC Type"]
pub type P0TxCrcTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P0_TX_CRC_REMOVE` reader - 13:13\\]
Port 0 Transmit CRC remove"]
pub type P0TxCrcRemoveR = crate::BitReader;
#[doc = "Field `P0_TX_CRC_REMOVE` writer - 13:13\\]
Port 0 Transmit CRC remove"]
pub type P0TxCrcRemoveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P0_RX_PAD` reader - 14:14\\]
Port 0 Receive Short Packet Pad"]
pub type P0RxPadR = crate::BitReader;
#[doc = "Field `P0_RX_PAD` writer - 14:14\\]
Port 0 Receive Short Packet Pad"]
pub type P0RxPadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P0_RX_PASS_CRC_ERR` reader - 15:15\\]
Port 0 Pass Received CRC errors"]
pub type P0RxPassCrcErrR = crate::BitReader;
#[doc = "Field `P0_RX_PASS_CRC_ERR` writer - 15:15\\]
Port 0 Pass Received CRC errors"]
pub type P0RxPassCrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEE_ENABLE` reader - 16:16\\]
Energy Efficient Ethernet enable"]
pub type EeeEnableR = crate::BitReader;
#[doc = "Field `EEE_ENABLE` writer - 16:16\\]
Energy Efficient Ethernet enable"]
pub type EeeEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IET_ENABLE` reader - 17:17\\]
Intersperced Express Traffic enable"]
pub type IetEnableR = crate::BitReader;
#[doc = "Field `IET_ENABLE` writer - 17:17\\]
Intersperced Express Traffic enable"]
pub type IetEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EST_ENABLE` reader - 18:18\\]
Intersperced Express Traffic enable"]
pub type EstEnableR = crate::BitReader;
#[doc = "Field `EST_ENABLE` writer - 18:18\\]
Intersperced Express Traffic enable"]
pub type EstEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUT_THRU_ENABLE` reader - 19:19\\]
Cut-Thru enable"]
pub type CutThruEnableR = crate::BitReader;
#[doc = "Field `CUT_THRU_ENABLE` writer - 19:19\\]
Cut-Thru enable"]
pub type CutThruEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_CRC_MODE` reader - 31:31\\]
ECC CRC Mode"]
pub type EccCrcModeR = crate::BitReader;
#[doc = "Field `ECC_CRC_MODE` writer - 31:31\\]
ECC CRC Mode"]
pub type EccCrcModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
VLAN Aware Mode"]
    #[inline(always)]
    pub fn s_cn_switch(&self) -> SCnSwitchR {
        SCnSwitchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
VLAN Aware Mode"]
    #[inline(always)]
    pub fn vlan_aware(&self) -> VlanAwareR {
        VlanAwareR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Port 0 Enable"]
    #[inline(always)]
    pub fn p0_enable(&self) -> P0EnableR {
        P0EnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Port 0 Pass Priority Tagged"]
    #[inline(always)]
    pub fn p0_pass_pri_tagged(&self) -> P0PassPriTaggedR {
        P0PassPriTaggedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Port 1 Pass Priority Tagged"]
    #[inline(always)]
    pub fn p1_pass_pri_tagged(&self) -> P1PassPriTaggedR {
        P1PassPriTaggedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Port 2 Pass Priority Tagged"]
    #[inline(always)]
    pub fn p2_pass_pri_tagged(&self) -> P2PassPriTaggedR {
        P2PassPriTaggedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Port 3 Pass Priority Tagged"]
    #[inline(always)]
    pub fn p3_pass_pri_tagged(&self) -> P3PassPriTaggedR {
        P3PassPriTaggedR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Port 4 Pass Priority Tagged"]
    #[inline(always)]
    pub fn p4_pass_pri_tagged(&self) -> P4PassPriTaggedR {
        P4PassPriTaggedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Port 5 Pass Priority Tagged"]
    #[inline(always)]
    pub fn p5_pass_pri_tagged(&self) -> P5PassPriTaggedR {
        P5PassPriTaggedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Port 6 Pass Priority Tagged"]
    #[inline(always)]
    pub fn p6_pass_pri_tagged(&self) -> P6PassPriTaggedR {
        P6PassPriTaggedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Port 7 Pass Priority Tagged"]
    #[inline(always)]
    pub fn p7_pass_pri_tagged(&self) -> P7PassPriTaggedR {
        P7PassPriTaggedR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Port 8 Pass Priority Tagged"]
    #[inline(always)]
    pub fn p8_pass_pri_tagged(&self) -> P8PassPriTaggedR {
        P8PassPriTaggedR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Port 0 Transmit CRC Type"]
    #[inline(always)]
    pub fn p0_tx_crc_type(&self) -> P0TxCrcTypeR {
        P0TxCrcTypeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Port 0 Transmit CRC remove"]
    #[inline(always)]
    pub fn p0_tx_crc_remove(&self) -> P0TxCrcRemoveR {
        P0TxCrcRemoveR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Port 0 Receive Short Packet Pad"]
    #[inline(always)]
    pub fn p0_rx_pad(&self) -> P0RxPadR {
        P0RxPadR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Port 0 Pass Received CRC errors"]
    #[inline(always)]
    pub fn p0_rx_pass_crc_err(&self) -> P0RxPassCrcErrR {
        P0RxPassCrcErrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Energy Efficient Ethernet enable"]
    #[inline(always)]
    pub fn eee_enable(&self) -> EeeEnableR {
        EeeEnableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Intersperced Express Traffic enable"]
    #[inline(always)]
    pub fn iet_enable(&self) -> IetEnableR {
        IetEnableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Intersperced Express Traffic enable"]
    #[inline(always)]
    pub fn est_enable(&self) -> EstEnableR {
        EstEnableR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Cut-Thru enable"]
    #[inline(always)]
    pub fn cut_thru_enable(&self) -> CutThruEnableR {
        CutThruEnableR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
ECC CRC Mode"]
    #[inline(always)]
    pub fn ecc_crc_mode(&self) -> EccCrcModeR {
        EccCrcModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
VLAN Aware Mode"]
    #[inline(always)]
    #[must_use]
    pub fn s_cn_switch(&mut self) -> SCnSwitchW<CpswNussVbuspControlReg_Spec> {
        SCnSwitchW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
VLAN Aware Mode"]
    #[inline(always)]
    #[must_use]
    pub fn vlan_aware(&mut self) -> VlanAwareW<CpswNussVbuspControlReg_Spec> {
        VlanAwareW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Port 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p0_enable(&mut self) -> P0EnableW<CpswNussVbuspControlReg_Spec> {
        P0EnableW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Port 0 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn p0_pass_pri_tagged(&mut self) -> P0PassPriTaggedW<CpswNussVbuspControlReg_Spec> {
        P0PassPriTaggedW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Port 1 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn p1_pass_pri_tagged(&mut self) -> P1PassPriTaggedW<CpswNussVbuspControlReg_Spec> {
        P1PassPriTaggedW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Port 2 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn p2_pass_pri_tagged(&mut self) -> P2PassPriTaggedW<CpswNussVbuspControlReg_Spec> {
        P2PassPriTaggedW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Port 3 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn p3_pass_pri_tagged(&mut self) -> P3PassPriTaggedW<CpswNussVbuspControlReg_Spec> {
        P3PassPriTaggedW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Port 4 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn p4_pass_pri_tagged(&mut self) -> P4PassPriTaggedW<CpswNussVbuspControlReg_Spec> {
        P4PassPriTaggedW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Port 5 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn p5_pass_pri_tagged(&mut self) -> P5PassPriTaggedW<CpswNussVbuspControlReg_Spec> {
        P5PassPriTaggedW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Port 6 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn p6_pass_pri_tagged(&mut self) -> P6PassPriTaggedW<CpswNussVbuspControlReg_Spec> {
        P6PassPriTaggedW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Port 7 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn p7_pass_pri_tagged(&mut self) -> P7PassPriTaggedW<CpswNussVbuspControlReg_Spec> {
        P7PassPriTaggedW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Port 8 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn p8_pass_pri_tagged(&mut self) -> P8PassPriTaggedW<CpswNussVbuspControlReg_Spec> {
        P8PassPriTaggedW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Port 0 Transmit CRC Type"]
    #[inline(always)]
    #[must_use]
    pub fn p0_tx_crc_type(&mut self) -> P0TxCrcTypeW<CpswNussVbuspControlReg_Spec> {
        P0TxCrcTypeW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Port 0 Transmit CRC remove"]
    #[inline(always)]
    #[must_use]
    pub fn p0_tx_crc_remove(&mut self) -> P0TxCrcRemoveW<CpswNussVbuspControlReg_Spec> {
        P0TxCrcRemoveW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Port 0 Receive Short Packet Pad"]
    #[inline(always)]
    #[must_use]
    pub fn p0_rx_pad(&mut self) -> P0RxPadW<CpswNussVbuspControlReg_Spec> {
        P0RxPadW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Port 0 Pass Received CRC errors"]
    #[inline(always)]
    #[must_use]
    pub fn p0_rx_pass_crc_err(&mut self) -> P0RxPassCrcErrW<CpswNussVbuspControlReg_Spec> {
        P0RxPassCrcErrW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Energy Efficient Ethernet enable"]
    #[inline(always)]
    #[must_use]
    pub fn eee_enable(&mut self) -> EeeEnableW<CpswNussVbuspControlReg_Spec> {
        EeeEnableW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Intersperced Express Traffic enable"]
    #[inline(always)]
    #[must_use]
    pub fn iet_enable(&mut self) -> IetEnableW<CpswNussVbuspControlReg_Spec> {
        IetEnableW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Intersperced Express Traffic enable"]
    #[inline(always)]
    #[must_use]
    pub fn est_enable(&mut self) -> EstEnableW<CpswNussVbuspControlReg_Spec> {
        EstEnableW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Cut-Thru enable"]
    #[inline(always)]
    #[must_use]
    pub fn cut_thru_enable(&mut self) -> CutThruEnableW<CpswNussVbuspControlReg_Spec> {
        CutThruEnableW::new(self, 19)
    }
    #[doc = "Bit 31 - 31:31\\]
ECC CRC Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_crc_mode(&mut self) -> EccCrcModeW<CpswNussVbuspControlReg_Spec> {
        EccCrcModeW::new(self, 31)
    }
}
#[doc = "CPSW Switch Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_control_reg___::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_control_reg___::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspControlReg_Spec;
impl crate::RegisterSpec for CpswNussVbuspControlReg_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_control_reg___::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspControlReg_Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_control_reg___::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspControlReg_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_CONTROL_REG___ to value 0"]
impl crate::Resettable for CpswNussVbuspControlReg_Spec {
    const RESET_VALUE: u32 = 0;
}
