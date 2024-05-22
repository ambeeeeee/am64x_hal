#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_CCCR` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsCccrSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_CCCR` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsCccrSpec>;
#[doc = "Field `INIT` reader - 0:0\\]
Initialization"]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - 0:0\\]
Initialization"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCE` reader - 1:1\\]
Configuration Change Enable"]
pub type CceR = crate::BitReader;
#[doc = "Field `CCE` writer - 1:1\\]
Configuration Change Enable"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASM` reader - 2:2\\]
Restricted Operation Mode"]
pub type AsmR = crate::BitReader;
#[doc = "Field `ASM` writer - 2:2\\]
Restricted Operation Mode"]
pub type AsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSA` reader - 3:3\\]
Clock Stop Acknowledge"]
pub type CsaR = crate::BitReader;
#[doc = "Field `CSA` writer - 3:3\\]
Clock Stop Acknowledge"]
pub type CsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSR` reader - 4:4\\]
Clock Stop Request"]
pub type CsrR = crate::BitReader;
#[doc = "Field `CSR` writer - 4:4\\]
Clock Stop Request"]
pub type CsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON` reader - 5:5\\]
Bus Monitoring Mode"]
pub type MonR = crate::BitReader;
#[doc = "Field `MON` writer - 5:5\\]
Bus Monitoring Mode"]
pub type MonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAR` reader - 6:6\\]
Disable Automatic Retransmission"]
pub type DarR = crate::BitReader;
#[doc = "Field `DAR` writer - 6:6\\]
Disable Automatic Retransmission"]
pub type DarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST` reader - 7:7\\]
Test Mode enable"]
pub type TestR = crate::BitReader;
#[doc = "Field `TEST` writer - 7:7\\]
Test Mode enable"]
pub type TestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOE` reader - 8:8\\]
FD Operation Enable"]
pub type FdoeR = crate::BitReader;
#[doc = "Field `FDOE` writer - 8:8\\]
FD Operation Enable"]
pub type FdoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRSE` reader - 9:9\\]
Bit Rate Switch Enable"]
pub type BrseR = crate::BitReader;
#[doc = "Field `BRSE` writer - 9:9\\]
Bit Rate Switch Enable"]
pub type BrseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXHD` reader - 12:12\\]
Protocol Exception Handling Disable"]
pub type PxhdR = crate::BitReader;
#[doc = "Field `PXHD` writer - 12:12\\]
Protocol Exception Handling Disable"]
pub type PxhdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFBI` reader - 13:13\\]
Edge Filtering during Bus Integration"]
pub type EfbiR = crate::BitReader;
#[doc = "Field `EFBI` writer - 13:13\\]
Edge Filtering during Bus Integration"]
pub type EfbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXP` reader - 14:14\\]
Transmit Pause"]
pub type TxpR = crate::BitReader;
#[doc = "Field `TXP` writer - 14:14\\]
Transmit Pause"]
pub type TxpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NISO` reader - 15:15\\]
Non ISO Operation. 0= CAN FD frame format according to ISO 11898-1:2015. 1= CAN FD frame format according to Bosch CAN FD Specification 1.0"]
pub type NisoR = crate::BitReader;
#[doc = "Field `NISO` writer - 15:15\\]
Non ISO Operation. 0= CAN FD frame format according to ISO 11898-1:2015. 1= CAN FD frame format according to Bosch CAN FD Specification 1.0"]
pub type NisoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Initialization"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Restricted Operation Mode"]
    #[inline(always)]
    pub fn asm(&self) -> AsmR {
        AsmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Clock Stop Acknowledge"]
    #[inline(always)]
    pub fn csa(&self) -> CsaR {
        CsaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Clock Stop Request"]
    #[inline(always)]
    pub fn csr(&self) -> CsrR {
        CsrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Bus Monitoring Mode"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Disable Automatic Retransmission"]
    #[inline(always)]
    pub fn dar(&self) -> DarR {
        DarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Test Mode enable"]
    #[inline(always)]
    pub fn test(&self) -> TestR {
        TestR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
FD Operation Enable"]
    #[inline(always)]
    pub fn fdoe(&self) -> FdoeR {
        FdoeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Bit Rate Switch Enable"]
    #[inline(always)]
    pub fn brse(&self) -> BrseR {
        BrseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Protocol Exception Handling Disable"]
    #[inline(always)]
    pub fn pxhd(&self) -> PxhdR {
        PxhdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Edge Filtering during Bus Integration"]
    #[inline(always)]
    pub fn efbi(&self) -> EfbiR {
        EfbiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Transmit Pause"]
    #[inline(always)]
    pub fn txp(&self) -> TxpR {
        TxpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Non ISO Operation. 0= CAN FD frame format according to ISO 11898-1:2015. 1= CAN FD frame format according to Bosch CAN FD Specification 1.0"]
    #[inline(always)]
    pub fn niso(&self) -> NisoR {
        NisoR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<McanWrap_McanCfgVbp_McanRegsCccrSpec> {
        InitW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Configuration Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CceW<McanWrap_McanCfgVbp_McanRegsCccrSpec> {
        CceW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Restricted Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn asm(&mut self) -> AsmW<McanWrap_McanCfgVbp_McanRegsCccrSpec> {
        AsmW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Clock Stop Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn csa(&mut self) -> CsaW<McanWrap_McanCfgVbp_McanRegsCccrSpec> {
        CsaW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Clock Stop Request"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CsrW<McanWrap_McanCfgVbp_McanRegsCccrSpec> {
        CsrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Bus Monitoring Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MonW<McanWrap_McanCfgVbp_McanRegsCccrSpec> {
        MonW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Disable Automatic Retransmission"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DarW<McanWrap_McanCfgVbp_McanRegsCccrSpec> {
        DarW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Test Mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TestW<McanWrap_McanCfgVbp_McanRegsCccrSpec> {
        TestW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
FD Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdoe(&mut self) -> FdoeW<McanWrap_McanCfgVbp_McanRegsCccrSpec> {
        FdoeW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Bit Rate Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brse(&mut self) -> BrseW<McanWrap_McanCfgVbp_McanRegsCccrSpec> {
        BrseW::new(self, 9)
    }
    #[doc = "Bit 12 - 12:12\\]
Protocol Exception Handling Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pxhd(&mut self) -> PxhdW<McanWrap_McanCfgVbp_McanRegsCccrSpec> {
        PxhdW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Edge Filtering during Bus Integration"]
    #[inline(always)]
    #[must_use]
    pub fn efbi(&mut self) -> EfbiW<McanWrap_McanCfgVbp_McanRegsCccrSpec> {
        EfbiW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Transmit Pause"]
    #[inline(always)]
    #[must_use]
    pub fn txp(&mut self) -> TxpW<McanWrap_McanCfgVbp_McanRegsCccrSpec> {
        TxpW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Non ISO Operation. 0= CAN FD frame format according to ISO 11898-1:2015. 1= CAN FD frame format according to Bosch CAN FD Specification 1.0"]
    #[inline(always)]
    #[must_use]
    pub fn niso(&mut self) -> NisoW<McanWrap_McanCfgVbp_McanRegsCccrSpec> {
        NisoW::new(self, 15)
    }
}
#[doc = "Operation mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_cccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_cccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsCccrSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsCccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_cccr::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsCccrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_cccr::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsCccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_CCCR to value 0x01"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsCccrSpec {
    const RESET_VALUE: u32 = 0x01;
}
