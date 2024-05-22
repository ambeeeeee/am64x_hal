#[doc = "Register `ECCAGGR_REGS_sec_status_reg0` reader"]
pub type R = crate::R<EccaggrRegsSecStatusReg0Spec>;
#[doc = "Register `ECCAGGR_REGS_sec_status_reg0` writer"]
pub type W = crate::W<EccaggrRegsSecStatusReg0Spec>;
#[doc = "Field `PKTDMA_CFG_RAMECC_PEND` reader - 0:0\\]
Interrupt Pending Status for pktdma_cfg_ramecc_pend"]
pub type PktdmaCfgRameccPendR = crate::BitReader;
#[doc = "Field `PKTDMA_CFG_RAMECC_PEND` writer - 0:0\\]
Interrupt Pending Status for pktdma_cfg_ramecc_pend"]
pub type PktdmaCfgRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDMA_STATE_RAMECC_PEND` reader - 1:1\\]
Interrupt Pending Status for pktdma_state_ramecc_pend"]
pub type PktdmaStateRameccPendR = crate::BitReader;
#[doc = "Field `PKTDMA_STATE_RAMECC_PEND` writer - 1:1\\]
Interrupt Pending Status for pktdma_state_ramecc_pend"]
pub type PktdmaStateRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDMA_TPCF0_RAMECC_PEND` reader - 2:2\\]
Interrupt Pending Status for pktdma_tpcf0_ramecc_pend"]
pub type PktdmaTpcf0RameccPendR = crate::BitReader;
#[doc = "Field `PKTDMA_TPCF0_RAMECC_PEND` writer - 2:2\\]
Interrupt Pending Status for pktdma_tpcf0_ramecc_pend"]
pub type PktdmaTpcf0RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDMA_TPCF1_RAMECC_PEND` reader - 3:3\\]
Interrupt Pending Status for pktdma_tpcf1_ramecc_pend"]
pub type PktdmaTpcf1RameccPendR = crate::BitReader;
#[doc = "Field `PKTDMA_TPCF1_RAMECC_PEND` writer - 3:3\\]
Interrupt Pending Status for pktdma_tpcf1_ramecc_pend"]
pub type PktdmaTpcf1RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDMA_RPCF0_RAMECC_PEND` reader - 4:4\\]
Interrupt Pending Status for pktdma_rpcf0_ramecc_pend"]
pub type PktdmaRpcf0RameccPendR = crate::BitReader;
#[doc = "Field `PKTDMA_RPCF0_RAMECC_PEND` writer - 4:4\\]
Interrupt Pending Status for pktdma_rpcf0_ramecc_pend"]
pub type PktdmaRpcf0RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDMA_RPCF1_RAMECC_PEND` reader - 5:5\\]
Interrupt Pending Status for pktdma_rpcf1_ramecc_pend"]
pub type PktdmaRpcf1RameccPendR = crate::BitReader;
#[doc = "Field `PKTDMA_RPCF1_RAMECC_PEND` writer - 5:5\\]
Interrupt Pending Status for pktdma_rpcf1_ramecc_pend"]
pub type PktdmaRpcf1RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDMA_RPCF2_RAMECC_PEND` reader - 6:6\\]
Interrupt Pending Status for pktdma_rpcf2_ramecc_pend"]
pub type PktdmaRpcf2RameccPendR = crate::BitReader;
#[doc = "Field `PKTDMA_RPCF2_RAMECC_PEND` writer - 6:6\\]
Interrupt Pending Status for pktdma_rpcf2_ramecc_pend"]
pub type PktdmaRpcf2RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDMA_STS_RAMECC0_PEND` reader - 7:7\\]
Interrupt Pending Status for pktdma_sts_ramecc0_pend"]
pub type PktdmaStsRamecc0PendR = crate::BitReader;
#[doc = "Field `PKTDMA_STS_RAMECC0_PEND` writer - 7:7\\]
Interrupt Pending Status for pktdma_sts_ramecc0_pend"]
pub type PktdmaStsRamecc0PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDMA_STS_RAMECC1_PEND` reader - 8:8\\]
Interrupt Pending Status for pktdma_sts_ramecc1_pend"]
pub type PktdmaStsRamecc1PendR = crate::BitReader;
#[doc = "Field `PKTDMA_STS_RAMECC1_PEND` writer - 8:8\\]
Interrupt Pending Status for pktdma_sts_ramecc1_pend"]
pub type PktdmaStsRamecc1PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDMA_RNGOCC_RAMECC_PEND` reader - 9:9\\]
Interrupt Pending Status for pktdma_rngocc_ramecc_pend"]
pub type PktdmaRngoccRameccPendR = crate::BitReader;
#[doc = "Field `PKTDMA_RNGOCC_RAMECC_PEND` writer - 9:9\\]
Interrupt Pending Status for pktdma_rngocc_ramecc_pend"]
pub type PktdmaRngoccRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCDMA_CFG_RAMECC_PEND` reader - 10:10\\]
Interrupt Pending Status for bcdma_cfg_ramecc_pend"]
pub type BcdmaCfgRameccPendR = crate::BitReader;
#[doc = "Field `BCDMA_CFG_RAMECC_PEND` writer - 10:10\\]
Interrupt Pending Status for bcdma_cfg_ramecc_pend"]
pub type BcdmaCfgRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCDMA_STATE_RAMECC_PEND` reader - 11:11\\]
Interrupt Pending Status for bcdma_state_ramecc_pend"]
pub type BcdmaStateRameccPendR = crate::BitReader;
#[doc = "Field `BCDMA_STATE_RAMECC_PEND` writer - 11:11\\]
Interrupt Pending Status for bcdma_state_ramecc_pend"]
pub type BcdmaStateRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCFD0_RAMECC_PEND` reader - 12:12\\]
Interrupt Pending Status for pcfd0_ramecc_pend"]
pub type Pcfd0RameccPendR = crate::BitReader;
#[doc = "Field `PCFD0_RAMECC_PEND` writer - 12:12\\]
Interrupt Pending Status for pcfd0_ramecc_pend"]
pub type Pcfd0RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCFD1_RAMECC_PEND` reader - 13:13\\]
Interrupt Pending Status for pcfd1_ramecc_pend"]
pub type Pcfd1RameccPendR = crate::BitReader;
#[doc = "Field `PCFD1_RAMECC_PEND` writer - 13:13\\]
Interrupt Pending Status for pcfd1_ramecc_pend"]
pub type Pcfd1RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCDMA_TPCF0_RAMECC_PEND` reader - 14:14\\]
Interrupt Pending Status for bcdma_tpcf0_ramecc_pend"]
pub type BcdmaTpcf0RameccPendR = crate::BitReader;
#[doc = "Field `BCDMA_TPCF0_RAMECC_PEND` writer - 14:14\\]
Interrupt Pending Status for bcdma_tpcf0_ramecc_pend"]
pub type BcdmaTpcf0RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCDMA_TPCF1_RAMECC_PEND` reader - 15:15\\]
Interrupt Pending Status for bcdma_tpcf1_ramecc_pend"]
pub type BcdmaTpcf1RameccPendR = crate::BitReader;
#[doc = "Field `BCDMA_TPCF1_RAMECC_PEND` writer - 15:15\\]
Interrupt Pending Status for bcdma_tpcf1_ramecc_pend"]
pub type BcdmaTpcf1RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCDMA_RPCF0_RAMECC_PEND` reader - 16:16\\]
Interrupt Pending Status for bcdma_rpcf0_ramecc_pend"]
pub type BcdmaRpcf0RameccPendR = crate::BitReader;
#[doc = "Field `BCDMA_RPCF0_RAMECC_PEND` writer - 16:16\\]
Interrupt Pending Status for bcdma_rpcf0_ramecc_pend"]
pub type BcdmaRpcf0RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCDMA_RPCF1_RAMECC_PEND` reader - 17:17\\]
Interrupt Pending Status for bcdma_rpcf1_ramecc_pend"]
pub type BcdmaRpcf1RameccPendR = crate::BitReader;
#[doc = "Field `BCDMA_RPCF1_RAMECC_PEND` writer - 17:17\\]
Interrupt Pending Status for bcdma_rpcf1_ramecc_pend"]
pub type BcdmaRpcf1RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCDMA_RPCF2_RAMECC_PEND` reader - 18:18\\]
Interrupt Pending Status for bcdma_rpcf2_ramecc_pend"]
pub type BcdmaRpcf2RameccPendR = crate::BitReader;
#[doc = "Field `BCDMA_RPCF2_RAMECC_PEND` writer - 18:18\\]
Interrupt Pending Status for bcdma_rpcf2_ramecc_pend"]
pub type BcdmaRpcf2RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCDMA_STS_RAMECC0_PEND` reader - 19:19\\]
Interrupt Pending Status for bcdma_sts_ramecc0_pend"]
pub type BcdmaStsRamecc0PendR = crate::BitReader;
#[doc = "Field `BCDMA_STS_RAMECC0_PEND` writer - 19:19\\]
Interrupt Pending Status for bcdma_sts_ramecc0_pend"]
pub type BcdmaStsRamecc0PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCDMA_STS_RAMECC1_PEND` reader - 20:20\\]
Interrupt Pending Status for bcdma_sts_ramecc1_pend"]
pub type BcdmaStsRamecc1PendR = crate::BitReader;
#[doc = "Field `BCDMA_STS_RAMECC1_PEND` writer - 20:20\\]
Interrupt Pending Status for bcdma_sts_ramecc1_pend"]
pub type BcdmaStsRamecc1PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCDMA_RNGOCC_RAMECC_PEND` reader - 21:21\\]
Interrupt Pending Status for bcdma_rngocc_ramecc_pend"]
pub type BcdmaRngoccRameccPendR = crate::BitReader;
#[doc = "Field `BCDMA_RNGOCC_RAMECC_PEND` writer - 21:21\\]
Interrupt Pending Status for bcdma_rngocc_ramecc_pend"]
pub type BcdmaRngoccRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_RAMECC_PEND` reader - 22:22\\]
Interrupt Pending Status for sr_ramecc_pend"]
pub type SrRameccPendR = crate::BitReader;
#[doc = "Field `SR_RAMECC_PEND` writer - 22:22\\]
Interrupt Pending Status for sr_ramecc_pend"]
pub type SrRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAP_RAMECC_PEND` reader - 23:23\\]
Interrupt Pending Status for map_ramecc_pend"]
pub type MapRameccPendR = crate::BitReader;
#[doc = "Field `MAP_RAMECC_PEND` writer - 23:23\\]
Interrupt Pending Status for map_ramecc_pend"]
pub type MapRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RINGACC_STRAM_RAMECC_PEND` reader - 24:24\\]
Interrupt Pending Status for ringacc_stram_ramecc_pend"]
pub type RingaccStramRameccPendR = crate::BitReader;
#[doc = "Field `RINGACC_STRAM_RAMECC_PEND` writer - 24:24\\]
Interrupt Pending Status for ringacc_stram_ramecc_pend"]
pub type RingaccStramRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_PROXY_STRAM_RAMECCC_PEND` reader - 25:25\\]
Interrupt Pending Status for sec_proxy_stram_rameccc_pend"]
pub type SecProxyStramRamecccPendR = crate::BitReader;
#[doc = "Field `SEC_PROXY_STRAM_RAMECCC_PEND` writer - 25:25\\]
Interrupt Pending Status for sec_proxy_stram_rameccc_pend"]
pub type SecProxyStramRamecccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_PROXY_BUFRAM_RAMECCC_PEND` reader - 26:26\\]
Interrupt Pending Status for sec_proxy_bufram_rameccc_pend"]
pub type SecProxyBuframRamecccPendR = crate::BitReader;
#[doc = "Field `SEC_PROXY_BUFRAM_RAMECCC_PEND` writer - 26:26\\]
Interrupt Pending Status for sec_proxy_bufram_rameccc_pend"]
pub type SecProxyBuframRamecccPendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for pktdma_cfg_ramecc_pend"]
    #[inline(always)]
    pub fn pktdma_cfg_ramecc_pend(&self) -> PktdmaCfgRameccPendR {
        PktdmaCfgRameccPendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for pktdma_state_ramecc_pend"]
    #[inline(always)]
    pub fn pktdma_state_ramecc_pend(&self) -> PktdmaStateRameccPendR {
        PktdmaStateRameccPendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for pktdma_tpcf0_ramecc_pend"]
    #[inline(always)]
    pub fn pktdma_tpcf0_ramecc_pend(&self) -> PktdmaTpcf0RameccPendR {
        PktdmaTpcf0RameccPendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for pktdma_tpcf1_ramecc_pend"]
    #[inline(always)]
    pub fn pktdma_tpcf1_ramecc_pend(&self) -> PktdmaTpcf1RameccPendR {
        PktdmaTpcf1RameccPendR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Pending Status for pktdma_rpcf0_ramecc_pend"]
    #[inline(always)]
    pub fn pktdma_rpcf0_ramecc_pend(&self) -> PktdmaRpcf0RameccPendR {
        PktdmaRpcf0RameccPendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Pending Status for pktdma_rpcf1_ramecc_pend"]
    #[inline(always)]
    pub fn pktdma_rpcf1_ramecc_pend(&self) -> PktdmaRpcf1RameccPendR {
        PktdmaRpcf1RameccPendR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Pending Status for pktdma_rpcf2_ramecc_pend"]
    #[inline(always)]
    pub fn pktdma_rpcf2_ramecc_pend(&self) -> PktdmaRpcf2RameccPendR {
        PktdmaRpcf2RameccPendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Pending Status for pktdma_sts_ramecc0_pend"]
    #[inline(always)]
    pub fn pktdma_sts_ramecc0_pend(&self) -> PktdmaStsRamecc0PendR {
        PktdmaStsRamecc0PendR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Pending Status for pktdma_sts_ramecc1_pend"]
    #[inline(always)]
    pub fn pktdma_sts_ramecc1_pend(&self) -> PktdmaStsRamecc1PendR {
        PktdmaStsRamecc1PendR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Pending Status for pktdma_rngocc_ramecc_pend"]
    #[inline(always)]
    pub fn pktdma_rngocc_ramecc_pend(&self) -> PktdmaRngoccRameccPendR {
        PktdmaRngoccRameccPendR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Pending Status for bcdma_cfg_ramecc_pend"]
    #[inline(always)]
    pub fn bcdma_cfg_ramecc_pend(&self) -> BcdmaCfgRameccPendR {
        BcdmaCfgRameccPendR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Pending Status for bcdma_state_ramecc_pend"]
    #[inline(always)]
    pub fn bcdma_state_ramecc_pend(&self) -> BcdmaStateRameccPendR {
        BcdmaStateRameccPendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Pending Status for pcfd0_ramecc_pend"]
    #[inline(always)]
    pub fn pcfd0_ramecc_pend(&self) -> Pcfd0RameccPendR {
        Pcfd0RameccPendR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Pending Status for pcfd1_ramecc_pend"]
    #[inline(always)]
    pub fn pcfd1_ramecc_pend(&self) -> Pcfd1RameccPendR {
        Pcfd1RameccPendR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt Pending Status for bcdma_tpcf0_ramecc_pend"]
    #[inline(always)]
    pub fn bcdma_tpcf0_ramecc_pend(&self) -> BcdmaTpcf0RameccPendR {
        BcdmaTpcf0RameccPendR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt Pending Status for bcdma_tpcf1_ramecc_pend"]
    #[inline(always)]
    pub fn bcdma_tpcf1_ramecc_pend(&self) -> BcdmaTpcf1RameccPendR {
        BcdmaTpcf1RameccPendR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt Pending Status for bcdma_rpcf0_ramecc_pend"]
    #[inline(always)]
    pub fn bcdma_rpcf0_ramecc_pend(&self) -> BcdmaRpcf0RameccPendR {
        BcdmaRpcf0RameccPendR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt Pending Status for bcdma_rpcf1_ramecc_pend"]
    #[inline(always)]
    pub fn bcdma_rpcf1_ramecc_pend(&self) -> BcdmaRpcf1RameccPendR {
        BcdmaRpcf1RameccPendR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt Pending Status for bcdma_rpcf2_ramecc_pend"]
    #[inline(always)]
    pub fn bcdma_rpcf2_ramecc_pend(&self) -> BcdmaRpcf2RameccPendR {
        BcdmaRpcf2RameccPendR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt Pending Status for bcdma_sts_ramecc0_pend"]
    #[inline(always)]
    pub fn bcdma_sts_ramecc0_pend(&self) -> BcdmaStsRamecc0PendR {
        BcdmaStsRamecc0PendR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Interrupt Pending Status for bcdma_sts_ramecc1_pend"]
    #[inline(always)]
    pub fn bcdma_sts_ramecc1_pend(&self) -> BcdmaStsRamecc1PendR {
        BcdmaStsRamecc1PendR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Interrupt Pending Status for bcdma_rngocc_ramecc_pend"]
    #[inline(always)]
    pub fn bcdma_rngocc_ramecc_pend(&self) -> BcdmaRngoccRameccPendR {
        BcdmaRngoccRameccPendR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt Pending Status for sr_ramecc_pend"]
    #[inline(always)]
    pub fn sr_ramecc_pend(&self) -> SrRameccPendR {
        SrRameccPendR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Interrupt Pending Status for map_ramecc_pend"]
    #[inline(always)]
    pub fn map_ramecc_pend(&self) -> MapRameccPendR {
        MapRameccPendR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Interrupt Pending Status for ringacc_stram_ramecc_pend"]
    #[inline(always)]
    pub fn ringacc_stram_ramecc_pend(&self) -> RingaccStramRameccPendR {
        RingaccStramRameccPendR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Interrupt Pending Status for sec_proxy_stram_rameccc_pend"]
    #[inline(always)]
    pub fn sec_proxy_stram_rameccc_pend(&self) -> SecProxyStramRamecccPendR {
        SecProxyStramRamecccPendR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Interrupt Pending Status for sec_proxy_bufram_rameccc_pend"]
    #[inline(always)]
    pub fn sec_proxy_bufram_rameccc_pend(&self) -> SecProxyBuframRamecccPendR {
        SecProxyBuframRamecccPendR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for pktdma_cfg_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pktdma_cfg_ramecc_pend(&mut self) -> PktdmaCfgRameccPendW<EccaggrRegsSecStatusReg0Spec> {
        PktdmaCfgRameccPendW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for pktdma_state_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pktdma_state_ramecc_pend(
        &mut self,
    ) -> PktdmaStateRameccPendW<EccaggrRegsSecStatusReg0Spec> {
        PktdmaStateRameccPendW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for pktdma_tpcf0_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pktdma_tpcf0_ramecc_pend(
        &mut self,
    ) -> PktdmaTpcf0RameccPendW<EccaggrRegsSecStatusReg0Spec> {
        PktdmaTpcf0RameccPendW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for pktdma_tpcf1_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pktdma_tpcf1_ramecc_pend(
        &mut self,
    ) -> PktdmaTpcf1RameccPendW<EccaggrRegsSecStatusReg0Spec> {
        PktdmaTpcf1RameccPendW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Pending Status for pktdma_rpcf0_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pktdma_rpcf0_ramecc_pend(
        &mut self,
    ) -> PktdmaRpcf0RameccPendW<EccaggrRegsSecStatusReg0Spec> {
        PktdmaRpcf0RameccPendW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Pending Status for pktdma_rpcf1_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pktdma_rpcf1_ramecc_pend(
        &mut self,
    ) -> PktdmaRpcf1RameccPendW<EccaggrRegsSecStatusReg0Spec> {
        PktdmaRpcf1RameccPendW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Pending Status for pktdma_rpcf2_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pktdma_rpcf2_ramecc_pend(
        &mut self,
    ) -> PktdmaRpcf2RameccPendW<EccaggrRegsSecStatusReg0Spec> {
        PktdmaRpcf2RameccPendW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Pending Status for pktdma_sts_ramecc0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pktdma_sts_ramecc0_pend(
        &mut self,
    ) -> PktdmaStsRamecc0PendW<EccaggrRegsSecStatusReg0Spec> {
        PktdmaStsRamecc0PendW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Pending Status for pktdma_sts_ramecc1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pktdma_sts_ramecc1_pend(
        &mut self,
    ) -> PktdmaStsRamecc1PendW<EccaggrRegsSecStatusReg0Spec> {
        PktdmaStsRamecc1PendW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Pending Status for pktdma_rngocc_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pktdma_rngocc_ramecc_pend(
        &mut self,
    ) -> PktdmaRngoccRameccPendW<EccaggrRegsSecStatusReg0Spec> {
        PktdmaRngoccRameccPendW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Pending Status for bcdma_cfg_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn bcdma_cfg_ramecc_pend(&mut self) -> BcdmaCfgRameccPendW<EccaggrRegsSecStatusReg0Spec> {
        BcdmaCfgRameccPendW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Pending Status for bcdma_state_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn bcdma_state_ramecc_pend(
        &mut self,
    ) -> BcdmaStateRameccPendW<EccaggrRegsSecStatusReg0Spec> {
        BcdmaStateRameccPendW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Pending Status for pcfd0_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pcfd0_ramecc_pend(&mut self) -> Pcfd0RameccPendW<EccaggrRegsSecStatusReg0Spec> {
        Pcfd0RameccPendW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Pending Status for pcfd1_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn pcfd1_ramecc_pend(&mut self) -> Pcfd1RameccPendW<EccaggrRegsSecStatusReg0Spec> {
        Pcfd1RameccPendW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt Pending Status for bcdma_tpcf0_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn bcdma_tpcf0_ramecc_pend(
        &mut self,
    ) -> BcdmaTpcf0RameccPendW<EccaggrRegsSecStatusReg0Spec> {
        BcdmaTpcf0RameccPendW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt Pending Status for bcdma_tpcf1_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn bcdma_tpcf1_ramecc_pend(
        &mut self,
    ) -> BcdmaTpcf1RameccPendW<EccaggrRegsSecStatusReg0Spec> {
        BcdmaTpcf1RameccPendW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt Pending Status for bcdma_rpcf0_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn bcdma_rpcf0_ramecc_pend(
        &mut self,
    ) -> BcdmaRpcf0RameccPendW<EccaggrRegsSecStatusReg0Spec> {
        BcdmaRpcf0RameccPendW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt Pending Status for bcdma_rpcf1_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn bcdma_rpcf1_ramecc_pend(
        &mut self,
    ) -> BcdmaRpcf1RameccPendW<EccaggrRegsSecStatusReg0Spec> {
        BcdmaRpcf1RameccPendW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt Pending Status for bcdma_rpcf2_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn bcdma_rpcf2_ramecc_pend(
        &mut self,
    ) -> BcdmaRpcf2RameccPendW<EccaggrRegsSecStatusReg0Spec> {
        BcdmaRpcf2RameccPendW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt Pending Status for bcdma_sts_ramecc0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn bcdma_sts_ramecc0_pend(&mut self) -> BcdmaStsRamecc0PendW<EccaggrRegsSecStatusReg0Spec> {
        BcdmaStsRamecc0PendW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Interrupt Pending Status for bcdma_sts_ramecc1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn bcdma_sts_ramecc1_pend(&mut self) -> BcdmaStsRamecc1PendW<EccaggrRegsSecStatusReg0Spec> {
        BcdmaStsRamecc1PendW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Interrupt Pending Status for bcdma_rngocc_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn bcdma_rngocc_ramecc_pend(
        &mut self,
    ) -> BcdmaRngoccRameccPendW<EccaggrRegsSecStatusReg0Spec> {
        BcdmaRngoccRameccPendW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt Pending Status for sr_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn sr_ramecc_pend(&mut self) -> SrRameccPendW<EccaggrRegsSecStatusReg0Spec> {
        SrRameccPendW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Interrupt Pending Status for map_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn map_ramecc_pend(&mut self) -> MapRameccPendW<EccaggrRegsSecStatusReg0Spec> {
        MapRameccPendW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Interrupt Pending Status for ringacc_stram_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn ringacc_stram_ramecc_pend(
        &mut self,
    ) -> RingaccStramRameccPendW<EccaggrRegsSecStatusReg0Spec> {
        RingaccStramRameccPendW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Interrupt Pending Status for sec_proxy_stram_rameccc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn sec_proxy_stram_rameccc_pend(
        &mut self,
    ) -> SecProxyStramRamecccPendW<EccaggrRegsSecStatusReg0Spec> {
        SecProxyStramRamecccPendW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Interrupt Pending Status for sec_proxy_bufram_rameccc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn sec_proxy_bufram_rameccc_pend(
        &mut self,
    ) -> SecProxyBuframRamecccPendW<EccaggrRegsSecStatusReg0Spec> {
        SecProxyBuframRamecccPendW::new(self, 26)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_sec_status_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_sec_status_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccaggrRegsSecStatusReg0Spec;
impl crate::RegisterSpec for EccaggrRegsSecStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccaggr_regs_sec_status_reg0::R`](R) reader structure"]
impl crate::Readable for EccaggrRegsSecStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`eccaggr_regs_sec_status_reg0::W`](W) writer structure"]
impl crate::Writable for EccaggrRegsSecStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCAGGR_REGS_sec_status_reg0 to value 0"]
impl crate::Resettable for EccaggrRegsSecStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}
