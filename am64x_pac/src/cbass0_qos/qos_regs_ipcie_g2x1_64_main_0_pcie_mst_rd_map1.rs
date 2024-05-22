#[doc = "Register `QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map1` reader"]
pub type R = crate::R<QosRegsIpcieG2x1_64Main0PcieMstRdMap1Spec>;
#[doc = "Register `QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map1` writer"]
pub type W = crate::W<QosRegsIpcieG2x1_64Main0PcieMstRdMap1Spec>;
#[doc = "Field `QOS` reader - 2:0\\]
qos signal for channel N."]
pub type QosR = crate::FieldReader;
#[doc = "Field `QOS` writer - 2:0\\]
qos signal for channel N."]
pub type QosW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ORDERID` reader - 7:4\\]
orderid signal for channel N."]
pub type OrderidR = crate::FieldReader;
#[doc = "Field `ORDERID` writer - 7:4\\]
orderid signal for channel N."]
pub type OrderidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ASEL` reader - 11:8\\]
asel signal for channel N. 0 = SOC address. 1-15 = peripheral address."]
pub type AselR = crate::FieldReader;
#[doc = "Field `ASEL` writer - 11:8\\]
asel signal for channel N. 0 = SOC address. 1-15 = peripheral address."]
pub type AselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPRIORITY` reader - 14:12\\]
epriority signal for channel N."]
pub type EpriorityR = crate::FieldReader;
#[doc = "Field `EPRIORITY` writer - 14:12\\]
epriority signal for channel N."]
pub type EpriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
qos signal for channel N."]
    #[inline(always)]
    pub fn qos(&self) -> QosR {
        QosR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
orderid signal for channel N."]
    #[inline(always)]
    pub fn orderid(&self) -> OrderidR {
        OrderidR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
asel signal for channel N. 0 = SOC address. 1-15 = peripheral address."]
    #[inline(always)]
    pub fn asel(&self) -> AselR {
        AselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
epriority signal for channel N."]
    #[inline(always)]
    pub fn epriority(&self) -> EpriorityR {
        EpriorityR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
qos signal for channel N."]
    #[inline(always)]
    #[must_use]
    pub fn qos(&mut self) -> QosW<QosRegsIpcieG2x1_64Main0PcieMstRdMap1Spec> {
        QosW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
orderid signal for channel N."]
    #[inline(always)]
    #[must_use]
    pub fn orderid(&mut self) -> OrderidW<QosRegsIpcieG2x1_64Main0PcieMstRdMap1Spec> {
        OrderidW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
asel signal for channel N. 0 = SOC address. 1-15 = peripheral address."]
    #[inline(always)]
    #[must_use]
    pub fn asel(&mut self) -> AselW<QosRegsIpcieG2x1_64Main0PcieMstRdMap1Spec> {
        AselW::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
epriority signal for channel N."]
    #[inline(always)]
    #[must_use]
    pub fn epriority(&mut self) -> EpriorityW<QosRegsIpcieG2x1_64Main0PcieMstRdMap1Spec> {
        EpriorityW::new(self, 12)
    }
}
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QosRegsIpcieG2x1_64Main0PcieMstRdMap1Spec;
impl crate::RegisterSpec for QosRegsIpcieG2x1_64Main0PcieMstRdMap1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map1::R`](R) reader structure"]
impl crate::Readable for QosRegsIpcieG2x1_64Main0PcieMstRdMap1Spec {}
#[doc = "`write(|w| ..)` method takes [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map1::W`](W) writer structure"]
impl crate::Writable for QosRegsIpcieG2x1_64Main0PcieMstRdMap1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map1 to value 0x7000"]
impl crate::Resettable for QosRegsIpcieG2x1_64Main0PcieMstRdMap1Spec {
    const RESET_VALUE: u32 = 0x7000;
}
