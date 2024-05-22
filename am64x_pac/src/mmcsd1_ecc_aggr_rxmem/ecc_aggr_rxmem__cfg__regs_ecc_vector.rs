#[doc = "Register `ECC_AGGR_RXMEM__CFG__REGS_ecc_vector` reader"]
pub type R = crate::R<EccAggrRxmem_Cfg_RegsEccVectorSpec>;
#[doc = "Register `ECC_AGGR_RXMEM__CFG__REGS_ecc_vector` writer"]
pub type W = crate::W<EccAggrRxmem_Cfg_RegsEccVectorSpec>;
#[doc = "Field `ECC_VECTOR` reader - 10:0\\]
Value written to select the corresponding ECC RAM for control or status"]
pub type EccVectorR = crate::FieldReader<u16>;
#[doc = "Field `ECC_VECTOR` writer - 10:0\\]
Value written to select the corresponding ECC RAM for control or status"]
pub type EccVectorW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RD_SVBUS` reader - 15:15\\]
Write 1 to trigger a read on the serial VBUS"]
pub type RdSvbusR = crate::BitReader;
#[doc = "Field `RD_SVBUS` writer - 15:15\\]
Write 1 to trigger a read on the serial VBUS"]
pub type RdSvbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_SVBUS_ADDRESS` reader - 23:16\\]
Read address"]
pub type RdSvbusAddressR = crate::FieldReader;
#[doc = "Field `RD_SVBUS_ADDRESS` writer - 23:16\\]
Read address"]
pub type RdSvbusAddressW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD_SVBUS_DONE` reader - 24:24\\]
Status to indicate if read on serial VBUS is complete, write of any value will clear this bit."]
pub type RdSvbusDoneR = crate::BitReader;
#[doc = "Field `RD_SVBUS_DONE` writer - 24:24\\]
Status to indicate if read on serial VBUS is complete, write of any value will clear this bit."]
pub type RdSvbusDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Value written to select the corresponding ECC RAM for control or status"]
    #[inline(always)]
    pub fn ecc_vector(&self) -> EccVectorR {
        EccVectorR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
Write 1 to trigger a read on the serial VBUS"]
    #[inline(always)]
    pub fn rd_svbus(&self) -> RdSvbusR {
        RdSvbusR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Read address"]
    #[inline(always)]
    pub fn rd_svbus_address(&self) -> RdSvbusAddressR {
        RdSvbusAddressR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Status to indicate if read on serial VBUS is complete, write of any value will clear this bit."]
    #[inline(always)]
    pub fn rd_svbus_done(&self) -> RdSvbusDoneR {
        RdSvbusDoneR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Value written to select the corresponding ECC RAM for control or status"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_vector(&mut self) -> EccVectorW<EccAggrRxmem_Cfg_RegsEccVectorSpec> {
        EccVectorW::new(self, 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Write 1 to trigger a read on the serial VBUS"]
    #[inline(always)]
    #[must_use]
    pub fn rd_svbus(&mut self) -> RdSvbusW<EccAggrRxmem_Cfg_RegsEccVectorSpec> {
        RdSvbusW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Read address"]
    #[inline(always)]
    #[must_use]
    pub fn rd_svbus_address(&mut self) -> RdSvbusAddressW<EccAggrRxmem_Cfg_RegsEccVectorSpec> {
        RdSvbusAddressW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Status to indicate if read on serial VBUS is complete, write of any value will clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn rd_svbus_done(&mut self) -> RdSvbusDoneW<EccAggrRxmem_Cfg_RegsEccVectorSpec> {
        RdSvbusDoneW::new(self, 24)
    }
}
#[doc = "ECC Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_ecc_vector::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_ecc_vector::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccAggrRxmem_Cfg_RegsEccVectorSpec;
impl crate::RegisterSpec for EccAggrRxmem_Cfg_RegsEccVectorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_aggr_rxmem__cfg__regs_ecc_vector::R`](R) reader structure"]
impl crate::Readable for EccAggrRxmem_Cfg_RegsEccVectorSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_aggr_rxmem__cfg__regs_ecc_vector::W`](W) writer structure"]
impl crate::Writable for EccAggrRxmem_Cfg_RegsEccVectorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_AGGR_RXMEM__CFG__REGS_ecc_vector to value 0"]
impl crate::Resettable for EccAggrRxmem_Cfg_RegsEccVectorSpec {
    const RESET_VALUE: u32 = 0;
}
