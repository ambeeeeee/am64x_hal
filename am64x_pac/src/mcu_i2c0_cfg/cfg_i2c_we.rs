#[doc = "Register `CFG_I2C_WE` reader"]
pub type R = crate::R<CfgI2cWeSpec>;
#[doc = "Register `CFG_I2C_WE` writer"]
pub type W = crate::W<CfgI2cWeSpec>;
#[doc = "Field `AL` reader - 0:0\\]
Arbitration lost IRQ wakeup set"]
pub type AlR = crate::BitReader;
#[doc = "Field `AL` writer - 0:0\\]
Arbitration lost IRQ wakeup set"]
pub type AlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - 1:1\\]
No acknowledgment IRQ wakeup set"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - 1:1\\]
No acknowledgment IRQ wakeup set"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDY` reader - 2:2\\]
Register access ready IRQ wakeup set"]
pub type ArdyR = crate::BitReader;
#[doc = "Field `ARDY` writer - 2:2\\]
Register access ready IRQ wakeup set"]
pub type ArdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRDY` reader - 3:3\\]
Receive/Transmit data ready IRQ wakeup set"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `DRDY` writer - 3:3\\]
Receive/Transmit data ready IRQ wakeup set"]
pub type DrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC` reader - 5:5\\]
General call IRQ wakeup set"]
pub type GcR = crate::BitReader;
#[doc = "Field `GC` writer - 5:5\\]
General call IRQ wakeup set"]
pub type GcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STC` reader - 6:6\\]
Start Condition IRQ wakeup set"]
pub type StcR = crate::BitReader;
#[doc = "Field `STC` writer - 6:6\\]
Start Condition IRQ wakeup set"]
pub type StcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF` reader - 8:8\\]
Bus Free IRQ wakeup set"]
pub type BfR = crate::BitReader;
#[doc = "Field `BF` writer - 8:8\\]
Bus Free IRQ wakeup set"]
pub type BfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAS` reader - 9:9\\]
Address as slave IRQ wakeup set"]
pub type AasR = crate::BitReader;
#[doc = "Field `AAS` writer - 9:9\\]
Address as slave IRQ wakeup set"]
pub type AasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XUDF` reader - 10:10\\]
Transmit underflow wakeup set"]
pub type XudfR = crate::BitReader;
#[doc = "Field `XUDF` writer - 10:10\\]
Transmit underflow wakeup set"]
pub type XudfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVR` reader - 11:11\\]
Receive overrun wakeup set"]
pub type RovrR = crate::BitReader;
#[doc = "Field `ROVR` writer - 11:11\\]
Receive overrun wakeup set"]
pub type RovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDR` reader - 13:13\\]
Receive Draining wakeup set"]
pub type RdrR = crate::BitReader;
#[doc = "Field `RDR` writer - 13:13\\]
Receive Draining wakeup set"]
pub type RdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XDR` reader - 14:14\\]
Transmit Draining wakeup set"]
pub type XdrR = crate::BitReader;
#[doc = "Field `XDR` writer - 14:14\\]
Transmit Draining wakeup set"]
pub type XdrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Arbitration lost IRQ wakeup set"]
    #[inline(always)]
    pub fn al(&self) -> AlR {
        AlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
No acknowledgment IRQ wakeup set"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Register access ready IRQ wakeup set"]
    #[inline(always)]
    pub fn ardy(&self) -> ArdyR {
        ArdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive/Transmit data ready IRQ wakeup set"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
General call IRQ wakeup set"]
    #[inline(always)]
    pub fn gc(&self) -> GcR {
        GcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Start Condition IRQ wakeup set"]
    #[inline(always)]
    pub fn stc(&self) -> StcR {
        StcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Bus Free IRQ wakeup set"]
    #[inline(always)]
    pub fn bf(&self) -> BfR {
        BfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Address as slave IRQ wakeup set"]
    #[inline(always)]
    pub fn aas(&self) -> AasR {
        AasR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Transmit underflow wakeup set"]
    #[inline(always)]
    pub fn xudf(&self) -> XudfR {
        XudfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Receive overrun wakeup set"]
    #[inline(always)]
    pub fn rovr(&self) -> RovrR {
        RovrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Receive Draining wakeup set"]
    #[inline(always)]
    pub fn rdr(&self) -> RdrR {
        RdrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Transmit Draining wakeup set"]
    #[inline(always)]
    pub fn xdr(&self) -> XdrR {
        XdrR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Arbitration lost IRQ wakeup set"]
    #[inline(always)]
    #[must_use]
    pub fn al(&mut self) -> AlW<CfgI2cWeSpec> {
        AlW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
No acknowledgment IRQ wakeup set"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<CfgI2cWeSpec> {
        NackW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Register access ready IRQ wakeup set"]
    #[inline(always)]
    #[must_use]
    pub fn ardy(&mut self) -> ArdyW<CfgI2cWeSpec> {
        ArdyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive/Transmit data ready IRQ wakeup set"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DrdyW<CfgI2cWeSpec> {
        DrdyW::new(self, 3)
    }
    #[doc = "Bit 5 - 5:5\\]
General call IRQ wakeup set"]
    #[inline(always)]
    #[must_use]
    pub fn gc(&mut self) -> GcW<CfgI2cWeSpec> {
        GcW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Start Condition IRQ wakeup set"]
    #[inline(always)]
    #[must_use]
    pub fn stc(&mut self) -> StcW<CfgI2cWeSpec> {
        StcW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Bus Free IRQ wakeup set"]
    #[inline(always)]
    #[must_use]
    pub fn bf(&mut self) -> BfW<CfgI2cWeSpec> {
        BfW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Address as slave IRQ wakeup set"]
    #[inline(always)]
    #[must_use]
    pub fn aas(&mut self) -> AasW<CfgI2cWeSpec> {
        AasW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Transmit underflow wakeup set"]
    #[inline(always)]
    #[must_use]
    pub fn xudf(&mut self) -> XudfW<CfgI2cWeSpec> {
        XudfW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Receive overrun wakeup set"]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> RovrW<CfgI2cWeSpec> {
        RovrW::new(self, 11)
    }
    #[doc = "Bit 13 - 13:13\\]
Receive Draining wakeup set"]
    #[inline(always)]
    #[must_use]
    pub fn rdr(&mut self) -> RdrW<CfgI2cWeSpec> {
        RdrW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Transmit Draining wakeup set"]
    #[inline(always)]
    #[must_use]
    pub fn xdr(&mut self) -> XdrW<CfgI2cWeSpec> {
        XdrW::new(self, 14)
    }
}
#[doc = "I2C wakeup enable vector (legacy).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_we::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_we::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cWeSpec;
impl crate::RegisterSpec for CfgI2cWeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_we::R`](R) reader structure"]
impl crate::Readable for CfgI2cWeSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_we::W`](W) writer structure"]
impl crate::Writable for CfgI2cWeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_WE to value 0"]
impl crate::Resettable for CfgI2cWeSpec {
    const RESET_VALUE: u32 = 0;
}
