#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TEST` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsTestSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TEST` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsTestSpec>;
#[doc = "Field `LBCK` reader - 4:4\\]
Loop Back Mode"]
pub type LbckR = crate::BitReader;
#[doc = "Field `LBCK` writer - 4:4\\]
Loop Back Mode"]
pub type LbckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX` reader - 6:5\\]
Control of Transmit Pin"]
pub type TxR = crate::FieldReader;
#[doc = "Field `TX` writer - 6:5\\]
Control of Transmit Pin"]
pub type TxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX` reader - 7:7\\]
Receive Pin"]
pub type RxR = crate::BitReader;
#[doc = "Field `RX` writer - 7:7\\]
Receive Pin"]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Loop Back Mode"]
    #[inline(always)]
    pub fn lbck(&self) -> LbckR {
        LbckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Control of Transmit Pin"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Receive Pin"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Loop Back Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lbck(&mut self) -> LbckW<McanWrap_McanCfgVbp_McanRegsTestSpec> {
        LbckW::new(self, 4)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Control of Transmit Pin"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TxW<McanWrap_McanCfgVbp_McanRegsTestSpec> {
        TxW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
Receive Pin"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RxW<McanWrap_McanCfgVbp_McanRegsTestSpec> {
        RxW::new(self, 7)
    }
}
#[doc = "Test mode selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsTestSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsTestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_test::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsTestSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_test::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsTestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TEST to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsTestSpec {
    const RESET_VALUE: u32 = 0;
}
