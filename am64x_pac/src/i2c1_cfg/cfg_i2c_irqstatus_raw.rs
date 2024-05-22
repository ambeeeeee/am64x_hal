#[doc = "Register `CFG_I2C_IRQSTATUS_RAW` reader"]
pub type R = crate::R<CfgI2cIrqstatusRawSpec>;
#[doc = "Register `CFG_I2C_IRQSTATUS_RAW` writer"]
pub type W = crate::W<CfgI2cIrqstatusRawSpec>;
#[doc = "Field `AL` reader - 0:0\\]
Arbitration lost IRQ status This bit is automatically set by the hardware when it loses the Arbitration in master transmit mode, an interrupt is signaled to MPUSS During reads, it always returns 0"]
pub type AlR = crate::BitReader;
#[doc = "Field `AL` writer - 0:0\\]
Arbitration lost IRQ status This bit is automatically set by the hardware when it loses the Arbitration in master transmit mode, an interrupt is signaled to MPUSS During reads, it always returns 0"]
pub type AlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - 1:1\\]
No acknowledgement IRQ status Bit is set when No Acknowledge has been received, an interrupt is signaled to MPUSS Write '1' to clear this bit"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - 1:1\\]
No acknowledgement IRQ status Bit is set when No Acknowledge has been received, an interrupt is signaled to MPUSS Write '1' to clear this bit"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDY` reader - 2:2\\]
Register access ready IRQ status When set to '1' it indicates that previous access has been performed and registers are ready to be accessed again An interrupt is signaled to MPUSS Write '1' to clear"]
pub type ArdyR = crate::BitReader;
#[doc = "Field `ARDY` writer - 2:2\\]
Register access ready IRQ status When set to '1' it indicates that previous access has been performed and registers are ready to be accessed again An interrupt is signaled to MPUSS Write '1' to clear"]
pub type ArdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRDY` reader - 3:3\\]
Receive data ready IRQ status Set to '1' by core when receiver mode, a new data is able to be read When set to '1' by core, an interrupt is signaled to MPUSS Write '1' to clear"]
pub type RrdyR = crate::BitReader;
#[doc = "Field `RRDY` writer - 3:3\\]
Receive data ready IRQ status Set to '1' by core when receiver mode, a new data is able to be read When set to '1' by core, an interrupt is signaled to MPUSS Write '1' to clear"]
pub type RrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XRDY` reader - 4:4\\]
Transmit data ready IRQ status Set to '1' by core when transmitter and when new data is requested When set to '1' by core, an interrupt is signaled to MPUSS Write '1' to clear"]
pub type XrdyR = crate::BitReader;
#[doc = "Field `XRDY` writer - 4:4\\]
Transmit data ready IRQ status Set to '1' by core when transmitter and when new data is requested When set to '1' by core, an interrupt is signaled to MPUSS Write '1' to clear"]
pub type XrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC` reader - 5:5\\]
General call IRQ status Set to '1' by core when General call address detected and interrupt signaled to MPUSS Write '1' to clear"]
pub type GcR = crate::BitReader;
#[doc = "Field `GC` writer - 5:5\\]
General call IRQ status Set to '1' by core when General call address detected and interrupt signaled to MPUSS Write '1' to clear"]
pub type GcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STC` reader - 6:6\\]
Start Condition IRQ status"]
pub type StcR = crate::BitReader;
#[doc = "Field `STC` writer - 6:6\\]
Start Condition IRQ status"]
pub type StcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AERR` reader - 7:7\\]
Access Error IRQ status"]
pub type AerrR = crate::BitReader;
#[doc = "Field `AERR` writer - 7:7\\]
Access Error IRQ status"]
pub type AerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF` reader - 8:8\\]
Bus Free IRQ status"]
pub type BfR = crate::BitReader;
#[doc = "Field `BF` writer - 8:8\\]
Bus Free IRQ status"]
pub type BfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAS` reader - 9:9\\]
Address recognized as slave IRQ status"]
pub type AasR = crate::BitReader;
#[doc = "Field `AAS` writer - 9:9\\]
Address recognized as slave IRQ status"]
pub type AasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XUDF` reader - 10:10\\]
Transmit underflow statusWriting into this bit has no effect"]
pub type XudfR = crate::BitReader;
#[doc = "Field `XUDF` writer - 10:10\\]
Transmit underflow statusWriting into this bit has no effect"]
pub type XudfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVR` reader - 11:11\\]
Receive overrun statusWriting into this bit has no effect"]
pub type RovrR = crate::BitReader;
#[doc = "Field `ROVR` writer - 11:11\\]
Receive overrun statusWriting into this bit has no effect"]
pub type RovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BB` reader - 12:12\\]
Bus busy statusWriting into this bit has no effect"]
pub type BbR = crate::BitReader;
#[doc = "Field `BB` writer - 12:12\\]
Bus busy statusWriting into this bit has no effect"]
pub type BbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDR` reader - 13:13\\]
Receive draining IRQ status"]
pub type RdrR = crate::BitReader;
#[doc = "Field `RDR` writer - 13:13\\]
Receive draining IRQ status"]
pub type RdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XDR` reader - 14:14\\]
Transmit draining IRQ status"]
pub type XdrR = crate::BitReader;
#[doc = "Field `XDR` writer - 14:14\\]
Transmit draining IRQ status"]
pub type XdrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Arbitration lost IRQ status This bit is automatically set by the hardware when it loses the Arbitration in master transmit mode, an interrupt is signaled to MPUSS During reads, it always returns 0"]
    #[inline(always)]
    pub fn al(&self) -> AlR {
        AlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
No acknowledgement IRQ status Bit is set when No Acknowledge has been received, an interrupt is signaled to MPUSS Write '1' to clear this bit"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Register access ready IRQ status When set to '1' it indicates that previous access has been performed and registers are ready to be accessed again An interrupt is signaled to MPUSS Write '1' to clear"]
    #[inline(always)]
    pub fn ardy(&self) -> ArdyR {
        ArdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive data ready IRQ status Set to '1' by core when receiver mode, a new data is able to be read When set to '1' by core, an interrupt is signaled to MPUSS Write '1' to clear"]
    #[inline(always)]
    pub fn rrdy(&self) -> RrdyR {
        RrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmit data ready IRQ status Set to '1' by core when transmitter and when new data is requested When set to '1' by core, an interrupt is signaled to MPUSS Write '1' to clear"]
    #[inline(always)]
    pub fn xrdy(&self) -> XrdyR {
        XrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
General call IRQ status Set to '1' by core when General call address detected and interrupt signaled to MPUSS Write '1' to clear"]
    #[inline(always)]
    pub fn gc(&self) -> GcR {
        GcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Start Condition IRQ status"]
    #[inline(always)]
    pub fn stc(&self) -> StcR {
        StcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Access Error IRQ status"]
    #[inline(always)]
    pub fn aerr(&self) -> AerrR {
        AerrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Bus Free IRQ status"]
    #[inline(always)]
    pub fn bf(&self) -> BfR {
        BfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Address recognized as slave IRQ status"]
    #[inline(always)]
    pub fn aas(&self) -> AasR {
        AasR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Transmit underflow statusWriting into this bit has no effect"]
    #[inline(always)]
    pub fn xudf(&self) -> XudfR {
        XudfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Receive overrun statusWriting into this bit has no effect"]
    #[inline(always)]
    pub fn rovr(&self) -> RovrR {
        RovrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Bus busy statusWriting into this bit has no effect"]
    #[inline(always)]
    pub fn bb(&self) -> BbR {
        BbR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Receive draining IRQ status"]
    #[inline(always)]
    pub fn rdr(&self) -> RdrR {
        RdrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Transmit draining IRQ status"]
    #[inline(always)]
    pub fn xdr(&self) -> XdrR {
        XdrR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Arbitration lost IRQ status This bit is automatically set by the hardware when it loses the Arbitration in master transmit mode, an interrupt is signaled to MPUSS During reads, it always returns 0"]
    #[inline(always)]
    #[must_use]
    pub fn al(&mut self) -> AlW<CfgI2cIrqstatusRawSpec> {
        AlW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
No acknowledgement IRQ status Bit is set when No Acknowledge has been received, an interrupt is signaled to MPUSS Write '1' to clear this bit"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<CfgI2cIrqstatusRawSpec> {
        NackW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Register access ready IRQ status When set to '1' it indicates that previous access has been performed and registers are ready to be accessed again An interrupt is signaled to MPUSS Write '1' to clear"]
    #[inline(always)]
    #[must_use]
    pub fn ardy(&mut self) -> ArdyW<CfgI2cIrqstatusRawSpec> {
        ArdyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive data ready IRQ status Set to '1' by core when receiver mode, a new data is able to be read When set to '1' by core, an interrupt is signaled to MPUSS Write '1' to clear"]
    #[inline(always)]
    #[must_use]
    pub fn rrdy(&mut self) -> RrdyW<CfgI2cIrqstatusRawSpec> {
        RrdyW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmit data ready IRQ status Set to '1' by core when transmitter and when new data is requested When set to '1' by core, an interrupt is signaled to MPUSS Write '1' to clear"]
    #[inline(always)]
    #[must_use]
    pub fn xrdy(&mut self) -> XrdyW<CfgI2cIrqstatusRawSpec> {
        XrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
General call IRQ status Set to '1' by core when General call address detected and interrupt signaled to MPUSS Write '1' to clear"]
    #[inline(always)]
    #[must_use]
    pub fn gc(&mut self) -> GcW<CfgI2cIrqstatusRawSpec> {
        GcW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Start Condition IRQ status"]
    #[inline(always)]
    #[must_use]
    pub fn stc(&mut self) -> StcW<CfgI2cIrqstatusRawSpec> {
        StcW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Access Error IRQ status"]
    #[inline(always)]
    #[must_use]
    pub fn aerr(&mut self) -> AerrW<CfgI2cIrqstatusRawSpec> {
        AerrW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Bus Free IRQ status"]
    #[inline(always)]
    #[must_use]
    pub fn bf(&mut self) -> BfW<CfgI2cIrqstatusRawSpec> {
        BfW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Address recognized as slave IRQ status"]
    #[inline(always)]
    #[must_use]
    pub fn aas(&mut self) -> AasW<CfgI2cIrqstatusRawSpec> {
        AasW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Transmit underflow statusWriting into this bit has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn xudf(&mut self) -> XudfW<CfgI2cIrqstatusRawSpec> {
        XudfW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Receive overrun statusWriting into this bit has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> RovrW<CfgI2cIrqstatusRawSpec> {
        RovrW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Bus busy statusWriting into this bit has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn bb(&mut self) -> BbW<CfgI2cIrqstatusRawSpec> {
        BbW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Receive draining IRQ status"]
    #[inline(always)]
    #[must_use]
    pub fn rdr(&mut self) -> RdrW<CfgI2cIrqstatusRawSpec> {
        RdrW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Transmit draining IRQ status"]
    #[inline(always)]
    #[must_use]
    pub fn xdr(&mut self) -> XdrW<CfgI2cIrqstatusRawSpec> {
        XdrW::new(self, 14)
    }
}
#[doc = "Per-event raw interrupt status vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_irqstatus_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_irqstatus_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cIrqstatusRawSpec;
impl crate::RegisterSpec for CfgI2cIrqstatusRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_irqstatus_raw::R`](R) reader structure"]
impl crate::Readable for CfgI2cIrqstatusRawSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_irqstatus_raw::W`](W) writer structure"]
impl crate::Writable for CfgI2cIrqstatusRawSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_IRQSTATUS_RAW to value 0"]
impl crate::Resettable for CfgI2cIrqstatusRawSpec {
    const RESET_VALUE: u32 = 0;
}
