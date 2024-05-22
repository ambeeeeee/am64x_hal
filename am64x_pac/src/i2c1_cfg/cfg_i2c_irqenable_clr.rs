#[doc = "Register `CFG_I2C_IRQENABLE_CLR` reader"]
pub type R = crate::R<CfgI2cIrqenableClrSpec>;
#[doc = "Register `CFG_I2C_IRQENABLE_CLR` writer"]
pub type W = crate::W<CfgI2cIrqenableClrSpec>;
#[doc = "Field `AL_IE` reader - 0:0\\]
Arbitration lost interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[AL\\]"]
pub type AlIeR = crate::BitReader;
#[doc = "Field `AL_IE` writer - 0:0\\]
Arbitration lost interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[AL\\]"]
pub type AlIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK_IE` reader - 1:1\\]
No acknowledgement interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[NACK\\]"]
pub type NackIeR = crate::BitReader;
#[doc = "Field `NACK_IE` writer - 1:1\\]
No acknowledgement interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[NACK\\]"]
pub type NackIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDY_IE` reader - 2:2\\]
Register access ready interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[ARDY\\]"]
pub type ArdyIeR = crate::BitReader;
#[doc = "Field `ARDY_IE` writer - 2:2\\]
Register access ready interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[ARDY\\]"]
pub type ArdyIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRDY_IE` reader - 3:3\\]
Receive data ready interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[RRDY\\]"]
pub type RrdyIeR = crate::BitReader;
#[doc = "Field `RRDY_IE` writer - 3:3\\]
Receive data ready interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[RRDY\\]"]
pub type RrdyIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XRDY_IE` reader - 4:4\\]
Transmit data ready interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[XRDY\\]"]
pub type XrdyIeR = crate::BitReader;
#[doc = "Field `XRDY_IE` writer - 4:4\\]
Transmit data ready interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[XRDY\\]"]
pub type XrdyIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC_IE` reader - 5:5\\]
General call Interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[GC\\]"]
pub type GcIeR = crate::BitReader;
#[doc = "Field `GC_IE` writer - 5:5\\]
General call Interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[GC\\]"]
pub type GcIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STC_IE` reader - 6:6\\]
Start Condition interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[STC\\]"]
pub type StcIeR = crate::BitReader;
#[doc = "Field `STC_IE` writer - 6:6\\]
Start Condition interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[STC\\]"]
pub type StcIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AERR_IE` reader - 7:7\\]
Access Error interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[AERR\\]"]
pub type AerrIeR = crate::BitReader;
#[doc = "Field `AERR_IE` writer - 7:7\\]
Access Error interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[AERR\\]"]
pub type AerrIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF_IE` reader - 8:8\\]
Bus Free interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[BF\\]"]
pub type BfIeR = crate::BitReader;
#[doc = "Field `BF_IE` writer - 8:8\\]
Bus Free interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[BF\\]"]
pub type BfIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASS_IE` reader - 9:9\\]
Addressed as Slave interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[AAS\\]"]
pub type AssIeR = crate::BitReader;
#[doc = "Field `ASS_IE` writer - 9:9\\]
Addressed as Slave interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[AAS\\]"]
pub type AssIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XUDF` reader - 10:10\\]
Transmit underflow enable clear"]
pub type XudfR = crate::BitReader;
#[doc = "Field `XUDF` writer - 10:10\\]
Transmit underflow enable clear"]
pub type XudfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVR` reader - 11:11\\]
Receive overrun enable clear"]
pub type RovrR = crate::BitReader;
#[doc = "Field `ROVR` writer - 11:11\\]
Receive overrun enable clear"]
pub type RovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDR_IE` reader - 13:13\\]
Receive Draining interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[RDR\\]"]
pub type RdrIeR = crate::BitReader;
#[doc = "Field `RDR_IE` writer - 13:13\\]
Receive Draining interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[RDR\\]"]
pub type RdrIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XDR_IE` reader - 14:14\\]
Transmit Draining interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[XDR\\]"]
pub type XdrIeR = crate::BitReader;
#[doc = "Field `XDR_IE` writer - 14:14\\]
Transmit Draining interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[XDR\\]"]
pub type XdrIeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Arbitration lost interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[AL\\]"]
    #[inline(always)]
    pub fn al_ie(&self) -> AlIeR {
        AlIeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
No acknowledgement interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[NACK\\]"]
    #[inline(always)]
    pub fn nack_ie(&self) -> NackIeR {
        NackIeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Register access ready interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[ARDY\\]"]
    #[inline(always)]
    pub fn ardy_ie(&self) -> ArdyIeR {
        ArdyIeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive data ready interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[RRDY\\]"]
    #[inline(always)]
    pub fn rrdy_ie(&self) -> RrdyIeR {
        RrdyIeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmit data ready interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[XRDY\\]"]
    #[inline(always)]
    pub fn xrdy_ie(&self) -> XrdyIeR {
        XrdyIeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
General call Interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[GC\\]"]
    #[inline(always)]
    pub fn gc_ie(&self) -> GcIeR {
        GcIeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Start Condition interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[STC\\]"]
    #[inline(always)]
    pub fn stc_ie(&self) -> StcIeR {
        StcIeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Access Error interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[AERR\\]"]
    #[inline(always)]
    pub fn aerr_ie(&self) -> AerrIeR {
        AerrIeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Bus Free interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[BF\\]"]
    #[inline(always)]
    pub fn bf_ie(&self) -> BfIeR {
        BfIeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Addressed as Slave interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[AAS\\]"]
    #[inline(always)]
    pub fn ass_ie(&self) -> AssIeR {
        AssIeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Transmit underflow enable clear"]
    #[inline(always)]
    pub fn xudf(&self) -> XudfR {
        XudfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Receive overrun enable clear"]
    #[inline(always)]
    pub fn rovr(&self) -> RovrR {
        RovrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Receive Draining interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[RDR\\]"]
    #[inline(always)]
    pub fn rdr_ie(&self) -> RdrIeR {
        RdrIeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Transmit Draining interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[XDR\\]"]
    #[inline(always)]
    pub fn xdr_ie(&self) -> XdrIeR {
        XdrIeR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Arbitration lost interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[AL\\]"]
    #[inline(always)]
    #[must_use]
    pub fn al_ie(&mut self) -> AlIeW<CfgI2cIrqenableClrSpec> {
        AlIeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
No acknowledgement interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[NACK\\]"]
    #[inline(always)]
    #[must_use]
    pub fn nack_ie(&mut self) -> NackIeW<CfgI2cIrqenableClrSpec> {
        NackIeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Register access ready interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[ARDY\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ardy_ie(&mut self) -> ArdyIeW<CfgI2cIrqenableClrSpec> {
        ArdyIeW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive data ready interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[RRDY\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rrdy_ie(&mut self) -> RrdyIeW<CfgI2cIrqenableClrSpec> {
        RrdyIeW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmit data ready interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[XRDY\\]"]
    #[inline(always)]
    #[must_use]
    pub fn xrdy_ie(&mut self) -> XrdyIeW<CfgI2cIrqenableClrSpec> {
        XrdyIeW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
General call Interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[GC\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gc_ie(&mut self) -> GcIeW<CfgI2cIrqenableClrSpec> {
        GcIeW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Start Condition interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[STC\\]"]
    #[inline(always)]
    #[must_use]
    pub fn stc_ie(&mut self) -> StcIeW<CfgI2cIrqenableClrSpec> {
        StcIeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Access Error interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[AERR\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aerr_ie(&mut self) -> AerrIeW<CfgI2cIrqenableClrSpec> {
        AerrIeW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Bus Free interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[BF\\]"]
    #[inline(always)]
    #[must_use]
    pub fn bf_ie(&mut self) -> BfIeW<CfgI2cIrqenableClrSpec> {
        BfIeW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Addressed as Slave interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[AAS\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ass_ie(&mut self) -> AssIeW<CfgI2cIrqenableClrSpec> {
        AssIeW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Transmit underflow enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn xudf(&mut self) -> XudfW<CfgI2cIrqenableClrSpec> {
        XudfW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Receive overrun enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> RovrW<CfgI2cIrqenableClrSpec> {
        RovrW::new(self, 11)
    }
    #[doc = "Bit 13 - 13:13\\]
Receive Draining interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[RDR\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rdr_ie(&mut self) -> RdrIeW<CfgI2cIrqenableClrSpec> {
        RdrIeW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Transmit Draining interrupt enable clear Mask or unmask the interrupt signaled by bit in I2C_STAT\\[XDR\\]"]
    #[inline(always)]
    #[must_use]
    pub fn xdr_ie(&mut self) -> XdrIeW<CfgI2cIrqenableClrSpec> {
        XdrIeW::new(self, 14)
    }
}
#[doc = "Per-event interrupt clear bit vector.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_irqenable_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_irqenable_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cIrqenableClrSpec;
impl crate::RegisterSpec for CfgI2cIrqenableClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_irqenable_clr::R`](R) reader structure"]
impl crate::Readable for CfgI2cIrqenableClrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_irqenable_clr::W`](W) writer structure"]
impl crate::Writable for CfgI2cIrqenableClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_IRQENABLE_CLR to value 0"]
impl crate::Resettable for CfgI2cIrqenableClrSpec {
    const RESET_VALUE: u32 = 0;
}
